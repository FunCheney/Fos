#![no_std]
#![feature(linkage)]  // 启用弱引用链接特性
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

#[no_mangle]
///使用 rust 的宏，将_start 这段代码编译后的汇编代码放在一个名为
/// .text.entry 的代码段中。方便我们在后续链接的时候调整它的位置使的它能作为用户库的入口。
#[link_section = ".text.entry"]
/// 定义了用户库的入口点 _start
pub extern "C" fn _start() -> ! {
    // 手动清空需要零初始化的 .bss 段
    clear_bss();
    // 然后调用 一个 main 函数的到一个类型值为 i32 的返回值，
    // 最后调用用户库提供的 exit 接口退出应用程序
    exit(main());
    panic!("sys exit");
}

/// 使用 rust 宏，将函数符号 main 标识为弱链接，这样在最后链接的时候，虽然在 lib.rs 和
/// bin 目录下的某个应用程序都有 main 符号，但是由于 lib.rs 中的main 是弱链接，链接器会使用
/// bin 目录下的应用主逻辑作为 main。
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("not found main");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }

    unsafe {
        core::slice::from_raw_parts_mut(
            start_bss as usize as *mut u8,
            end_bss as usize - start_bss as usize,
        ).fill(0);
    }
}


use syscall::*; 

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}


