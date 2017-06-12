// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NodeIndex(pub(crate) u8);

impl Index for NodeIndex
{
	#[inline(always)]
	fn to_c_uint(&self) -> c_uint
	{
		self.0 as c_uint
	}
}

impl Default for NodeIndex
{
	#[inline(always)]
	fn default() -> Self
	{
		NodeIndex(0)
	}
}

//noinspection SpellCheckingInspection
impl NodeIndex
{
	#[inline(always)]
	pub fn new(value: u8) -> Self
	{
		NodeIndex(value)
	}
	
	/// length, size, count
	/// Equivalent to Kernel's MAX_NUMNODES; should match nodemask_t's NUMA_NUM_NODES
	/// Read from /proc/self/status, field Mems_allowed
	#[inline(always)]
	pub fn number_of_nodes_in_nodemask() -> usize
	{
		match unsafe { numa_num_possible_nodes() }
		{
			x if x.is_negative() => panic!("numa_num_possible_nodes returned a negative value {}", x),
			x @ _ => x as usize,
		}
	}
	
	/// Number (Count) of nodes we can use
	#[inline(always)]
	pub fn number_of_nodes_permitted() -> usize
	{
		match unsafe { numa_num_task_nodes() }
		{
			x if x.is_negative() => panic!("numa_num_task_nodes returned a negative value {}", x),
			0 => panic!("numa_num_task_nodes returned 0"),
			x @ _ => x as usize,
		}
	}
	
	pub fn number_of_node_configured() -> usize
	{
		match unsafe { numa_num_configured_nodes() }
		{
			x if x.is_negative() => panic!("numa_num_configured_nodes returned a negative value {}", x),
			0 => panic!("numa_num_configured_nodes returned 0"),
			x @ _ => x as usize,
		}
	}
	
	/// numa_max_possible_node; not particularly useful, should be number_of_nodes_in_nodemask - 1
	#[inline(always)]
	pub fn highest_possible() -> Self
	{
		match unsafe { numa_max_possible_node() }
		{
			x if x.is_negative() => panic!("numa_max_possible_node returned a negative value {}", x),
			x if x > 255 => panic!("Wow, a node index greater than 255 should be impossible"),
			x @ _ => NodeIndex(x as u8),
		}
	}
	
	/// USE THIS
	pub fn highest_available_currently() -> Self
	{
		match unsafe { numa_max_node() }
		{
			x if x.is_negative() => panic!("numa_max_node returned a negative value {}", x),
			x if x > 255 => panic!("Wow, a node index greater than 255 should be impossible"),
			x @ _ => NodeIndex(x as u8),
		}
	}
	
	/// If there is no preferred node, or the memory binding policy is not PREFERRED or BIND, then NodeIndex will be the default (0)
	/// Hence it is impossible to distinguish no preference (or default to whatever CPU currently in use) from a preference for NodeIndex 0...
	#[inline(always)]
	pub fn preferred() -> Self
	{
		match unsafe { numa_preferred() }
		{
			x if x.is_negative() => panic!("numa_preferred returned a negative value {}", x),
			x if x > 255 => panic!("Wow, a node index greater than 255 should be impossible"),
			x @ _ => NodeIndex(x as u8),
		}
	}
	
	/// It is impossible to distinguish no result from the default node
	#[inline(always)]
	pub fn interleaved() -> Self
	{
		match unsafe { numa_get_interleave_node() }
		{
			x if x.is_negative() => panic!("numa_get_interleave_node returned a negative value {}", x),
			x if x > 255 => panic!("Wow, a node index greater than 255 should be impossible"),
			x @ _ => NodeIndex(x as u8),
		}
	}
	
	#[inline(always)]
	pub fn allows_try_to_allocate_from_this_node_for_the_current_thread_before_falling_back_to_other_nodes(&self)
	{
		unsafe { numa_set_preferred(self.0 as c_int) }
	}
	
