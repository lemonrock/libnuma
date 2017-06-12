// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.


use super::*;
use ::errno::errno;
use ::libc::c_int;
use ::libc::c_longlong;
use ::libc::c_uint;
use ::libc::size_t;
use ::std::mem::uninitialized;


include!("CpuIndex.rs");
include!("Index.rs");
include!("NodeIndex.rs");
