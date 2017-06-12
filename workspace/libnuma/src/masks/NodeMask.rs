// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeMask(BitMask);

impl Default for NodeMask
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::allocate()
	}
}

impl Mask<NodeIndex> for NodeMask
{
	#[inline(always)]
	fn allocate() -> Self
	{
		NodeMask(BitMask(unsafe { numa_allocate_nodemask() }))
	}
	
	/// This operation is a touch expensive, as it clones an otherwise static value (otherwise a drop could occur that wiped out the static)
	#[inline(always)]
	fn all() -> Self
	{
		NodeMask(BitMask(unsafe { numa_all_nodes_ptr })).clone()
	}
	
	#[inline(always)]
	fn parse_string(string: &CStr) -> Self
	{
		NodeMask(BitMask(unsafe { numa_parse_nodestring(string.as_ptr()) }))
	}

	#[inline(always)]
	fn parse_string_all(string: &CStr) -> Self
	{
		NodeMask(BitMask(unsafe { numa_parse_nodestring_all(string.as_ptr()) }))
	}
	
	#[inline(always)]
	#[doc(hidden)]
	fn bit_mask(&self) -> &BitMask
	{
		&self.0
	}
}

//noinspection SpellCheckingInspection
impl NodeMask
{
	/// This operation is a touch expensive, as it clones an otherwise static value (otherwise a drop could occur that wiped out the static)
	#[inline(always)]
	pub fn no_nodes() -> Self
	{
		NodeMask(BitMask(unsafe { numa_no_nodes_ptr })).clone()
	}

	/// This operation is a touch expensive, as it clones an otherwise static value (otherwise a drop could occur that wiped out the static)
	/// ?not in man pages?
	// #[inline(always)]
	// pub fn nodes() -> Self
	// {
	// 	NodeMake(BitMask(unsafe { numa_nodes_ptr })).clone()
	// }
	
	#[inline(always)]
	pub fn get_interleave_mask() -> NodeMask
	{
		NodeMask(BitMask(unsafe { numa_get_interleave_mask() }))
	}
	
	#[inline(always)]
	pub fn get_membind() -> NodeMask
	{
		NodeMask(BitMask(unsafe { numa_get_membind() }))
	}
	
	#[inline(always)]
	pub fn get_mems_allowed() -> NodeMask
	{
		NodeMask(BitMask(unsafe { numa_get_mems_allowed() }))
	}
	
	#[inline(always)]
	pub fn set_interleave_mask(&mut self)
	{
		unsafe { numa_set_interleave_mask(self.bitmask()) }
	}
	
	#[inline(always)]
	pub fn set_membind(&mut self)
	{
		unsafe { numa_set_membind(self.bitmask()) }
	}
	
	#[inline(always)]
	pub fn allocate_interleaved_subset(&mut self, size: size_t) -> NumaMemory
	{
		NumaMemory::wrap(unsafe { numa_alloc_interleaved_subset(size, self.bitmask()) }, size)
	}
	
	#[inline(always)]
	pub fn interleave_memory<M: Memory>(&mut self, memory: M)
	{
		unsafe { numa_interleave_memory(memory.address(), memory.size(), self.bitmask()) }
	}
	
	#[inline(always)]
	pub fn to_nodemask_memory<M: Memory>(&mut self, memory: M)
	{
		unsafe { numa_tonodemask_memory(memory.address(), memory.size(), self.bitmask()) }
	}

	#[inline(always)]
	pub fn migrate_pages_to(&mut self, task_id: pid_t, to: &NodeMask) -> c_int
	{
		unsafe { numa_migrate_pages(task_id, self.bitmask(), to.bitmask()) }
	}
	
	#[inline(always)]
	pub fn bind<M: Memory>(&self, memory: M, memory_policy: MemoryPolicy, flags: MovePagesFlags::Flags) -> Result<(), ErrorKind>
	{
		memory.bind(&self, memory_policy, flags)
	}
	
	/// Returns number of pages that could not be moved; 0 is total success
	pub fn migrate_pages(&self, task_id: pid_t, to: &NodeMask) -> Result<usize, ErrorKind>
	{
		let from_bitmask = self.bitmask();
		let to_bitmask = to.bitmask();
		
		let maximum_node = unsafe { *from_bitmask }.size;
		
		debug_assert!(maximum_node == unsafe { *to_bitmask }.size, "NodeMask size differs, from {} vs to {}", maximum_node, unsafe { *to_bitmask }.size);
		
		match unsafe { migrate_pages(task_id, maximum_node, (*from_bitmask).maskp, (*to_bitmask).maskp) }
		{
			-1 => match errno().0
			{
				::libc::EFAULT => panic!("Used an invalid address"),
				::libc::EINVAL => Err(ErrorKind::InvalidInput),
				::libc::EPERM => Err(ErrorKind::PermissionDenied),
				::libc::ESRCH => Err(ErrorKind::NotFound),
				unexpected @ _ => panic!("Did not expect migrate_pages to set errno {}", unexpected),
			},
			unexpected if unexpected < -1 => panic!("Did not expect migrate_pages to return {}", unexpected),
			number_of_pages_that_could_not_be_moved => Ok(number_of_pages_that_could_not_be_moved as usize)
		}
	}
	
	pub fn set_memory_policy_of_current_thread(&self, memory_policy: MemoryPolicy) -> Result<(), ErrorKind>
	{
		let bitmask = self.bitmask();
		
		match unsafe { set_mempolicy(memory_policy as c_int, (*bitmask).maskp, (*bitmask).size) }
		{
			0 => Ok(()),
			-1 => match errno().0
			{
				::libc::EFAULT => panic!("Used an invalid address"),
				::libc::EINVAL => Err(ErrorKind::InvalidInput),
				::libc::ENOMEM => Err(ErrorKind::Other),
				unexpected @ _ => panic!("Did not expect set_mempolicy to set errno {}", unexpected),
			},
			unexpected @ _ => panic!("Did not expect set_mempolicy to return {}", unexpected),
		}
	}
	
	#[inline(always)]
	pub(crate) fn bitmask(&self) -> *mut bitmask
	{
		(self.0).0
	}
}
