// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


use std::default::Default;
extern crate libc;
use self::libc::c_int;
use self::libc::c_uint;
use self::libc::EINVAL;
extern crate errno;
use self::errno::errno;
use ::bits::Node;
use ::bits::Bit;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Cpu(c_int);

impl Bit for Cpu
{
	#[inline(always)]
	fn to_c_uint(&self) -> c_uint
	{
		self.0 as c_uint
	}
}

impl Cpu
{
	/// length, size, count
	/// Kernel limit on number of cpus - same as NR_CPUS
	/// Equivalent concept to Node::number_of_nodes_in_nodemask()
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
	pub fn node_for_cpu(&self) -> Option<Node>
	{
		match unsafe { numa_node_of_cpu(self.0) }
		{
			-1 => match errno().0
			{
				EINVAL => None,
				unexpected @ _ => panic!("numa_node_for_cpu set an unexpected errno {}", unexpected),
			},
			x if x.is_negative() => panic!("numa_num_possible_cpus returned a negative value {}", x),
			value @ _ => Some(Node::new(value))
		}
	}
}

impl Default for Cpu
{
	fn default() -> Self
	{
		Cpu(0)
	}
}

extern "C"
{
	fn numa_num_possible_cpus() -> c_int;
	fn numa_num_task_cpus() -> c_int;
	// Delegates to numa_num_task_cpus
	// fn numa_num_thread_cpus() -> c_int;
	fn numa_num_configured_cpus() -> c_int;
	fn numa_node_of_cpu(cpu: c_int) -> c_int;
}
