use core::panicking::panic;
use std::{process::exit, rc::Weak};

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
    panic!("unreachable after sys_exit!");
}


#[linkage = "Weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("not found main");
}

