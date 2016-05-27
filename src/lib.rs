// This file is part of libnuma-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT. No part of libnuma-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma-sys/master/COPYRIGHT.


//#![cfg(target_os = "linux")]
#![feature(unsafe_no_drop_flag)]
#![feature(const_fn)]


#[macro_use] extern crate bitflags;
extern crate libc;
use libc::c_int;
use libc::c_ulong;
use libc::c_void;
use libc::c_uint;
use libc::c_long;

pub use _bitmask::bitmask;
#[path="bitmask.rs"] mod _bitmask;

pub use cpuMask::CpuMask;
mod cpuMask;

pub use nodeMask::NodeMask;
mod nodeMask;

pub use memoryPolicyFlags::MemoryPolicyFlags;
mod memoryPolicyFlags;

pub use memoryPolicy::MemoryPolicy;
mod memoryPolicy;

pub use movePagesFlags::MovePagesFlags;
mod movePagesFlags;

pub use numaNode::NumaNode;
mod numaNode;

pub use cpu::Cpu;
mod cpu;

pub use memory::Memory;
mod memory;

pub use allocatableMemory::AllocatableMemory;
mod allocatableMemory;

pub use reAllocatableMemory::ReAllocatableMemory;
mod reAllocatableMemory;

pub use numaMemory::NumaMemory;
mod numaMemory;


#[link(name = "numa")]
extern "C"
{
}


pub const LIBNUMA_API_VERSION: c_int = 2;


pub fn initialize() -> bool
{
	match unsafe { numa_available() }
	{
		0 => true,
		-1 => false,
		unexpected @ _ => panic!("Did not expected numa_available to return {}", unexpected),
	}
}

#[inline(always)]
pub fn cached_page_size() -> usize
{
	match unsafe { numa_pagesize() }
	{
		x if x.is_positive() => x as usize,
		unexpected @ _ => panic!("numa_pagesize returned a non-positive value {}", unexpected),
	}
}

#[inline(always)]
pub fn locally_allocate_memory()
{
	unsafe { numa_set_localalloc() }
}

// Should normally be set as false (otherwise we can exhaust memory sooner)
pub fn set_strict(is_strict: bool)
{
	unsafe { numa_set_strict(if is_strict { 1 } else { 0 }) }
}

// Should normally be set as false (otherwise we can exhaust memory sooner)
pub fn set_bind_policy(is_strict: bool)
{
	unsafe { numa_set_bind_policy(if is_strict { 1 } else { 0 }) }
}

extern "C"
{
	fn numa_available() -> c_int;
	fn numa_pagesize() -> c_int;
	fn numa_set_localalloc();
	fn numa_set_bind_policy(strict: c_int);
	fn numa_set_strict(strict: c_int);
	
	// Not obviously useful; defined as weak symbols
	// pub fn numa_error(_where: *mut c_char);
	// pub fn numa_warn(num: c_int, fmt: *mut c_char, ...);
	
	
	
	
	
		
	pub static mut numa_all_nodes_ptr: *mut bitmask;
	pub static mut numa_no_nodes_ptr: *mut bitmask;
	pub static mut numa_nodes_ptr: *mut bitmask;
	
	pub static mut numa_all_cpus_ptr: *mut bitmask;
	
	pub static mut numa_exit_on_error: c_int;
	pub static mut numa_exit_on_warn: c_int;
	
	pub fn get_mempolicy(policy: *mut c_int, nmask: *const c_ulong, maxnode: c_ulong, addr: *mut c_void, flags: c_int) -> c_long;
	pub fn set_mempolicy(mode: c_int, nmask: *const c_ulong, maxnode: c_ulong) -> c_long;
	pub fn mbind(start: *mut c_void, len: c_ulong, mode: c_int, nmask: *const c_ulong, maxnode: c_ulong, flags: c_uint) -> c_long;
	pub fn migrate_pages(pid: c_int, maxnode: c_ulong, frommask: *const c_ulong, tomask: *const c_ulong) -> c_long;
}
