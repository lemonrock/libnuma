// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


extern crate libc;
use self::libc::size_t;
use std::ops::Drop;
use ::memories::Memory;


pub trait AllocatableMemory : Memory + Drop
{
	#[inline(always)]
	fn allocate(size: size_t) -> Self;
}
