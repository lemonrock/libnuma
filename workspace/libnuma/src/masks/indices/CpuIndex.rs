// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CpuIndex(u16);

impl Index for CpuIndex
{
	#[inline(always)]
	fn to_c_uint(&self) -> c_uint
	{
		self.0 as c_uint
	}
}

//noinspection SpellCheckingInspection
impl CpuIndex
{
	#[inline(always)]
	pub fn new(value: u16) -> Self
	{
		CpuIndex(value)
	}
	
	/// length, size, count
	/// Kernel limit on number of cpus - same as NR_CPUS
	/// Equivalent concept to NodeIndex::number_of_nodes_in_nodemask()
	pub fn number_of_possible_cpus() -> usize
	{
		match unsafe { numa_num_possible_cpus() }
		{
			x if x.is_negative() => panic!("numa_num_possible_cpus returned a negative value {}", x),
			0 => panic!("numa_num_possible_cpus returned 0"),
			x @ _ => x as usize,
		}
	}
	
	/// Number (Count) of CPUs we can use
	pub fn number_of_permitted_cpus() -> usize
	{
		match unsafe { numa_num_task_cpus() }
		{
			x if x.is_negative() => panic!("numa_num_task_cpus returned a negative value {}", x),
			0 => panic!("numa_num_task_cpus returned 0"),
			x @ _ => x as usize,
		}
	}
	
	/// Number (Count) of CPUs we can use
	pub fn number_of_configured_cpus() -> usize
	{
		match unsafe { numa_num_configured_cpus() }
		{
			x if x.is_negative() => panic!("numa_num_configured_cpus returned a negative value {}", x),
			0 => panic!("numa_num_configured_cpus returned 0"),
			x @ _ => x as usize,
		}
	}
	
	/// Returns None if the cpu has no Node. This is because the CPU does not exist or is not online
	pub fn node_for_cpu(&self) -> Option<NodeIndex>
	{
		match unsafe { numa_node_of_cpu(self.0 as c_int) }
		{
			-1 => match errno().0
			{
				::libc::EINVAL => None,
				unexpected @ _ => panic!("numa_node_for_cpu set an unexpected errno {}", unexpected),
			},
			x if x.is_negative() => panic!("numa_num_possible_cpus returned a negative value {}", x),
			value @ _ => Some(NodeIndex(value as u8))
		}
	}
}
