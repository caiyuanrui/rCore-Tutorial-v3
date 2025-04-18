#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;
extern crate core;
use core::slice;
use user_lib::{STDOUT, write};

const STACK_SIZE: usize = 0x1000;

unsafe fn r_sp() -> usize {
    let mut sp: usize;
    unsafe {
        asm!("mv {}, sp", out(reg) sp);
    }
    sp
}

unsafe fn stack_range() -> (usize, usize) {
    let sp = unsafe { r_sp() };
    let top = (sp + STACK_SIZE - 1) & (!(STACK_SIZE - 1));
    (top - STACK_SIZE, top)
}

/// 正确输出：
/// Test write0 OK!
#[unsafe(no_mangle)]
pub fn main() -> i32 {
    assert_eq!(
        write(STDOUT, unsafe {
            #[allow(clippy::zero_ptr)]
            slice::from_raw_parts(0x0 as *const _, 10)
        }),
        -1
    );
    let (bottom, top) = unsafe { stack_range() };
    // println!("[user] stack range: ({:#x}, {:#x})", bottom, top);

    assert_eq!(
        write(STDOUT, unsafe {
            slice::from_raw_parts((top - 5) as *const _, 10)
        }),
        -1
    );
    // I'm not sure if it's a correct test case
    // because the user stack size is 8KB in OS but here it's defined as 4KB
    assert_eq!(
        write(STDOUT, unsafe {
            slice::from_raw_parts((bottom - 5 - 0x1000) as *const _, 10)
        }),
        -1
    );
    // TODO: test string located in .data section
    println!("Test write0 OK!");
    0
}
