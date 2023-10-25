use std::fs::File;
use std::io::{Read, Write};
use std::mem::size_of;
use std::path::Path;

use bincode::config::{BigEndian, Configuration, Fixint, LittleEndian};
use bincode::error::{DecodeError, EncodeError};
use bincode::{decode_from_slice, encode_into_slice};
use byteorder::{BigEndian as Big, ByteOrder, LittleEndian as Little};

use crate::binary::SacBinary;
pub use crate::enums::SacFileType;
pub use crate::error::SacError;
pub use crate::header::SacHeader;
pub use crate::sac::Sac;

mod binary;
mod enums;
mod error;
mod header;
mod sac;

#[derive(Debug, Copy, Clone)]
pub enum Endian {
    Little,
    Big,
}

const SAC_HEADER_SIZE: usize = size_of::<SacBinary>();
const SAC_HEADER_MAJOR_VERSION: i32 = 6;

const LITTLE_ENDIAN_CONFIG: Configuration<LittleEndian, Fixint> = bincode::config::standard()
    .with_little_endian()
    .with_fixed_int_encoding();
const BIG_ENDIAN_CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

impl SacBinary {
    fn decode_header(src: &[u8], endian: Endian) -> Result<SacBinary, DecodeError> {
        let decode: (SacBinary, usize) = match endian {
            Endian::Little => decode_from_slice(src, LITTLE_ENDIAN_CONFIG),
            Endian::Big => decode_from_slice(src, BIG_ENDIAN_CONFIG),
        }?;

        Ok(decode.0)
    }

    fn encode_header(val: SacBinary, dst: &mut [u8], endian: Endian) -> Result<usize, EncodeError> {
        match endian {
            Endian::Little => encode_into_slice(val, dst, LITTLE_ENDIAN_CONFIG),
            Endian::Big => encode_into_slice(val, dst, BIG_ENDIAN_CONFIG),
        }
    }

    fn decode_data(src: &[u8], endian: Endian) -> Vec<f32> {
        let read_f32 = match endian {
            Endian::Little => Little::read_f32,
            Endian::Big => Big::read_f32,
        };

        src.chunks(4).map(|b| read_f32(b)).collect()
    }

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
    pub(crate) fn check_header(&self) -> Result<(), SacError> {
        if self.nvhdr != SAC_HEADER_MAJOR_VERSION {
            let msg = format!("Unsupported header version: {}", self.nvhdr);
            return Err(SacError::Unsupported(msg));
        }

        match self.iftype {
            SacFileType::XYZ => {
                let msg = format!("Unsupported file type: {:?}", self.iftype);
                Err(SacError::Unsupported(msg))
            }
            SacFileType::Unknown(_) => {
                let msg = format!("Unknown file type: {:?}", self.iftype);
                Err(SacError::Unsupported(msg))
            }
            _ => Ok(()),
        }
    }

    pub fn read(path: &Path, endian: Endian) -> Result<SacHeader, SacError> {
        let sac = Sac::read_header(path, endian)?;
        Ok(sac.h)
    }
}

impl Sac {
    pub(crate) fn read_in(p: &Path, e: Endian, only_h: bool) -> Result<Sac, SacError> {
        let mut f = match File::open(p) {
            Ok(f) => f,
            Err(e) => return Err(SacError::IO(e.to_string())),
        };

        let mut f_buf = Vec::new();
        match f.read_to_end(&mut f_buf) {
            Ok(v) => v,
            Err(e) => return Err(SacError::IO(e.to_string())),
        };

        let h_buf = &f_buf[..SAC_HEADER_SIZE];
        let d_buf = &f_buf[SAC_HEADER_SIZE..];

        let binary = match SacBinary::decode_header(h_buf, e) {
            Ok(b) => b,
            Err(e) => return Err(SacError::Unsupported(e.to_string())),
        };

        let mut sac = Sac::build(&binary, p, e);
        sac.check_header()?;

        if only_h {
            return Ok(sac);
        }

        let data = SacBinary::decode_data(d_buf, e);
        if sac.iftype == SacFileType::Time && sac.leven {
            sac.y = data;
            return Ok(sac);
        }

        let size = sac.npts as usize;
        sac.x = data[..size].to_vec();
        sac.y = data[size..].to_vec();
        Ok(sac)
    }

    pub(crate) fn write_out(&self, p: &Path, e: Endian, only_h: bool) -> Result<(), SacError> {
        self.check_header()?;
        let header = SacBinary::from(self);
        let mut h_buf = [0; SAC_HEADER_SIZE];

        match SacBinary::encode_header(header, &mut h_buf, e) {
            Ok(v) => v,
            Err(e) => return Err(SacError::Unsupported(e.to_string())),
        };

        let d_buf = if only_h {
            let mut f = match File::open(p) {
                Ok(f) => f,
                Err(e) => return Err(SacError::IO(e.to_string())),
            };

            let mut f_buf = Vec::new();
            match f.read_to_end(&mut f_buf) {
                Ok(v) => v,
                Err(e) => return Err(SacError::IO(e.to_string())),
            };

            f_buf[SAC_HEADER_SIZE..].to_vec()
        } else {
            let mut data = self.x.clone();
            data.extend_from_slice(&self.y);

            SacBinary::encode_data(&data, e)
        };

        let mut f_buf = h_buf.to_vec();
        f_buf.extend_from_slice(&d_buf);

        let mut f = match File::create(p) {
            Ok(v) => v,
            Err(e) => return Err(SacError::IO(e.to_string())),
        };

        match f.write_all(&f_buf) {
            Ok(v) => v,
            Err(e) => return Err(SacError::IO(e.to_string())),
        };

        Ok(())
    }

    pub fn set_header(&mut self, h: SacHeader) {
        self.h = h
    }

    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian
    }

    pub fn read_header(path: &Path, endian: Endian) -> Result<Sac, SacError> {
        Sac::read_in(path, endian, true)
    }

    pub fn write_header(&self) -> Result<(), SacError> {
        let path = Path::new(&self.path);
        self.write_out(path, self.endian, true)
    }

    pub fn read(path: &Path, endian: Endian) -> Result<Sac, SacError> {
        Sac::read_in(path, endian, false)
    }

    pub fn write(&self) -> Result<(), SacError> {
        let path = Path::new(&self.path);
        self.write_out(path, self.endian, false)
    }

    pub fn write_to(&self, path: &Path) -> Result<(), SacError> {
        self.write_out(path, self.endian, false)
    }
}
