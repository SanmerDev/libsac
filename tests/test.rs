use std::fs;
use std::path::Path;

use sac::{Endian, Sac, SacFileType};

#[test]
fn read() {
    let path = Path::new("tests/test.sac");
    let sac = Sac::from_file(path, Endian::Little).unwrap();
    let y = &sac.first;

    assert_eq!(sac.delta, 0.01);
    assert_eq!(sac.npts, 1000);
    assert_eq!(sac.kstnm, "CDV");

    assert_eq!(y.first().unwrap(), &-0.09728001);
    assert_eq!(y.last().unwrap(), &-0.07680000);
    assert_eq!(y.len(), sac.npts as usize);
}

#[test]
fn write() {
    let path = Path::new("tests/test.sac");
    let sac = Sac::from_file(path, Endian::Little).unwrap();

    let new = Path::new("tests/test_big.sac");
    sac.to_file(new, Endian::Big).unwrap();

    let sac = Sac::from_file(new, Endian::Big).unwrap();
    let y = &sac.first;

    assert_eq!(sac.delta, 0.01);
    assert_eq!(sac.npts, 1000);
    assert_eq!(sac.kstnm, "CDV");

    assert_eq!(y.first().unwrap(), &-0.09728001);
    assert_eq!(y.last().unwrap(), &-0.07680000);
    assert_eq!(y.len(), sac.npts as usize);

    fs::remove_file(new).unwrap();
}

#[test]
fn new() {
    let new = Path::new("tests/test_new.sac");
    let mut sac = Sac::empty();
    sac.iftype = SacFileType::Time;
    sac.to_file(new, Endian::Little).unwrap();

    let sac = Sac::from_file(new, Endian::Little).unwrap();
    let y = &sac.first;

    assert_eq!(sac.delta, -12345.0);
    assert_eq!(sac.npts, 0);
    assert_eq!(sac.kstnm, "-12345");

    assert_eq!(y.first(), None);
    assert_eq!(y.last(), None);
    assert_eq!(y.len(), 0);

    fs::remove_file(new).unwrap();
}
