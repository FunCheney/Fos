#![no_std]
#![no_main]

use core::arch::asm;
// 外部库引用
// 这个外部库引用就是 user 目录下的lib.rs 以及它引用的若干子模块
// 在 user/Cargo.toml 中指定了库的名字
#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("hello, world");
    unsafe {
        asm!("sret");
    }
    0
}
