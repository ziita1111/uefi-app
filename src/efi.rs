use core::ffi::c_void;

const ERROR_BIT: usize = 1 << (core::mem::size_of::<usize>() * 8 - 1);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EFI_HANDLE(*mut c_void);

#[derive(PartialEq, Debug)]
#[repr(usize)]
pub enum EFI_STATUS {
    /// The operation completed successfully.
    SUCCESS                 =  0,

    /// The string contained characters that could not be rendered and were skipped.
    WARN_UNKNOWN_GLYPH      =  1,
    /// The handle was closed, but the file was not deleted.
    WARN_DELETE_FAILURE     =  2,
    /// The handle was closed, but the data to the file was not flushed properly.
    WARN_WRITE_FAILURE      =  3,
    /// The resulting buffer was too small, and the data was truncated.
    WARN_BUFFER_TOO_SMALL   =  4,
    /// The data has not been updated within the timeframe set by local policy.
    WARN_STALE_DATA         =  5,
    /// The resulting buffer contains UEFI-compliant file system.
    WARN_FILE_SYSTEM        =  6,
    /// The operation will be processed across a system reset.
    WARN_RESET_REQUIRED     =  7,

    /// The image failed to load.
    LOAD_ERROR              = ERROR_BIT |  1,
    /// A parameter was incorrect.
    INVALID_PARAMETER       = ERROR_BIT |  2,
    /// The operation is not supported.
    UNSUPPORTED             = ERROR_BIT |  3,
    /// The buffer was not the proper size for the request.
    BAD_BUFFER_SIZE         = ERROR_BIT |  4,
    /// The buffer is not large enough to hold the requested data.
    /// The required buffer size is returned in the appropriate parameter.
    BUFFER_TOO_SMALL        = ERROR_BIT |  5,
    /// There is no data pending upon return.
    NOT_READY               = ERROR_BIT |  6,
    /// The physical device reported an error while attempting the operation.
    DEVICE_ERROR            = ERROR_BIT |  7,
    /// The device cannot be written to.
    WRITE_PROTECTED         = ERROR_BIT |  8,
    /// A resource has run out.
    OUT_OF_RESOURCES        = ERROR_BIT |  9,
    /// An inconstency was detected on the file system.
    VOLUME_CORRUPTED        = ERROR_BIT | 10,
    /// There is no more space on the file system.
    VOLUME_FULL             = ERROR_BIT | 11,
    /// The device does not contain any medium to perform the operation.
    NO_MEDIA                = ERROR_BIT | 12,
    /// The medium in the device has changed since the last access.
    MEDIA_CHANGED           = ERROR_BIT | 13,
    /// The item was not found.
    NOT_FOUND               = ERROR_BIT | 14,
    /// Access was denied.
    ACCESS_DENIED           = ERROR_BIT | 15,
    /// The server was not found or did not respond to the request.
    NO_RESPONSE             = ERROR_BIT | 16,
    /// A mapping to a device does not exist.
    NO_MAPPING              = ERROR_BIT | 17,
    /// The timeout time expired.
    TIMEOUT                 = ERROR_BIT | 18,
    /// The protocol has not been started.
    NOT_STARTED             = ERROR_BIT | 19,
    /// The protocol has already been started.
    ALREADY_STARTED         = ERROR_BIT | 20,
    /// The operation was aborted.
    ABORTED                 = ERROR_BIT | 21,
    /// An ICMP error occurred during the network operation.
    ICMP_ERROR              = ERROR_BIT | 22,
    /// A TFTP error occurred during the network operation.
    TFTP_ERROR              = ERROR_BIT | 23,
    /// A protocol error occurred during the network operation.
    PROTOCOL_ERROR          = ERROR_BIT | 24,
    /// The function encountered an internal version that was
    /// incompatible with a version requested by the caller.
    INCOMPATIBLE_VERSION    = ERROR_BIT | 25,
    /// The function was not performed due to a security violation.
    SECURITY_VIOLATION      = ERROR_BIT | 26,
    /// A CRC error was detected.
    CRC_ERROR               = ERROR_BIT | 27,
    /// Beginning or end of media was reached
    END_OF_MEDIA            = ERROR_BIT | 28,
    /// The end of the file was reached.
    END_OF_FILE             = ERROR_BIT | 31,
    /// The language specified was invalid.
    INVALID_LANGUAGE        = ERROR_BIT | 32,
    /// The security status of the data is unknown or compromised and
    /// the data must be updated or replaced to restore a valid security status.
    COMPROMISED_DATA        = ERROR_BIT | 33,
    /// There is an address conflict address allocation
    IP_ADDRESS_CONFLICT     = ERROR_BIT | 34,
    /// A HTTP error occurred during the network operation.
    HTTP_ERROR              = ERROR_BIT | 35,
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
	// Task Priority Services
	dummy1: [usize;2], 
	pub AllocatePages: unsafe extern "C" fn(Type: EFI_ALLOCATE_TYPE, MemoryType: EFI_MEMORY_TYPE, 
		Pages:usize, Memory: &mut u64) -> EFI_STATUS,

