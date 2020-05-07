#![no_std]
#![no_main]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use core::panic::PanicInfo;

use uefi_bootloader::prelude::*;
use uefi_bootloader::init::*;
use uefi_bootloader::console::*;
use uefi_bootloader::mm::*;

use core::mem::MaybeUninit;
use core::ptr;

#[no_mangle]
pub extern "C" fn efi_main(image: &EFI_HANDLE, st: &EFI_SYSTEM_TABLE) -> EFI_STATUS {
	efi_init(&st);
	efi_print_clear();
	efi_println!("start efi_main");

	let mut memory = 0x7fffffff;
	// efi_print("before allocate page\n", &st);

	let mut buffer = [0_u8;10000];
	let mut map_size = buffer.len();
	let map_buffer: *mut EFI_MEMORY_DESCRIPTOR = buffer.as_ptr() as *mut EFI_MEMORY_DESCRIPTOR;
	let mut map_key = 0;
    let mut entry_size = 0;
    let mut entry_version = 0;

	unsafe {
		((*BOOT_SERVICES).GetMemoryMap)(
			&mut map_size,
			map_buffer,
			&mut map_key,
			&mut entry_size,
			&mut entry_version,
		);
		((*BOOT_SERVICES).SetWatchdogTimer)(0,0,0,ptr::null());
	}
	mm_init();
	// efi_print("allocate page\n", &st);
	efi_println!("end efi_main");
	loop{

	}
    EFI_STATUS::SUCCESS
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}