use log::error;

use crate::batch::{get_current_app_range, get_user_stack_range};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if !check_range(buf, len) {
        error!("[kernel] Buffer is out of range");
        return -1;
    }

    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let s = core::str::from_utf8(slice).unwrap();
            print!("{}", s);
            len as isize
        }
        _ => {
            error!("[kernel] Unsupported fd in sys_write!");
            -1
        }
    }
}

/// Check if the `buf` is located within the range of data section or user stack section
fn check_range(buf: *const u8, len: usize) -> bool {
    let app_range = get_current_app_range();
    let user_stack_range = get_user_stack_range();
    let buf_range = (buf as usize, unsafe { buf.add(len) } as usize);

    (is_within(buf_range.0, app_range) && is_within(buf_range.1, app_range))
        || (is_within(buf_range.0, user_stack_range) && is_within(buf_range.1, user_stack_range))
}

#[inline(always)]
fn is_within(addr: usize, range: (usize, usize)) -> bool {
    addr >= range.0 && addr < range.1
}
