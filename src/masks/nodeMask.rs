// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


use std::ops::Drop;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::hash::Hash;
use std::hash::Hasher;
use std::clone::Clone;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::null_mut;
use std::ffi::CStr;
use std::io::ErrorKind;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_void;
use ::libc::size_t;
use ::bitmask;
use ::masks::Mask;
use ::memories::NumaMemory;
use ::memories::Memory;
use ::memories::MemoryPolicy;
use ::memories::MovePagesFlags;
use ::bits::Node;


#[derive(Debug)]
#[unsafe_no_drop_flag]
pub struct NodeMask(pub *mut bitmask);

impl Drop for NodeMask
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.0.is_null()
		{
			return;
		}
		drop(self.0);
		self.0 = null_mut()
	}
}

impl PartialEq for NodeMask
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.0.eq(&other.0)
	}
}

impl Eq for NodeMask
{
}

impl Hash for NodeMask
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.deref().hash(state)
	}
}

impl Clone for NodeMask
{
	fn clone(&self) -> Self
	{
		NodeMask((*self).internal_clone())
	}
}

impl Deref for NodeMask
{
	type Target = bitmask;

	fn deref(&self) -> &bitmask
	{
		unsafe { &*self.0 }
	}
}

impl DerefMut for NodeMask
{
	fn deref_mut(&mut self) -> &mut bitmask
	{
		unsafe { self.0.as_mut().unwrap() }
	}
}

impl Mask<Node> for NodeMask
{
	#[inline(always)]
	fn allocate() -> NodeMask
	{
		NodeMask(unsafe { numa_allocate_nodemask() })
	}
	
	#[inline(always)]
	fn parse_string(string: &CStr) -> NodeMask
	{
		NodeMask(unsafe { numa_parse_nodestring(string.as_ptr()) })
	}

	#[inline(always)]
	fn parse_string_all(string: &CStr) -> NodeMask
	{
		NodeMask(unsafe { numa_parse_nodestring_all(string.as_ptr()) })
	}
}

impl NodeMask
{
	/// This operation is a touch expensive, as it clones an otherwise static value (otherwise a drop could occur that wiped out the static)
	#[inline(always)]
	pub fn all_nodes<'a>() -> NodeMask
	{
		NodeMask(unsafe { numa_all_nodes_ptr }).clone()
	}

	/// This operation is a touch expensive, as it clones an otherwise static value (otherwise a drop could occur that wiped out the static)
	#[inline(always)]
	pub fn no_nodes<'a>() -> NodeMask
	{
		NodeMask(unsafe { numa_no_nodes_ptr }).clone()
	}

	/// This operation is a touch expensive, as it clones an otherwise static value (otherwise a drop could occur that wiped out the static)
	/// ?not in man pages?
	// #[inline(always)]
	// pub fn nodes<'a>() -> NodeMask
	// {
	// 	NodeMask(unsafe { numa_nodes_ptr }).clone()
	// }
	
	#[inline(always)]
	pub fn get_interleave_mask() -> NodeMask
	{
		NodeMask(unsafe { numa_get_interleave_mask() })
	}
	
	#[inline(always)]
	pub fn get_membind() -> NodeMask
	{
		NodeMask(unsafe { numa_get_membind() })
	}
	
	#[inline(always)]
	pub fn get_mems_allowed() -> NodeMask
	{
		NodeMask(unsafe { numa_get_mems_allowed() })
	}
	
	#[inline(always)]
	pub fn set_interleave_mask(&mut self)
	{
		unsafe { numa_set_interleave_mask(self.0) }
	}
	
	#[inline(always)]
	pub fn set_membind(&mut self)
	{
		unsafe { numa_set_membind(self.0) }
	}
	
	#[inline(always)]
	pub fn allocate_interleaved_subset(&mut self, size: size_t) -> NumaMemory
	{
		NumaMemory(unsafe { numa_alloc_interleaved_subset(size, self.0) }, size)
	}
	
	#[inline(always)]
	pub fn interleave_memory<M: Memory>(&mut self, memory: M)
	{
		unsafe { numa_interleave_memory(memory.pointer(), memory.size(), self.0) }
	}
	
	#[inline(always)]
	pub fn to_nodemask_memory<M: Memory>(&mut self, memory: M)
	{
		unsafe { numa_tonodemask_memory(memory.pointer(), memory.size(), self.0) }
	}

	#[inline(always)]
	pub fn migrate_pages_to(&mut self, pid: c_int, to: &NodeMask) -> c_int
	{
		unsafe { numa_migrate_pages(pid, self.0, to.0) }
	}
	
	#[inline(always)]
	pub fn bind<M: Memory>(&self, memory: M, mode: MemoryPolicy, flags: MovePagesFlags) -> Result<(), ErrorKind>
	{
		memory.bind(mode, &self, flags)
	}
}

extern "C"
{
	static mut numa_all_nodes_ptr: *mut bitmask;
	static mut numa_no_nodes_ptr: *mut bitmask;
	//static mut numa_nodes_ptr: *mut bitmask;
	fn numa_allocate_nodemask() -> *mut bitmask;
	fn numa_parse_nodestring(string: *const c_char) -> *mut bitmask;
	fn numa_parse_nodestring_all(string: *const c_char) -> *mut bitmask;
	fn numa_get_interleave_mask() -> *mut bitmask;
	fn numa_get_membind() -> *mut bitmask;
	fn numa_get_mems_allowed() -> *mut bitmask;
	fn numa_set_interleave_mask(nodemask: *mut bitmask);
	fn numa_set_membind(nodemask: *mut bitmask);
	fn numa_alloc_interleaved_subset(size: size_t, nodemask: *mut bitmask) -> *mut c_void;
	fn numa_interleave_memory(mem: *mut c_void, size: size_t, mask: *mut bitmask);
	fn numa_tonodemask_memory(mem: *mut c_void, size: size_t, mask: *mut bitmask);
	fn numa_migrate_pages(pid: c_int, from: *mut bitmask, to: *mut bitmask) -> c_int;
}
