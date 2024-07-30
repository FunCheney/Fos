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

const VA_WIDTH_SV39: usize = 39;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

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

impl From<usize> for VirtAddr{
    fn from(value: usize) -> Self {
        Self(value & ((1 << VA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << VPN_WIDTH_SV39) - 1))
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

impl From<VirtAddr> for usize {
    fn from(value: VirtAddr) -> Self {
        if value.0 >= (1 << (VA_WIDTH_SV39 - 1)) {
            value.0 | (!((1 << VA_WIDTH_SV39) - 1))
        }else {
            value.0
        }
    }
}

impl From<VirtPageNum> for usize {
    fn from(value: VirtPageNum) -> Self {
        value.0
    }
}

impl From<PhyAddr> for PhyPageNum {
    fn from(value: PhyAddr) -> Self {
        assert_eq!(value.page_offset(), 0);
        value.floor()
    }
}

impl From<PhyPageNum> for PhyAddr {
    fn from(value: PhyPageNum) -> Self {
        Self(value.0 << PAGE_SIZE_BITS)
    }
}

impl From<VirtAddr> for VirtPageNum {
   fn from(value: VirtAddr) -> Self {

       value.floor()
   } 

}

impl From<VirtPageNum> for VirtAddr {
    fn from(value: VirtPageNum) -> Self {
        Self(value.0 << PAGE_SIZE_BITS)
    }
}

impl PhyAddr {
pub fn floor(&self) -> PhyPageNum {
    PhyPageNum(self.0 / PAGE_SIZE)
}

pub fn ceil(&self) -> PhyPageNum {
    if self.0 == 0 {
        PhyPageNum(0)
    }else {
        PhyPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }
}

pub fn page_offset(&self) -> usize {
    self.0 & (PAGE_SIZE - 1)
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
impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> VirtPageNum {
        if self.0 == 0 {
            VirtPageNum(0)
        }else {
            VirtPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
        }
    }

    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
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

pub trait StepByOne {
   fn step (&mut self); 
}

impl StepByOne for VirtPageNum {
    fn step (&mut self) {
        self.0 += 1;
    }
}

impl StepByOne for PhyPageNum {
    fn step (&mut self) {
        self.0 += 1;
    }
}


#[derive(Clone, Copy)]
pub struct SimpleRange<T> 
where 
     T: StepByOne+ Copy + PartialOrd + PartialEq + Debug,
{
    l: T,
    r: T,
}

impl<T> SimpleRange<T>
where 
    T: StepByOne + Copy + PartialOrd + PartialEq + Debug
{
    pub fn new(start: T, end: T) -> Self{
        assert!(start < end, "start {:?} ? end {:?}", start, end);
        Self{
           l: start,
           r: end
        }
    }

    pub fn get_start(&self) -> T {
        self.l
    }

    pub fn get_end(&self) -> T{
        self.r
    }
}

impl<T> IntoIterator for SimpleRange<T> 
where 
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
   type Item = T;
   type IntoIter = SimpleRangeIterator<T>;
   fn into_iter(self) -> Self::IntoIter {
       SimpleRangeIterator::new(self.l, self.r)
   }
}

pub struct SimpleRangeIterator<T>
where 
     T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    current: T,
    end: T
}

impl<T> SimpleRangeIterator<T>
where 
     T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(l: T, r: T) -> Self {

        Self{
            current: l,
            end: r
        }
    }
}

impl<T> Iterator for SimpleRangeIterator<T>  
where 
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        }else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
    
}
pub type VPNRange = SimpleRange<VirtPageNum>;
