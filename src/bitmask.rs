// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


#[cfg(not(feature = "use_std"))] extern crate std as core;
use self::core::ops::Drop;
use ::libc::c_ulong;
use ::libc::c_uint;
use ::libc::c_int;
use ::libc::size_t;
use ::libc::pid_t;
use ::libc::c_void;
use ::libc::c_char;
use super::nodemask_t;


#[repr(C)]
#[derive(Debug)]
#[unsafe_no_drop_flag]
pub struct bitmask
{
	pub size: c_ulong,
	pub maskp: *mut c_ulong,
}

// https://stackoverflow.com/questions/30742004/correct-idiom-for-freeing-reprc-structs-using-drop-trait#30742180
impl Drop for bitmask
{
	#[allow(trivial_casts)]
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.maskp as c_ulong == 0xdeadcdef as c_ulong
		{
			return;
		}
		unsafe { numa_bitmask_free(self as *mut bitmask) }
	}
}

extern "C"
{
	fn numa_allocate_nodemask() -> *mut bitmask;
	fn numa_allocate_cpumask() -> *mut bitmask;
	fn numa_get_interleave_mask() -> *mut bitmask;
	fn numa_get_membind() -> *mut bitmask;
	fn numa_get_mems_allowed() -> *mut bitmask;
	fn numa_get_run_node_mask() -> *mut bitmask;
	fn numa_bitmask_alloc(bmp: c_uint) -> *mut bitmask;
	
	fn numa_parse_nodestring(string: *const c_char) -> *mut bitmask;
	fn numa_parse_cpustring(string: *const c_char) -> *mut bitmask;
	fn numa_parse_nodestring_all(string: *const c_char) -> *mut bitmask;
	fn numa_parse_cpustring_all(string: *const c_char) -> *mut bitmask;
	
	fn numa_bind(nodes: *mut bitmask);
	fn numa_set_interleave_mask(nodemask: *mut bitmask);
	fn numa_set_membind(nodemask: *mut bitmask);
	fn numa_run_on_node_mask(mask: *mut bitmask) -> c_int;
	fn numa_run_on_node_mask_all(mask: *mut bitmask) -> c_int;
	fn numa_alloc_interleaved_subset(size: size_t, nodemask: *mut bitmask) -> *mut c_void;
	fn numa_interleave_memory(mem: *mut c_void, size: size_t, mask: *mut bitmask);
	fn numa_tonodemask_memory(mem: *mut c_void, size: size_t, mask: *mut bitmask);
	fn numa_node_to_cpus(node: c_int, mask: *mut bitmask) -> c_int;
	fn numa_migrate_pages(pid: c_int, from: *mut bitmask, to: *mut bitmask) -> c_int;
	fn numa_sched_getaffinity(pid: pid_t, mask: *mut bitmask) -> c_int;
	fn numa_sched_setaffinity(pid: pid_t, mask: *mut bitmask) -> c_int;
	
	fn numa_bitmask_clearall(bmp: *mut bitmask) -> *mut bitmask;
	fn numa_bitmask_clearbit(bmp: *mut bitmask, n: c_uint) -> *mut bitmask;
	fn numa_bitmask_equal(bmp1: *const bitmask, bmp2: *const bitmask) -> c_int;
	fn numa_bitmask_free(bmp: *mut bitmask);
	fn numa_bitmask_isbitset(bmp: *const bitmask, n: c_uint) -> c_int;
	fn numa_bitmask_nbytes(bmp: *mut bitmask) -> c_uint;
	fn numa_bitmask_setall(bmp: *mut bitmask) -> *mut bitmask;
	fn numa_bitmask_setbit(bmp: *mut bitmask, n: c_uint) -> *mut bitmask;
	fn copy_bitmask_to_nodemask(bmp: *mut bitmask, nodemask: *mut nodemask_t);
	fn copy_bitmask_to_bitmask(bmpfrom: *mut bitmask, bmpto: *mut bitmask);
	fn numa_bitmask_weight(bmp: *const bitmask) -> c_uint;
}

