// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "core_text"]
#![crate_type = "rlib"]

#![allow(non_snake_case_functions)]

extern crate libc;

extern crate core_foundation;
extern crate core_graphics;

#[cfg(target_os="macos")]
pub mod font;
#[cfg(target_os="macos")]
pub mod font_collection;
#[cfg(target_os="macos")]
pub mod font_descriptor;
#[cfg(target_os="macos")]
pub mod font_manager;
