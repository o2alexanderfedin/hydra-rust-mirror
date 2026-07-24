#![allow(unused_imports, dead_code)]

mod hydra;
mod hydra_h;

pub(crate) type DarwinSizeT = u64;

pub(crate) type Uint32T = u32;

pub(crate) type TcflagT = u64;

pub(crate) type CcT = u8;

pub(crate) type DarwinSsizeT = i64;

pub(crate) type SpeedT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Winsize {
    pub(crate) ws_row: u16,
    pub(crate) ws_col: u16,
    pub(crate) ws_xpixel: u16,
    pub(crate) ws_ypixel: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Termios {
    pub(crate) c_iflag: u64,
    pub(crate) c_oflag: u64,
    pub(crate) c_cflag: u64,
    pub(crate) c_lflag: u64,
    pub(crate) c_cc: [u8; 20],
    pub(crate) c_ispeed: u64,
    pub(crate) c_ospeed: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SFILE {
    pub(crate) _opaque: [u8; 0],
}

pub(crate) type FILE = SFILE;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn calloc(__count: u64, __size: u64)
    -> *mut ();
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn ioctl(_: i32, _: u64, ...)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn tcgetattr(_: i32, _: *mut Termios)
    -> i32;
    fn perror(_: *const i8)
    -> ();
    fn tcsetattr(_: i32, _: i32, _: *const Termios)
    -> i32;
    fn read(_: i32, _: *mut (), __nbyte: u64)
    -> i64;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn exit(_: i32)
    -> ();
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn ftell(_: *mut FILE)
    -> i64;
    fn rewind(_: *mut FILE)
    -> ();
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn setbuf(_: *mut FILE, _: *mut i8)
    -> ();
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
}
