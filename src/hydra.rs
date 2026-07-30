use super::*;
use crate::hydra_h::{Command, RightMargin};

pub(crate) extern "C" fn new_command(key: i8, name: *mut i8, command: *mut i8) -> *mut Command {
    let cmd: *mut Command =
        unsafe { calloc(1 as u64, core::mem::size_of::<Command>() as u64) } as *mut Command;
    unsafe {
        *cmd = Command {
            key: key,
            name: name,
            command: command,
            children: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        }
    };
    return cmd;
}

pub(crate) extern "C" fn command_run(c: *mut Command) -> i32 {
    unsafe {
        if c as *mut () != 0 as *mut () && unsafe { (*c).command } as *mut () != 0 as *mut () {
            return unsafe {
                fprintf(__stdoutp, c"%s".as_ptr() as *mut i8 as *const i8, unsafe {
                    (*c).command
                })
            };
        }
        return 0;
    }
}

pub(crate) extern "C" fn command_add_child(c: &mut Command, child: *mut Command) -> () {
    if (*c).children as *mut () == 0 as *mut () {
        (*c).children = child;
        return;
    }
    if unsafe { (*(*c).children).key } as i32 > unsafe { (*child).key } as i32 {
        unsafe { (*child).next = (*c).children };
        (*c).children = child;
        return;
    }
    let mut last_child: *mut Command = (*c).children;
    while unsafe { (*last_child).next } as *mut () != 0 as *mut ()
        && unsafe { (*unsafe { (*last_child).next }).key } as i32 <= unsafe { (*child).key } as i32
    {
        last_child = unsafe { (*last_child).next };
    }
    unsafe { (*child).next = unsafe { (*last_child).next } };
    unsafe { (*last_child).next = child };
}

