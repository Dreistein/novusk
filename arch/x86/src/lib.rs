#![no_std]
#![feature(global_asm, llvm_asm)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
#[macro_use] extern crate uefi;
#[macro_use] extern crate uefi_macros;
#[macro_use] extern crate uefi_services;

pub extern crate ps2_keyboard;

// Include
extern crate ctypes;
pub extern crate novusk;

// Kernel
#[macro_use] pub extern crate kerror;
#[macro_use] pub extern crate kinfo;
#[macro_use] pub extern crate printk;

extern crate userspace;

#[cfg(target_arch = "x86")]
extern crate i686;

use ctypes::c_char;

pub(crate) mod boot;
pub mod drivers;
pub mod include;
pub mod kernel;
