use crate::prelude::*;

use core::intrinsics::transmute;

static MEMORY_MAP_SIZE: usize = 0x3000;
static EFI_MAX_USABLE_ADDRESS: u64 = 0xffffffff;

extern "C" fn efi_allocate_pages_real(address: u64, pages: usize, 
	alloctype: EFI_ALLOCATE_TYPE, memtype: EFI_MEMORY_TYPE) -> Result<u64, ()> {
	let mut memory = address;
	unsafe{
		match ((*BOOT_SERVICES).AllocatePages)(alloctype, memtype ,pages, &mut memory) {
			EFI_STATUS::SUCCESS => Ok(memory),
			_ => Err(())
		}
	}
}

extern "C" fn efi_allocate_any_pages(pages: usize) -> Result<u64, ()> {
	efi_allocate_pages_real(
		EFI_MAX_USABLE_ADDRESS, 
		pages, 
		EFI_ALLOCATE_TYPE::AllocateMaxAddress, 
		EFI_MEMORY_TYPE::EfiLoaderData
		)
}

extern "C" fn efi_get_memory_map(map_size: &mut usize, memory_map:  *mut EFI_MEMORY_DESCRIPTOR, 
	map_key: &mut usize, desc_size: &mut usize, desc_ver: &mut u32) -> Result<(),()> {
	unsafe {
		match ((*BOOT_SERVICES).GetMemoryMap)(map_size, memory_map, map_key, desc_size, desc_ver) {
			EFI_STATUS::SUCCESS => Ok(()),
			_ => Err(())
		}
	}
}

pub extern "C" fn mm_init() {
	let memory_map : *mut EFI_MEMORY_DESCRIPTOR = match efi_allocate_any_pages(2*((MEMORY_MAP_SIZE+0xfff)>>12)) {
		Ok(val) => {
			efi_println!("memory_map: {:x}", val);
			unsafe{ transmute::<u64, *mut EFI_MEMORY_DESCRIPTOR>(val) }
		},
		Err(_) => {
			efi_println!("memory allocation failed");
			panic!()
		},
	};

	let mut map_size = MEMORY_MAP_SIZE;
	let mut map_key = 0;
	let mut desc_size = 0;
	let mut desc_ver = 0;
	match efi_get_memory_map(&mut map_size, memory_map, &mut map_key, &mut desc_size, &mut desc_ver) {
		Ok(_) => {
			efi_println!("map size: {:x}", map_size);
		},
		Err(_) => {
			efi_println!("get memory map failed");
			panic!()
		}
	};
}