use alloc::vec::Vec;

use super::address::PhyPageNum;



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

impl FrameAllocator for StackFrameAllocator{
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhyPageNum> {
        if let Some(ppn) = self.recycled.pop(){
            Some(ppn.into())
        }else {
            if self.current == self.end {
                None
            }else {
                self.current += 1;
                Some((self.current - 1).into())
            }
        }

    }

    fn dealloc(&mut self, ppn: PhyPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter()
        .find(|&v| {*v == ppn})
        .is_some(){
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
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> = unsafe {
        UPSafeCell::new(FrameAllocatorImpl::new())
    };
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }

    FRAME_ALLOCATOR.exclusive_access()
        .init(PhyAddr::from(ekernel as usize).ceil(), PhyAddr::from(MEMORY_END).floor());
}

