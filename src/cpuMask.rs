// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


use std::ops::Drop;
use std::ptr::null_mut;
use std::ffi::CStr;
use ::libc::c_char;
use ::libc::c_int;
extern crate errno;
use self::errno::errno;
use super::bitmask;


#[derive(Debug)]
#[unsafe_no_drop_flag]
pub struct CpuMask(pub *mut bitmask);

impl Drop for CpuMask
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.0.is_null()
		{
			return;
		}
		self.0 = null_mut()
	}
}

impl CpuMask
{
	#[inline(always)]
	pub fn allocate() -> CpuMask
	{
		CpuMask(unsafe { numa_allocate_cpumask() })
	}
	
	#[inline(always)]
	pub fn get_run_node_mask() -> CpuMask
	{
		CpuMask(unsafe { numa_get_run_node_mask() })
	}
	
	#[inline(always)]
	pub fn parse_cpu_string(string: &CStr) -> CpuMask
	{
		CpuMask(unsafe { numa_parse_cpustring(string.as_ptr()) })
	}
	
	#[inline(always)]
	pub fn parse_cpu_string_all(string: &CStr) -> CpuMask
	{
		CpuMask(unsafe { numa_parse_cpustring_all(string.as_ptr()) })
	}
	
	#[inline(always)]
	pub fn run_on_node_mask(&mut self)
	{
		match unsafe { numa_run_on_node_mask(self.0) }
		{
			0 => (),
			-1 => match errno().0
			{
				unexpected @ _ => panic!("numa_run_on_node_mask set errno {} but the man pages helpfully don't tell us what errors to expect", unexpected),
			},
			unexpected @ _ => panic!("Did not expect numa_run_on_node_mask to return {}", unexpected),
		}
	}
	
	#[inline(always)]
	pub fn bind(&mut self)
	{
		unsafe { numa_bind(self.0) }
	}
}

extern "C"
{
	fn numa_allocate_cpumask() -> *mut bitmask;
	fn numa_get_run_node_mask() -> *mut bitmask;
	fn numa_parse_cpustring(string: *const c_char) -> *mut bitmask;
	fn numa_parse_cpustring_all(string: *const c_char) -> *mut bitmask;
	fn numa_run_on_node_mask(mask: *mut bitmask) -> c_int;
	fn numa_bind(nodes: *mut bitmask);
}
