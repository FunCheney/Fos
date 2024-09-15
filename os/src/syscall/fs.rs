//! File and filesystem-related syscalls

use crate::{mm::translated_byte_buffer, task::current_user_token};

const FD_STDOUT: usize = 1;

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
