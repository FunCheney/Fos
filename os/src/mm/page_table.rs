//! Implementation of [`PageTableEntry`] and [`PageTable`].

use super::{frame_alloc, FrameTracker, PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use bitflags::*;
/// bitflags 是一个 Rust 中常用来比特标志位的 crate
/// 将一个 u8 封装成一个标志位的集合类型，支持一些常见的集合运算
bitflags! {
    /// page table entry flags
    pub struct PTEFlags: u8 {
        // 仅当位 V 为 1 时，页表项才是合法的
        const V = 1 << 0;
        // R 位表示 页表项对应的虚拟页面允许读
        const R = 1 << 1;
        // W 位表示 页表项对应的虚拟页面允许写
        const W = 1 << 2;
        // X 位表示 页表项对应的虚拟页面是否允许可执行
        const X = 1 << 3;
        // 控制索引到这个页表项的对应虚拟页面是否在 CPU 处于 U 特权级的情况下是否被允许访问
        const U = 1 << 4;
        const G = 1 << 5;
        // 处理器记录自从页表项上的这一位被清零之后，页表项的对应虚拟页面是否被访问过
        const A = 1 << 6;
        // 处理器记录自从页表项上的这一位被清零之后，页表项的对应虚拟页面是否被修改过
        const D = 1 << 7;
    }
}
/// 让编译器自动实现 Copy/Clone Trait
/// 让这个类型以值语义赋值/传参时不会发生所有权转转移，而是拷贝一份新的副本。
/// PageTable 是 usize 的一层简单的封装
#[derive(Copy, Clone)]
#[repr(C)]
/// page table entry structure
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    /// 通过一个物理页号 和 一个页表项标志位 PTEFlags 来生成一个页表项实例
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }

    /// 生成一个全零的页表项，注意这隐含着该页表项的 V 标志位为 0 ，因此它是不合法的。
    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }

    /// 通过页表项取出 物理页号 PhysPageNum
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }

    /// 通过页表项取出 页表项标志为 PTEFlags
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }

    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }

    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }

    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}

/// page table structure
/// 每个应用的地址空间都对应一个不同的多级页表，不同页表的起始地址不一样
/// 负责建立好虚拟页号和物理页号的对应关系
/// 以页为单位维护
/// 对计算机系统的整个虚拟/物理 内存空间并没有一个全局的映射和掌控
pub struct PageTable {
    // 根节点的物理页号，做为页表唯一的区分标志
    root_ppn: PhysPageNum,
    // 以向量 FrameTracker 的形式保存了页表所有节点的物理页帧
    // 将这些 FrameTracker 的生命周期绑定到 PageTable 下面
    // 当 PageTable 的生命周期结束，向量 frames 的生命周期结束
    // 意味着存放多级页表节点的那些物理页帧也被回收
    frames: Vec<FrameTracker>,
}

/// Assume that it won't oom when creating/mapping.
impl PageTable {
    // 通过 new 方法创建 PageTable
    pub fn new() -> Self {
        // 创建一个根节点
        let frame = frame_alloc().unwrap();
        PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }
    /// Temporarily used to get arguments from user space.
    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
        }
    }

    //在多级页表中找到一个虚拟页号对应的页表项的可变引用
    //如果在遍历的过程中发现有节点尚未创建，则会新建一个节点
    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        // 当前的物理页号
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, idx) in idxs.iter().enumerate() {
            // 取出当前节点的页表项数组
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
                // 如果是叶子节点，直接返回该页表项的可变引用
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                // 新建一个节点，更新做为下级节点指针的页表项
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                // 将新分配的页帧移动到 frames ，方便后续自动回收
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }

    // 在多级页表中找到一个虚拟页号对应的页表项的可变引用
    // 如果遍历过程中找不到合法的叶子节点直接返回 None 不会创建新节点
    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        result
    }
    #[allow(unused)]
    /// 通过 map 方法在多级页表中插入一个键值对
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }
    #[allow(unused)]
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }

    /// Translate `VirtPageNum` to `PageTableEntry`
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }

    /// Translate `VirtAddr` to `PhysAddr`
    pub fn translate_va(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find_pte(va.clone().floor()).map(|pte| {
            let aligned_pa: PhysAddr = pte.ppn().into();
            let offset = va.page_offset();
            let aligned_pa_usize: usize = aligned_pa.into();
            (aligned_pa_usize + offset).into()
        })
    }

    /// PageTable::token 会按照 satp CSR 格式要求 构造一个无符号 64 位无符号整数，使得其分页模式为 SV39 ，
    /// 且将当前多级页表的根节点所在的物理页号填充进去。在 MemorySet 的 activate 中，我们将这个值写入当前 CPU 的 satp CSR ，
    /// 从这一刻开始 SV39 分页模式就被启用了，而且 MMU 会使用内核地址空间的多级页表进行地址转换。
    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }
}

