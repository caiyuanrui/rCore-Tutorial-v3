use core::arch::asm;

pub unsafe fn print_trace_stack() {
    let mut fp: *const usize;
    unsafe {
        asm!("mv {}, fp", out(reg) fp);
    }
    println!("== Begin stack trace ==");
    while !fp.is_null() {
        let saved_ra = unsafe { *fp.sub(1) };
        let saved_fp = unsafe { *fp.sub(2) };
        println!("0x{:016x}, fp=0x{:016x}", saved_ra, saved_fp);
        fp = saved_fp as *const _;
    }
    println!("== End stack trace ==");
}
