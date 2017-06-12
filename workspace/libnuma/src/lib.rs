// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


#![cfg(any(target_os="linux", target_os="android"))]
#![allow(non_snake_case)]


#[macro_use] extern crate bitflags;
extern crate errno;
extern crate libc;
extern crate libnuma_sys;


use ::libnuma_sys::*;


pub mod masks;
pub mod memories;


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
