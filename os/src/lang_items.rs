use core::panic::PanicInfo;

use crate::{sbi::shutdown, stack_trace::print_trace_stack};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{}:{} {}",
            location.file(),
            location.line(),
            location.column(),
            info.message()
        );
    } else {
        println!("Panicked: {}", info.message());
    }
    unsafe { print_trace_stack() };
    shutdown(true)
}
