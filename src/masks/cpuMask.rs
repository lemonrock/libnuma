// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


use std::ops::Drop;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::hash::Hash;
use std::hash::Hasher;
use std::clone::Clone;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::null_mut;
use std::ffi::CStr;
use std::result::Result;
use std::io::ErrorKind;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::pid_t;
use ::libc::EFAULT;
use ::libc::EINVAL;
use ::libc::EPERM;
use ::libc::ESRCH;
extern crate errno;
use self::errno::errno;
use ::bitmask;
use ::masks::Mask;
use ::bits::Cpu;


#[derive(Debug)]
pub struct CpuMask(pub *mut bitmask);

impl Drop for CpuMask
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.0.is_null()
		{
			return;
		}
		drop(self.0);
		self.0 = null_mut()
	}
}

impl PartialEq for CpuMask
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.0.eq(&other.0)
	}
}

impl Eq for CpuMask
{
}

impl Hash for CpuMask
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.deref().hash(state)
	}
}

impl Clone for CpuMask
{
	fn clone(&self) -> Self
	{
		CpuMask((*self).internal_clone())
	}
}

impl Deref for CpuMask
{
	type Target = bitmask;

	fn deref(&self) -> &bitmask
	{
		unsafe { &*self.0 }
	}
}

impl DerefMut for CpuMask
{
	fn deref_mut(&mut self) -> &mut bitmask
	{
		unsafe { self.0.as_mut().unwrap() }
	}
}

impl Mask<Cpu> for CpuMask
{
	#[inline(always)]
	fn allocate() -> CpuMask
	{
		CpuMask(unsafe { numa_allocate_cpumask() })
	}
	
	#[inline(always)]
	fn parse_string(string: &CStr) -> CpuMask
	{
		CpuMask(unsafe { numa_parse_cpustring(string.as_ptr()) })
	}
	
	#[inline(always)]
	fn parse_string_all(string: &CStr) -> CpuMask
	{
		CpuMask(unsafe { numa_parse_cpustring_all(string.as_ptr()) })
	}
}

impl CpuMask
{
	/// This operation is a touch expensive, as it clones an otherwise static value (otherwise a drop could occur that wiped out the static)
	#[inline(always)]
	pub fn all_cpus<'a>() -> CpuMask
	{
		CpuMask(unsafe { numa_all_cpus_ptr }).clone()
	}
	
	#[inline(always)]
	pub fn get_run_node_mask() -> CpuMask
	{
		CpuMask(unsafe { numa_get_run_node_mask() })
	}
	
	#[inline(always)]
	pub fn run_on_node_mask(&mut self)
	{
		match unsafe { numa_run_on_node_mask(self.0) }
		{
			0 => (),
			-1 => match errno().0
			{
				unexpected @ _ => panic!("numa_run_on_node_mask set errno {} but the man pages helpfully don't tell us what errors to expect", unexpected),
			},
			unexpected @ _ => panic!("Did not expect numa_run_on_node_mask to return {}", unexpected),
		}
	}
	
	#[inline(always)]
	pub fn bind(&mut self)
	{
		unsafe { numa_bind(self.0) }
	}
	
	pub fn sched_get_affinity_for_current_thread() -> Self
	{
		let mut all_nodes = Self::allocate();
		let result = all_nodes.sched_get_affinity_for_task(0);
		if result.is_err()
		{
			panic!("sched_get_affinity failed for current thread");
		} 
		all_nodes
	}
	
	/// task_id is either thread id (tid) or process id (pid); the main thread in every process has the same id value as the process id
	/// task_id 0 refers to self
	/// thread id IS NOT the same as pthread_t, although thre struct pointed to be pthread_t in Bionic and Musl contains it as a field
	/// Bionic makes this available via the non-portable pthread_gettid_np functions
	/// On Glibc, one must execute a syscall for gettid
	/// Result is sizeof(cpumask_t)
	pub fn sched_get_affinity_for_task(&mut self, task_id: pid_t) -> Result<(usize), ErrorKind>
	{
		match unsafe { numa_sched_getaffinity(task_id, self.0) }
		{
			size_of_cpumask_t if size_of_cpumask_t >= 0 => Ok(size_of_cpumask_t as usize),
			-1 => match errno().0
			{
				EFAULT => panic!("EFAULT for numa_sched_getaffinity"),
				EINVAL => panic!("? no processors or mask too small ? Great. Can't distinguish. numa_sched_getaffinity. EINVAL."),
				ESRCH => Err(ErrorKind::NotFound),
				unexpected @ _ => panic!("Did not expect numa_move_pages to set errno {}", unexpected),
			},
			unexpected @ _ => panic!("Did not expect numa_sched_getaffinity to return {}", unexpected),
		}
	}
	
	pub fn sched_set_affinity_for_current_thread(&mut self) -> bool
	{
		match self.sched_set_affinity_for_task(0)
		{
			Ok(_) => true,
			Err(err) =>
			{
				match err
				{
					ErrorKind::Other => false,
					ErrorKind::PermissionDenied | ErrorKind::NotFound => panic!("sched_get_affinity_for_task for self (task_id 0) should not produce an ErrorKind of {:?}", err),
					_ => panic!("Unexpected ErrorKind {:?}", err),
				}
			}
		}
	}
	
	/// task_id is either thread id (tid) or process id (pid); the main thread in every process has the same id value as the process id
	/// thread id IS NOT the same as pthread_t, although thre struct pointed to be pthread_t in Bionic and Musl contains it as a field
	/// Bionic makes this available via the non-portable pthread_gettid_np functions
	/// On Glibc, one must execute a syscall for gettid
	pub fn sched_set_affinity_for_task(&mut self, task_id: pid_t) -> Result<(), ErrorKind>
	{
		match unsafe { numa_sched_setaffinity(task_id, self.0) }
		{
			0 => Ok(()),
			-1 => match errno().0
			{
				EFAULT => panic!("EFAULT for numa_sched_setaffinity"),
				EINVAL => Err(ErrorKind::Other),
				EPERM => Err(ErrorKind::PermissionDenied),
				ESRCH => Err(ErrorKind::NotFound),
				unexpected @ _ => panic!("Did not expect numa_move_pages to set errno {}", unexpected),
			},
			unexpected @ _ => panic!("Did not expect numa_sched_getaffinity to return {}", unexpected),
		}
	}
}

extern "C"
{
	static mut numa_all_cpus_ptr: *mut bitmask;
	fn numa_allocate_cpumask() -> *mut bitmask;
	fn numa_parse_cpustring(string: *const c_char) -> *mut bitmask;
	fn numa_parse_cpustring_all(string: *const c_char) -> *mut bitmask;
	fn numa_get_run_node_mask() -> *mut bitmask;
	fn numa_run_on_node_mask(mask: *mut bitmask) -> c_int;
	fn numa_bind(nodes: *mut bitmask);
	fn numa_sched_getaffinity(pid: pid_t, mask: *mut bitmask) -> c_int; // result is NOT as documented in manpages...
	fn numa_sched_setaffinity(pid: pid_t, mask: *mut bitmask) -> c_int;
}
