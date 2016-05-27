// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


extern crate libc;
use self::libc::size_t;
use std::ops::Drop;
use std::result::Result;
use std::io::Error;
use ::memories::Memory;


pub trait ReAllocatableMemory : Memory + Drop
{
	#[inline(always)]
	fn reallocate(&mut self, new_size: size_t) -> Result<(), Error>;
}
