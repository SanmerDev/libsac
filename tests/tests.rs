
#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use libsac::{Endian, Sac};

    #[test]
    fn test_read() {
        let path = Path::new("tests/test.sac");
        let sac = Sac::read(path, Endian::Little).unwrap();
        let y = &sac.y;

        assert_eq!(sac.delta, 0.01);
        assert_eq!(sac.npts, 1000);
        assert_eq!(sac.kstnm, "CDV");

        assert_eq!(y.first().unwrap(), &-0.09728001);
        assert_eq!(y.last().unwrap(), &-0.07680000);
        assert_eq!(y.len(), sac.npts as usize);
    }

    #[test]
    fn test_read_header() {
        let path = Path::new("tests/test.sac");
        let sac = Sac::read_header(path, Endian::Little).unwrap();
        let y = &sac.y;

        // header
        assert_eq!(sac.delta, 0.01);
        assert_eq!(sac.npts, 1000);
        assert_eq!(sac.kstnm, "CDV");

        // data
        assert_eq!(y.first(), None);
        assert_eq!(y.last(), None);
        assert_eq!(y.len(), 0);
    }

    #[test]
    fn test_write() {
        let path = Path::new("tests/test.sac");
        let mut sac = Sac::read(path, Endian::Little).unwrap();

        // write_to (big endian)
        let new = Path::new("tests/test_big.sac");
        sac.set_endian(Endian::Big);
        sac.write_to(new).unwrap();

        // read (big endian)
        let sac = Sac::read(new, Endian::Big).unwrap();
        let y = &sac.y;

        // header
        assert_eq!(sac.delta, 0.01);
        assert_eq!(sac.npts, 1000);
        assert_eq!(sac.kstnm, "CDV");

        // data
        assert_eq!(y.first().unwrap(), &-0.09728001);
        assert_eq!(y.last().unwrap(), &-0.07680000);
        assert_eq!(y.len(), sac.npts as usize);

        // remove new
        fs::remove_file(new).unwrap();
    }

    #[test]
    fn test_write_header() {
        // copy
        let path = Path::new("tests/test.sac");
        let new = Path::new("tests/test_h.sac");
        fs::copy(path, new).unwrap();

        // write_header
        let mut sac = Sac::read_header(new, Endian::Little).unwrap();
        sac.t[0] = 10.0;
        sac.kt[0] = "P".to_string();
        sac.kstnm = "VDC".to_string();
        sac.write_header().unwrap();

        // read (with data)
        let sac = Sac::read(new, Endian::Little).unwrap();
        let y = &sac.y;

        // header
        assert_eq!(sac.t[0], 10.0);
        assert_eq!(sac.kt[0], "P");
        assert_eq!(sac.kstnm, "VDC");

        // data
        assert_eq!(y.first().unwrap(), &-0.09728001);
        assert_eq!(y.last().unwrap(), &-0.07680000);
        assert_eq!(y.len(), sac.npts as usize);

        // remove new
        fs::remove_file(new).unwrap();
    }
}