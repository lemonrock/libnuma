// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


#![feature(unsafe_no_drop_flag)]


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


pub use _bitmask::bitmask;
#[path="bitmask.rs"] mod _bitmask;


pub use _nodemask_t::nodemask_t;
pub use _nodemask_t::NUMA_NUM_NODES;
#[path="nodemask_t.rs"] mod _nodemask_t;

#[link(name = "numa")]
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
	
	

	
	pub fn numa_available() -> c_int;
	pub fn numa_max_node() -> c_int;
	pub fn numa_max_possible_node() -> c_int;
	pub fn numa_preferred() -> c_int;
	pub fn numa_node_size64(node: c_int, freep: *mut c_longlong) -> c_longlong;
	pub fn numa_node_size(node: c_int, freep: *mut c_long) -> c_long;
	pub fn numa_pagesize() -> c_int;
	pub fn numa_set_preferred(node: c_int);
	pub fn numa_set_localalloc();
	pub fn numa_get_interleave_node() -> c_int;
	pub fn numa_alloc_interleaved(size: size_t) -> *mut c_void;
	pub fn numa_alloc_onnode(size: size_t, node: c_int) -> *mut c_void;
	pub fn numa_alloc_local(size: size_t) -> *mut c_void;
	pub fn numa_alloc(size: size_t) -> *mut c_void;
	pub fn numa_realloc(old_addr: *mut c_void, old_size: size_t, new_size: size_t) -> *mut c_void;
	pub fn numa_free(mem: *mut c_void, size: size_t);
	pub fn numa_tonode_memory(start: *mut c_void, size: size_t, node: c_int);
	pub fn numa_setlocal_memory(start: *mut c_void, size: size_t);
	pub fn numa_police_memory(start: *mut c_void, size: size_t);
	pub fn numa_run_on_node(node: c_int) -> c_int;
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
	pub fn numa_node_of_cpu(cpu: c_int) -> c_int;
	pub fn numa_distance(node1: c_int, node2: c_int) -> c_int;
	pub fn numa_move_pages(pid: c_int, count: c_ulong, pages: *mut *mut c_void, nodes: *const c_int, status: *mut c_int, flags: c_int) -> c_int;
	pub fn numa_error(_where: *mut c_char);
	pub fn numa_warn(num: c_int, fmt: *mut c_char, ...);
}
