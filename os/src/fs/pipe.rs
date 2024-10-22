/// 管道是一种进程间通信机制，由操作系统提供，并可通过直接编程或在shell程序的帮助下轻松地把不同进程（目前是父子进程之间或子子进程之间）的输入和输出对接起来。
/// 将管道看成一个有一定缓冲区大小的字节队列，它分为读和写两端，需要通过不同的文件描述符来访问。读端只能用来从管道中读取，而写端只能用来将数据写入管道。


use super::File;
use crate::mm::UserBuffer;
use crate::sync::UPSafeCell;
use alloc::sync::{Arc, Weak};
use crate::task::suspend_current_and_run_next;

/// 管道
pub struct Pipe {
    /// 是否可读
    readable: bool,
    /// 是否可写
    writeable: bool,
    /// 找到管道所在的管道自身
    buffer: Arc<Mutex<PipeRingBuffer>>,
}

const RING_BUFFER_SIZE: usize = 32;

/// 用来记录缓冲区状态
#[derive(Copy, Clone, PartialEq)]
enum RingBufferStatus {
    // 缓冲区已满，不能继续写入
    Full,
    // 缓冲区为空，不能从里面读取
    Empty,
    // 除了 FULL 和 EMPTY 之外的其他状态
    Normal,
}

/// 管道自身，带有一定大小缓冲区的字节队列
pub struct PipeRingBuffer {
    // arr，head，tail 用来维护一个循环队列
    // arr 存放数据的数组
    arr: [u8; RING_BUFFER_SIZE],
    // 队列头
    head: usize,
    // 队列尾
    tail: usize,
    // 可写状态
    status: RingBufferStatus,
    // 保存了它的写端的一个弱引用计数，这是由于在某些情况下需要确认该管道所有的写端是否都已经被关闭了，
    // 通过这个字段很容易确认这一点。
    write_end: Option<Weak<Pipe>>,
}

impl PipeRingBuffer {
    pub fn new() -> Self {
        Self {
            arr: [0; RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            status: RingBufferStatus::Empty,
            write_end: None,
        }
    }

    pub fn set_write_end(&mut self, write_end: &Arc<Pipe>) {
        self.write_end = Some(Arc::downgrade(write_end));
    }

    /// 从管道符中读取一个字节，将读端和写端的文件描述符写回到应用地址空间
    /// 仅仅通过比较队头和队尾是否相同不能确定循环队列是否为空，因为它既有可能表示队列为空，也有可能表示队列已满。
    /// 因此需要在 read_byte 的同时进行状态更新。
    pub fn read_byte(&mut self) -> u8 {
        self.status = RingBufferStatus::Normal;
        let c = self.arr[self.head];
        // 更新循环队列队头的位置，并比较队头和队尾是否相同，如果相同的话则说明管道的状态变为空 EMPTY
        self.head = (self.head + 1) % RING_BUFFER_SIZE;
        if self.head == self.tail {
            self.status = RingBufferStatus::Empty;
        }
        c
    }

    /// 计算管道中还有多少个字符可以读取
    /// 因为队头和队尾相等可能表示队列为空或队列已满
    /// 如果队列为空的话直接返回 0，否则根据队头和队尾的相对位置进行计算
    pub fn available_read(&self) -> usize {
        // 首先判断队列是否为空
        if self.status == RingBufferStatus::Empty {
            0
        } else {
            if self.tail > self.head {
                self.tail - self.head
            } else {
                self.tail + RING_BUFFER_SIZE - self.head
            }
        }
    }

    /// 判断管道的所有写端是否都被关闭了
    /// 通过尝试将管道中保存的写端的弱引用计数升级为强引用计数来实现的
    /// 如果升级失败的话，说明管道写端的强引用计数为 0 ，也就意味着管道所有写端都被关闭了，
    /// 从而管道中的数据不会再得到补充，待管道中仅剩的数据被读取完毕之后，管道就可以被销毁了。
    pub fn all_write_ends_closed(&self) -> bool {
        self.write_end.as_ref().unwrap().upgrade().is_none()
    }

    pub fn available_write(&self) -> usize {
        if self.status == RingBufferStatus::Full {
            0
        } else {
            RING_BUFFER_SIZE - self.available_read()
        }
    }
}

impl Pipe {
    pub fn read_end_with_buffer(buffer: Arc<Mutex<PipeRingBuffer>>) -> Self {
        Self {
            readable: true,
            writeable: false,
            buffer,
        }
    }

    pub fn write_end_with_buffer(buffer: Arc<Mutex<PipeRingBuffer>>) -> Self {
        Self {
            readable: false,
            writeable: true,
            buffer,
        }
    }
}

pub fn make_pipe() -> (Arc<Pipe>, Arc<Pipe>) {
    let buffer = Arc::new(Mutex::new(PipeRingBuffer::new()));
    let read_end = Arc::new(
        Pipe::read_end_with_buffer(buffer.clone())
    );

    let write_end = Arc::new(
        Pipe::write_end_with_buffer(buffer.clone())
    );

    buffer.lock().set_write_end(&write_end);
    (read_end, write_end)
}

impl File for Pipe {
    fn readable(&self) -> bool {
        self.readable
    }

    fn writable(&self) -> bool {
        self.writeable
    }

    /// 从文件中最多读取应用缓冲区大小那么多字符。这可能超出了循环队列的大小，或者由于尚未有进程从管道的写端写入足够
    /// 的字符，因此我们需要将整个读取的过程放在一个循环中，当循环队列中不存在足够字符的时候暂时进行任务切换，等待循
    /// 环队列中的字符得到补充之后再继续读取。
    fn read(&self, buf: UserBuffer) -> usize {
        assert!(self.readable);
        let want_to_read = buf.len();
        // 将传入的应用缓冲区的 buf 转化为一个能够逐字节对于缓冲区进行访问的迭代器，
        let mut buff_iter = buf.into_iter();
        // 用来维护实际有多少字节从管道读入应用的缓冲区
        let mut already_read = 0usize;
        loop {
            let mut ring_buffer = self.buffer.exclusive_access();
            let loop_read = ring_buffer.available_read();
            if loop_read == 0 {
                if ring_buffer.all_write_ends_closed() {
                    return already_read;
                }
                drop(ring_buffer);
                suspend_current_and_run_next();
                continue;
            }
            // 如果 loop_read 不为 0 ，在这一轮次中管道中就有 loop_read 个字节可以读取
            for _ in 0..loop_read {
                // 按顺序取出用于访问缓冲区中一个字节的裸指针
                if let Some(byte_ref) = buff_iter.next() {
                    unsafe {
                        // 从管道中读取
                        *byte_ref = ring_buffer.read_byte();
                    }
                    already_read += 1;
                    if already_read == want_to_read {
                        return want_to_read;
                    }
                } else {
                    return already_read;
                }
            }
        }
    }

    fn write(&self, buf: UserBuffer) -> usize {
        assert!(self.writable());
        let want_to_write = buf.len();
        let mut buf_iter = buf.into_iter();
        let mut already_write = 0usize;
        loop {
            let mut ring_buffer = self.buffer.exclusive_access();
            let loop_write = ring_buffer.available_write();
            if loop_write == 0 {
                drop(ring_buffer);
                suspend_current_and_run_next();
                continue;
            }
            // write at most loop_write bytes
            for _ in 0..loop_write {
                if let Some(byte_ref) = buf_iter.next() {
                    ring_buffer.write_byte(unsafe { *byte_ref });
                    already_write += 1;
                    if already_write == want_to_write {
                        return want_to_write;
                    }
                } else {
                    return already_write;
                }
            }
        }
    }
}
