#![no_std]
#![no_main]

#[macro_use]
mod console;

mod lang_items;
mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() {
    clear_bss();
    println!("Hello world");
    panic!("oops");
}

pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        assert!(
            (ebss as usize - sbss as usize) % 8 == 0,
            "misaligned bss section"
        );
        core::ptr::write_bytes(sbss as *mut u8, 0, (ebss as usize - sbss as usize) / 8);
    }
}
