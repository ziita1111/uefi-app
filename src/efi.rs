use core::ffi::c_void;

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
    pub StdErr: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub RuntimeServices: *mut EFI_RUNTIME_SERVICES,
    pub BootServices: *mut EFI_BOOT_SERVICES,
    pub NumberOfTableEntries: usize,
    pub ConfigurationTable: *mut EFI_CONFIGURATION_TABLE,
}

#[repr(C)]
pub struct EFI_RUNTIME_SERVICES {
	pub Hdr: EFI_TABLE_HEADER,
	pub GetTime: unsafe extern "C" fn(Time: *mut EFI_TIME, Capabilities: *mut EFI_TIME_CAPABILITIES) -> EFI_STATUS,
	dummy1: [usize;3], // Time Services
	dummy2: [usize;2], // Virtual Memory Services
	dummy3: [usize;3], // Variable Services
	dummy4: [usize;2], // Miscellaneous Services
	dummy5: [usize;2], // UEFI 2.0 Capsule Services
	dummy6: [usize;1], // Miscellaneous UEFI 2.0 Service
}

#[repr(C)]
pub struct EFI_BOOT_SERVICES {
	pub Hdr: EFI_TABLE_HEADER,
	dummy1: [usize;2], // Task Priority Services
	pub AllocatePages: unsafe extern "C" fn(Type: EFI_ALLOCATE_TYPE, MemoryType: EFI_MEMORY_TYPE, 
		Pages:usize, Memory: &mut u64) -> EFI_STATUS,
	dymmy21: [usize;1],
	pub GetMemoryMap: unsafe extern "C" fn(MemoryMapSize: &mut usize, MemoryMap: *mut EFI_MEMORY_DESCRIPTOR,  
		MapKey: &mut usize, DescriptorSize: &mut usize, DescriptorVersion: &mut u32) -> EFI_STATUS,
	dummy22: [usize;2], // Memory Services
	dummy3: [usize;6], // Event & Timer Services
	dummy4: [usize;8], // Protocol Handler Services
	dummy5: [usize;5], // Image Services
	dummy6: [usize;2], // Miscellaneous Services
	pub SetWatchdogTimer: unsafe extern "C" fn (Timeout: usize, WatchdogCode: u64, DataSize: usize, 
		WatchdogData: *const u16) -> EFI_STATUS,
	dummy7: [usize;2], // DriverSupport Services
	dummy8: [usize;3], // Open and Close Protocol Services
	dummy9: [usize;5], // Library Services
	dummy10: [usize;1], // 32-bit CRC Services
	dummy11: [usize;3], // Miscellaneous Services
}


#[repr(C)]
pub struct EFI_CONFIGURATION_TABLE {
	pub Hdr: EFI_TABLE_HEADER,
	// TBD
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

#[derive(PartialEq)]
#[repr(usize)]
pub enum EFI_ALLOCATE_TYPE {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[derive(PartialEq)]
#[repr(usize)]
pub enum EFI_MEMORY_TYPE {
	EfiReservedMemoryType,
	EfiLoaderCode,
	EfiLoaderData,
	EfiBootServicesCode,
	EfiBootServicesData,
	EfiRuntimeServicesCode,
	EfiRuntimeServicesData,
	EfiConventionalMemory,
	EfiUnusableMemory,
	EfiACPIReclaimMemory,
	EfiACPIMemoryNVS,
	EfiMemoryMappedIO,
	EfiMemoryMappedIOPortSpace,
	EfiPalCode,
	EfiPersistentMemory,
	EfiMaxMemoryType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct MemoryMapKey(usize);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EFI_MEMORY_DESCRIPTOR {
	pub Type: u32,
	padding: u32,
	pub PhysicalStart: u64,
	pub VirtualStart: u64,
	pub NumberOfPages: u64,
	pub Attribute: u64,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct EFI_TIME {
	pub Year: u16,
	pub Month: u8,
	pub Day: u8,
	pub Hour: u8,
	pub Minute: u8,
	pub Second: u8,
	pub Pad1: u8,
	pub Nanosecond: u32,
	pub TimeZone: u16,
	pub Daylight: u8,
	pub Pad2: u8,
}

#[repr(C)]
pub struct EFI_TIME_CAPABILITIES {
	pub Resolution: u32,
	pub Accuracy: u32,
	pub SetsToZero: bool,
}