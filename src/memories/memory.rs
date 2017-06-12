// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


use std::io::ErrorKind;
use std::ops::Deref;
use ::libc::c_void;
use ::libc::size_t;
use ::libc::c_int;
use ::libc::c_long;
use ::libc::c_ulong;
use ::libc::c_uint;
use ::libc::EFAULT;
use ::libc::EINVAL;
use ::libc::EIO;
use ::libc::ENOMEM;
use ::libc::EPERM;
extern crate errno;
use self::errno::errno;
use ::bits::Node;
use ::memories::MemoryPolicy;
use ::memories::MovePagesFlags;
use ::masks::NodeMask;


pub trait Memory
{
	#[inline(always)]
	fn pointer(&self) -> *mut c_void;
	
	#[inline(always)]
	fn size(&self) -> size_t;
	
	#[inline(always)]
	fn is_null(&self) -> bool
	{
		self.pointer().is_null()
	}
	
	#[inline(always)]
	fn to_node(&self, node: Node)
	{
		unsafe { numa_tonode_memory(self.pointer(), self.size(), node.0) }
	}
	
	#[inline(always)]
	fn set_local(&self)
	{
		unsafe { numa_setlocal_memory(self.pointer(), self.size()) }
	}
	
	#[inline(always)]
	fn police(&self)
	{
		unsafe { numa_police_memory(self.pointer(), self.size()) }
	}
	
	fn bind(&self, nodes: &NodeMask, memory_policy: MemoryPolicy, flags: MovePagesFlags) -> Result<(), ErrorKind>
	{
		let bitmask = nodes.deref();
		match unsafe { mbind(self.pointer(), self.size() as c_ulong, memory_policy as c_int, bitmask.maskp, bitmask.size, flags.bits()) }
		{
			0 => Ok(()),
			-1 => match errno().0
			{
				EFAULT => panic!("Used an invalid address, an address not in this process or their was an unmapped hole in the length supplied"),
				EINVAL => Err(ErrorKind::InvalidInput),
				EIO => Err(ErrorKind::PermissionDenied),
				ENOMEM => Err(ErrorKind::Other),
				EPERM => Err(ErrorKind::PermissionDenied),
				unexpected @ _ => panic!("Did not expect numa_move_pages to set errno {}", unexpected),
			},
			unexpected @ _ => panic!("Did not expect numa_move_pages to return {}", unexpected),
		}
	}
}

extern "C"
{
	fn numa_tonode_memory(start: *mut c_void, size: size_t, node: c_int);
	fn numa_setlocal_memory(start: *mut c_void, size: size_t);
	fn numa_police_memory(start: *mut c_void, size: size_t);
	fn mbind(start: *mut c_void, len: c_ulong, mode: c_int, nmask: *const c_ulong, maxnode: c_ulong, flags: c_uint) -> c_long;
}