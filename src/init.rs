use crate::prelude::*;
use core::ffi::c_void;
use core::ptr;

pub static mut SYSTEM_TABLE: *const EFI_SYSTEM_TABLE = 0 as *const _;
pub static mut BOOT_SERVICES: *const EFI_BOOT_SERVICES = 0 as *const _;
pub static mut RUNTIME_SERVICES: *const EFI_RUNTIME_SERVICES = 0 as *const _;
pub static mut CON: *mut Console = 0 as *mut Console;

pub static mut LIP: *mut EFI_LOADED_IMAGE_PROTOCOL = 0 as *mut EFI_LOADED_IMAGE_PROTOCOL;
pub static mut SFSP: *mut EFI_SIMPLE_FILE_SYSTEM_PROTOCOL = 0 as *mut EFI_SIMPLE_FILE_SYSTEM_PROTOCOL;
pub static mut ROOT: *mut EFI_FILE_PROTOCOL = 0 as *mut EFI_FILE_PROTOCOL;

pub static simple_file_system_protocol_guid: EFI_GUID = EFI_GUID { 
	a: 0x0964e5b22, 
	b: 0x6459, 
	c: 0x11d2,
	d: [0x8e, 0x39, 0x00, 0xa0,
	0xc9, 0x69, 0x72, 0x3b]
};

pub static loaded_image_protocol_guid: EFI_GUID = EFI_GUID {
	a: 0x5B1B31A1,
	b: 0x9562,
	c: 0x11d2,
	d: [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

pub extern "C" fn efi_init(image: EFI_HANDLE, st: &EFI_SYSTEM_TABLE) {
	unsafe {
		SYSTEM_TABLE = st;
		BOOT_SERVICES = &mut *(st.BootServices);
		RUNTIME_SERVICES = &mut *(st.RuntimeServices);
		CON = &mut Console::new();
	}

	unsafe{ (*CON).efi_print_clear(); }

	unsafe{ ((*BOOT_SERVICES).SetWatchdogTimer)(0,0,0,ptr::null()); }

	// efi_println!("efi_init");

	unsafe {
		let mut fp = ptr::null_mut();
		let ret = ((*BOOT_SERVICES).HandleProtocol)(image, &loaded_image_protocol_guid, &mut fp);
		LIP = fp as *mut EFI_LOADED_IMAGE_PROTOCOL;
		if ret!=EFI_STATUS::SUCCESS{
			efi_println!("(*BOOT_SERVICES).HandleProtocol: {:?}", ret);
			panic!();
		}
		efi_println!("LIP(EFI_LOADED_IMAGE_PROTOCOL): {:?}", LIP);
	}

	unsafe{
		let mut fp = ptr::null_mut();
		let ret = ((*BOOT_SERVICES).HandleProtocol)((*LIP).DeviceHandle, &simple_file_system_protocol_guid, &mut fp);
		if ret!=EFI_STATUS::SUCCESS{
			efi_println!("(*BOOT_SERVICES).HandleProtocol: {:?}", ret);
			panic!();
		}
		SFSP = fp as *mut EFI_SIMPLE_FILE_SYSTEM_PROTOCOL;
		efi_println!("SFSP(EFI_SIMPLE_FILE_SYSTEM_PROTOCOL): {:?}", SFSP);
	}

	unsafe{
		let ret = ((*SFSP).OpenVolume)(&mut (*SFSP), &mut ROOT);
		if ret!=EFI_STATUS::SUCCESS{
			efi_println!("(*SFSP).OpenVolume: {:?}", ret);
			panic!();
		}
		efi_println!("ROOT(EFI_FILE_PROTOCOL): {:?}", ROOT);
	}
}