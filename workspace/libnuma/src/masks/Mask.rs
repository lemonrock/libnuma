// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


pub trait Mask<I: Index>: Default + Debug + Clone + Eq + PartialEq + Hash
{
	#[inline(always)]
	fn allocate() -> Self;
	
	#[inline(always)]
	fn all() -> Self;
	
	#[inline(always)]
	fn parse_string(string: &CStr) -> Self;
	
	#[inline(always)]
	fn parse_string_all(string: &CStr) -> Self;
	
	#[inline(always)]
	#[doc(hidden)]
	fn bit_mask(&self) -> &BitMask;
	
	#[inline(always)]
	fn clear(&self, index: I)
	{
		self.bit_mask().clear_bit(index.to_c_uint())
	}
	
	#[inline(always)]
	fn is_set(&self, index: I) -> bool
	{
		self.bit_mask().is_bit_set(index.to_c_uint())
	}
	
	#[inline(always)]
	fn set(&self, index: I)
	{
		self.bit_mask().set_bit(index.to_c_uint())
	}
	
	#[inline(always)]
	fn clear_all(&self)
	{
		self.bit_mask().clear_all_bits()
	}
	
	#[inline(always)]
	fn set_all(&self)
	{
		self.bit_mask().set_all_bits()
	}

	#[inline(always)]
	fn weight_x(&self) -> usize
	{
		self.bit_mask().weight()
	}
}
