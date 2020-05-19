use crate::prelude::*;
use core::ptr;

pub struct FILE<'a> {
	name: &'a str,
	buf: [u16;1000],
	nf: *mut EFI_FILE_PROTOCOL
}

impl<'a> FILE<'a> {
	pub fn new(name: &'a str) -> FILE {
		let mut f = FILE {
			name: name,
			buf: [0;1000],
			nf: ptr::null_mut(),
		};
		f.open();
		f.read();
		f
	}

	fn open(&mut self) -> EFI_STATUS{
		let name = str_to_u16(self.name);
		unsafe{
			((*ROOT).Open)(&(*ROOT), &mut self.nf, name.as_ptr(), 
				EFI_FILE_MODE::EFI_FILE_MODE_READ, 
				EFI_FILE_ATTRIBUTE::EFI_FILE_READ_ONLY
				)
		}
	}

	fn read(&mut self){
		let mut idx = 0;
		let mut buf_size = 1000;
		let mut buf = [0_u8;1000];
		let ret = unsafe {
			((*self.nf).Read)(&(*self.nf), &mut buf_size, buf.as_mut_ptr())
		};
		if ret!=EFI_STATUS::SUCCESS{
			efi_println!("(*nf).Read: {:?}", ret);
			panic!();
		}
		for i in 0..buf_size{
			if buf[i]=='\n' as u8{
				self.buf[idx] = '\r' as u16;
				idx += 1;
			}
			self.buf[idx] = buf[i] as u16;
			idx += 1;
		}
	}

	pub fn show(&self) {
		print_u16(&self.buf);
	}
	
}