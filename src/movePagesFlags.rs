// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


extern crate libc;
use self::libc::c_int;
use self::libc::c_void;
use self::libc::c_ulong;
use self::libc::pid_t;
use self::libc::E2BIG;
use self::libc::EACCES;
use self::libc::EFAULT;
use self::libc::EINVAL;
use self::libc::ENODEV;
use self::libc::ENOENT;
use self::libc::EPERM;
use self::libc::ESRCH;
extern crate errno;
use self::errno::errno;
use std::io::ErrorKind;


bitflags!
{
	pub flags MovePagesFlags: ::libc::c_int
	{
		#[allow(dead_code)] const MPOL_MF_STRICT = 1 << 0,
		#[allow(dead_code)] const MPOL_MF_MOVE = 1 << 1,
		#[allow(dead_code)] const MPOL_MF_MOVE_ALL = 1 << 2,
	}
}

impl MovePagesFlags
{
	/// pid can also be tid
	/// TODO: Consider using the smallvec optimisation here
	pub fn move_pages(&self, pid: pid_t, pages: &mut Vec<*mut c_void>, nodes: &mut Vec<c_int>) -> Result<Vec<c_int>, ErrorKind>
	{
		let count = pages.len();
		debug_assert!(count == nodes.len(), "pages length {} is not equal to nodes length {}", count, nodes.len());
		
		let mut status: Vec<c_int> = vec![0 as c_int; count];
		
		match unsafe { numa_move_pages(pid as c_int, count as c_ulong, pages.as_mut_ptr(), nodes.as_ptr(), status.as_mut_ptr(), self.bits) }
		{
			0 => Ok(status),
			-1 => match errno().0
			{
				E2BIG => Err(ErrorKind::InvalidInput),
				EACCES => Err(ErrorKind::PermissionDenied),
				EFAULT => panic!("EFAULT for numa_move_pages"),
				EINVAL => panic!("Flags other than MPOL_MF_MOVE and MPOL_MF_MOVE_ALL were specified or an attempt was made to migrate pages of a kernel thread (EINVAL)"),
				ENODEV => Err(ErrorKind::PermissionDenied),
				ENOENT => Err(ErrorKind::InvalidData),
				EPERM => Err(ErrorKind::PermissionDenied),
				ESRCH => Err(ErrorKind::NotFound),
				unexpected @ _ => panic!("Did not expect numa_move_pages to set errno {}", unexpected),
			},
			unexpected @ _ => panic!("Did not expect numa_move_pages to return {}", unexpected),
		}
	}
}

extern "C"
{
	// numa_move_pages just wraps move_pages
	//fn move_pages(pid: c_int, count: c_ulong, pages: *mut *mut c_void, nodes: *const c_int, status: *mut c_int, flags: c_int) -> c_long;
	fn numa_move_pages(pid: c_int, count: c_ulong, pages: *mut *mut c_void, nodes: *const c_int, status: *mut c_int, flags: c_int) -> c_int;
}
