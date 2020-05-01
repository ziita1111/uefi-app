#![no_std]
#![no_main]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EFI_HANDLE(*mut c_void);

#[derive(PartialEq)]
#[repr(usize)]
pub enum EFI_STATUS {
    SUCCESS                 =  0,
}

#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    pub Hdr: EFI_TABLE_HEADER,
    pub FirmwareVendor: *const u16,
    pub FirmwareRevision: u32,
    pub ConsoleInHandle: EFI_HANDLE,
    pub ConIn: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    pub ConsoleOutHandle: EFI_HANDLE,
    pub ConOut: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub StandardErrorHandle: EFI_HANDLE,
    pub StdErr: EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,// TBD
}

#[repr(C)]
pub struct EFI_TABLE_HEADER {
	pub Signature: u64,
	pub Revision: u32,
	pub HeaderSize: u32,
	pub CRC32: u32,
	pub Reserved: u32,
}

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    pub Reset: unsafe extern "C" fn(This: &EFI_SIMPLE_TEXT_INPUT_PROTOCOL, ExtendedVerification: bool) -> EFI_STATUS,
    pub ReadKeyStroke: unsafe extern "C" fn(This: &EFI_SIMPLE_TEXT_INPUT_PROTOCOL, Key: &EFI_INPUT_KEY) -> EFI_STATUS,
    // TBD	
}

#[repr(C)]
pub struct EFI_INPUT_KEY {
	pub ScanCode: u16,
	pub UnicodeChar: u16,
}

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset: unsafe extern "C" fn(This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: bool) -> EFI_STATUS,
    pub OutputString: unsafe extern "C" fn(This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *const u16) -> EFI_STATUS,
    // TBD	
}


#[no_mangle]
pub extern "C" fn efi_main(image: EFI_HANDLE, st: EFI_SYSTEM_TABLE) -> EFI_STATUS {
    EFI_STATUS::SUCCESS
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}