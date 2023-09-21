use std::array;
use std::path::Path;

use bincode::{Decode, Encode};
use crate::Endian;

use crate::sac::Sac;

const SAC_INT_UNDEF : i32 = -12345;
const SAC_BOOL_UNDEF : i32 = 0;
const SAC_FLOAT_UNDEF : f32 = -12345.0;
const SAC_STR8_UNDEF: [u8; 8] = [
    b'-', b'1', b'2', b'3', b'4', b'5',
    b' ', b' '
];
const SAC_STR16_UNDEF: [u8; 16] = [
    b'-', b'1', b'2', b'3', b'4', b'5',
    b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' '
];

fn write_string<const N: usize>(v: &String, length: usize) -> [u8; N] {
    let mut bytes: [u8; N] = [b' '; N];
    let v_bytes = v.as_bytes();

    let length = v_bytes.len().min(length);
    bytes[..length].copy_from_slice(&v_bytes[..length]);

    bytes
}

fn read_string<const N: usize>(v: &[u8; N]) -> String {
    std::str::from_utf8(v)
        .unwrap_or("-12345")
        .trim()
        .to_string()
}

// noinspection SpellCheckingInspection
#[derive(Debug, Encode, Decode)]
pub struct SacBinary {
    // float
    delta: f32, depmin: f32, depmax: f32, scale: f32, odelta: f32,
    b: f32, e: f32, o: f32, a: f32, internal1: f32,
    t: [f32; 10], f: f32,
    resp: [f32; 10], stla: f32, stlo: f32, stel: f32, stdp: f32,
    evla: f32, evlo: f32, evel: f32, evdp: f32, mag: f32,
    user: [f32; 10],
    dist: f32, az: f32, baz: f32, gcarc: f32, internal2: f32,
    internal3: f32, depmen: f32, cmpaz: f32, cmpinc: f32, xminimum: f32,
    xmaximum: f32, yminimum: f32, ymaximum: f32, unused0: [f32; 7],

    // int
    nzyear: i32, nzjday: i32, nzhour: i32, nzmin: i32, nzsec: i32,
    nzmsec: i32, nvhdr: i32, norid: i32, nevid: i32, npts: i32,
    internal4: i32, nwfid: i32, nxsize: i32, nysize: i32, unused1: i32,

    // enum
    iftype: i32, idep: i32, iztype: i32, unused2: i32, iinst: i32,
    istreg: i32, ievreg: i32, ievtyp: i32, iqual: i32, isynth: i32,
    imagtyp: i32, imagsrc: i32, unused3: [i32; 8],

    // bool
    leven: i32, lpspol: i32, lovrok: i32, lcalda: i32, unused4: i32,

    // string
    kstnm: [u8; 8], kevnm: [u8; 16], khole: [u8; 8], ko: [u8; 8], ka: [u8; 8],
    kt: [[u8; 8]; 10], kf: [u8; 8],
    kuser0: [u8; 8], kuser1: [u8; 8], kuser2: [u8; 8],
    kcmpnm: [u8; 8], knetwk: [u8; 8], kdatrd: [u8; 8], kinst: [u8; 8]
}

