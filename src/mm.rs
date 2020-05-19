use crate::prelude::*;

use core::intrinsics::transmute;

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
		0, 
		pages, 
		EFI_ALLOCATE_TYPE::AllocateAnyPages, 
		EFI_MEMORY_TYPE::EfiLoaderData
		)
}

extern "C" fn efi_get_memory_map(mut map_size: &mut usize, memory_map:  *mut EFI_MEMORY_DESCRIPTOR, 
	mut map_key: &mut usize, mut desc_size: &mut usize, mut desc_ver: &mut u32) -> Result<(),()> {
	unsafe {
		match ((*BOOT_SERVICES).GetMemoryMap)(&mut map_size, memory_map, &mut map_key, &mut desc_size, &mut desc_ver) {
			EFI_STATUS::SUCCESS => Ok(()),
			_ => Err(())
		}
	}
}

pub extern "C" fn show_memory_map() {
	let pages = 2;
	let memory_map : *mut EFI_MEMORY_DESCRIPTOR = match efi_allocate_any_pages(pages) {
		Ok(val) => {
			efi_println!("memory_map: {:x}", val);
			unsafe{ transmute::<u64, *mut EFI_MEMORY_DESCRIPTOR>(val) }
		},
		Err(_) => {
			efi_println!("memory allocation failed");
			panic!()
		},
	};

	let mut map_size = pages*4096;
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

	let mut p = memory_map as *mut u8;
	for i in 0..map_size/desc_size{
		unsafe{ 
			efi_println!("{} ", (*(p as *mut EFI_MEMORY_DESCRIPTOR)));
		}
		unsafe{ p = p.add(desc_size); }
	}
}