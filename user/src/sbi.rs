/// We will write asm code (according to sbi spec ch3) to achieve the same functionality in Chapter 2.
#[must_use]
pub fn console_putchar(c: usize) -> bool {
    #[allow(deprecated)]
    let errno = sbi_rt::legacy::console_putchar(c);
    errno == 0
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{NoReason, Shutdown, SystemFailure, system_reset};
    if failure {
        system_reset(Shutdown, SystemFailure);
    } else {
        system_reset(Shutdown, NoReason);
    }
    unreachable!()
}
