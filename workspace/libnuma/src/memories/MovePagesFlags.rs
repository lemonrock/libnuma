// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


pub mod MovePagesFlags
{
	use ::errno::errno;
	use ::libc::c_int;
	use ::libc::c_void;
	use ::libc::c_ulong;
	use ::libc::pid_t;
	use ::libnuma_sys::*;
	use ::std::io::ErrorKind;
	
	bitflags!
	{
		pub flags Flags: ::libc::c_uint
		{
			const MPOL_MF_STRICT = 1 << 0,
			const MPOL_MF_MOVE = 1 << 1,
			const MPOL_MF_MOVE_ALL = 1 << 2,
		}
	}
	
	impl Flags
	{
		//noinspection SpellCheckingInspection
		/// pid can also be tid
		/// TODO: Consider using the smallvec optimisation here
		pub fn move_pages(&self, pid: pid_t, pages: &mut Vec<*mut c_void>, nodes: &mut Vec<c_int>) -> Result<Vec<c_int>, ErrorKind>
		{
			use ::std::io::ErrorKind::*;
			
			let count = pages.len();
			debug_assert!(count == nodes.len(), "pages length {} is not equal to nodes length {}", count, nodes.len());
			
			let mut status: Vec<c_int> = vec![0 as c_int; count];
			
			match unsafe { numa_move_pages(pid as c_int, count as c_ulong, pages.as_mut_ptr(), nodes.as_ptr(), status.as_mut_ptr(), self.bits as c_int) }
			{
				0 => Ok(status),
				-1 => match errno().0
				{
					::libc::E2BIG => Err(InvalidInput),
					::libc::EACCES => Err(PermissionDenied),
					::libc::EFAULT => panic!("EFAULT for numa_move_pages"),
					::libc::EINVAL => panic!("Flags other than MPOL_MF_MOVE and MPOL_MF_MOVE_ALL were specified or an attempt was made to migrate pages of a kernel thread (EINVAL)"),
					::libc::ENODEV => Err(PermissionDenied),
					::libc::ENOENT => Err(InvalidData),
					::libc::EPERM => Err(PermissionDenied),
					::libc::ESRCH => Err(NotFound),
					unexpected @ _ => panic!("Did not expect numa_move_pages to set errno {}", unexpected),
				},
				unexpected @ _ => panic!("Did not expect numa_move_pages to return {}", unexpected),
			}
		}
	}
}
