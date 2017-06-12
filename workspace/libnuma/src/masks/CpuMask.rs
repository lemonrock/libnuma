// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CpuMask(BitMask);

impl Default for CpuMask
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::allocate()
	}
}

impl Mask<CpuIndex> for CpuMask
{
	#[inline(always)]
	fn allocate() -> Self
	{
		CpuMask(BitMask(unsafe { numa_allocate_cpumask() }))
	}
	
	/// This operation is a touch expensive, as it clones an otherwise static value (otherwise a drop could occur that wiped out the static)
	#[inline(always)]
	fn all() -> Self
	{
		CpuMask(BitMask(unsafe { numa_all_cpus_ptr })).clone()
	}
	
	#[inline(always)]
	fn parse_string(string: &CStr) -> Self
	{
		CpuMask(BitMask(unsafe { numa_parse_cpustring(string.as_ptr()) }))
	}
	
	#[inline(always)]
	fn parse_string_all(string: &CStr) -> Self
	{
		CpuMask(BitMask(unsafe { numa_parse_cpustring_all(string.as_ptr()) }))
	}
	
	#[inline(always)]
	#[doc(hidden)]
	fn bit_mask(&self) -> &BitMask
	{
		&self.0
	}
}

//noinspection SpellCheckingInspection
impl CpuMask
{
	#[inline(always)]
	pub fn get_run_node_mask() -> CpuMask
	{
		CpuMask(BitMask(unsafe { numa_get_run_node_mask() }))
	}
	
	#[inline(always)]
	pub fn run_on_node_mask(&self)
	{
		match unsafe { numa_run_on_node_mask(self.bitmask()) }
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
	pub fn bind(&self)
	{
		unsafe { numa_bind(self.bitmask()) }
	}
	
	pub fn sched_get_affinity_for_current_thread() -> Self
	{
		let all_nodes = Self::allocate();
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
	pub fn sched_get_affinity_for_task(&self, task_id: pid_t) -> Result<usize, ErrorKind>
	{
		match unsafe { numa_sched_getaffinity(task_id, self.bitmask()) }
		{
			size_of_cpumask_t if size_of_cpumask_t >= 0 => Ok(size_of_cpumask_t as usize),
			-1 => match errno().0
			{
				::libc::EFAULT => panic!("EFAULT for numa_sched_getaffinity"),
				::libc::EINVAL => panic!("? no processors or mask too small ? Great. Can't distinguish. numa_sched_getaffinity. EINVAL."),
				::libc::ESRCH => Err(ErrorKind::NotFound),
				unexpected @ _ => panic!("Did not expect numa_move_pages to set errno {}", unexpected),
			},
			unexpected @ _ => panic!("Did not expect numa_sched_getaffinity to return {}", unexpected),
		}
	}
	
	pub fn sched_set_affinity_for_current_thread(&self) -> bool
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
	pub fn sched_set_affinity_for_task(&self, task_id: pid_t) -> Result<(), ErrorKind>
	{
		match unsafe { numa_sched_setaffinity(task_id, self.bitmask()) }
		{
			0 => Ok(()),
			-1 => match errno().0
			{
				::libc::EFAULT => panic!("EFAULT for numa_sched_setaffinity"),
				::libc::EINVAL => Err(ErrorKind::Other),
				::libc::EPERM => Err(ErrorKind::PermissionDenied),
				::libc::ESRCH => Err(ErrorKind::NotFound),
				unexpected @ _ => panic!("Did not expect numa_move_pages to set errno {}", unexpected),
			},
			unexpected @ _ => panic!("Did not expect numa_sched_getaffinity to return {}", unexpected),
		}
	}
	
	#[inline(always)]
	fn bitmask(&self) -> *mut bitmask
	{
		(self.0).0
	}
}
