use core::fmt::Debug;

use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhyAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhyPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

impl Debug for VirtAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}",Self.0))
    }
}



const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;

impl From<usize> for PhyAddr {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhyPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<PhyAddr> for usize {
    fn from(value: PhyAddr) -> Self {
        value.0
    }
}

impl From<PhyPageNum> for usize {
    fn from(value: PhyPageNum) -> Self {
        value.0
    }
}

impl PhyAddr {
    pub fn page_offser(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
}

impl From<PhyAddr> for PhyPageNum {
    fn from(value: PhyAddr) -> Self {
        assert_eq!(value.page_offser(), 0);
        value.floor()
    }
}

impl From<PhyPageNum> for PhyAddr {
    fn from(value: PhyPageNum) -> Self {
        Self(value.0 << PAGE_SIZE_BITS)
    }
}

impl PhyAddr {
    pub fn floor(&self) -> PhyPageNum {
        PhyPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> PhyPageNum {
        PhyPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}
