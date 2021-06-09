#![no_std]

#![feature(asm, global_asm)]

#[macro_use] pub extern crate kerror;
#[macro_use] pub extern crate kinfo;
#[macro_use] pub extern crate printk;
extern crate userspace;

#[macro_use] extern crate lazy_static;

pub extern crate novusk;
extern crate arm_lib;

pub mod boot;
pub mod drivers;
pub mod kernel;

