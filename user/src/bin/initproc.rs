#![no_std]
#![no_main]


#[macro_use]
extern crate user_lib;

use user_lib::{
    fork,
    wait,
    exec,
    yield_
};


#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        // fork 返回 0 表示子进程，通过 exec 直接执行shell 程序
        // 执行程序 user_shell， 字符串末尾手动加入 \0, 因为 rust
        // 将这些字符串连接到只读数据段的时候不会加入 \0
        exec("user_shell\0");
    }else {
        // 返回值不为 0 表示调用 fork 的用户初始程序 initproc 自身。
        // 在不断的循环调用 wait 来等待那些被移交到他下面的子进程，并回收其资源
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_();
                continue;
            }

            println!(
                "[initproc] Release a zombie process, pid={}, exit_code={}",
                pid,
                exit_code,
            );
        }
    }

    0
}
