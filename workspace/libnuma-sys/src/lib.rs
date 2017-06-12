// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


#![cfg(any(target_os="linux", target_os="android"))]


extern crate libc;


use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_long;
use ::libc::c_longlong;
use ::libc::c_uint;
use ::libc::c_ulong;
use ::libc::c_void;
use ::libc::pid_t;
use ::libc::size_t;
use std::mem::zeroed;


//noinspection SpellCheckingInspection
pub const LIBNUMA_API_VERSION: c_uint = 2;


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct bitmask
{
	pub size: c_ulong,
	pub maskp: *mut c_ulong,
}

impl Default for bitmask
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[link(name = "numa")]
extern "C"
{
}

// lib
//noinspection SpellCheckingInspection
extern "C"
{
	pub fn numa_available() -> c_int;
	pub fn numa_pagesize() -> c_int;
	pub fn numa_set_localalloc();
	pub fn numa_set_bind_policy(strict: c_int);
	pub fn numa_set_strict(strict: c_int);
	
	pub static mut numa_exit_on_error: c_int;
	pub static mut numa_exit_on_warn: c_int;
	
	// Not obviously useful; defined as weak symbols
	// pub fn numa_error(_where: *mut c_char);
	// pub fn numa_warn(num: c_int, fmt: *mut c_char, ...);
}

// bitmask
//noinspection SpellCheckingInspection
extern "C"
{
	pub fn numa_bitmask_alloc(bmp: c_uint) -> *mut bitmask;
	pub fn numa_bitmask_clearall(bmp: *mut bitmask) -> *mut bitmask;
	pub fn numa_bitmask_clearbit(bmp: *mut bitmask, n: c_uint) -> *mut bitmask;
	pub fn numa_bitmask_equal(bmp1: *const bitmask, bmp2: *const bitmask) -> c_int;
	pub fn numa_bitmask_free(bmp: *mut bitmask);
	pub fn numa_bitmask_isbitset(bmp: *const bitmask, n: c_uint) -> c_int;
	// NOTE: The first argument is actually "*mut" but we make it *const to support hash; the API implementation does not mutate bmp
	pub fn numa_bitmask_setall(bmp: *mut bitmask) -> *mut bitmask;
	pub fn numa_bitmask_setbit(bmp: *mut bitmask, n: c_uint) -> *mut bitmask;
	// NOTE: The first argument is actually "*mut" but we make it *const to support not_quite_clone; the API implementation does not mutate bmpfrom
	pub fn copy_bitmask_to_bitmask(bmpfrom: *const bitmask, bmpto: *mut bitmask);
	pub fn numa_bitmask_weight(bmp: *const bitmask) -> c_uint;
	pub fn numa_bitmask_nbytes(bmp: *const bitmask) -> c_uint;
}

// bits/cpu
//noinspection SpellCheckingInspection
extern "C"
{
	pub fn numa_num_possible_cpus() -> c_int;
	pub fn numa_num_task_cpus() -> c_int;
	// Delegates to numa_num_task_cpus
	// pub fn numa_num_thread_cpus() -> c_int;
	pub fn numa_num_configured_cpus() -> c_int;
	pub fn numa_node_of_cpu(cpu: c_int) -> c_int;
}

// bits/node
//noinspection SpellCheckingInspection
extern "C"
{
	pub fn numa_num_possible_nodes() -> c_int;
	// Delegates to numa_num_task_nodes
	// pub fn numa_num_thread_nodes() -> c_int;
	pub fn numa_num_task_nodes() -> c_int;
	pub fn numa_num_configured_nodes() -> c_int;
	pub fn numa_max_possible_node() -> c_int;
	pub fn numa_max_node() -> c_int;
	pub fn numa_preferred() -> c_int;
	pub fn numa_get_interleave_node() -> c_int;
	pub fn numa_set_preferred(node: c_int);
	pub fn numa_distance(node1: c_int, node2: c_int) -> c_int;
	// Does not seem to offer much except potential problems
	// pub fn numa_node_size(node: c_int, freep: *mut c_long) -> c_long;
	pub fn numa_node_size64(node: c_int, freep: *mut c_longlong) -> c_longlong;
	pub fn numa_run_on_node(node: c_int) -> c_int;
	pub fn numa_alloc_onnode(size: size_t, node: c_int) -> *mut c_void;
	pub fn numa_node_to_cpus(node: c_int, mask: *mut bitmask) -> c_int;
}


