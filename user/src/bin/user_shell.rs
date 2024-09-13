#![no_std]
#![no_main]

use core::char;

use alloc::string::String;
use user_lib::{console::getchar, exec, fork, wait_pid};

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;


#[no_mangle]
pub fn main()-> i32 {
    print!("RUST user shell");
    let mut line: String = String::new();
    print!(">>");
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!(" ");
                if !line.is_empty() {
                    line.push('\0');
                    let pid = fork();
                    if pid == 0 {
                        // child process
                        if exec(line.as_str()) == -1 {
                            println!("error when exec");
                            return -4;
                        }
                        unreachable!();
                    } else {
                        let mut exit_code: i32 = 0;
                        let exit_pid = wait_pid(pid as usize, &mut  exit_code);
                        assert_eq!(pid, exit_pid);
                        println!("Shell: Process {} exited with code {}", pid, exit_code);
                    }

                    line.clear();
                }

                print!(">>>>");
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            
            _=> {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}


