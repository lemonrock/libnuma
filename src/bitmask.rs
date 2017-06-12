// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


use std::ops::Drop;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem::size_of;
use ::libc::c_ulong;
use ::libc::c_uint;
use ::libc::c_int;


#[repr(C)]
#[derive(Debug)]
#[unsafe_no_drop_flag]
pub struct bitmask
{
	pub size: c_ulong,
	pub maskp: *mut c_ulong,
}

// https://stackoverflow.com/questions/30742004/correct-idiom-for-freeing-reprc-structs-using-drop-trait#30742180
impl Drop for bitmask
{
	#[allow(trivial_casts)]
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.maskp as c_ulong == 0xdeadcdef as c_ulong
		{
			return;
		}
		unsafe { numa_bitmask_free(self as *mut bitmask) }
	}
}

impl PartialEq for bitmask
{
	#[allow(trivial_casts)]
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { numa_bitmask_equal(self, other as *const bitmask) == 1 }
	}
}

impl Eq for bitmask
{
}

impl Hash for bitmask
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		let pointer = self.maskp;
		for offset in 0 .. (self.number_of_bytes() / size_of::<c_ulong>()) as isize
		{
			unsafe { pointer.offset(offset) }.hash(state);
		}
	}
}

impl bitmask
{
	pub fn allocate<'a>(size: usize) -> &'a bitmask
	{
		let to = unsafe { numa_bitmask_alloc(size as c_uint) };
		unsafe
		{
			&*to
		}
	}

	#[allow(trivial_casts)]
	pub fn internal_clone(&self) -> *mut bitmask
	{
		let to = unsafe { numa_bitmask_alloc(self.size as c_uint) };
		unsafe { copy_bitmask_to_bitmask(self as *const bitmask, to as *mut bitmask) }
		to
	}

	#[allow(trivial_casts)]
	pub fn not_quite_clone(&self) -> &bitmask
	{
		let to = unsafe { numa_bitmask_alloc(self.size as c_uint) };
		unsafe { copy_bitmask_to_bitmask(self as *const bitmask, to as *mut bitmask) }
		unsafe
		{
			&*to
		}
	}
	
	#[inline(always)]
	pub fn clear_all_bits(&mut self) -> &mut Self
	{
		unsafe { numa_bitmask_clearall(self) };
		self
	}

	#[inline(always)]
	pub fn clear_bit(&mut self, bit: c_uint) -> &mut Self
	{
		unsafe { numa_bitmask_clearbit(self, bit) };
		self
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn is_bit_set(&self, bit: c_uint) -> bool
	{
		unsafe { numa_bitmask_isbitset(self as *const bitmask, bit) != 0 }
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn set_all_bits(&mut self) -> &mut Self
	{
		unsafe { numa_bitmask_setall(self as *mut bitmask) };
		self
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn set_bit(&mut self, bit: c_uint) -> &mut Self
	{
		unsafe { numa_bitmask_setbit(self as *mut bitmask, bit) };
		self
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn weight(&self) -> usize
	{
		unsafe { numa_bitmask_weight(self as *const bitmask) as usize }
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn number_of_bytes(&self) -> usize
	{
		unsafe { numa_bitmask_nbytes(self as *const bitmask) as usize }
	}
}

extern "C"
{
	fn numa_bitmask_alloc(bmp: c_uint) -> *mut bitmask;
	fn numa_bitmask_clearall(bmp: *mut bitmask) -> *mut bitmask;
	fn numa_bitmask_clearbit(bmp: *mut bitmask, n: c_uint) -> *mut bitmask;
	fn numa_bitmask_equal(bmp1: *const bitmask, bmp2: *const bitmask) -> c_int;
	fn numa_bitmask_free(bmp: *mut bitmask);
	fn numa_bitmask_isbitset(bmp: *const bitmask, n: c_uint) -> c_int;
	// NOTE: The first argument is actually "*mut" but we make it *const to support hash; the API implementation does not mutate bmp
	fn numa_bitmask_setall(bmp: *mut bitmask) -> *mut bitmask;
	fn numa_bitmask_setbit(bmp: *mut bitmask, n: c_uint) -> *mut bitmask;
	// NOTE: The first argument is actually "*mut" but we make it *const to support not_quite_clone; the API implementation does not mutate bmpfrom
	fn copy_bitmask_to_bitmask(bmpfrom: *const bitmask, bmpto: *mut bitmask);
	fn numa_bitmask_weight(bmp: *const bitmask) -> c_uint;
	fn numa_bitmask_nbytes(bmp: *const bitmask) -> c_uint;
}