impl bitmask
{
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn get_interleave_mask<'a>() -> &'a mut bitmask
	{
		let reference = unsafe { numa_get_interleave_mask().as_mut() };
		reference.expect("Underlying libnuma API numa_get_interleave_mask failed")
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn allocate_nodemask<'a>() -> &'a mut bitmask
	{
		let reference = unsafe { numa_allocate_nodemask().as_mut() };
		reference.expect("Underlying libnuma API numa_allocate_nodemask failed")
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn allocate_cpumask<'a>() -> &'a mut bitmask
	{
		let reference = unsafe { numa_allocate_cpumask().as_mut() };
		reference.expect("Underlying libnuma API numa_allocate_cpumask failed")
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn get_membind<'a>() -> &'a mut bitmask
	{
		let reference = unsafe { numa_get_membind().as_mut() };
		reference.expect("Underlying libnuma API numa_get_membind failed")
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn get_mems_allowed<'a>() -> &'a mut bitmask
	{
		let reference = unsafe { numa_get_mems_allowed().as_mut() };
		reference.expect("Underlying libnuma API numa_get_mems_allowed failed")
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn get_run_node_mask<'a>() -> &'a mut bitmask
	{
		let reference = unsafe { numa_get_run_node_mask().as_mut() };
		reference.expect("Underlying libnuma API numa_get_run_node_mask failed")
	}
	
	// #[cfg(not(feature = "use_std"))]
	// #[allow(trivial_casts)]
	// #[inline(always)]
	// pub fn parse_node_string<'a>(string: &std::ffi::CStr) -> &'a mut bitmask
	// {
	// 	let reference = unsafe { numa_parse_nodestring(string.as_ptr()).as_mut() };
	// 	reference.expect("Underlying libnuma API numa_parse_nodestring failed")
	// }
	//
	// #[cfg(not(feature = "use_std"))]
	// #[allow(trivial_casts)]
	// #[inline(always)]
	// pub fn parse_cpu_string<'a>(string: &std::ffi::CStr) -> &'a mut bitmask
	// {
	// 	let reference = unsafe { numa_parse_cpustring(string.as_ptr()).as_mut() };
	// 	reference.expect("Underlying libnuma API numa_parse_cpustring failed")
	// }
	//
	// #[cfg(not(feature = "use_std"))]
	// #[allow(trivial_casts)]
	// #[inline(always)]
	// pub fn parse_node_string_all<'a>(string: &std::ffi::CStr) -> &'a mut bitmask
	// {
	// 	let reference = unsafe { numa_parse_nodestring_all(string.as_ptr()).as_mut() };
	// 	reference.expect("Underlying libnuma API numa_parse_nodestring failed")
	// }
	//
	// #[cfg(not(feature = "use_std"))]
	// #[allow(trivial_casts)]
	// #[inline(always)]
	// pub fn parse_cpu_string_all<'a>(string: &std::ffi::CStr) -> &'a mut bitmask
	// {
	// 	let reference = unsafe { numa_parse_cpustring_all(string.as_ptr()).as_mut() };
	// 	reference.expect("Underlying libnuma API numa_parse_cpustring failed")
	// }
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn allocate<'a>(size: usize) -> &'a mut bitmask
	{
		let reference = unsafe { numa_bitmask_alloc(size as c_uint).as_mut() };
		reference.expect("Underlying libnuma API numa_bitmask_alloc failed")
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn bind(&mut self)
	{
		unsafe { numa_bind(self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn set_interleave_mask(&mut self)
	{
		unsafe { numa_set_interleave_mask(self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn set_membind(&mut self)
	{
		unsafe { numa_set_membind(self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn run_on_node_mask(&mut self) -> c_int
	{
		unsafe { numa_run_on_node_mask(self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn run_on_node_mask_all(&mut self) -> c_int
	{
		unsafe { numa_run_on_node_mask_all(self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn allocate_interleaved_subset(&mut self, size: size_t) -> *mut c_void
	{
		unsafe { numa_alloc_interleaved_subset(size, self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn interleave_memory(&mut self, size: size_t, memory: *mut c_void)
	{
		unsafe { numa_interleave_memory(memory, size, self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn to_nodemask_memory(&mut self, size: size_t, memory: *mut c_void)
	{
		unsafe { numa_tonodemask_memory(memory, size, self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn node_to_cpus(&mut self, node: c_int) -> c_int
	{
		unsafe { numa_node_to_cpus(node, self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn migrate_pages(&mut self, pid: c_int, to: &mut bitmask) -> c_int
	{
		unsafe { numa_migrate_pages(pid, self as *mut bitmask, to as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn sched_getaffinity(&mut self, pid: pid_t) -> c_int
	{
		unsafe { numa_sched_getaffinity(pid, self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn sched_setaffinity(&mut self, pid: pid_t) -> c_int
	{
		unsafe { numa_sched_setaffinity(pid, self as *mut bitmask) }
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn clear_all(&mut self) -> &mut Self
	{
		unsafe { numa_bitmask_clearall(self) };
		self
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn clear_bit(&mut self, bit: usize) -> &mut Self
	{
		unsafe { numa_bitmask_clearbit(self, bit as c_uint) };
		self
	}
	
	// make this into the PartialEq trait...
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn equal(&self, other: *const bitmask) -> bool
	{
		unsafe { numa_bitmask_equal(self, other) == 1 }
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn is_bit_set(&self, bit: usize) -> bool
	{
		unsafe { numa_bitmask_isbitset(self as *const bitmask, bit as c_uint) != 0 }
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn len(&mut self) -> c_uint
	{
		unsafe { numa_bitmask_nbytes(self as *mut bitmask) }
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn set_all(&mut self) -> &mut Self
	{
		unsafe { numa_bitmask_setall(self as *mut bitmask) };
		self
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn set_bit(&mut self, bit: usize) -> &mut Self
	{
		unsafe { numa_bitmask_setbit(self as *mut bitmask, bit as c_uint) };
		self
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn copy_into_nodemask(&mut self, nodemask: &mut nodemask_t)
	{
		unsafe { copy_bitmask_to_nodemask(self as *mut bitmask, nodemask as *mut nodemask_t) }
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn copy_into(&mut self, to: &mut bitmask)
	{
		unsafe { copy_bitmask_to_bitmask(self as *mut bitmask, to as *mut bitmask) }
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn weight(&self) -> usize
	{
		unsafe { numa_bitmask_weight(self as *const bitmask) as usize }
	}
}