impl Default for SacBinary {
    fn default() -> Self {
        SacBinary {
            delta: SAC_FLOAT_UNDEF,
            depmin: SAC_FLOAT_UNDEF,
            depmax: SAC_FLOAT_UNDEF,
            scale: SAC_FLOAT_UNDEF,
            odelta: SAC_FLOAT_UNDEF,
            b: SAC_FLOAT_UNDEF,
            e: SAC_FLOAT_UNDEF,
            o: SAC_FLOAT_UNDEF,
            a: SAC_FLOAT_UNDEF,
            internal1: SAC_FLOAT_UNDEF,
            t: [SAC_FLOAT_UNDEF; 10],
            f: SAC_FLOAT_UNDEF,
            resp: [SAC_FLOAT_UNDEF; 10],
            stla: SAC_FLOAT_UNDEF,
            stlo: SAC_FLOAT_UNDEF,
            stel: SAC_FLOAT_UNDEF,
            stdp: SAC_FLOAT_UNDEF,
            evla: SAC_FLOAT_UNDEF,
            evlo: SAC_FLOAT_UNDEF,
            evel: SAC_FLOAT_UNDEF,
            evdp: SAC_FLOAT_UNDEF,
            mag: SAC_FLOAT_UNDEF,
            user: [SAC_FLOAT_UNDEF; 10],
            dist: SAC_FLOAT_UNDEF,
            az: SAC_FLOAT_UNDEF,
            baz: SAC_FLOAT_UNDEF,
            gcarc: SAC_FLOAT_UNDEF,
            internal2: SAC_FLOAT_UNDEF,
            internal3: SAC_FLOAT_UNDEF,
            depmen: SAC_FLOAT_UNDEF,
            cmpaz: SAC_FLOAT_UNDEF,
            cmpinc: SAC_FLOAT_UNDEF,
            xminimum: SAC_FLOAT_UNDEF,
            xmaximum: SAC_FLOAT_UNDEF,
            yminimum: SAC_FLOAT_UNDEF,
            ymaximum: SAC_FLOAT_UNDEF,
            unused0: [SAC_FLOAT_UNDEF; 7],
            nzyear: SAC_INT_UNDEF,
            nzjday: SAC_INT_UNDEF,
            nzhour: SAC_INT_UNDEF,
            nzmin: SAC_INT_UNDEF,
            nzsec: SAC_INT_UNDEF,
            nzmsec: SAC_INT_UNDEF,
            nvhdr: SAC_INT_UNDEF,
            norid: SAC_INT_UNDEF,
            nevid: SAC_INT_UNDEF,
            npts: SAC_INT_UNDEF,
            internal4: SAC_INT_UNDEF,
            nwfid: SAC_INT_UNDEF,
            nxsize: SAC_INT_UNDEF,
            nysize: SAC_INT_UNDEF,
            unused1: SAC_INT_UNDEF,
            iftype: SAC_INT_UNDEF,
            idep: SAC_INT_UNDEF,
            iztype: SAC_INT_UNDEF,
            unused2: SAC_INT_UNDEF,
            iinst: SAC_INT_UNDEF,
            istreg: SAC_INT_UNDEF,
            ievreg: SAC_INT_UNDEF,
            ievtyp: SAC_INT_UNDEF,
            iqual: SAC_INT_UNDEF,
            isynth: SAC_INT_UNDEF,
            imagtyp: SAC_INT_UNDEF,
            imagsrc: SAC_INT_UNDEF,
            unused3: [SAC_INT_UNDEF; 8],
            leven: SAC_BOOL_UNDEF,
            lpspol: SAC_BOOL_UNDEF,
            lovrok: SAC_BOOL_UNDEF,
            lcalda: SAC_BOOL_UNDEF,
            unused4: SAC_BOOL_UNDEF,
            kstnm: SAC_STR8_UNDEF,
            kevnm: SAC_STR16_UNDEF,
            khole: SAC_STR8_UNDEF,
            ko: SAC_STR8_UNDEF,
            ka: SAC_STR8_UNDEF,
            kt: [SAC_STR8_UNDEF; 10],
            kf: SAC_STR8_UNDEF,
            kuser0: SAC_STR8_UNDEF,
            kuser1: SAC_STR8_UNDEF,
            kuser2: SAC_STR8_UNDEF,
            kcmpnm: SAC_STR8_UNDEF,
            knetwk: SAC_STR8_UNDEF,
            kdatrd: SAC_STR8_UNDEF,
            kinst: SAC_STR8_UNDEF,
        }
    }
}

impl SacBinary {
    pub fn from(v: &Sac) -> Self {
        // string to bytes
        let mut kt_vec: Vec<[u8; 8]> = v.kt.iter()
            .map(|s|write_string(s, 8))
            .collect();

        // fill with default value to 10
        kt_vec.resize(10, SAC_STR8_UNDEF);

        // vec to array
        let mut kt = [SAC_STR8_UNDEF; 10];
        kt.clone_from_slice(&kt_vec);

        SacBinary {
            // undef
            internal1: SAC_FLOAT_UNDEF,
            internal2: SAC_FLOAT_UNDEF,
            internal3: SAC_FLOAT_UNDEF,
            internal4: SAC_INT_UNDEF,
            unused0: [SAC_FLOAT_UNDEF; 7],
            unused1: SAC_INT_UNDEF,
            unused2: SAC_INT_UNDEF,
            unused3: [SAC_INT_UNDEF; 8],
            unused4: SAC_BOOL_UNDEF,

            kt,
            delta: v.delta,
            depmin: v.depmin,
            depmax: v.depmax,
            scale: v.scale,
            odelta: v.odelta,
            b: v.b,
            e: v.e,
            o: v.o,
            a: v.a,
            t: v.t,
            f: v.f,
            resp: v.resp,
            stla: v.stla,
            stlo: v.stlo,
            stel: v.stel,
            stdp: v.stdp,
            evla: v.evla,
            evlo: v.evlo,
            evel: v.evel,
            evdp: v.evdp,
            mag: v.mag,
            user: v.user,
            dist: v.dist,
            az: v.az,
            baz: v.baz,
            gcarc: v.gcarc,
            depmen: v.depmen,
            cmpaz: v.cmpaz,
            cmpinc: v.cmpinc,
            xminimum: v.xminimum,
            xmaximum: v.xmaximum,
            yminimum: v.yminimum,
            ymaximum: v.ymaximum,
            nzyear: v.nzyear,
            nzjday: v.nzjday,
            nzhour: v.nzhour,
            nzmin: v.nzmin,
            nzsec: v.nzsec,
            nzmsec: v.nzmsec,
            nvhdr: v.nvhdr,
            norid: v.norid,
            nevid: v.nevid,
            npts: v.npts,
            nwfid: v.nwfid,
            nxsize: v.nxsize,
            nysize: v.nysize,
            iftype: v.iftype.into(),
            idep: v.idep,
            iztype: v.iztype,
            iinst: v.iinst,
            istreg: v.istreg,
            ievreg: v.ievreg,
            ievtyp: v.ievtyp,
            iqual: v.iqual,
            isynth: v.isynth,
            imagtyp: v.imagtyp,
            imagsrc: v.imagsrc,
            leven: if v.leven { 1 } else { 0 },
            lpspol: if v.lpspol { 1 } else { 0 },
            lovrok: if v.lovrok { 1 } else { 0 },
            lcalda: if v.lcalda { 1 } else { 0 },
            kstnm: write_string(&v.kstnm, 8),
            kevnm: write_string(&v.kevnm, 16),
            khole: write_string(&v.khole, 8),
            ko: write_string(&v.ko, 8),
            ka: write_string(&v.ka, 8),
            kf: write_string(&v.kf, 8),
            kuser0: write_string(&v.kuser0, 8),
            kuser1: write_string(&v.kuser1, 8),
            kuser2: write_string(&v.kuser2, 8),
            kcmpnm: write_string(&v.kcmpnm, 8),
            knetwk: write_string(&v.knetwk, 8),
            kdatrd: write_string(&v.kdatrd, 8),
            kinst: write_string(&v.kinst, 8),
        }
    }
}

