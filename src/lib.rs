#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::path::Path;

use bincode::config::{BigEndian, Configuration, Fixint, LittleEndian};
use bincode::error::{DecodeError, EncodeError};
use bincode::{decode_from_slice, encode_into_slice};
use byteorder::{BigEndian as Big, ByteOrder, LittleEndian as Little};

use crate::binary::SacBinary;
pub use crate::enums::SacFileType;
use crate::error::SacError;
pub use crate::header::SacHeader;
pub use crate::sac::Sac;

mod binary;
mod enums;
pub mod error;
mod header;
mod sac;

#[derive(Copy, Clone)]
pub enum Endian {
    Little,
    Big,
}

const SAC_HEADER_SIZE: usize = 632;
const SAC_HEADER_MAJOR_VERSION: i32 = 6;

const LITTLE_ENDIAN_CONFIG: Configuration<LittleEndian, Fixint> = bincode::config::standard()
    .with_little_endian()
    .with_fixed_int_encoding();
const BIG_ENDIAN_CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

impl SacBinary {
    #[inline]
    fn decode_header(src: &[u8], endian: Endian) -> Result<SacBinary, DecodeError> {
        let decode: (SacBinary, usize) = match endian {
            Endian::Little => decode_from_slice(src, LITTLE_ENDIAN_CONFIG),
            Endian::Big => decode_from_slice(src, BIG_ENDIAN_CONFIG),
        }?;

        Ok(decode.0)
    }

    #[inline]
    fn encode_header(val: SacBinary, dst: &mut [u8], endian: Endian) -> Result<usize, EncodeError> {
        match endian {
            Endian::Little => encode_into_slice(val, dst, LITTLE_ENDIAN_CONFIG),
            Endian::Big => encode_into_slice(val, dst, BIG_ENDIAN_CONFIG),
        }
    }

    #[inline]
    fn decode_data(src: &[u8], endian: Endian) -> Vec<f32> {
        let read_f32 = match endian {
            Endian::Little => Little::read_f32,
            Endian::Big => Big::read_f32,
        };

        src.chunks_exact(4).map(|b| read_f32(b)).collect()
    }

    #[inline]
    fn encode_data(val: &Vec<f32>, endian: Endian) -> Vec<u8> {
        let write_f32 = match endian {
            Endian::Little => Little::write_f32,
            Endian::Big => Big::write_f32,
        };

        val.iter()
            .flat_map(|v| {
                let mut byte = [0; 4];
                write_f32(&mut byte, *v);
                byte
            })
            .collect()
    }
}

macro_rules! check_header {
    ($self:ident) => {
        if $self.nvhdr != SAC_HEADER_MAJOR_VERSION {
            let msg = format!("Unsupported major version (nvhdr = {})", $self.nvhdr);
            return Err(SacError::custom(msg));
        }

        match $self.iftype {
            SacFileType::Unknown(v) => {
                let msg = format!("Unsupported file type (iftype = {})", v);
                return Err(SacError::custom(msg));
            }
            _ => {}
        }
    };
}

impl Sac {
    pub fn set_header(&mut self, h: SacHeader) {
        self.h = h
    }

    pub unsafe fn from_slice_unchecked(src: &[u8], endian: Endian) -> error::Result<Sac> {
        let mut h_src = Vec::new();
        let mut d_src = Vec::new();

        if src.len() > SAC_HEADER_SIZE {
            h_src.extend_from_slice(&src[..SAC_HEADER_SIZE]);
            d_src.extend_from_slice(&src[SAC_HEADER_SIZE..]);
        } else {
            h_src.extend_from_slice(src);
        };

        let binary = match SacBinary::decode_header(&h_src, endian) {
            Ok(b) => b,
            Err(err) => return Err(SacError::custom(err)),
        };

        let mut sac = Sac::build(&binary);

        let data = SacBinary::decode_data(&d_src, endian);
        if sac.iftype == SacFileType::Time && sac.leven {
            sac.first = data;
            return Ok(sac);
        }

        let size = usize::try_from(sac.npts).unwrap_or(data.len());
        if size > data.len() {
            sac.first = data
        } else {
            sac.first = data[..size].to_vec();
            sac.second = data[size..].to_vec();
        }

        Ok(sac)
    }

    pub fn from_slice(src: &[u8], endian: Endian) -> error::Result<Sac> {
        let sac = unsafe { Self::from_slice_unchecked(src, endian) }?;
        check_header!(sac);
        Ok(sac)
    }

    pub unsafe fn to_slice_unchecked(&self, endian: Endian) -> error::Result<Vec<u8>> {
        let mut h_val = [0; SAC_HEADER_SIZE];

        let header = SacBinary::from(self);
        match SacBinary::encode_header(header, &mut h_val, endian) {
            Ok(v) => v,
            Err(err) => return Err(SacError::custom(err)),
        };

        let mut data = self.first.clone();
        data.extend_from_slice(&self.second);
        let d_val = SacBinary::encode_data(&data, endian);

        let mut val = h_val.to_vec();
        val.extend_from_slice(&d_val);

        Ok(val)
    }

    pub fn to_slice(&self, endian: Endian) -> error::Result<Vec<u8>> {
        check_header!(self);
        unsafe { self.to_slice_unchecked(endian) }
    }
}

#[cfg(feature = "std")]
impl Sac {
    pub fn from_file(path: &Path, endian: Endian) -> error::Result<Sac> {
        use std::fs::File;
        use std::io::Read;

        let mut f = match File::open(path) {
            Ok(f) => f,
            Err(err) => return Err(SacError::custom(err)),
        };

        let mut src = Vec::new();
        match f.read_to_end(&mut src) {
            Ok(v) => v,
            Err(err) => return Err(SacError::custom(err)),
        };

        Self::from_slice(&src, endian)
    }

    pub fn to_file(&self, path: &Path, endian: Endian) -> error::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut f = match File::create(path) {
            Ok(v) => v,
            Err(err) => return Err(SacError::custom(err)),
        };

        let val = self.to_slice(endian)?;
        match f.write_all(&val) {
            Ok(v) => v,
            Err(err) => return Err(SacError::custom(err)),
        };

        Ok(())
    }
}
