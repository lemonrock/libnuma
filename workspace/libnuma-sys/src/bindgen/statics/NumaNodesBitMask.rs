// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}numa_all_cpus_ptr"] pub static mut numa_all_cpus_ptr: *mut bitmask;
	#[link_name = "\u{1}numa_all_nodes"] pub static mut numa_all_nodes: nodemask_t;
	#[link_name = "\u{1}numa_all_nodes_ptr"] pub static mut numa_all_nodes_ptr: *mut bitmask;
	#[link_name = "\u{1}numa_no_nodes_ptr"] pub static mut numa_no_nodes_ptr: *mut bitmask;
	#[link_name = "\u{1}numa_nodes_ptr"] pub static mut numa_nodes_ptr: *mut bitmask;
}
