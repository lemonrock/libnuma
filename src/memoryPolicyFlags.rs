// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


bitflags!
{
	pub flags MemoryPolicyFlags: ::libc::c_uint
	{
		/* Flags for get_mem_policy */
		#[allow(dead_code)] const MPOL_F_NODE = 1 << 0,
		#[allow(dead_code)] const MPOL_F_ADDR = 1 << 1,
		#[allow(dead_code)] const MPOL_F_MEMS_ALLOWED = 1 << 2,
	}
}
