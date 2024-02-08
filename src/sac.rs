use std::ops::{Deref, DerefMut};
use std::path::Path;

use crate::binary::SacBinary;
use crate::header::SacHeader;
use crate::Endian;

#[derive(Debug, Clone)]
pub struct Sac {
    pub(crate) path: String,
    pub(crate) endian: Endian,
    pub(crate) h: SacHeader,
    pub first: Vec<f32>,
    pub second: Vec<f32>,
}

impl Deref for Sac {
    type Target = SacHeader;

    fn deref(&self) -> &Self::Target {
        &self.h
    }
}

impl DerefMut for Sac {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.h
    }
}

impl Sac {
    pub(crate) fn build(b: &SacBinary, p: &Path, e: Endian) -> Option<Self> {
        Some(Sac {
            path: p.to_str()?.to_string(),
            endian: e,
            h: SacHeader::from(b),
            first: Vec::with_capacity(0),
            second: Vec::with_capacity(0),
        })
    }

    pub fn new(path: &Path, endian: Endian) -> Option<Self> {
        Sac::build(&SacBinary::default(), path, endian)
    }
}
