#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exec, fork, wait, yield_};

#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        // fork 返回 0 表示子进程，通过 exec 执行shell 程序 user_shell
        exec("user_shell\0");
    } else {
        // 返回不是 0,表示调用 fork 的用户初始程序 initproc自身
        loop {
            let mut exit_code: i32 = 0;
            // 不断的调研 wait 来等待那些被移交到它下面的子进程
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_();
                continue;
            }
            println!(
                "[initproc] Released a zombie process, pid={}, exit_code={}",
                pid, exit_code,
            );
        }
    }
    0
}
