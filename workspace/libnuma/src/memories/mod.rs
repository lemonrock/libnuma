// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


use super::masks::indices::NodeIndex;
use super::masks::NodeMask;
use ::errno::errno;
use ::libc::c_int;
use ::libc::c_ulong;
use ::libc::c_void;
use ::libc::size_t;
use ::libnuma_sys::*;
use ::std::io::Error;
use ::std::io::ErrorKind;
use ::std::ptr::null_mut;


include!("AllocatableMemory.rs");
include!("Memory.rs");
include!("MemoryPolicy.rs");
include!("MemoryPolicyFlags.rs");
include!("MovePagesFlags.rs");
include!("NumaMemory.rs");
include!("ReAllocatableMemory.rs");
