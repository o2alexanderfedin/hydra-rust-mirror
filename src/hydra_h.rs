use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Command {
    pub(crate) key: i8,
    pub(crate) name: *mut i8,
    pub(crate) command: *mut i8,
    pub(crate) children: *mut Command,
    pub(crate) next: *mut Command,
}
