// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


use std::fmt::Debug;
use std::clone::Clone;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::hash::Hash;
use ::libc::c_uint;


pub trait Bit: Debug + Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash
{
	#[inline(always)]
	fn to_c_uint(&self) -> c_uint;
}
