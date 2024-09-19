#![no_std]
#![feature(linkage)]  // 启用弱引用链接特性
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;


use buddy_system_allocator::LockedHeap;
use syscall::*;

const USER_HEAP_SIZE: usize = 16384;
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) ->! {
    panic!("Heap alloc error, layout =  {:?}", layout);
}



#[no_mangle]
///使用 rust 的宏，将_start 这段代码编译后的汇编代码放在一个名为
/// .text.entry 的代码段中。方便我们在后续链接的时候调整它的位置使的它能作为用户库的入口。
#[link_section = ".text.entry"]
/// 定义了用户库的入口点 _start
pub extern "C" fn _start() -> ! {
    // 手动清空需要零初始化的 .bss 段
    //clear_bss();
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    // 然后调用 一个 main 函数的到一个类型值为 i32 的返回值，
    // 最后调用用户库提供的 exit 接口退出应用程序
    exit(main());
    panic!("sys exit");
}

/// 使用 rust 宏，将函数符号 main 标识为弱链接，这样在最后链接的时候，虽然在 lib.rs 和
/// bin 目录下的某个应用程序都有 main 符号，但是由于 lib.rs 中的main 是弱链接，链接器会使用
/// bin 目录下的应用主逻辑作为 main。
/// 如果 bin 目录下找不到 任何 main, 那么编译也能通过，但是在运行时会报错
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("not found main");
}


#[allow(unused)]
fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }

    //unsafe {
    //    core::slice::from_raw_parts_mut(
    //        start_bss as usize as *mut u8,
    //        end_bss as usize - start_bss as usize,
    //    ).fill(0);
    //}
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}


/// 对 syscall 模块中的 sys_exit, sys_write 进一步封装
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn yield_() -> isize {
    sys_yield()
}


pub fn get_time() -> isize {
    sys_get_time()
}

/// 等待任意一个子进程结束
pub fn wait(exit_code: &mut i32)-> isize {
    loop {
        // 传入的参数是 -1
        match sys_waitpid(-1, exit_code as *mut _) {
            // 等待的进程存在，但是尚未结束返回 -2
            -2 => { yield_(); }
            // -1 or real pid
            exit_pid => return exit_pid,
        }
    }
}

/// 等待一个进程标识符为 pid 的进程结束
pub fn waitpid(pid: usize, exit_code: &mut i32)->isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            // 要等待的子进程存在但是尚未退出
            -2 => { yield_(); } // 调用 yield_ 主动让出 cpu
            // -1 or real pid
            exit_pid => return exit_pid,
        }
    }
}

pub fn getpid() ->isize {
    sys_getpid()
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn exec(path: &str) -> isize{
    sys_exec(path)
}

pub fn read(fd: usize, buf: &mut [u8])->isize {
    sys_read(fd, buf)
}
