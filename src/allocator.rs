// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


#![feature(allocator)]
#![allocator]
#![no_std]
#![crate_type = "rlib"]

#![feature(libc)] extern crate libc;
use libc::size_t;
use libc::c_void;

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8
{
	unsafe { libc::malloc(size as size_t) as *mut u8 }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize)
{
	unsafe { libc::free(ptr as *mut c_void) }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize, _align: usize) -> *mut u8
{
	unsafe { libc::realloc(ptr as *mut c_void, size as size_t) as *mut u8 }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize) -> usize
{
	old_size // this api is not supported by libc
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize
{
	size
}
