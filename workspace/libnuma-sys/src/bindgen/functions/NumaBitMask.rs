// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


extern "C"
{
	pub fn numa_bitmask_alloc(arg1: c_uint) -> *mut bitmask;
	pub fn numa_bitmask_clearall(arg1: *mut bitmask) -> *mut bitmask;
	pub fn numa_bitmask_clearbit(arg1: *mut bitmask, arg2: c_uint) -> *mut bitmask;
	pub fn numa_bitmask_equal(arg1: *const bitmask, arg2: *const bitmask) -> c_int;
	pub fn numa_bitmask_free(arg1: *mut bitmask);
	pub fn numa_bitmask_isbitset(arg1: *const bitmask, arg2: c_uint) -> c_int;
	pub fn numa_bitmask_nbytes(arg1: *mut bitmask) -> c_uint;
	pub fn numa_bitmask_setall(arg1: *mut bitmask) -> *mut bitmask;
	pub fn numa_bitmask_setbit(arg1: *mut bitmask, arg2: c_uint) -> *mut bitmask;
	pub fn numa_bitmask_weight(arg1: *const bitmask) -> c_uint;
}
