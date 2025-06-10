#![allow(dead_code, unused_variables)]
use crate::backend::c;
use crate::fd::{AsFd as _, BorrowedFd, OwnedFd, RawFd};
use crate::io::{self, FdFlags, IoSlice, IoSliceMut};
use crate::ioctl::{IoctlOutput, Opcode};

pub(crate) unsafe fn close(_raw_fd: RawFd) {
    todo!();
}

pub(crate) fn fcntl_getfd(_fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    todo!();
}

pub(crate) fn fcntl_setfd(_fd: BorrowedFd<'_>, _flags: FdFlags) -> io::Result<()> {
    todo!();
}

pub(crate) fn fcntl_dupfd_cloexec(_fd: BorrowedFd<'_>, _min: RawFd) -> io::Result<OwnedFd> {
    todo!();
}

pub(crate) unsafe fn read(_fd: BorrowedFd<'_>, _buf: (*mut u8, usize)) -> io::Result<usize> {
    todo!();
}

pub(crate) fn write(_fd: BorrowedFd<'_>, _buf: &[u8]) -> io::Result<usize> {
    todo!();
}

pub(crate) fn pwrite(_fd: BorrowedFd<'_>, _buf: &[u8], _pos: u64) -> io::Result<usize> {
    todo!();
}

pub(crate) unsafe fn pread(
    fd: BorrowedFd<'_>,
    buf: (*mut u8, usize),
    pos: u64,
) -> io::Result<usize> {
    todo!();
}

pub(crate) fn writev(fd: BorrowedFd<'_>, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    todo!();
}

pub(crate) fn preadv(
    fd: BorrowedFd<'_>,
    bufs: &mut [IoSliceMut<'_>],
    pos: u64,
) -> io::Result<usize> {
    todo!();
}

pub(crate) unsafe fn ioctl(
    fd: BorrowedFd<'_>,
    request: Opcode,
    arg: *mut c::c_void,
) -> io::Result<IoctlOutput> {
    todo!();
}

pub(crate) fn readv(fd: BorrowedFd<'_>, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    todo!()
}

pub(crate) fn pwritev(fd: BorrowedFd<'_>, bufs: &[IoSlice<'_>], pos: u64) -> io::Result<usize> {
    todo!();
}

pub(crate) unsafe fn ioctl_readonly(
    fd: BorrowedFd<'_>,
    request: Opcode,
    arg: *mut c::c_void,
) -> io::Result<IoctlOutput> {
    todo!();
}
