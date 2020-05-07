use crate::efi::*;
use crate::console::*;

pub static mut SYSTEM_TABLE: *const EFI_SYSTEM_TABLE = 0 as *const _;
pub static mut BOOT_SERVICES: *const EFI_BOOT_SERVICES = 0 as *const _;
pub static mut RUNTIME_SERVICES: *const EFI_RUNTIME_SERVICES = 0 as *const _;
pub static mut CON: *mut Console = 0 as *mut Console;

pub extern "C" fn efi_init(st: &EFI_SYSTEM_TABLE) {
	unsafe {
		SYSTEM_TABLE = st;
		BOOT_SERVICES = &mut *(st.BootServices);
		RUNTIME_SERVICES = &mut *(st.RuntimeServices);
		CON = &mut Console::new();
	}
}