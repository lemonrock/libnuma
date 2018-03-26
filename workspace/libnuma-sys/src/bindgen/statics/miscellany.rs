// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}numa_exit_on_error"] pub static mut numa_exit_on_error: c_int;
	#[link_name = "\u{1}numa_exit_on_warn"] pub static mut numa_exit_on_warn: c_int;
	#[link_name = "\u{1}numa_no_nodes"] pub static mut numa_no_nodes: nodemask_t;
}
