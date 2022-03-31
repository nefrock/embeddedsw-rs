extern crate embeddedsw_sys;
use core::fmt::{self, Write};
use embeddedsw_sys as esys;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::xil_printf::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n\r")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n\r"), $($arg)*));
}

struct UartWriter;

pub fn _print(args: fmt::Arguments) {
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