	// Memory Services
	dymmy2a: [usize;1], 
	pub GetMemoryMap: unsafe extern "C" fn(MemoryMapSize: &mut usize, MemoryMap: *mut EFI_MEMORY_DESCRIPTOR,  
		MapKey: &mut usize, DescriptorSize: &mut usize, DescriptorVersion: &mut u32) -> EFI_STATUS,
	dummy2b: [usize;2],

	// Event & Timer Services
	dummy3: [usize;6], 

	// Protocol Handler Services
	dummy4a: [usize;3], 
	pub HandleProtocol: unsafe extern "C" fn(Handle: EFI_HANDLE, Protocol: &EFI_GUID, Interface: &mut *mut c_void) -> EFI_STATUS,
	dummy4b: [usize;4], 

	// Image Services
	dummy5: [usize;5], 

	// Miscellaneous Services
	dummy6: [usize;2], 
	pub SetWatchdogTimer: unsafe extern "C" fn (Timeout: usize, WatchdogCode: u64, DataSize: usize, 
		WatchdogData: *const u16) -> EFI_STATUS,

	// DriverSupport Services
	dummy7: [usize;2], 

	// Open and Close Protocol Services
	dummy8: [usize;3], 

	// Library Services
	dummy9a: [usize;2], 
	pub LocateProtocol: unsafe extern "C" fn (Protocol: &EFI_GUID, Registration: *mut c_void, Interface: &mut *mut c_void) -> EFI_STATUS,
	dummy9b: [usize;2], 

	// 32-bit CRC Services
	dummy10: [usize;1], 

	// Miscellaneous Services
	dummy11: [usize;3], 
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct EFI_GUID {
	pub a: u32,
	pub b: u16,
	pub c: u16,
	pub d: [u8;8],
}

#[repr(C)]
pub struct EFI_SIMPLE_FILE_SYSTEM_PROTOCOL {
	pub Revision: u64,
	pub OpenVolume: unsafe extern "C" fn(This: &mut EFI_SIMPLE_FILE_SYSTEM_PROTOCOL, Root: &mut *mut  EFI_FILE_PROTOCOL) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_FILE_PROTOCOL {
	pub Revision: u64,
	pub Open: unsafe extern "C" fn(This: &EFI_FILE_PROTOCOL, NewHandle: &mut *mut EFI_FILE_PROTOCOL, FileName: *const u16,
		OpenMode: EFI_FILE_MODE, Attributes: EFI_FILE_ATTRIBUTE) -> EFI_STATUS,
	pub Close: unsafe extern "C" fn(This: &EFI_FILE_PROTOCOL) -> EFI_STATUS,
	pub Delete: unsafe extern "C" fn(This: &EFI_FILE_PROTOCOL) -> EFI_STATUS,
	pub Read: unsafe extern "C" fn(This: &EFI_FILE_PROTOCOL, BufferSize: &mut usize, Buffer: *mut u8) -> EFI_STATUS,
	// TBD
}

#[repr(C)]
pub struct EFI_LOADED_IMAGE_PROTOCOL {
	pub Revision: u32,
	pub ParentHandle: EFI_HANDLE,
	pub SystemTable: *const EFI_SYSTEM_TABLE,
	pub DeviceHandle: EFI_HANDLE,
	pub FilePath: *const c_void,
	pub Reserved: *const c_void,
	pub LoadOptionsSize: u32,
	pub LoadOptions: *const c_void,
	pub ImageBase: usize,
	pub ImageSize: u64,
	pub ImageCodeType: EFI_MEMORY_TYPE,
	pub ImageDataType: EFI_MEMORY_TYPE,
	pub Unload: unsafe extern "C" fn(ImageHandle: EFI_HANDLE) -> EFI_STATUS,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u64)]
pub enum EFI_FILE_MODE {
    EFI_FILE_MODE_READ = 1,
    EFI_FILE_MODE_WRITE = 2 | 1,
    EFI_FILE_MODE_CREATE = (1 << 63) | 2 | 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u64)]
pub enum EFI_FILE_ATTRIBUTE {
	EFI_FILE_READ_ONLY = 1,
	EFI_FILE_HIDDEN = 1 << 1,
	EFI_FILE_SYSTEM = 1 << 2,
	EFI_FILE_RESERVED = 1 << 3,
	EFI_FILE_DIRECTORY = 1 << 4,
	EFI_FILE_ARCHIVE = 1 << 5,
	EFI_FILE_VALID_ATTR = 0x37,
}