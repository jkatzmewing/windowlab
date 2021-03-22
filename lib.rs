#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![register_tool(c2rust)]

extern crate libc;

pub mod src {
pub mod client;
pub mod events;
pub mod manage;
pub mod menufile;
pub mod misc;
pub mod new;
pub mod taskbar;
} // mod src

