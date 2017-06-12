// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


pub trait Index: Default + Debug + Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash
{
	#[inline(always)]
	fn to_c_uint(&self) -> c_uint;
	
	#[inline(always)]
	fn clear<M: Mask<Self>>(self, mask: &M)
	{
		mask.clear(self)
	}
	
	#[inline(always)]
	fn is_set<M: Mask<Self>>(self, mask: &M) -> bool
	{
		mask.is_set(self)
	}
	
	#[inline(always)]
	fn set<M: Mask<Self>>(self, mask: &M)
	{
		mask.set(self)
	}
}
