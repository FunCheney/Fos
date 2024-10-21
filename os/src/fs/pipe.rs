/// 管道是一种进程间通信机制，由操作系统提供，并可通过直接编程或在shell程序的帮助下轻松地把不同进程（目前是父子进程之间或子子进程之间）的输入和输出对接起来。
/// 将管道看成一个有一定缓冲区大小的字节队列，它分为读和写两端，需要通过不同的文件描述符来访问。读端只能用来从管道中读取，而写端只能用来将数据写入管道。


use alloc::rc::Weak;
use alloc::sync::Arc;
use spin::Mutex;

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
    FULL,
    // 缓冲区为空，不能从里面读取
    EMPTY,
    // 除了 FULL 和 EMPTY 之外的其他状态
    NORMAL,
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
            status: RingBufferStatus::EMPTY,
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
        self.status = RingBufferStatus::NORMAL;
        let c = self.arr[self.head];
        // 更新循环队列队头的位置，并比较队头和队尾是否相同，如果相同的话则说明管道的状态变为空 EMPTY
        self.head = (self.head + 1) % RING_BUFFER_SIZE;
        if self.head == self.tail {
            self.status = RingBufferStatus::EMPTY;
        }
        c
    }

    /// 计算管道中还有多少个字符可以读取
    /// 因为队头和队尾相等可能表示队列为空或队列已满
    /// 如果队列为空的话直接返回 0，否则根据队头和队尾的相对位置进行计算
    pub fn available_read(&self) -> usize {
        // 首先判断队列是否为空
        if self.status == RingBufferStatus::EMPTY {
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
