# libsac
A Rust library for reading/writing SAC data

## usage
```toml
[dependencies.sac]
git = "https://github.com/SanmerDev/libsac.git"
```

## usage (no-std)
```toml
[dependencies.sac]
git = "https://github.com/SanmerDev/libsac.git"
default-features = false
features = ["alloc"]
```

## demo
```rust
use std::path::Path;
use sac::{Endian, Sac};

fn main() {
    let path = Path::new("tests/test.sac");

    let mut sac = Sac::from_file(path, Endian::Little).unwrap();
    sac.t[0] = 10.0;
    sac.kt[0] = "P".to_owned();
    sac.kstnm = "VDC".to_owned();
    sac.to_file(path, Endian::Little).unwrap();

    let sac = Sac::from_file(path, Endian::Little).unwrap();
    let y = &sac.first;

    assert_eq!(sac.t[0], 10.0);
    assert_eq!(sac.kt[0], "P");
    assert_eq!(sac.kstnm, "VDC");

    assert_eq!(y.first().unwrap(), &-0.09728001);
    assert_eq!(y.last().unwrap(), &-0.07680000);
    assert_eq!(y.len(), sac.npts as usize);
}
```
