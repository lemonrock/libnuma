// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


use std::ops::Drop;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;
use std::clone::Clone;
use std::ffi::CStr;
use super::bitmask;


pub trait Mask : Hash + Eq + PartialEq + Drop + Debug + Clone
{
	#[inline(always)]
	fn as_ref_bitmask(&self) -> &bitmask;
	
	#[inline(always)]
	fn allocate() -> Self;
	
	#[inline(always)]
	fn parse_string(string: &CStr) -> Self;
	
	#[inline(always)]
	fn parse_string_all(string: &CStr) -> Self;
}
