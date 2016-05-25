// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


#[cfg(not(feature = "use_std"))] extern crate std as core;
use self::core::clone::Clone;
use self::core::default::Default;
use self::core::mem::zeroed;
use ::libc::c_ulong;
use super::bitmask;


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))] pub const NUMA_NUM_NODES: usize = 128;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))] pub const NUMA_NUM_NODES: usize = 2048;

// size_of is irritatingly not a const fn!
//const SizeOfUnsignedLong: usize = size_of::<c_ulong>();
#[cfg(all(unix, target_pointer_width = "32"))] const SizeOfUnsignedLong: usize = 4;
#[cfg(all(unix, target_pointer_width = "64"))] const SizeOfUnsignedLong: usize = 8;
#[cfg(windows)] const SizeOfUnsignedLong: usize = 4;


#[repr(C)]
#[derive(Copy, Debug)]
pub struct nodemask_t
{
	pub n: [c_ulong; NUMA_NUM_NODES / (SizeOfUnsignedLong * 8)],
}

impl Clone for nodemask_t
{
	fn clone(&self) -> Self { *self }
}

impl Default for nodemask_t
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl nodemask_t
{
	pub fn copy_into_bitmask(&mut self, to: &mut bitmask)
	{
		unsafe { copy_nodemask_to_bitmask(self as *mut nodemask_t, to as *mut bitmask) }
	}
}

extern "C"
{
	fn copy_nodemask_to_bitmask(nodemask: *mut nodemask_t, bmp: *mut bitmask);
}
