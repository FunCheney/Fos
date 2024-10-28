//! Implementation of [`FrameAllocator`] which
//! controls all the frames in the operating system.
//! 操作系统内核能够以物理页帧为单位分配和回收内存

use super::{PhysAddr, PhysPageNum};
use crate::config::MEMORY_END;
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};
use lazy_static::*;

/// manage a frame which has the same lifecycle as the tracker
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    // 通过物理页帧的物理页号来创建 FrameTracker
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        // 这个物理页帧可能之前被分配过，并用作其他用途
        // 将物理页帧上的所有字节清零
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

/// 实现 Drop Trait 当一个 FrameTracker 被回收时，
/// drop 方法会自动被调用
impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

/// 来描述一个物理页帧管理器，需要提供那些功能
trait FrameAllocator {
    // 创建一个物理页帧
    fn new() -> Self;
    // 以物理页号为单位进行物理页帧的分配
    fn alloc(&mut self) -> Option<PhysPageNum>;
    // 以物理页号为单位进行物理页帧的回收
    fn dealloc(&mut self, ppn: PhysPageNum);
}

/// an implementation for frame allocator
/// 栈式物理页帧管理策略
/// 物理页号 [current, end) 此前均为被分出去过
pub struct StackFrameAllocator {
    // 空闲内存的起始物理页号
    current: usize,
    // 空闲内存的结束物理页号
    end: usize,
    // recycled 已后入先出的方式保存了被回收的物理页号
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.current = l.0;
        self.end = r.0;
    }
}
impl FrameAllocator for StackFrameAllocator {
    // 实现 new 方法，初始化时将区间两端设置为 0
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            // 创建一个新的向量
            recycled: Vec::new(),
        }
    }
    fn alloc(&mut self) -> Option<PhysPageNum> {
        // 首先检查栈 recycled 内有没有之前回收的物理页号
        // 如果有，直弹出并返回
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else if self.current == self.end {
            // 如果 current 与 end 相等返回 None
            None
        } else {
            // 从之前未分配的 [current, end) 上进行分配
            // 分配左端点，并将 current + 1，代表 current 已经被分配
            self.current += 1;
            // 使用 into  将 uszie 转换为物理页号 PhysPageNum
            Some((self.current - 1).into())
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        // validity check
        // 该页面之前一定分出去过，物理页号一定小于 current
        // 该页面正在回收状态，物理也号不能在 recycled 中找到
        if ppn >= self.current || self.recycled.iter().any(|&v| v == ppn) {
            // 如果找到了就会是一个 Option::Some, 说明内核的其他地方实现有误
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        // recycle
        self.recycled.push(ppn);
    }
}

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    /// frame allocator instance through lazy_static!
    /// 用 UPSafeCell 来包裹栈式物理页帧分配器，每次对该分配器进行操作之前
    /// 都需要通过 FRAME_ALLOCATOR.exclusive_access 拿到分配器的可变借用
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> =
        unsafe { UPSafeCell::new(FrameAllocatorImpl::new()) };
}

/// initiate the frame allocator using `ekernel` and `MEMORY_END`
pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR.exclusive_access().init(
        // 上取整获取可用的物理页号
        PhysAddr::from(ekernel as usize).ceil(),
        // 下取整获取可用的物理页号
        PhysAddr::from(MEMORY_END).floor(),
    );
}

/// allocate a frame
/// 返回的值类型不是 FrameAllocator 要求的物理页号 PhysPageNum
/// 而是将其进一步包装的 FrameTracker
pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(FrameTracker::new)
}

/// deallocate a frame
pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
}

#[allow(unused)]
/// a simple test for frame allocator
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}
