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
use ::libc::c_ulong;
use ::libc::c_long;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_void;
use ::libc::size_t;
use ::libc::pid_t;
use ::libc::EFAULT;
use ::libc::EINVAL;
use ::libc::EPERM;
use ::libc::ESRCH;
use ::libc::ENOMEM;
extern crate errno;
use self::errno::errno;
use ::bits::Node;
use ::bitmask;
use ::masks::Mask;
use ::memories::NumaMemory;
use ::memories::Memory;
use ::memories::MemoryPolicy;
use ::memories::MovePagesFlags;


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
	pub fn migrate_pages_to(&mut self, task_id: pid_t, to: &NodeMask) -> c_int
	{
		unsafe { numa_migrate_pages(task_id, self.0, to.0) }
	}
	
	#[inline(always)]
	pub fn bind<M: Memory>(&self, memory: M, memory_policy: MemoryPolicy, flags: MovePagesFlags) -> Result<(), ErrorKind>
	{
		memory.bind(&self, memory_policy, flags)
	}
	
	/// Returns number of pages that could not be moved; 0 is total success
	pub fn migrate_pages(&self, task_id: pid_t, to: &NodeMask) -> Result<usize, ErrorKind>
	{
		let from_bitmask = &self.deref();
		let to_bitmask = &to.deref();
		
		let maximum_node = from_bitmask.size;
		
		debug_assert!(maximum_node == to_bitmask.size, "NodeMask size differs, from {} vs to {}", maximum_node, to_bitmask.size);
		
		match unsafe { migrate_pages(task_id, maximum_node, from_bitmask.maskp, to_bitmask.maskp) }
		{
			-1 => match errno().0
			{
				EFAULT => panic!("Used an invalid address"),
				EINVAL => Err(ErrorKind::InvalidInput),
				EPERM => Err(ErrorKind::PermissionDenied),
				ESRCH => Err(ErrorKind::NotFound),
				unexpected @ _ => panic!("Did not expect migrate_pages to set errno {}", unexpected),
			},
			unexpected if unexpected < -1 => panic!("Did not expect migrate_pages to return {}", unexpected),
			number_of_pages_that_could_not_be_moved => Ok(number_of_pages_that_could_not_be_moved as usize)
		}
	}
	
	pub fn set_memory_policy_of_current_thread(&self, memory_policy: MemoryPolicy) -> Result<(), ErrorKind>
	{
		let bitmask = &self.deref();
		
		match unsafe { set_mempolicy(memory_policy as c_int, bitmask.maskp, bitmask.size) }
		{
			0 => Ok(()),
			-1 => match errno().0
			{
				EFAULT => panic!("Used an invalid address"),
				EINVAL => Err(ErrorKind::InvalidInput),
				ENOMEM => Err(ErrorKind::Other),
				unexpected @ _ => panic!("Did not expect set_mempolicy to set errno {}", unexpected),
			},
			unexpected @ _ => panic!("Did not expect set_mempolicy to return {}", unexpected),
		}
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
	fn migrate_pages(pid: c_int, maxnode: c_ulong, frommask: *const c_ulong, tomask: *const c_ulong) -> c_long;
	fn set_mempolicy(mode: c_int, nmask: *const c_ulong, maxnode: c_ulong) -> c_long;
	
	// No wrapper implemented at this time as this call does several different things that have nothing to do with each other...
	// See http://man7.org/linux/man-pages/man2/get_mempolicy.2.html
	// fn get_mempolicy(policy: *mut c_int, nmask: *const c_ulong, maxnode: c_ulong, addr: *mut c_void, flags: c_int) -> c_long;
}
