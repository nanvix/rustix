//! The Nanvix backend.

#[cfg(not(windows))]
pub(crate) mod fd {
    pub use crate::maybe_polyfill::os::fd::*;
    #[allow(unused_imports)]
    pub(crate) use RawFd as LibcFd;
}

pub(crate) mod c;
pub(crate) mod io;
