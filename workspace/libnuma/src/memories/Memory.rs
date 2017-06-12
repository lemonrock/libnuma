// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


pub trait Memory
{
	#[inline(always)]
	fn wrap(address: *mut c_void, size: size_t) -> Self;
	
	#[inline(always)]
	fn address(&self) -> *mut c_void;
	
	#[inline(always)]
	fn size(&self) -> size_t;
	
	#[inline(always)]
	fn is_null(&self) -> bool
	{
		self.address().is_null()
	}
	
	#[inline(always)]
	fn to_node(&self, node: NodeIndex)
	{
		unsafe { numa_tonode_memory(self.address(), self.size(), node.0 as c_int) }
	}
	
	#[inline(always)]
	fn set_local(&self)
	{
		unsafe { numa_setlocal_memory(self.address(), self.size()) }
	}
	
	#[inline(always)]
	fn police(&self)
	{
		unsafe { numa_police_memory(self.address(), self.size()) }
	}
	
	//noinspection SpellCheckingInspection
	fn bind(&self, nodes: &NodeMask, memory_policy: MemoryPolicy, flags: MovePagesFlags::Flags) -> Result<(), ErrorKind>
	{
		use ::std::io::ErrorKind::*;
		
		let bitmask = nodes.bitmask();
		match unsafe { mbind(self.address(), self.size() as c_ulong, memory_policy as c_int, (*bitmask).maskp, (*bitmask).size, flags.bits()) }
		{
			0 => Ok(()),
			-1 => match errno().0
			{
				::libc::EFAULT => panic!("Used an invalid address, an address not in this process or their was an unmapped hole in the length supplied"),
				::libc::EINVAL => Err(InvalidInput),
				::libc::EIO => Err(PermissionDenied),
				::libc::ENOMEM => Err(Other),
				::libc::EPERM => Err(PermissionDenied),
				unexpected @ _ => panic!("Did not expect numa_move_pages to set errno {}", unexpected),
			},
			unexpected @ _ => panic!("Did not expect numa_move_pages to return {}", unexpected),
		}
	}
}
