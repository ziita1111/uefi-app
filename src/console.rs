use crate::prelude::*;
use core::fmt;

pub struct Console;

impl Console {
    pub fn new() -> Console {
        Console{}
    }

    pub extern "C" fn efi_print(&self, s: &str) -> EFI_STATUS{
        let stdout: &mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL = unsafe { &mut *((*SYSTEM_TABLE).ConOut) };
        let string = s.as_bytes();
        let mut buf = [0u16; 100];

        for i in 0..string.len() {
            buf[i] = string[i] as u16;
        }

        unsafe {
            (stdout.OutputString)(stdout, buf.as_ptr());
        }

        EFI_STATUS::SUCCESS
    }
}

pub extern "C" fn efi_print_clear() -> EFI_STATUS {
    let stdout: &mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL = unsafe { &mut *((*SYSTEM_TABLE).ConOut) };
    unsafe {
        (stdout.Reset)(stdout, false);
    }
    EFI_STATUS::SUCCESS
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.efi_print(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe{ (*CON).write_fmt(args); }
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! efi_print {
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! efi_println {
    () => ($crate::efi_print!("\r\n"));
    ($fmt:expr) => ($crate::efi_print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::efi_print!(
        concat!($fmt, "\r\n"), $($arg)*));
}