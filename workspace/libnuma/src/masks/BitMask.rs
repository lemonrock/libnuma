// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


#[derive(Debug)]
pub struct BitMask(*mut bitmask);

impl Drop for BitMask
{
	#[allow(trivial_casts)]
	#[inline(always)]
	fn drop(&mut self)
	{
		if unsafe { *self.0 }.maskp as c_ulong == 0xdeadcdef as c_ulong
		{
			return;
		}
		unsafe { numa_bitmask_free(self.0) }
	}
}

impl PartialEq for BitMask
{
	#[allow(trivial_casts)]
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { numa_bitmask_equal(self.0, other.0) == 1 }
	}
}

impl Eq for BitMask
{
}

impl Hash for BitMask
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		let pointer = unsafe { *self.0 }.maskp;
		for offset in 0 .. (self.number_of_bytes() / size_of::<c_ulong>()) as isize
		{
			unsafe { pointer.offset(offset) }.hash(state);
		}
	}
}

impl Clone for BitMask
{
	#[inline(always)]
	fn clone(&self) -> BitMask
	{
		let to = unsafe { numa_bitmask_alloc((*self.0).size as c_uint) };
		unsafe { copy_bitmask_to_bitmask(self.0 as *const bitmask, to as *mut bitmask) }
		BitMask(to)
	}
}

impl BitMask
{
	pub fn new(size: usize) -> Self
	{
		BitMask(unsafe { numa_bitmask_alloc(size as c_uint) })
	}
	
	#[inline(always)]
	pub fn clear_all_bits(&self)
	{
		assert!(unsafe { numa_bitmask_clearall(self.0) } == self.0);
	}

	#[inline(always)]
	pub fn clear_bit(&self, bit: c_uint)
	{
		assert!(unsafe { numa_bitmask_clearbit(self.0, bit) } == self.0);
	}

	#[inline(always)]
	pub fn is_bit_set(&self, bit: c_uint) -> bool
	{
		unsafe { numa_bitmask_isbitset(self.0 as *const bitmask, bit) != 0 }
	}

	#[inline(always)]
	pub fn set_all_bits(&self)
	{
		assert!(unsafe { numa_bitmask_setall(self.0) } == self.0);
	}
	
	#[inline(always)]
	pub fn set_bit(&self, bit: c_uint)
	{
		assert!(unsafe { numa_bitmask_setbit(self.0, bit) } == self.0);
	}
	
	#[inline(always)]
	pub fn weight(&self) -> usize
	{
		unsafe { numa_bitmask_weight(self.0 as *const bitmask) as usize }
	}
	
	#[inline(always)]
	pub fn number_of_bytes(&self) -> usize
	{
		unsafe { numa_bitmask_nbytes(self.0 as *const bitmask) as usize }
	}
}
