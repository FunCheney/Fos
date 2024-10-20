#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{close, fork, pipe, read, wait, write};

static STR: &str = "Hello,world";

#[no_mangle]
pub fn main() -> i32{
    let mut pipe_fd = [0usize; 2];
    pipe(&mut pipe_fd);
    assert_eq!(pipe_fd[0], 3);
    assert_eq!(pipe_fd[1], 4);
    if fork() == 0 {
        close(pipe_fd[1]);
        let mut buffer = [0u8; 32];
        let len_read = read(pipe_fd[0], &mut buffer) as usize;
        close(pipe_fd[0]);
        assert_eq!(core::str::from_utf8(&buffer[0..len_read]).unwrap(), STR);
        println!("Read OK. child process exited");
        0
    } else {
        close(pipe_fd[0]);
        assert_eq!(write(pipe_fd[1], STR.as_bytes()), STR.len() as isize);
        close(pipe_fd[1]);
        let mut child_ext_code: i32 = 0;
        wait(&mut  child_ext_code);
        assert_eq!(child_ext_code, 0);
        println!("pipe test passed!");
        0
    }
}