pub(crate) extern "C" fn find_command(c: &Command, key: i8) -> *mut Command {
    let mut child: *mut Command = (*c).children;
    while child as *mut () != 0 as *mut () && unsafe { (*child).key } as i32 != key as i32 {
        child = unsafe { (*child).next };
    }
    return child;
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn tree_add_command(
    tree: *mut Command,
    keys: *mut i8,
    name: *mut i8,
    command: *mut i8,
) -> () {
    let mut c: *mut Command = find_command(unsafe { &*tree }, unsafe { *keys });
    if unsafe { *unsafe { keys.offset(1 as isize) } } as i32 == 0 {
        if c as *mut () == 0 as *mut () {
            command_add_child(
                unsafe { &mut *tree },
                new_command(unsafe { *keys }, name, command),
            );
        } else {
            /// if command already exists update name and command fields
            unsafe {
                (*c).name = name
            };
            unsafe { (*c).command = command };
        }
        return;
    }
    if c as *mut () == 0 as *mut () {
        c = new_command(
            unsafe { *keys },
            c"unnamed".as_ptr() as *const i8 as *mut i8,
            core::ptr::null_mut(),
        );
        command_add_child(unsafe { &mut *tree }, c);
    }
    tree_add_command(c, unsafe { keys.offset(1 as isize) }, name, command);
}

/// Returns number of printed lines
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn print_command(c: &Command) -> i32 {
    unsafe {
        let mut terminal: Winsize = Winsize::default();
        unsafe {
            ioctl(
                2,
                1073741824 as Uint32T as u64
                    | (core::mem::size_of::<Winsize>() as u64 & 8191 as u64) << 16
                    | (('t' as i32) << 8) as u64
                    | 104 as u64,
                &raw mut terminal as *mut Winsize,
            )
        };
        let width: i32 = terminal.ws_col as i32;
        /// Keep track of how many characters printed
        let mut lines: i32 = 0;
        if !((*c).name).is_null() {
            unsafe {
                fprintf(
                    __stderrp,
                    c"%s%s:%s\n".as_ptr() as *mut i8 as *const i8,
                    c"\u{1b}[0;34m".as_ptr() as *const i8,
                    (*c).name,
                    c"\u{1b}[0m".as_ptr() as *const i8,
                )
            };
            lines += 1;
        }
        /// Find longest item
        let mut max_line_width: i32 = 0;
        let mut child: *const Command = (*c).children as *const Command;
        while !(child).is_null() {
            let line_width: i32 = unsafe { strlen(unsafe { (*child).name } as *const i8) } as i32;
            if line_width > max_line_width {
                max_line_width = line_width;
            }
            child = unsafe { (*child).next };
        }
        max_line_width += RightMargin as i32;
        if max_line_width > width {
            max_line_width = width;
        }
        let items_per_row: i32 = width / (max_line_width + 5);

        /// 5 is extra character printed before each item
        (child = (*c).children);
        let mut current_item: i32 = 0;
        while !(child).is_null() {
            current_item += 1;
            if unsafe { (*child).children } != core::ptr::null_mut() {
                unsafe {
                    fprintf(
                        __stderrp,
                        c"%s%c%s %s\u{2794}%s %s+%-*s%s".as_ptr() as *mut i8 as *const i8,
                        c"\u{1b}[0;33m".as_ptr() as *const i8,
                        unsafe { (*child).key } as i32,
                        c"\u{1b}[0m".as_ptr() as *const i8,
                        c"\u{1b}[0;35m".as_ptr() as *const i8,
                        c"\u{1b}[0m".as_ptr() as *const i8,
                        c"\u{1b}[0;34m".as_ptr() as *const i8,
                        max_line_width,
                        unsafe { (*child).name },
                        c"\u{1b}[0m".as_ptr() as *const i8,
                    )
                };
            } else {
                unsafe {
                    fprintf(
                        __stderrp,
                        c"%s%c%s %s\u{2794}%s  %-*s".as_ptr() as *mut i8 as *const i8,
                        c"\u{1b}[0;33m".as_ptr() as *const i8,
                        unsafe { (*child).key } as i32,
                        c"\u{1b}[0m".as_ptr() as *const i8,
                        c"\u{1b}[0;35m".as_ptr() as *const i8,
                        c"\u{1b}[0m".as_ptr() as *const i8,
                        max_line_width,
                        unsafe { (*child).name },
                    )
                };
            }
            if current_item % items_per_row == 0 {
                eprintln!("");
                lines += 1;
            }
            child = unsafe { (*child).next };
        }
        eprintln!("");
        lines += 1;
        return lines;
    }
}

/// Copied from: https://stackoverflow.com/a/912796/458436
pub(crate) extern "C" fn getch() -> i8 {
    let mut old: Termios = Termios {
        c_iflag: 0 as u64,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: unsafe { core::mem::zeroed() },
        c_ispeed: 0,
        c_ospeed: 0,
    };
    if unsafe { tcgetattr(0, &mut old) } < 0 {
        unsafe { perror(c"tcsetattr()".as_ptr() as *mut i8 as *const i8) };
    }
    let oldflags: TcflagT = old.c_lflag;
    old.c_lflag &= !256 as TcflagT;
    old.c_lflag &= !8 as TcflagT;
    old.c_cc[16 as usize] = 1 as CcT;
    old.c_cc[17 as usize] = 0 as CcT;
    if unsafe { tcsetattr(0, 0, &raw mut old as *const Termios) } < 0 {
        unsafe { perror(c"tcsetattr ICANON".as_ptr() as *mut i8 as *const i8) };
    }
    let mut buf: i8 = 0 as i8;
    if unsafe { read(0, &raw mut buf as *mut (), 1 as u64) } < 0 as i64 {
        unsafe { perror(c"read()".as_ptr() as *mut i8 as *const i8) };
    }
    old.c_lflag = oldflags;
    if unsafe { tcsetattr(0, 1, &raw mut old as *const Termios) } < 0 {
        unsafe { perror(c"tcsetattr ~ICANON".as_ptr() as *mut i8 as *const i8) };
    }
    return buf;
}

pub(crate) extern "C" fn read_file(file: *mut i8) -> *mut i8 {
    let f: *mut FILE = unsafe { fopen(file as *const i8, c"r".as_ptr() as *mut i8 as *const i8) };
    if f as *mut () == 0 as *mut () {
        unsafe { perror(c"Failed to open file".as_ptr() as *mut i8 as *const i8) };
        unsafe { exit(1) };
    }
    unsafe { fseek(f, 0 as i64, 2) };
    let file_size: i64 = unsafe { ftell(f) };
    unsafe { rewind(f) };
    let content: *mut i8 = unsafe {
        calloc(
            (file_size + 1 as i64) as u64,
            core::mem::size_of::<i8>() as u64,
        )
    } as *mut i8;
    unsafe { fread(content as *mut (), file_size as u64, 1 as u64, f) };
    unsafe { fclose(f) };
    return content;
}

pub(crate) extern "C" fn read_field(file: &mut *mut i8, field: *mut i8) -> *mut i8 {
    unsafe {
        let key: *mut i8 = *file;
        while unsafe { **file } as i32 != ',' as i32
            && unsafe { **file } as i32 != '\n' as i32
            && unsafe { **file } as i32 != 0
        {
            {
                let __p = &mut *file;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if unsafe { **file } as i32 != ',' as i32 {
            unsafe {
                fprintf(
                    __stderrp,
                    c"Found incorrect end after %s, found: %c".as_ptr() as *mut i8 as *const i8,
                    field,
                    unsafe { **file } as i32,
                )
            };
            unsafe { exit(1) };
        }
        unsafe { **file = 0 as i8 };
        {
            let __n = 1;
            let __p = &mut *file;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        return key;
    }
}

/// Read until end of line character
pub(crate) extern "C" fn read_until_eol(file: &mut *mut i8) -> *mut i8 {
    let s: *mut i8 = *file;
    while unsafe { **file } as i32 != '\n' as i32 && unsafe { **file } as i32 != 0 {
        {
            let __p = &mut *file;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    if unsafe { **file } as i32 == '\n' as i32 {
        unsafe { **file = 0 as i8 };
        {
            let __n = 1;
            let __p = &mut *file;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    return s;
}

pub(crate) extern "C" fn read_line(c: *mut Command, file: *mut *mut i8) -> () {
    let key: *mut i8 = read_field(unsafe { &mut *file }, c"key".as_ptr() as *mut i8);
    let name: *mut i8 = read_field(unsafe { &mut *file }, c"name".as_ptr() as *mut i8);
    let command: *mut i8 = read_until_eol(unsafe { &mut *file });
    tree_add_command(c, key, name, command);
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn clear_lines(count: i32) -> () {
    unsafe {
        /// make sure we print directly to stdout without bufferring
        /// This allow us to clear lines without waiting for new line
        /// without it executing `system` will output lines then our output will go after
        unsafe {
            setbuf(__stdoutp, 0 as *mut () as *mut i8)
        };
        {
            let mut i: i32 = 0;
            '__b6: loop {
                if !(i < count) {
                    break '__b6;
                }
                '__c6: loop {
                    eprint!("[A\r[2K");
                    break '__c6;
                }
                i += 1;
            }
        }
    }
}

pub(crate) extern "C" fn load_file(c: *mut Command, file: *mut i8) -> () {
    let mut content: *mut i8 = read_file(file);
    while unsafe { *content } as i32 != 0 {
        read_line(c, &mut content);
    }
}

pub(crate) extern "C" fn start(mut c: *mut Command) -> () {
    while c as *mut () != 0 as *mut () && unsafe { (*c).children } as *mut () != 0 as *mut () {
        let last_printed_lines: i32 = print_command(unsafe { &*c });
        c = find_command(unsafe { &*c }, getch());
        clear_lines(last_printed_lines);
        if command_run(c) > 0 {
            return;
        }
    }
}
