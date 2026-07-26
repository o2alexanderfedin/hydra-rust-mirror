use super::*;

pub const ColorOff: &str = "\u{1b}[0m";

pub const Yellow: &str = "\u{1b}[0;33m";

pub const Blue: &str = "\u{1b}[0;34m";

pub const Purple: &str = "\u{1b}[0;35m";

pub const RightMargin: i32 = 5;

pub const DefaultName: &str = "unnamed";

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Command {
    pub(crate) key: i8,
    pub(crate) name: *mut i8,
    pub(crate) command: *mut i8,
    pub(crate) children: *mut Command,
    pub(crate) next: *mut Command,
}
