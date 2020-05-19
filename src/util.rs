use crate::prelude::*;

pub extern "C" fn str_to_u16(s: &str) -> [u16;50] {
	let string = s.as_bytes();
    let mut buf = [0u16; 50];

    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }
    buf
}

pub extern "C" fn print_u8(s: &[u8]) {
	unsafe{
		(*CON).efi_print_u8(s);
	}
}

pub extern "C" fn print_u16(s: &[u16]) {
	unsafe{
		(*CON).efi_print_u16(s);
	}
}