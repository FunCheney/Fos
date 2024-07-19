use bitflags::*;

use super::address::PhyPageNum;

bitflags! {
    pub struct PTEFlags: u8 {
    /// SV39 分页模式下的页表项中，[53:10] 这 44 位是物理页号，
    /// 最低位的 8 位为标志位，具体含义如下
    /// V 仅当位 V 为 1 时，表示页表项合法
    const V = 1 << 0;
    /// 控制索引到该页表项对应的虚拟页是否可读
    const R = 1 << 1;
    /// 是否可写
    const W = 1 << 2;
    /// 是否可执行
    const X = 1 << 3;
    /// 控制索引到这个页表项对应的虚拟页面是否在 CPU 处于 U 特权级下允许访问
    const U = 1 << 4;
    const G = 1 << 5;
    /// 处理器记录，自从页表项的这一位被清零之后，页表项对应的虚拟页是否被访问过
    const A = 1 << 6;
    /// 处理器记录，自从页表项的这一位被清零之后，页表项对应的虚拟也是否被修改过
    const D = 1 << 7;
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhyPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }

    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }

    pub fn ppn(&self) -> PhyPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
}
