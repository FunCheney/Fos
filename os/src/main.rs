#![no_main]
#![no_std]
mod lang_items;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

