// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(i32)]
#[allow(non_camel_case_types)] 
pub enum MemoryPolicy
{
	MPOL_DEFAULT = 0,
	MPOL_PREFERRED = 1,
	MPOL_BIND = 2,
	MPOL_INTERLEAVE = 3,
}