	/// Does not use factors or 10, but instead rescales and subtracts 1, so 0 is self
	/// Also converts to unsigned form
	pub fn distance(&self, to: &NodeIndex) -> Option<u8>
	{
		match unsafe { numa_distance(self.0 as c_int, to.0 as c_int) }
		{
			0 => None,
			x if x.is_negative() => panic!("numa_distance returned a negative value {}", x),
			x if x % 10 == 0 => Some(((x as u64 / 10) - 1) as u8),
			x @ _ => panic!("numa_distance returned a-non-power-of-ten factor {}", x),
		}
	}
	
	/// Returns a tuple of Size, Free Memory
	/// Expensive to call, uses /sys
	pub fn size(&self) -> (usize, usize)
	{
		let mut free: c_longlong = unsafe { uninitialized() };
		let free_pointer: *mut c_longlong = &mut free;
		
		match unsafe { numa_node_size64(self.0 as c_int, free_pointer) }
		{
			-1 => panic!("numa_node_size64 returned an error"),
			x if x.is_negative() => panic!("numa_node_size64 returned a negative memory size {}", x),
			size @ _ =>
				{
					if free.is_negative()
					{
						panic!("numa_node_size64 returned a negative free memory size {}", free);
					}
					(size as usize, free as usize)
				},
		}
	}
	
	// equivalent to setaffinity, setting the cpumask to be all the cpus belonging to this NodeIndex
	pub fn run_current_thread_on_this(&self)
	{
		match unsafe { numa_run_on_node(self.0 as c_int) }
		{
			0 => (),
			-1 => match errno().0
			{
				::libc::EFAULT => panic!("numa_run_on_node EFAULT"),
				::libc::EINVAL => panic!("numa_run_on_node EINVAL - is the NodeIndex value in range?"), // bad node number or internal call to sched_setaffinity failed
				::libc::EPERM => panic!("numa_run_on_node EPERM"),
				::libc::ESRCH => panic!("numa_run_on_node ESRCH"),
				unexpected @ _ => panic!("numa_run_on_node set an unexpected errno {}; this is possible because not all paths in its code seem to set errno", unexpected),
			},
			unexpected @ _ => panic!("numa_run_on_node returned unexpected value {}", unexpected),
		}
	}
	
	// equivalent to setaffinity, setting the cpumask to be all the cpus (ie default situation)
	pub fn run_on_any_cpu()
	{
		match unsafe { numa_run_on_node(-1) }
		{
			0 => (),
			-1 => match errno().0
			{
				::libc::EFAULT => panic!("numa_run_on_node EFAULT"),
				::libc::EINVAL => panic!("numa_run_on_node EINVAL"),
				::libc::EPERM => panic!("numa_run_on_node EPERM"),
				::libc::ESRCH => panic!("numa_run_on_node ESRCH"),
				unexpected @ _ => panic!("numa_run_on_node set an unexpected errno {}; this is possible because not all paths in its code seem to set errno", unexpected),
			},
			unexpected @ _ => panic!("numa_run_on_node returned unexpected value {}", unexpected),
		}
	}
	
	#[inline(always)]
	pub fn allocate(&self, size: size_t) -> NumaMemory
	{
		NumaMemory::wrap(unsafe { numa_alloc_onnode(size, self.0 as c_int) }, size)
	}
	
	pub fn node_to_cpus(&mut self) -> CpuMask
	{
		let cpu_mask = CpuMask::allocate();
		
		match unsafe { numa_node_to_cpus(self.0 as c_int, (cpu_mask.0).0) }
		{
			0 => cpu_mask,
			-1 => match errno().0
			{
				::libc::ERANGE => panic!("numa_node_to_cpus ERANGE"),
				unexpected @ _ => panic!("numa_node_to_cpus set an unexpected errno {}", unexpected),
			},
			unexpected @ _ => panic!("numa_node_to_cpus returned unexpected value {}", unexpected),
		}
	}
}
