use crate::binary::SAC_INT_UNDEF;

const ITIME: i32 = 1;
const IRLIM: i32 = 2;
const IAMPH: i32 = 3;
const IXY: i32 = 4;
const IXYZ: i32 = 51;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SacFileType {
    Time = ITIME,
    RealImag = IRLIM,
    AmpPhase = IAMPH,
    XY = IXY,
    XYZ = IXYZ,
    Unknown = SAC_INT_UNDEF,
}

impl From<SacFileType> for i32 {
    fn from(t: SacFileType) -> i32 {
        t as i32
    }
}

impl From<i32> for SacFileType {
    fn from(t: i32) -> SacFileType {
        match t {
            ITIME => SacFileType::Time,
            IRLIM => SacFileType::RealImag,
            IAMPH => SacFileType::AmpPhase,
            IXY => SacFileType::XY,
            IXYZ => SacFileType::XYZ,
            _ => SacFileType::Unknown,
        }
    }
}
