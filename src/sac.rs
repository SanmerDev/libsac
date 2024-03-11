use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

use crate::binary::SacBinary;
use crate::header::SacHeader;

pub struct Sac {
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
    pub(crate) fn build(b: &SacBinary) -> Self {
        Sac {
            h: SacHeader::from(b),
            first: Vec::with_capacity(0),
            second: Vec::with_capacity(0),
        }
    }

    pub fn new() -> Self {
        Sac::build(&SacBinary::default())
    }
}
