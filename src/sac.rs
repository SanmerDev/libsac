use std::ops::{Deref, DerefMut};
use std::path::Path;

use crate::binary::SacBinary;
use crate::Endian;
use crate::header::SacHeader;

#[derive(Debug, Clone)]
pub struct Sac<'a> {
    pub(crate) path: &'a Path,
    pub(crate) endian: Endian,

    pub(crate) h: SacHeader,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

impl<'a> Deref for Sac<'a> {
    type Target = SacHeader;

    fn deref(&self) -> &Self::Target {
        &self.h
    }
}

impl<'a> DerefMut for Sac<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.h
    }
}

#[allow(unused)]
impl <'a> Sac<'a> {
    pub(crate) fn build(b: &SacBinary, p: &'a Path, e: Endian) -> Self {
        Sac {
            path: p,
            endian: e,

            h: SacHeader::from(b),
            x: Vec::with_capacity(0),
            y: Vec::with_capacity(0)
        }
    }

    pub fn new_empty(path: &'a Path, endian: Endian) -> Self {
        Sac {
            path,
            endian,

            h: SacHeader::empty(),
            x: Vec::with_capacity(0),
            y: Vec::with_capacity(0)
        }
    }
}