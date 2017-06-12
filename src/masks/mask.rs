// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


use std::ops::Drop;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;
use std::clone::Clone;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ffi::CStr;
use ::bitmask;
use ::bits::Bit;


pub trait Mask<B: Bit> : Hash + Eq + PartialEq + Drop + Debug + Clone + Deref<Target=bitmask> + DerefMut
{
	#[inline(always)]
	fn allocate() -> Self;
	
	#[inline(always)]
	fn parse_string(string: &CStr) -> Self;
	
	#[inline(always)]
	fn parse_string_all(string: &CStr) -> Self;
	
	#[inline(always)]
	fn clear_all(&mut self) -> &mut Self
	{
		(*self).clear_all_bits();
		self
	}

	#[inline(always)]
	fn clear(&mut self, bit: B) -> &mut Self
	{
		(*self).clear_bit(bit.to_c_uint());
		self
	}
	
	#[inline(always)]
	fn is_set(&self, bit: B) -> bool
	{
		(*self).is_bit_set(bit.to_c_uint())
	}
	
	#[inline(always)]
	fn set_all(&mut self) -> &mut Self
	{
		(*self).set_all_bits();
		self
	}

	#[inline(always)]
	fn set(&mut self, bit: B) -> &mut Self
	{
		(*self).set_bit(bit.to_c_uint());
		self
	}

	#[inline(always)]
	fn weight_x(&self) -> usize
	{
		self.deref().weight()
	}
}
