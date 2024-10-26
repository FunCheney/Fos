#![no_std]
#![feature(linkage)] // 启用弱引用链接特性
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

#[macro_use]
extern crate bitflags;
extern crate alloc;

use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
use syscall::*;

const USER_HEAP_SIZE: usize = 16384;
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap alloc error, layout =  {:?}", layout);
}

#[no_mangle]
///使用 rust 的宏，将_start 这段代码编译后的汇编代码放在一个名为
/// .text.entry 的代码段中。方便我们在后续链接的时候调整它的位置使的它能作为用户库的入口。
#[link_section = ".text.entry"]
/// 定义了用户库的入口点 _start
/// 在入口 _start 中我们就接收到了命令行参数个数 argc 和字符串数组的起始地址 argv
pub extern "C" fn _start(argc: usize, argv: usize) -> ! {
    // 手动清空需要零初始化的 .bss 段
    //clear_bss();
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let mut v: Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start =
        unsafe {((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile()};

        let len = (0usize..)
            .find(|i| unsafe {((str_start + *i) as *const u8).read_volatile() == 0 })
            .unwrap();

        v.push(
            core::str::from_utf8(
                unsafe {
                    core::slice::from_raw_parts(str_start as *const u8, len)
                }
            ).unwrap()
        );
    }

    // 然后调用 一个 main 函数的到一个类型值为 i32 的返回值，
    // 最后调用用户库提供的 exit 接口退出应用程序
    exit(main(argc, v.as_slice()));
    panic!("sys exit");
}

/// 使用 rust 宏，将函数符号 main 标识为弱链接，这样在最后链接的时候，虽然在 lib.rs 和
/// bin 目录下的某个应用程序都有 main 符号，但是由于 lib.rs 中的main 是弱链接，链接器会使用
/// bin 目录下的应用主逻辑作为 main。
/// 如果 bin 目录下找不到 任何 main, 那么编译也能通过，但是在运行时会报错
#[linkage = "weak"]
#[no_mangle]
fn main(_argc: usize, _argv: &[&str]) -> i32 {
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

pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
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
pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        // 传入的参数是 -1
        match sys_waitpid(-1, exit_code as *mut _) {
            // 等待的进程存在，但是尚未结束返回 -2
            -2 => {
                yield_();
            }
            // -1 or real pid
            exit_pid => return exit_pid,
        }
    }
}

/// 等待一个进程标识符为 pid 的进程结束
pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            // 要等待的子进程存在但是尚未退出
            -2 => {
                yield_();
            } // 调用 yield_ 主动让出 cpu
            // -1 or real pid
            exit_pid => return exit_pid,
        }
    }
}

pub fn getpid() -> isize {
    sys_getpid()
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn exec(path: &str, args: &[*const u8]) -> isize {
    sys_exec(path, args)
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

bitflags! {
    pub struct OpenFlags: u32 {
        // 第 0 位，只读模式打开
        const READONLY = 0;
        // 0x001，只写模式打开
        const WRONLY = 1 << 0;
        // 第 1 位被设置，可读可写模式
        const RDWR = 1 << 1;
        // 第 9 位被设置，表示允许创建文件 CREATE，
        // 找不到对应该文件的时候应该创建文件，如果该文件存在，
        // 则将对应的文件大小归 0.
        const CREATE = 1 << 9;
        // 第 10 位被设置为 0，则在打开时候清空文件内容，并将文件大小归为0
        const TRUNC = 1 << 10;
    }
}

pub fn open(path: &str, flags: OpenFlags) -> isize {
    sys_open(path, flags.bits)
}

pub fn close(fd: usize) -> isize {
    sys_close(fd)
}

pub fn sleep(period_ms: usize) {
    let start = sys_get_time();
    while sys_get_time() < start + period_ms as isize {
        sys_yield();
    }
}

pub fn pipe(pipe_fd: &mut [usize]) -> isize {
    sys_pipe(pipe_fd)
}

/// Action for a signal
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct SignalAction {
    pub handler: usize,
    pub mask: SignalFlags,
}

impl Default for SignalAction {
    fn default() -> Self {
        Self {
            handler: 0,
            mask: SignalFlags::empty(),
        }
    }
}

pub const SIGDEF: i32 = 0; // Default signal handling
pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
pub const SIGTRAP: i32 = 5;
pub const SIGABRT: i32 = 6;
pub const SIGBUS: i32 = 7;
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGUSR1: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGUSR2: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
pub const SIGSTKFLT: i32 = 16;
pub const SIGCHLD: i32 = 17;
pub const SIGCONT: i32 = 18;
pub const SIGSTOP: i32 = 19;
pub const SIGTSTP: i32 = 20;
pub const SIGTTIN: i32 = 21;
pub const SIGTTOU: i32 = 22;
pub const SIGURG: i32 = 23;
pub const SIGXCPU: i32 = 24;
pub const SIGXFSZ: i32 = 25;
pub const SIGVTALRM: i32 = 26;
pub const SIGPROF: i32 = 27;
pub const SIGWINCH: i32 = 28;
pub const SIGIO: i32 = 29;
pub const SIGPWR: i32 = 30;
pub const SIGSYS: i32 = 31;

bitflags! {
    pub struct SignalFlags: i32 {
        const SIGDEF = 1; // Default signal handling
        const SIGHUP = 1 << 1;
        const SIGINT = 1 << 2;
        const SIGQUIT = 1 << 3;
        const SIGILL = 1 << 4;
        const SIGTRAP = 1 << 5;
        const SIGABRT = 1 << 6;
        const SIGBUS = 1 << 7;
        const SIGFPE = 1 << 8;
        const SIGKILL = 1 << 9;
        const SIGUSR1 = 1 << 10;
        const SIGSEGV = 1 << 11;
        const SIGUSR2 = 1 << 12;
        const SIGPIPE = 1 << 13;
        const SIGALRM = 1 << 14;
        const SIGTERM = 1 << 15;
        const SIGSTKFLT = 1 << 16;
        const SIGCHLD = 1 << 17;
        const SIGCONT = 1 << 18;
        const SIGSTOP = 1 << 19;
        const SIGTSTP = 1 << 20;
        const SIGTTIN = 1 << 21;
        const SIGTTOU = 1 << 22;
        const SIGURG = 1 << 23;
        const SIGXCPU = 1 << 24;
        const SIGXFSZ = 1 << 25;
        const SIGVTALRM = 1 << 26;
        const SIGPROF = 1 << 27;
        const SIGWINCH = 1 << 28;
        const SIGIO = 1 << 29;
        const SIGPWR = 1 << 30;
        const SIGSYS = 1 << 31;
    }
}

pub fn kill(pid: usize, signum: i32) -> isize {
    sys_kill(pid, signum)
}

pub fn sigaction(
    signum: i32,
    action: Option<&SignalAction>,
    old_action: Option<&mut SignalAction>,
) -> isize {
    sys_sigaction(
        signum,
        action.map_or(core::ptr::null(), |a| a),
        old_action.map_or(core::ptr::null_mut(), |a| a),
    )
}

pub fn sigprocmask(mask: u32) -> isize {
    sys_sigprocmask(mask)
}

pub fn sigreturn() -> isize {
    sys_sigreturn()
}
