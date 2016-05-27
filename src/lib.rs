// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


#![cfg(target_os = "linux")]
#![feature(unsafe_no_drop_flag)]
#![feature(const_fn)]


#[macro_use] extern crate bitflags;
extern crate libc;
use libc::c_uint;
use libc::c_int;

#[path="bitmask.rs"] mod _bitmask;
pub use _bitmask::bitmask;

mod masks;
pub use masks::*;

mod bits;
pub use bits::*;

mod memories;
pub use memories::*;

pub use memoryPolicyFlags::MemoryPolicyFlags;
mod memoryPolicyFlags;


#[link(name = "numa")]
extern "C"
{
}


pub const LIBNUMA_API_VERSION: c_uint = 2;


pub fn initialize() -> bool
{
	match unsafe { numa_available() }
	{
		0 => true,
		-1 => false,
		unexpected @ _ => panic!("Did not expected numa_available to return {}", unexpected),
	}
}

#[inline(always)]
pub fn cached_page_size() -> usize
{
	match unsafe { numa_pagesize() }
	{
		x if x.is_positive() => x as usize,
		unexpected @ _ => panic!("numa_pagesize returned a non-positive value {}", unexpected),
	}
}

#[inline(always)]
pub fn locally_allocate_memory()
{
	unsafe { numa_set_localalloc() }
}

// Should normally be set as false (otherwise we can exhaust memory sooner)
pub fn set_strict(is_strict: bool)
{
	unsafe { numa_set_strict(if is_strict { 1 } else { 0 }) }
}

// Should normally be set as false (otherwise we can exhaust memory sooner)
pub fn set_bind_policy(is_strict: bool)
{
	unsafe { numa_set_bind_policy(if is_strict { 1 } else { 0 }) }
}

pub fn will_exit_on_error() -> bool
{
	unsafe { numa_exit_on_error != 0 }
}

pub fn will_exit_on_warning() -> bool
{
	unsafe { numa_exit_on_warn != 0 }
}

pub fn exit_on_error(exit: bool)
{
	unsafe { numa_exit_on_error = if exit {1} else {0} }
}

pub fn exit_on_warning(exit: bool)
{
	unsafe { numa_exit_on_warn = if exit {1} else {0} }
}

extern "C"
{
	fn numa_available() -> c_int;
	fn numa_pagesize() -> c_int;
	fn numa_set_localalloc();
	fn numa_set_bind_policy(strict: c_int);
	fn numa_set_strict(strict: c_int);
	
	static mut numa_exit_on_error: c_int;
	static mut numa_exit_on_warn: c_int;
	
	// Not obviously useful; defined as weak symbols
	// pub fn numa_error(_where: *mut c_char);
	// pub fn numa_warn(num: c_int, fmt: *mut c_char, ...);
}

#[test]
fn check_binding()
{
	CpuMask::all_cpus().sched_get_affinity_for_current_thread();
}
