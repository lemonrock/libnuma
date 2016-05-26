// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


extern crate libc;
use self::libc::c_void;
use self::libc::size_t;
use self::libc::c_int;
use super::NumaNode;


extern "C"
{
	fn numa_tonode_memory(start: *mut c_void, size: size_t, node: c_int);
	fn numa_setlocal_memory(start: *mut c_void, size: size_t);
	fn numa_police_memory(start: *mut c_void, size: size_t);
}

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
	fn to_node(&self, node: NumaNode)
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
}
