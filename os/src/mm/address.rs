use core::fmt::Debug;

use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};

use super::page_table::PageTableEntry;

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
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}

impl Debug for VirtPageNum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("VPN:{:#x}", self.0))
    }
}

impl Debug for PhyAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl Debug for PhyPageNum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("PPN:{:#x}", self.0))
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

    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe { (self.0 as *mut T).as_mut().unwrap() }
    }
}

impl PhyPageNum {
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhyAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512) }
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhyAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096) }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhyAddr = (*self).into();
        pa.get_mut()
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}
