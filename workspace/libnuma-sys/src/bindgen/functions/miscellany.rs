// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


extern "C"
{
	pub fn numa_alloc(size: usize) -> *mut c_void;
	pub fn numa_alloc_interleaved(size: usize) -> *mut c_void;
	pub fn numa_alloc_interleaved_subset(size: usize, nodemask: *mut bitmask) -> *mut c_void;
	pub fn numa_alloc_local(size: usize) -> *mut c_void;
	pub fn numa_alloc_onnode(size: usize, node: c_int) -> *mut c_void;
	pub fn numa_allocate_cpumask() -> *mut bitmask;
	pub fn numa_allocate_nodemask() -> *mut bitmask;
	pub fn numa_available() -> c_int;
	pub fn numa_bind(nodes: *mut bitmask);
	pub fn numa_distance(node1: c_int, node2: c_int) -> c_int;
	pub fn numa_error(where_: *mut c_char);
	pub fn numa_free(mem: *mut c_void, size: usize);
	pub fn numa_get_interleave_mask() -> *mut bitmask;
	pub fn numa_get_interleave_node() -> c_int;
	pub fn numa_get_membind() -> *mut bitmask;
	pub fn numa_get_mems_allowed() -> *mut bitmask;
	pub fn numa_get_run_node_mask() -> *mut bitmask;
	pub fn numa_interleave_memory(mem: *mut c_void, size: usize, mask: *mut bitmask);
	pub fn numa_max_node() -> c_int;
	pub fn numa_max_possible_node() -> c_int;
	pub fn numa_migrate_pages(pid: c_int, from: *mut bitmask, to: *mut bitmask) -> c_int;
	pub fn numa_move_pages(pid: c_int, count: c_ulong, pages: *mut *mut c_void, nodes: *const c_int, status: *mut c_int, flags: c_int) -> c_int;
	pub fn numa_node_of_cpu(cpu: c_int) -> c_int;
	pub fn numa_node_size(node: c_int, freep: *mut c_long) -> c_long;
	pub fn numa_node_size64(node: c_int, freep: *mut c_longlong) -> c_longlong;
	pub fn numa_node_to_cpus(arg1: c_int, arg2: *mut bitmask) -> c_int;
	pub fn numa_num_configured_cpus() -> c_int;
	pub fn numa_num_configured_nodes() -> c_int;
	pub fn numa_num_possible_cpus() -> c_int;
	pub fn numa_num_possible_nodes() -> c_int;
	pub fn numa_num_task_cpus() -> c_int;
	pub fn numa_num_task_nodes() -> c_int;
	pub fn numa_num_thread_cpus() -> c_int;
	pub fn numa_num_thread_nodes() -> c_int;
	pub fn numa_pagesize() -> c_int;
	pub fn numa_parse_cpustring(arg1: *const c_char) -> *mut bitmask;
	pub fn numa_parse_cpustring_all(arg1: *const c_char) -> *mut bitmask;
	pub fn numa_parse_nodestring(arg1: *const c_char) -> *mut bitmask;
	pub fn numa_parse_nodestring_all(arg1: *const c_char) -> *mut bitmask;
	pub fn numa_police_memory(start: *mut c_void, size: usize);
	pub fn numa_preferred() -> c_int;
	pub fn numa_realloc(old_addr: *mut c_void, old_size: usize, new_size: usize) -> *mut c_void;
	pub fn numa_run_on_node(node: c_int) -> c_int;
	pub fn numa_run_on_node_mask(mask: *mut bitmask) -> c_int;
	pub fn numa_run_on_node_mask_all(mask: *mut bitmask) -> c_int;
	pub fn numa_sched_getaffinity(arg1: pid_t, arg2: *mut bitmask) -> c_int;
	pub fn numa_sched_setaffinity(arg1: pid_t, arg2: *mut bitmask) -> c_int;
	pub fn numa_set_bind_policy(strict: c_int);
	pub fn numa_set_interleave_mask(nodemask: *mut bitmask);
	pub fn numa_set_localalloc();
	pub fn numa_set_membind(nodemask: *mut bitmask);
	pub fn numa_set_preferred(node: c_int);
	pub fn numa_set_strict(flag: c_int);
	pub fn numa_setlocal_memory(start: *mut c_void, size: usize);
	pub fn numa_tonode_memory(start: *mut c_void, size: usize, node: c_int);
	pub fn numa_tonodemask_memory(mem: *mut c_void, size: usize, mask: *mut bitmask);
	pub fn numa_warn(num: c_int, fmt: *mut c_char, ...);
}
