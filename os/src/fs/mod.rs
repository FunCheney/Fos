//！ File system
mod inode;
mod pipe;
mod stdio;

use crate::mm::UserBuffer;
/// File trait
/// 操作系统内核就可把能读写并持久存储的数据按文件来进行管理，
/// 并把文件分配给进程.这个接口在内存和存储设备之间建立了数据交换的通道
/// 将 UserBuffer 看成一个 &[u8] 切片，它是一个同时给出了缓冲区起始地址和长度的胖指针。
pub trait File: Send + Sync {
    /// If readable
    fn readable(&self) -> bool;
    /// If writable
    fn writable(&self) -> bool;
    /// Read file to `UserBuffer`
    /// read 指的是从文件中读取数据放到缓冲区中，最多将缓冲区填满（即读取缓冲区的长度那么多字节），
    /// 并返回实际读取的字节数
    fn read(&self, buf: UserBuffer) -> usize;
    /// Write `UserBuffer` to file
    /// write 指的是将缓冲区中的数据写入文件，最多将缓冲区中的数据全部写入，
    /// 并返回直接写入的字节数
    fn write(&self, buf: UserBuffer) -> usize;
}

pub use inode::{list_apps, open_file, OSInode, OpenFlags};
pub use pipe::make_pipe;
pub use stdio::{Stdin, Stdout};
