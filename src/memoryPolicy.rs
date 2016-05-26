// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


extern crate libc;


bitflags!
{
	pub flags MemoryPolicy: ::libc::c_int
	{
		#[allow(dead_code)] const MPOL_DEFAULT = 0,
		#[allow(dead_code)] const MPOL_PREFERRED = 1,
		#[allow(dead_code)] const MPOL_BIND = 2,
		#[allow(dead_code)] const MPOL_INTERLEAVE = 3,
	}
}
