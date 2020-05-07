use crate::prelude::*;

use core::mem::MaybeUninit;
use core::ptr;

pub extern "C" fn get_time() -> EFI_TIME {
	let mut time = MaybeUninit::<EFI_TIME>::uninit();
	unsafe {
		((*RUNTIME_SERVICES).GetTime)(time.as_mut_ptr(), ptr::null_mut());
		*time.as_ptr() as EFI_TIME
	}
}