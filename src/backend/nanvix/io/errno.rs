use crate::backend::c;
use std::os::nanvix::sys::error::ErrorCode;

/// Errno
#[repr(transparent)]
#[doc(alias = "errno")]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Errno(pub(crate) c::c_int);

impl Errno {
    /// Interrupted.
    pub const INTR: Errno = Errno(ErrorCode::Interrupted as i32);
    /// Returns the raw OS error code.
    #[inline]
    pub fn raw_os_error(self) -> c::c_int {
        self.0
    }
}
