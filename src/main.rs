#![no_std]
#![no_main]

#[macro_use]
mod console;

mod lang_items;
mod logging;
mod sbi;

use core::arch::global_asm;

use log::info;
use sbi::shutdown;
global_asm!(include_str!("entry.asm"));

unsafe extern "C" {
    fn sbss();
    fn ebss();
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn boot_stack_lower_bound();
    fn boot_stack_top();
}

#[unsafe(no_mangle)]
pub fn rust_main() {
    clear_bss();
    logging::init().expect("kernel logger init failed");
    info!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize, etext as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    info!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    info!(
        "[kernel] .stack [{:#x}, {:#x})",
        boot_stack_lower_bound as usize, boot_stack_top as usize
    );
    shutdown(false)
}

pub fn clear_bss() {
    unsafe {
        assert!(
            (ebss as usize - sbss as usize) % 8 == 0,
            "misaligned bss section"
        );
        core::ptr::write_bytes(sbss as *mut u8, 0, (ebss as usize - sbss as usize) / 8);
    }
}
