use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::mem::size_of;
use std::path::Path;

use bincode::{decode_from_slice, encode_into_slice};
use bincode::config::{BigEndian, Configuration, Fixint, LittleEndian};
use bincode::error::{DecodeError, EncodeError};
use byteorder::{BigEndian as Big, ByteOrder, LittleEndian as Little};

use crate::binary::SacBinary;
use crate::enums::SacFileType;
pub use crate::header::SacHeader;
pub use crate::sac::Sac;

mod binary;
mod sac;
mod enums;
mod header;

#[derive(Debug, Copy, Clone)]
pub enum Endian {
    Little,
    Big,
}

const SAC_HEADER_SIZE : usize = size_of::<SacBinary>();
const SAC_HEADER_MAJOR_VERSION: i32 = 6;

const LITTLE_ENDIAN_CONFIG: Configuration<LittleEndian, Fixint> =
    bincode::config::standard().with_little_endian().with_fixed_int_encoding();
const BIG_ENDIAN_CONFIG: Configuration<BigEndian, Fixint> =
    bincode::config::standard().with_big_endian().with_fixed_int_encoding();

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

        src.chunks(4)
            .map(|b|read_f32(b))
            .collect()
    }

    fn encode_data(val: &Vec<f32>, endian: Endian) -> Vec<u8> {
        let write_f32 = match endian {
            Endian::Little => Little::write_f32,
            Endian::Big => Big::write_f32,
        };

        val.iter()
            .flat_map(|v|{
                let mut byte = [0; 4];
                write_f32(&mut byte, *v);
                byte
            })
            .collect()
    }
}

impl SacHeader {
    pub(crate) fn check(&self) -> Result<(), io::Error> {
        if self.nvhdr != SAC_HEADER_MAJOR_VERSION {
            let msg = format!("Unsupported Sac Header Version, {}", self.nvhdr);
            let err = io::Error::new(io::ErrorKind::Unsupported, msg);
            return Err(err)
        }

        if self.iftype == SacFileType::XYZ {
            let msg = format!("Unsupported Sac File Type: {}", self.iftype as i32);
            let err = io::Error::new(io::ErrorKind::Unsupported, msg);
            return Err(err)
        }

        Ok(())
    }

    pub fn read(path: &Path, endian: Endian) -> Result<SacHeader, Box<dyn Error>> {
        let sac = Sac::read_header(path, endian)?;
        Ok(sac.h)
    }
}

impl Sac {
    pub(crate) fn read_all(p: &Path, e: Endian, only_h: bool) -> Result<Sac, Box<dyn Error>> {
        let mut f = File::open(p)?;
        let mut f_buf = Vec::new();
        f.read_to_end(&mut f_buf)?;

        let h_buf = &f_buf[..SAC_HEADER_SIZE];
        let d_buf = &f_buf[SAC_HEADER_SIZE..];

        let binary = SacBinary::decode_header(h_buf, e)?;
        let mut sac = Sac::build(&binary, p, e);

        if only_h {
            return Ok(sac)
        }

        if let Err(err) = sac.check() {
            return Err(Box::new(err));
        }

        let data = SacBinary::decode_data(d_buf, e);

        if sac.iftype == SacFileType::Time && sac.leven {
            sac.y = data;
            return Ok(sac)
        }

        let size = sac.npts as usize;
        sac.x = data[..size].to_vec();
        sac.y = data[size..].to_vec();
        Ok(sac)
    }

    pub(crate) fn write_all(&self, p: &Path, e: Endian, only_h: bool) -> Result<(), Box<dyn Error>> {
        if let Err(err) = self.check() {
            return Err(Box::new(err));
        }

        let header = SacBinary::from(self);
        let mut h_buf = [0; SAC_HEADER_SIZE];
        SacBinary::encode_header(header, &mut h_buf, e)?;

        let d_buf = if only_h {
            let mut f = File::open(p)?;
            let mut f_buf = Vec::new();
            f.read_to_end(&mut f_buf)?;
            f_buf[SAC_HEADER_SIZE..].to_vec()

        } else {
            let mut data = self.x.clone();
            data.extend_from_slice(&self.y);
            SacBinary::encode_data(&data, e)
        };

        let mut f_buf = h_buf.to_vec();
        f_buf.extend_from_slice(&d_buf);

        let mut f = File::create(p)?;
        f.write_all(&f_buf)?;
        Ok(())
    }

    pub fn read_header(path: &Path, endian: Endian) -> Result<Sac, Box<dyn Error>> {
        Sac::read_all(path, endian, true)
    }

    pub fn set_header(&mut self, h: SacHeader) {
        self.h = h
    }

    pub fn write_header(&self) -> Result<(), Box<dyn Error>> {
        self.write_all(self.path(), self.endian, true)
    }

    pub fn read(path: &Path, endian: Endian) -> Result<Sac, Box<dyn Error>> {
        Sac::read_all(path, endian, false)
    }

    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        self.write_all(self.path(), self.endian, false)
    }

    pub fn write_to(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        self.write_all(path, self.endian, false)
    }
}
