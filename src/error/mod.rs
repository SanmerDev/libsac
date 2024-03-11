#[cfg(all(feature = "unstable", not(feature = "std")))]
pub use core::error::Error as StdError;
#[cfg(feature = "std")]
pub use std::error::Error as StdError;
#[cfg(not(any(feature = "std", feature = "unstable")))]
pub use std_error::Error as StdError;

use alloc::string::String;
use alloc::string::ToString;
use core::fmt;
use core::result;

#[cfg(not(any(feature = "std", feature = "unstable")))]
mod std_error;

pub type Result<T> = result::Result<T, SacError>;

pub struct SacError {
    msg: String,
}

impl SacError {
    pub(crate) fn custom<T: fmt::Display>(msg: T) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Debug for SacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl fmt::Display for SacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl StdError for SacError {}
