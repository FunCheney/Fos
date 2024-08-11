#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, task_info, yield_};

#[no_mangle]
fn main() -> i32 {
    let currnet_time = get_time();
    let wait_for = currnet_time + 3000;
    while get_time() < wait_for {
        task_info();
        yield_();
    }

    println!("[user] Test sleep OK!");
    0
}