// masks/cpuMask
//noinspection SpellCheckingInspection
extern "C"
{
	pub static mut numa_all_cpus_ptr: *mut bitmask;
	
	pub fn numa_allocate_cpumask() -> *mut bitmask;
	pub fn numa_parse_cpustring(string: *const c_char) -> *mut bitmask;
	pub fn numa_parse_cpustring_all(string: *const c_char) -> *mut bitmask;
	pub fn numa_get_run_node_mask() -> *mut bitmask;
	pub fn numa_run_on_node_mask(mask: *mut bitmask) -> c_int;
	pub fn numa_bind(nodes: *mut bitmask);
	pub fn numa_sched_getaffinity(pid: pid_t, mask: *mut bitmask) -> c_int; // result is NOT as documented in manpages...
	pub fn numa_sched_setaffinity(pid: pid_t, mask: *mut bitmask) -> c_int;
}

// masks/nodeMask
//noinspection SpellCheckingInspection
extern "C"
{
	pub static mut numa_all_nodes_ptr: *mut bitmask;
	pub static mut numa_no_nodes_ptr: *mut bitmask;
	//pub static mut numa_nodes_ptr: *mut bitmask;
	
	pub fn numa_allocate_nodemask() -> *mut bitmask;
	pub fn numa_parse_nodestring(string: *const c_char) -> *mut bitmask;
	pub fn numa_parse_nodestring_all(string: *const c_char) -> *mut bitmask;
	pub fn numa_get_interleave_mask() -> *mut bitmask;
	pub fn numa_get_membind() -> *mut bitmask;
	pub fn numa_get_mems_allowed() -> *mut bitmask;
	pub fn numa_set_interleave_mask(nodemask: *mut bitmask);
	pub fn numa_set_membind(nodemask: *mut bitmask);
	pub fn numa_alloc_interleaved_subset(size: size_t, nodemask: *mut bitmask) -> *mut c_void;
	pub fn numa_interleave_memory(mem: *mut c_void, size: size_t, mask: *mut bitmask);
	pub fn numa_tonodemask_memory(mem: *mut c_void, size: size_t, mask: *mut bitmask);
	pub fn numa_migrate_pages(pid: c_int, from: *mut bitmask, to: *mut bitmask) -> c_int;
	pub fn migrate_pages(pid: c_int, maxnode: c_ulong, frommask: *const c_ulong, tomask: *const c_ulong) -> c_long;
	pub fn set_mempolicy(mode: c_int, nmask: *const c_ulong, maxnode: c_ulong) -> c_long;
	
	// No wrapper implemented at this time as this call does several different things that have nothing to do with each other...
	// See http://man7.org/linux/man-pages/man2/get_mempolicy.2.html
	// pub fn get_mempolicy(policy: *mut c_int, nmask: *const c_ulong, maxnode: c_ulong, addr: *mut c_void, flags: c_int) -> c_long;
}

// memories/memory
//noinspection SpellCheckingInspection
extern "C"
{
	pub fn numa_tonode_memory(start: *mut c_void, size: size_t, node: c_int);
	pub fn numa_setlocal_memory(start: *mut c_void, size: size_t);
	pub fn numa_police_memory(start: *mut c_void, size: size_t);
	pub fn mbind(start: *mut c_void, len: c_ulong, mode: c_int, nmask: *const c_ulong, maxnode: c_ulong, flags: c_uint) -> c_long;
}

// memories/MovePagesFlags
//noinspection SpellCheckingInspection
extern "C"
{
	// numa_move_pages just wraps move_pages
	//pub fn move_pages(pid: c_int, count: c_ulong, pages: *mut *mut c_void, nodes: *const c_int, status: *mut c_int, flags: c_int) -> c_long;
	pub fn numa_move_pages(pid: c_int, count: c_ulong, pages: *mut *mut c_void, nodes: *const c_int, status: *mut c_int, flags: c_int) -> c_int;
}

// memories/numaMemory
//noinspection SpellCheckingInspection
extern "C"
{
	pub fn numa_alloc_interleaved(size: size_t) -> *mut c_void;
	pub fn numa_alloc_local(size: size_t) -> *mut c_void;
	pub fn numa_alloc(size: size_t) -> *mut c_void;
	pub fn numa_realloc(old_addr: *mut c_void, old_size: size_t, new_size: size_t) -> *mut c_void;
	pub fn numa_free(mem: *mut c_void, size: size_t);
}
