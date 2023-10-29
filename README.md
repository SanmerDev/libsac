# libsac
A Rust library for reading/writing SAC data

## usage
```toml
libsac = { git = "https://github.com/SanmerDev/libsac.git", branch = "main" }
```

## demo
```rust
use std::path::Path;
use libsac::{Endian, Sac};

fn main() {
    let path = Path::new("tests/test.sac");

    let mut sac = Sac::read_header(path, Endian::Little).unwrap();
    sac.t[0] = 10.0;
    sac.kt[0] = "P".to_string();
    sac.kstnm = "VDC".to_string();
    sac.write_header().unwrap();

    let sac = Sac::read(path, Endian::Little).unwrap();
    let y = &sac.first;

    assert_eq!(sac.t[0], 10.0);
    assert_eq!(sac.kt[0], "P");
    assert_eq!(sac.kstnm, "VDC");

    assert_eq!(y.first().unwrap(), &-0.09728001);
    assert_eq!(y.last().unwrap(), &-0.07680000);
    assert_eq!(y.len(), sac.npts as usize);
}
```
