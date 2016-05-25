// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


extern crate libc;
use libc::c_int;
use libc::c_ulong;
use libc::c_void;
use libc::c_uint;
use libc::c_longlong;
use libc::c_long;
use libc::c_char;
use libc::pid_t;
use libc::size_t;
use ::std::clone::Clone;
use ::std::default::Default;
use ::std::mem::zeroed;


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))] pub const NUMA_NUM_NODES: usize = 128;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))] pub const NUMA_NUM_NODES: usize = 2048;

// size_of is not a const fn!
//const SizeOfUnsignedLong: usize = size_of::<c_ulong>();
#[cfg(all(unix, target_pointer_width = "32"))] const SizeOfUnsignedLong: usize = 4;
#[cfg(all(unix, target_pointer_width = "64"))] const SizeOfUnsignedLong: usize = 8;
#[cfg(windows)] const SizeOfUnsignedLong: usize = 4;


#[repr(C)]
#[derive(Copy, Debug)]
pub struct nodemask_t
{
	pub n: [c_ulong; NUMA_NUM_NODES / (SizeOfUnsignedLong * 8)],
}

impl Clone for nodemask_t
{
	fn clone(&self) -> Self { *self }
}

impl Default for nodemask_t
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}


#[repr(C)]
#[derive(Copy, Debug)]
pub struct bitmask
{
	pub size: c_ulong,
	pub maskp: *mut c_ulong,
}

impl Clone for bitmask
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for bitmask
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}


