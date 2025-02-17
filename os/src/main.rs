//! call [println!] display Hello
#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use core::arch::global_asm;

use log::info;
#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod config;
// pub mod batch;
mod drivers;
pub mod fs;
mod lang_items;
mod loader;
mod logging;
mod mm;
mod sbi;
mod sync;
pub mod syscall;
mod task;
mod timer;
pub mod trap;

extern crate alloc;

#[macro_use]
extern crate bitflags;

global_asm!(include_str!("entry.asm"));

// 引入汇编代码 link_app.S 一开始并不存在，而是在构建操作系统时自动生成
// 执行 cargo build 时，由脚本 os/build.rs 控制生成
// global_asm!(include_str!("link_app.S"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    logging::init();
    info!("[kernel] hello, gjh os!");
    // 初始内存管理模块
    mm::init();
    info!("[kernel] back to os");
    mm::remap_test();
    info!("[kernel] init process...");
    trap::init();
    info!("[kernel] trap init...");
    // 设置了 sie.stie 使得 S 特权级时钟不会被屏蔽
    trap::enable_timer_interrupt();
    // 设置了 10 ms 的计时器
    timer::set_next_trigger();
    info!("[kernel| get user app...");
    // 系统中的 app 列表
    // loader::list_apps();
    fs::list_apps();
    // 调用 run-tasks
    info!("[kernel] run task...");
    task::add_initproc();
    task::run_tasks();
    panic!("unreachable in rust main");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
