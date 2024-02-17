#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::path::Path;

use bincode::config::{BigEndian, Configuration, Fixint, LittleEndian};
use bincode::error::{DecodeError, EncodeError};
use bincode::{decode_from_slice, encode_into_slice};
use byteorder::{BigEndian as Big, ByteOrder, LittleEndian as Little};

use crate::binary::SacBinary;
pub use crate::enums::SacFileType;
pub use crate::header::SacHeader;
pub use crate::sac::Sac;

mod binary;
mod enums;
mod header;
mod sac;

#[derive(Debug, Copy, Clone)]
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

        src.chunks(4).map(|b| read_f32(b)).collect()
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

impl SacHeader {
    #[inline]
    fn check_header(&self) -> Option<()> {
        if self.nvhdr != SAC_HEADER_MAJOR_VERSION {
            return None;
        }

        match self.iftype {
            SacFileType::Unknown(_) => None,
            _ => Some(()),
        }
    }
}

impl Sac {
    pub fn set_header(&mut self, h: SacHeader) {
        self.h = h
    }

    pub fn from_slice(src: &[u8], endian: Endian) -> Option<Sac> {
        if src.len() < SAC_HEADER_SIZE {
            return None;
        }

        let h_src = &src[..SAC_HEADER_SIZE];
        let d_src = &src[SAC_HEADER_SIZE..];

        let binary = match SacBinary::decode_header(h_src, endian) {
            Ok(b) => b,
            Err(_) => return None,
        };

        let mut sac = Sac::build(&binary);
        sac.check_header()?;

        let data = SacBinary::decode_data(d_src, endian);
        if sac.iftype == SacFileType::Time && sac.leven {
            sac.first = data;
            return Some(sac);
        }

        let size = sac.npts as usize;
        sac.first = data[..size].to_vec();
        sac.second = data[size..].to_vec();
        Some(sac)
    }

    pub fn to_slice(&self, endian: Endian) -> Option<Vec<u8>> {
        self.check_header()?;
        let mut h_val = [0; SAC_HEADER_SIZE];

        let header = SacBinary::from(self);
        match SacBinary::encode_header(header, &mut h_val, endian) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let mut data = self.first.clone();
        data.extend_from_slice(&self.second);
        let d_val = SacBinary::encode_data(&data, endian);

        let mut val = h_val.to_vec();
        val.extend_from_slice(&d_val);

        Some(val)
    }

    #[cfg(feature = "std")]
    pub fn from_file(path: &Path, endian: Endian) -> Option<Sac> {
        use std::fs::File;
        use std::io::Read;

        let mut f = match File::open(path) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut src = Vec::new();
        match f.read_to_end(&mut src) {
            Ok(v) => v,
            Err(_) => return None,
        };

        Sac::from_slice(&src, endian)
    }

    #[cfg(feature = "std")]
    pub fn to_file(&self, path: &Path, endian: Endian) -> Option<()> {
        use std::fs::File;
        use std::io::Write;

        let mut f = match File::create(path) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let val = self.to_slice(endian)?;
        match f.write_all(&val) {
            Ok(v) => v,
            Err(_) => return None,
        };

        Some(())
    }
}
