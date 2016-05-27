// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


extern crate libc;
use self::libc::c_void;
use self::libc::size_t;
use std::ops::Drop;
use std::ptr::null_mut;
use std::result::Result;
use std::io::Error;
use ::memories::Memory;
use ::memories::AllocatableMemory;
use ::memories::ReAllocatableMemory;
use ::bits::Node;


#[derive(Debug)]
#[unsafe_no_drop_flag]
pub struct NumaMemory(pub *mut c_void, pub size_t);

impl Memory for NumaMemory
{
	#[inline(always)]
	fn pointer(&self) -> *mut c_void
	{
		self.0
	}

	#[inline(always)]
	fn size(&self) -> size_t
	{
		self.1
	}
}

impl ReAllocatableMemory for NumaMemory
{
	#[inline(always)]
	fn reallocate(&mut self, new_size: size_t) -> Result<(), Error>
	{
		if self.1 == new_size
		{
			return Ok(())
		}
		
		let result = unsafe { numa_realloc(self.0, self.1, new_size) };
		if result.is_null()
		{
			return Err(Error::last_os_error())
		}
		self.0 = result;
		self.1 = new_size;
		Ok(())
	}
}

impl AllocatableMemory for NumaMemory
{
	/// Slow because it calls numa_police_memory on every single page (pagesize as per numa_pagesize())
	#[inline(always)]
	fn allocate(size: size_t) -> NumaMemory
	{
		NumaMemory(unsafe { numa_alloc(size) }, size)
	}
}

impl Drop for NumaMemory
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.is_null()
		{
			return
		}
		unsafe { numa_free(self.0, self.1) };
		self.0 = null_mut();
	}
}

impl NumaMemory
{	
	#[inline(always)]
	pub fn allocate_interleaved(size: size_t) -> NumaMemory
	{
		NumaMemory(unsafe { numa_alloc_interleaved(size) }, size)
	}

	#[inline(always)]
	pub fn allocate_local(size: size_t) -> NumaMemory
	{
		NumaMemory(unsafe { numa_alloc_local(size) }, size)
	}

	#[inline(always)]
	pub fn allocate_on_node(size: size_t, node: Node) -> NumaMemory
	{
		node.allocate(size)
	}
}

extern "C"
{
	fn numa_alloc_interleaved(size: size_t) -> *mut c_void;
	fn numa_alloc_local(size: size_t) -> *mut c_void;
	fn numa_alloc(size: size_t) -> *mut c_void;
	fn numa_realloc(old_addr: *mut c_void, old_size: size_t, new_size: size_t) -> *mut c_void;
	fn numa_free(mem: *mut c_void, size: size_t);
}