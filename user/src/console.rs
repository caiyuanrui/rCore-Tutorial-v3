use core::fmt::{self, Write};

use crate::write;

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(STDOUT, s.as_bytes());
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
