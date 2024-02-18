const ITIME: i32 = 1;
const IRLIM: i32 = 2;
const IAMPH: i32 = 3;
const IXY: i32 = 4;
const _IXYZ: i32 = 51;

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum SacFileType {
    Time = ITIME,
    RealImag = IRLIM,
    AmpPhase = IAMPH,
    XY = IXY,
    Unknown(i32),
}

impl From<SacFileType> for i32 {
    fn from(t: SacFileType) -> i32 {
        match t {
            SacFileType::Time => ITIME,
            SacFileType::RealImag => IRLIM,
            SacFileType::AmpPhase => IAMPH,
            SacFileType::XY => IXY,
            SacFileType::Unknown(v) => v,
        }
    }
}

impl From<i32> for SacFileType {
    fn from(t: i32) -> SacFileType {
        match t {
            ITIME => SacFileType::Time,
            IRLIM => SacFileType::RealImag,
            IAMPH => SacFileType::AmpPhase,
            IXY => SacFileType::XY,
            _ => SacFileType::Unknown(t),
        }
    }
}
