#![no_std]
#![no_main]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use core::panic::PanicInfo;

use uefi_bootloader::prelude::*;
use uefi_bootloader::init::*;
use uefi_bootloader::mm::*;
use uefi_bootloader::file::*;

use core::ptr;

#[no_mangle]
pub extern "C" fn efi_main(image: EFI_HANDLE, st: &EFI_SYSTEM_TABLE) -> EFI_STATUS {
	efi_init(image, &st);
	mm_init();
	let buf: [u16;1000] = read_file("configure");
	print_u16(&buf);
	loop{

	}
    EFI_STATUS::SUCCESS
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	efi_println!("panic!");
    loop {}
}