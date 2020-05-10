use crate::prelude::*;
use core::ptr;

pub extern "C" fn read_file(s: &str) -> [u16;1000] {
	let mut nf: *mut EFI_FILE_PROTOCOL = ptr::null_mut();
	let mut ret_buf = [0_u16;1000];

	unsafe{
		let buf = str_to_u16(s);
		let ret = ((*ROOT).Open)(&(*ROOT), &mut nf, buf.as_ptr(), 
			EFI_FILE_MODE::EFI_FILE_MODE_READ, 
			EFI_FILE_ATTRIBUTE::EFI_FILE_READ_ONLY
			);
		if ret!=EFI_STATUS::SUCCESS{
			efi_println!("ROOT.Open: {:?}", ret);
			panic!();
		}
	}

	unsafe{
		let mut idx = 0;
		loop{
			let mut buf_size = 1000;
			let mut buf = [0_u8;1000];
			let ret = ((*nf).Read)(&(*nf), &mut buf_size, buf.as_mut_ptr());
			if ret!=EFI_STATUS::SUCCESS{
				efi_println!("(*nf).Read: {:?}", ret);
				panic!();
			}
			if buf_size==0 {
				break;
			}
			for i in 0..buf_size{
				if buf[i]=='\n' as u8{
					ret_buf[idx] = '\r' as u16;
					idx += 1;
				}
				ret_buf[idx] = buf[i] as u16;
				idx += 1;
			}
		}		
	}

	ret_buf
}