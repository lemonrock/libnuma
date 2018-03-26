// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


#[derive(Debug)]
pub struct NumaMemory(*mut c_void, size_t);

impl Memory for NumaMemory
{
	#[inline(always)]
	fn wrap(address: *mut c_void, size: size_t) -> Self
	{
		NumaMemory(address, size)
	}
	
	#[inline(always)]
	fn address(&self) -> *mut c_void
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
	pub fn allocate_interleaved(size: size_t) -> Self
	{
		NumaMemory(unsafe { numa_alloc_interleaved(size) }, size)
	}

	#[inline(always)]
	pub fn allocate_local(size: size_t) -> Self
	{
		NumaMemory(unsafe { numa_alloc_local(size) }, size)
	}

	#[inline(always)]
	pub fn allocate_on_node(size: size_t, node: NodeIndex) -> Self
	{
		node.allocate(size)
	}
}
