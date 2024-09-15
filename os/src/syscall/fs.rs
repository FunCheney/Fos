//! File and filesystem-related syscalls

use crate::task::suspend_current_and_run_next;
use crate::{mm::translated_byte_buffer, task::current_user_token};

use crate::sbi::{console_getchar};

const FD_STDOUT: usize = 1;
const FD_IN: usize = 0;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            // 按应用的虚地址指向的缓冲区转换为一组按内核虚地址指向的字节数组切片构成的向量，然后把
            // 每个字节数组切片转化为字符串 &str 然后输出即可。
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                println!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}


pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_IN => {
            assert_eq!(len, 1, "Only support len = 1 in sys_read");
            let mut c: usize;
            loop {
                c = console_getchar();
                if c==0 {
                    suspend_current_and_run_next();
                    continue;
                }else {
                    break;
                }
            }

            let ch = c as u8;
            let mut buffers = translated_byte_buffer(current_user_token(), buf, len);
            unsafe {
                buffers[0].as_mut_ptr().write_volatile(ch);
            }
            1
        }

        _ => {
            panic!("Unsupported fd in sys_read!");
        }
    }
}
