extern crate embeddedsw_sys;
use core::fmt::{self, Write};
use embeddedsw_sys as esys;

/// Print to the UART
/// This macro uses Rust's formatter, so it can print floating point numbers.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::xil_printf::_print(format_args!($($arg)*)));
}

/// Print to the UART, with a newline
/// This macro uses Rust's formatter, so it can print floating point numbers.
#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n\r")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n\r"), $($arg)*));
}

#[derive(Debug)]
struct UartWriter;

fn _print(args: fmt::Arguments) {
    let mut writer = UartWriter {};
    writer.write_fmt(args).unwrap();
}

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            unsafe {
                esys::outbyte(c);
            };
        }
        Ok(())
    }
}
