use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            if !console_putchar(c as usize) {
                return Err(core::fmt::Error);
            }
        }
        Ok(())
    }
}

/// # Panics
/// Panics if writing to debug console fails.
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg:tt)+)?) => {
      $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg:tt)+)?) => {
      $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}
