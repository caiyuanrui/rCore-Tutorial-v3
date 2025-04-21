#![no_std]
#![feature(linkage)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

pub use console::{STDIN, STDOUT};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    unreachable!()
}

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::ptr::write_bytes(sbss as *mut u8, 0, ebss as usize - sbss as usize);
    }
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    unimplemented!()
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn yield_() -> isize {
    sys_yield()
}

pub fn get_taskinfo() -> isize {
    sys_get_taskinfo()
}