/// translate a pointer to a mutable u8 Vec through page table
/// 提供了将应用地址空间中一个缓冲区转化为在内核空间中能够直接访问的形式
/// token: 某个应用地址空间的 token
/// ptr 和 len 则分别表示该地址空间中的一段缓冲区的起始地址和长度
/// 以向量的形式返回一组可以在内核空间中直接访问的字节数组切片
pub fn translated_byte_buffer(token: usize, ptr: *const u8, len: usize) -> Vec<&'static mut [u8]> {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut v = Vec::new();
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppn = page_table.translate(vpn).unwrap().ppn();
        vpn.step();
        let mut end_va: VirtAddr = vpn.into();
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0 {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..]);
        } else {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
        }
        start = end_va.into();
    }
    v
}

pub fn translated_str(token: usize, ptr: *const u8) -> String {
    let page_table = PageTable::from_token(token);
    let mut string = String::new();
    let mut va = ptr as usize;
    loop {
        let ch: u8 = *(page_table
            .translate_va(VirtAddr::from(va))
            .unwrap()
            .get_mut());
        if ch == 0 {
            break;
        }
        string.push(ch as char);
        va += 1;
    }
    string
}

#[allow(unused)]
///Translate a generic through page table and return a reference
pub fn translated_ref<T>(token: usize, ptr: *const T) -> &'static T {
    let page_table = PageTable::from_token(token);
    page_table
        .translate_va(VirtAddr::from(ptr as usize))
        .unwrap()
        .get_ref()
}

pub fn translated_refmut<T>(token: usize, ptr: *mut T) -> &'static mut T {
    let page_table = PageTable::from_token(token);
    let va = ptr as usize;
    page_table
        .translate_va(VirtAddr::from(va))
        .unwrap()
        .get_mut()
}

/// UserBuffer
pub struct UserBuffer {
    ///U8 vec
    pub buffers: Vec<&'static mut [u8]>,
}

impl UserBuffer {
    pub fn new(buffers: Vec<&'static mut [u8]>) -> Self {
        Self { buffers }
    }

    pub fn len(&self) -> usize {
        let mut total: usize = 0;
        for b in self.buffers.iter() {
            total += b.len();
        }

        total
    }
}

impl IntoIterator for UserBuffer {
    type Item = *mut u8;
    type IntoIter = UserBufferIterator;
    fn into_iter(self) -> Self::IntoIter {
        UserBufferIterator {
            buffers: self.buffers,
            current_buffer: 0,
            current_idx: 0,
        }
    }
}

pub struct UserBufferIterator {
    buffers: Vec<&'static mut [u8]>,
    current_buffer: usize,
    current_idx: usize,
}

impl Iterator for UserBufferIterator {
    type Item = *mut u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_buffer >= self.buffers.len() {
            None
        } else {
            let r = &mut self.buffers[self.current_buffer][self.current_idx] as *mut _;
            if self.current_idx + 1 == self.buffers[self.current_buffer].len() {
                self.current_idx = 0;
                self.current_buffer += 1;
            } else {
                self.current_idx += 1;
            }
            Some(r)
        }
    }
}
