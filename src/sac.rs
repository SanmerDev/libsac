use std::path::Path;

use crate::binary::SacBinary;
use crate::Endian;
use crate::enums::SacFileType;

// noinspection SpellCheckingInspection
#[derive(Debug, Clone)]
pub struct Sac<'a> {
    pub(crate) path: &'a Path,
    pub(crate) endian: Endian,

    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub delta: f32,
    pub depmin: f32,
    pub depmax: f32,
    pub scale: f32,
    pub odelta: f32,
    pub b: f32,
    pub e: f32,
    pub o: f32,
    pub a: f32,
    pub t: [f32; 10],
    pub f: f32,
    pub resp: [f32; 10],
    pub stla: f32,
    pub stlo: f32,
    pub stel: f32,
    pub stdp: f32,
    pub evla: f32,
    pub evlo: f32,
    pub evel: f32,
    pub evdp: f32,
    pub mag: f32,
    pub user: [f32; 10],
    pub dist: f32,
    pub az: f32,
    pub baz: f32,
    pub gcarc: f32,
    pub depmen: f32,
    pub cmpaz: f32,
    pub cmpinc: f32,
    pub xminimum: f32,
    pub xmaximum: f32,
    pub yminimum: f32,
    pub ymaximum: f32,
    pub nzyear: i32,
    pub nzjday: i32,
    pub nzhour: i32,
    pub nzmin: i32,
    pub nzsec: i32,
    pub nzmsec: i32,
    pub nvhdr: i32,
    pub norid: i32,
    pub nevid: i32,
    pub npts: i32,
    pub nwfid: i32,
    pub nxsize: i32,
    pub nysize: i32,
    pub iftype: SacFileType,
    pub idep: i32,
    pub iztype: i32,
    pub iinst: i32,
    pub istreg: i32,
    pub ievreg: i32,
    pub ievtyp: i32,
    pub iqual: i32,
    pub isynth: i32,
    pub imagtyp: i32,
    pub imagsrc: i32,
    pub leven: bool,
    pub lpspol: bool,
    pub lovrok: bool,
    pub lcalda: bool,
    pub kstnm: String,
    pub kevnm: String,
    pub khole: String,
    pub ko: String,
    pub ka: String,
    pub kt: [String; 10],
    pub kf: String,
    pub kuser0: String,
    pub kuser1: String,
    pub kuser2: String,
    pub kcmpnm: String,
    pub knetwk: String,
    pub kdatrd: String,
    pub kinst: String,
}

#[allow(dead_code)]
impl <'a> Sac<'a> {
    fn new(path: &'a Path, endian: Endian) -> Self {
        let default = SacBinary::default();
        Sac::build(&default, path, endian)
    }
}