impl <'a> Sac<'a> {
    pub(crate) fn build(v: &SacBinary, p: &'a Path, e: Endian) -> Self {
        // bytes to string
        let mut kt_vec: Vec<String> = v.kt.iter()
            .map(|b|read_string(b))
            .collect();

        // fill with default value to 10
        kt_vec.resize(10, "-12345  ".to_string());

        // vec to array
        let mut kt: [String; 10] = array::from_fn(|_| " ".to_string());
        kt.clone_from_slice(&kt_vec);

        Sac {
            // inner
            path: p,
            endian: e,

            kt,
            x: Vec::with_capacity(0),
            y: Vec::with_capacity(0),
            delta: v.delta,
            depmin: v.depmin,
            depmax: v.depmax,
            scale: v.scale,
            odelta: v.odelta,
            b: v.b,
            e: v.e,
            o: v.o,
            a: v.a,
            t: v.t,
            f: v.f,
            resp: v.resp,
            stla: v.stla,
            stlo: v.stlo,
            stel: v.stel,
            stdp: v.stdp,
            evla: v.evla,
            evlo: v.evlo,
            evel: v.evel,
            evdp: v.evdp,
            mag: v.mag,
            user: v.user,
            dist: v.dist,
            az: v.az,
            baz: v.baz,
            gcarc: v.gcarc,
            depmen: v.depmen,
            cmpaz: v.cmpaz,
            cmpinc: v.cmpinc,
            xminimum: v.xminimum,
            xmaximum: v.xmaximum,
            yminimum: v.yminimum,
            ymaximum: v.ymaximum,
            nzyear: v.nzyear,
            nzjday: v.nzjday,
            nzhour: v.nzhour,
            nzmin: v.nzmin,
            nzsec: v.nzsec,
            nzmsec: v.nzmsec,
            nvhdr: v.nvhdr,
            norid: v.norid,
            nevid: v.nevid,
            npts: v.npts,
            nwfid: v.nwfid,
            nxsize: v.nxsize,
            nysize: v.nysize,
            iftype: v.iftype.into(),
            idep: v.idep,
            iztype: v.iztype,
            iinst: v.iinst,
            istreg: v.istreg,
            ievreg: v.ievreg,
            ievtyp: v.ievtyp,
            iqual: v.iqual,
            isynth: v.isynth,
            imagtyp: v.imagtyp,
            imagsrc: v.imagsrc,
            leven: v.leven == 1,
            lpspol: v.lpspol == 1,
            lovrok: v.lovrok == 1,
            lcalda: v.lcalda == 1,
            kstnm: read_string(&v.kstnm),
            kevnm: read_string(&v.kevnm),
            khole: read_string(&v.khole),
            ko: read_string(&v.ko),
            ka: read_string(&v.ka),
            kf: read_string(&v.kf),
            kuser0: read_string(&v.kuser0),
            kuser1: read_string(&v.kuser1),
            kuser2: read_string(&v.kuser2),
            kcmpnm: read_string(&v.kcmpnm),
            knetwk: read_string(&v.knetwk),
            kdatrd: read_string(&v.kdatrd),
            kinst: read_string(&v.kinst),
        }
    }
}