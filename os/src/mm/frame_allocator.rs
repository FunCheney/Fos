use core::fmt::Debug;

use alloc::vec::Vec;

use super::address::PhyPageNum;

use lazy_static::*;

/// 描述页帧管理器需要那些功能
trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhyPageNum>;
    fn dealloc(&mut self, ppn: PhyPageNum);
}

/// 实现栈式页帧管理策略
pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhyPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else {
            if self.current == self.end {
                None
            } else {
                self.current += 1;
                Some((self.current - 1).into())
            }
        }
    }

    fn dealloc(&mut self, ppn: PhyPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().find(|&v| *v == ppn).is_some() {
            panic!("Frame ppn = {:#x} has not bean allocated", ppn)
        }

        self.recycled.push(ppn);
    }
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhyPageNum, r: PhyPageNum) {
        self.current = l.0;
        self.end = r.0;
    }
}

use crate::{config::MEMORY_END, mm::address::PhyAddr, sync::UPSafeCell};

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> =
        unsafe { UPSafeCell::new(FrameAllocatorImpl::new()) };
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }

    FRAME_ALLOCATOR.exclusive_access().init(
        PhyAddr::from(ekernel as usize).ceil(),
        PhyAddr::from(MEMORY_END).floor(),
    );
}

pub struct FrameTracker {
    pub ppn: PhyPageNum,
}

impl FrameTracker {
    pub fn new(ppn: PhyPageNum) -> Self {
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }

        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(|ppn| FrameTracker::new(ppn))
}

pub fn frame_dealloc(ppn: PhyPageNum) {
    FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
}

#[allow(unused)]
pub fn frame_alloc_test() {
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
    println!("frame_alloc_test passred");
}
