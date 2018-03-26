// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.



extern crate libc;


use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::fmt::Result;
use ::std::mem::zeroed;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_long;
use ::libc::c_longlong;
use ::libc::c_uint;
use ::libc::c_ulong;
use ::libc::c_void;

use ::libc::pid_t;

#[link(name = "numa", kind = "static-nobundle")]
extern "C"
{
}

include!("constants.rs");
include!("functions.rs");
include!("statics.rs");
include!("structs.rs");