extern "C"
{
	pub static mut numa_all_nodes_ptr: *mut bitmask;
	pub static mut numa_nodes_ptr: *mut bitmask;
	pub static mut numa_all_nodes: nodemask_t;
	pub static mut numa_all_cpus_ptr: *mut bitmask;
	pub static mut numa_no_nodes_ptr: *mut bitmask;
	pub static mut numa_no_nodes: nodemask_t;
	pub static mut numa_exit_on_error: c_int;
	pub static mut numa_exit_on_warn: c_int;
	
	pub fn get_mempolicy(policy: *mut c_int, nmask: *const c_ulong, maxnode: c_ulong, addr: *mut c_void, flags: c_int) -> c_long;
	pub fn set_mempolicy(mode: c_int, nmask: *const c_ulong, maxnode: c_ulong) -> c_long;
	pub fn mbind(start: *mut c_void, len: c_ulong, mode: c_int, nmask: *const c_ulong, maxnode: c_ulong, flags: c_uint) -> c_long;
	pub fn migrate_pages(pid: c_int, maxnode: c_ulong, frommask: *const c_ulong, tomask: *const c_ulong) -> c_long;
	pub fn move_pages(pid: c_int, count: c_ulong, pages: *mut *mut c_void, nodes: *const c_int, status: *mut c_int, flags: c_int) -> c_long;
	
	pub fn numa_bitmask_isbitset(arg1: *const bitmask, arg2: c_uint) -> c_int;
	pub fn numa_bitmask_setall(arg1: *mut bitmask) -> *mut bitmask;
	pub fn numa_bitmask_clearall(arg1: *mut bitmask) -> *mut bitmask;
	pub fn numa_bitmask_setbit(arg1: *mut bitmask, arg2: c_uint) -> *mut bitmask;
	pub fn numa_bitmask_clearbit(arg1: *mut bitmask, arg2: c_uint) -> *mut bitmask;
	pub fn numa_bitmask_nbytes(arg1: *mut bitmask) -> c_uint;
	pub fn numa_bitmask_weight(arg1: *const bitmask) -> c_uint;
	pub fn numa_bitmask_alloc(arg1: c_uint) -> *mut bitmask;
	pub fn numa_bitmask_free(arg1: *mut bitmask);
	pub fn numa_bitmask_equal(arg1: *const bitmask, arg2: *const bitmask) -> c_int;
	pub fn copy_nodemask_to_bitmask(arg1: *mut nodemask_t, arg2: *mut bitmask);
	pub fn copy_bitmask_to_nodemask(arg1: *mut bitmask, arg2: *mut nodemask_t);
	pub fn copy_bitmask_to_bitmask(arg1: *mut bitmask, arg2: *mut bitmask);
	pub fn numa_available() -> c_int;
	pub fn numa_max_node() -> c_int;
	pub fn numa_max_possible_node() -> c_int;
	pub fn numa_preferred() -> c_int;
	pub fn numa_node_size64(node: c_int, freep: *mut c_longlong) -> c_longlong;
	pub fn numa_node_size(node: c_int, freep: *mut c_long) -> c_long;
	pub fn numa_pagesize() -> c_int;
	pub fn numa_bind(nodes: *mut bitmask);
	pub fn numa_set_interleave_mask(nodemask: *mut bitmask);
	pub fn numa_get_interleave_mask() -> *mut bitmask;
	pub fn numa_allocate_nodemask() -> *mut bitmask;
	pub fn numa_set_preferred(node: c_int);
	pub fn numa_set_localalloc();
	pub fn numa_set_membind(nodemask: *mut bitmask);
	pub fn numa_get_membind() -> *mut bitmask;
	pub fn numa_get_mems_allowed() -> *mut bitmask;
	pub fn numa_get_interleave_node() -> c_int;
	pub fn numa_alloc_interleaved_subset(size: size_t, nodemask: *mut bitmask) -> *mut c_void;
	pub fn numa_alloc_interleaved(size: size_t) -> *mut c_void;
	pub fn numa_alloc_onnode(size: size_t, node: c_int) -> *mut c_void;
	pub fn numa_alloc_local(size: size_t) -> *mut c_void;
	pub fn numa_alloc(size: size_t) -> *mut c_void;
	pub fn numa_realloc(old_addr: *mut c_void, old_size: size_t, new_size: size_t) -> *mut c_void;
	pub fn numa_free(mem: *mut c_void, size: size_t);
	pub fn numa_interleave_memory(mem: *mut c_void, size: size_t, mask: *mut bitmask);
	pub fn numa_tonode_memory(start: *mut c_void, size: size_t, node: c_int);
	pub fn numa_tonodemask_memory(mem: *mut c_void, size: size_t, mask: *mut bitmask);
	pub fn numa_setlocal_memory(start: *mut c_void, size: size_t);
	pub fn numa_police_memory(start: *mut c_void, size: size_t);
	pub fn numa_run_on_node_mask(mask: *mut bitmask) -> c_int;
	pub fn numa_run_on_node_mask_all(mask: *mut bitmask) -> c_int;
	pub fn numa_run_on_node(node: c_int) -> c_int;
	pub fn numa_get_run_node_mask() -> *mut bitmask;
	pub fn numa_set_bind_policy(strict: c_int);
	pub fn numa_set_strict(flag: c_int);
	pub fn numa_num_possible_nodes() -> c_int;
	pub fn numa_num_possible_cpus() -> c_int;
	pub fn numa_num_configured_nodes() -> c_int;
	pub fn numa_num_configured_cpus() -> c_int;
	pub fn numa_num_task_cpus() -> c_int;
	pub fn numa_num_thread_cpus() -> c_int;
	pub fn numa_num_task_nodes() -> c_int;
	pub fn numa_num_thread_nodes() -> c_int;
	pub fn numa_allocate_cpumask() -> *mut bitmask;
	pub fn numa_node_to_cpus(arg1: c_int, arg2: *mut bitmask) -> c_int;
	pub fn numa_node_of_cpu(cpu: c_int) -> c_int;
	pub fn numa_distance(node1: c_int, node2: c_int) -> c_int;
	pub fn numa_error(_where: *mut c_char);
	pub fn numa_warn(num: c_int, fmt: *mut c_char, ...);
	pub fn numa_migrate_pages(pid: c_int, from: *mut bitmask, to: *mut bitmask) -> c_int;
	pub fn numa_move_pages(pid: c_int, count: c_ulong, pages: *mut *mut c_void, nodes: *const c_int, status: *mut c_int, flags: c_int) -> c_int;
	pub fn numa_sched_getaffinity(arg1: pid_t, arg2: *mut bitmask) -> c_int;
	pub fn numa_sched_setaffinity(arg1: pid_t, arg2: *mut bitmask) -> c_int;
	pub fn numa_parse_nodestring(arg1: *const c_char) -> *mut bitmask;
	pub fn numa_parse_nodestring_all(arg1: *const c_char) -> *mut bitmask;
	pub fn numa_parse_cpustring(arg1: *const c_char) -> *mut bitmask;
	pub fn numa_parse_cpustring_all(arg1: *const c_char) -> *mut bitmask;
}
