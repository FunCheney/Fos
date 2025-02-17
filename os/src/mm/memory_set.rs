//! Implementation of [`MapArea`] and [`MemorySet`].

use super::{frame_alloc, FrameTracker};
use super::{PTEFlags, PageTable, PageTableEntry};
use super::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use super::{StepByOne, VPNRange};
use crate::board::MMIO;
use crate::config::{MEMORY_END, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT, USER_STACK_SIZE};
use crate::sync::UPSafeCell;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::arch::asm;
use lazy_static::*;
use riscv::register::satp;

// 从 os/src/linker.ld 中应用了很多表示各段位置的符号
extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

lazy_static! {
    /// a memory set instance through lazy_static! managing kernel space
    /// 在 KERNEL_SPACE 第一次被用时进行初始化
    /// 创建内核地址空间
    pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>> =
        Arc::new(unsafe { UPSafeCell::new(MemorySet::new_kernel()) });
}

///Get kernelspace root ppn
pub fn kernel_token() -> usize {
    KERNEL_SPACE.exclusive_access().token()
}

/// MemorySet 用来管理虚拟地址空间
///
/// 地址空间：一些列有关联的不一定连续的逻辑段
/// 这种关联一般是指这些逻辑段组成的虚拟内存空间与一个运行的程序绑定
/// 即这个运行的程序对代码和数据的直接访问范围限制在它关联的虚拟地址空间之内。
/// 这样我们就有任务的地址空间，内核的地址空间等说法了。
pub struct MemorySet {
    // 多级页表 PageTable，下挂着所有多级页表的节点所在的物理页帧
    page_table: PageTable,
    // 逻辑段向量，挂着对应逻辑段中的数据所在的物理页帧，
    // 这两部分合在一起构成了一个地址空间所需的所有物理页帧
    // 在一个虚拟地址空间中，有代码段，数据段等不同属性且不一定连续的子空间，它们通过一个重要的
    // 数据结构 MapArea 来表示和管理
    areas: Vec<MapArea>,
}

impl MemorySet {
    // 新建一个新的地址空间
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    pub fn token(&self) -> usize {
        self.page_table.token()
    }

    /// Assume that no conflicts.
    /// 调用 push ，可以在当前地址空间插入一个 Framed 方式映射到物理内存的逻辑段
    /// 该方法的调用者要保证同一地址空间内的任意两个逻辑段不能存在交集
    pub fn insert_framed_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
    ) {
        self.push(
            MapArea::new(start_va, end_va, MapType::Framed, permission),
            None,
        );
    }

    pub fn remove_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        if let Some((idx, area)) = self
            .areas
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.vpn_range.get_start() == start_vpn)
        {
            area.unmap(&mut self.page_table);
            self.areas.remove(idx);
        }
    }
    // 在当前地址空间插入一个新的逻辑段 map_area ，如果它是以 Framed 方式映射到物理内存，
    // 还可以可选地在那些被映射到的物理页帧上写入一些初始化数据 data
    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&self.page_table, data);
        }
        self.areas.push(map_area);
    }
    /// Mention that trampoline is not collected by areas.
    /// 在内置页表中将虚拟地址 trampoline 所对应的虚拟页映射到 __alltraps 对应的页
    fn map_trampoline(&mut self) {
        self.page_table.map(
            // 虚拟页是: TRAMPOLINE 虚拟页里面中的次高页的位置
            VirtAddr::from(TRAMPOLINE).into(),
            // 物理页是 strampoline 之间的映射，对应 __alltraps 所对应的物理页起始地址
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        );
    }

    /// Without kernel stacks.
    /// 生成内核的地址空间
    /// 通过该方法后，整个内核的地址空间都是对等映射，说明虚拟页就是物理页，
    /// 想访问那个地址，直接访问就行
    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new_bare();
        // map trampoline
        // 映射内核中的 TRAMPOLINE 和 __alltraps 中断的位置
        memory_set.map_trampoline();
        // map kernel sections
        println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        println!(
            ".bss [{:#x}, {:#x})",
            sbss_with_stack as usize, ebss as usize
        );
        println!("mapping .text section");
        // 内核代码段作对等映射
        memory_set.push(
            MapArea::new(
                (stext as usize).into(),
                (etext as usize).into(),
                // 映射方式为恒等映射
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );
        println!("mapping .rodata section");
        // rodata 段作对等映射
        memory_set.push(
            MapArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                // 映射方式为恒等映射
                MapType::Identical,
                MapPermission::R,
            ),
            None,
        );
        println!("mapping .data section");
        // data 段作对等映射
        memory_set.push(
            MapArea::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                // 恒等映射
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        println!("mapping .bss section");
        // bss 段作对等映射
        memory_set.push(
            MapArea::new(
                (sbss_with_stack as usize).into(),
                (ebss as usize).into(),
                // 恒等映射
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        println!("mapping physical memory");
        // 内核到内存的尽头作对等映射
        memory_set.push(
            MapArea::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                // 恒等映射
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        println!("mapping memory-mapped registers");
        for pair in MMIO {
            memory_set.push(
                MapArea::new(
                    (*pair).0.into(),
                    ((*pair).0 + (*pair).1).into(),
                    // 恒等映射
                    MapType::Identical,
                    MapPermission::R | MapPermission::W,
                ),
                None,
            );
        }
        memory_set
    }

    /// Include sections in elf and trampoline and TrapContext and user stack,
    /// also returns user_sp and entry point.
    /// 分析应用的 ELF 文件格式的内容，解析出各数据段并生成对应的地址空间
    /// 返回三元组: 进程的 MemorySet，用户栈地址，入口地址
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        // 创建一个新的空间
        let mut memory_set = Self::new_bare();
        // map trampoline
        // 映射 translate 跳板页
        memory_set.map_trampoline();
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        // 应用程序在链接的时候就已经确定了每个应用的虚拟地址（逻辑地址）
        // 在载入系统的时候，数据在程序中的虚拟地址和在内存中的虚拟地址是一致的
        // 这样才能保证，程序在进入虚拟内存后才能正常运行
        let ph_count = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            // 确认 program_header 的类型是 Load，表明有被内核加载的需要
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                // 计算应用在地址空间中的开始位置
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                // 计算应用在地址空间中的结束位置
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                // 表示程序在用户态下运行
                let mut map_perm = MapPermission::U;
                // 确认这异区域的访问限制，并将其转化为 MapPermission 类型
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                // 创建 map_area 逻辑地址空间（虚拟地址）
                let map_area = MapArea::new(start_va, end_va, MapType::Framed, map_perm);
                max_end_vpn = map_area.vpn_range.get_end();
                // push 到应用的地址空间
                memory_set.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
            }
        }
        // map user stack with U flags
        let max_end_va: VirtAddr = max_end_vpn.into();
        // 放置一个保护页面的用户栈
        let mut user_stack_bottom: usize = max_end_va.into();
        // guard page 多加一页作为保护，计算出用户栈栈底的位置
        user_stack_bottom += PAGE_SIZE;
        // 分配用户栈，在用户程序的最后 方了一个页作保护，然后开始分配用户栈
        // 加上 用户栈的大小，确定用户栈栈顶的位置
        let user_stack_top = user_stack_bottom + USER_STACK_SIZE;
        memory_set.push(
            MapArea::new(
                user_stack_bottom.into(),
                user_stack_top.into(),
                // 设置为 Framed 映射方式 （随机映射）
                MapType::Framed,
                // 设置用户栈 可读/可写/用户态下可用
                MapPermission::R | MapPermission::W | MapPermission::U,
            ),
            None,
        );
        // used in sbrk
        memory_set.push(
            MapArea::new(
                user_stack_top.into(),
                user_stack_top.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W | MapPermission::U,
            ),
            None,
        );
        // map TrapContext
        // 预留了 TRAMPOLINE 前面的一个虚拟页用来放 TrapContext
        memory_set.push(
            MapArea::new(
                // 开始位置
                TRAP_CONTEXT.into(),
                // TRAP_CONTEXT 结束的位置为 TRAMPOLINE 开始的位置
                TRAMPOLINE.into(),
                // 随机映射
                MapType::Framed,
                // 设置
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        // 返回数据
        (
            memory_set,                            // 应用地址空间
            user_stack_top,                        // 用户栈虚拟地址
            elf.header.pt2.entry_point() as usize, // 解析 ELF 得到的应用入口点点地址
        )
    }

    pub fn from_existed_user(user_space: &Self) -> Self {
        let mut memory_set = Self::new_bare();
        memory_set.map_trampoline();

        for area in user_space.areas.iter() {
            let new_area = MapArea::from_another(area);
            memory_set.push(new_area, None);
            for vpn in area.vpn_range {
                // 父进程的物理地址页号
                let src_ppn = user_space.translate(vpn).unwrap().ppn();
                // 子进程的物理地址页号
                let dst_ppn = memory_set.translate(vpn).unwrap().ppn();
                dst_ppn
                    .get_bytes_array()
                    .copy_from_slice(&src_ppn.get_bytes_array());
            }
        }

        memory_set
    }

    /// 使能分页机制
    /// 使用 activate 方法使其生效
    /// 使能分页机制后，cpu 访问的地址都是虚拟地址，内河中页是基于虚拟地址进行虚存的访问
    ///  1. 在给应用添加虚拟地址空间之前，内核自己也会建立一个页表
    ///         把整块物理地址通过恒等映射，映射到内核的虚拟地址空间中。
    ///  2. 在应用执行之前，操作系统帮助其建立一个虚拟地址空间。
    pub fn activate(&self) {
        // 调用 token 方法
        let satp = self.page_table.token();
        unsafe {
            // 让 CPU 开启分页模式
            satp::write(satp);
            // 为了确保 MMU 的地址转换能够及时与 satp 的修改同步，我们需要立即使用 sfence.vma 指令将快表清空，
            // 这样 MMU 就不会看到快表中已经过期的键值对了。
            asm!("sfence.vma");
        }
    }
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.page_table.translate(vpn)
    }
    #[allow(unused)]
    pub fn shrink_to(&mut self, start: VirtAddr, new_end: VirtAddr) -> bool {
        if let Some(area) = self
            .areas
            .iter_mut()
            .find(|area| area.vpn_range.get_start() == start.floor())
        {
            area.shrink_to(&mut self.page_table, new_end.ceil());
            true
        } else {
            false
        }
    }
    #[allow(unused)]
    pub fn append_to(&mut self, start: VirtAddr, new_end: VirtAddr) -> bool {
        if let Some(area) = self
            .areas
            .iter_mut()
            .find(|area| area.vpn_range.get_start() == start.floor())
        {
            area.append_to(&mut self.page_table, new_end.ceil());
            true
        } else {
            false
        }
    }

    pub fn recycle_data_pages(&mut self) {
        self.areas.clear();
    }
}

/// map area structure, controls a contiguous piece of virtual memory
/// 以逻辑段为单位描述一段连续的虚拟内存
/// 逻辑段就是指地址区间中一段实际可用的地址连续的虚拟地址区间，该区间内
/// 包含的所有虚拟页都以一种相同的方式映射到物理页帧，具有可读、可写、可执行等属性。
pub struct MapArea {
    // 描述一段虚拟页号的连续空间，表示该逻辑段在地址区间中的长度和位置
    // 它是一个迭代器，可以使用 Rust 的语法糖 for-loop 进行迭代
    vpn_range: VPNRange,
    // 当逻辑段采用 MapType::Framed 方式映射到物理内存的时候，data_frames 是一个保存了该逻辑段内
    // 的每个虚拟页面和它被映射到的物理页帧 FrameTracker 的一个键值对容器 BTreeMap 中，这些物理页帧
    // 被用来存放实际内存数据而不是作为多级页表中的中间节点
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    // 描述该逻辑段内的所有虚拟页面映射到物理页帧的同一种方式
    // 恒等映射: 主要为内核地址空间服务
    // 非恒等映射: 主要为用户程序服务
    map_type: MapType,
    // 页表项标志位的子集
    map_perm: MapPermission,
}

impl MapArea {
    // 新建一个逻辑段结构体，注意传入的起始/终止虚拟地址会分别被下取整/上取整为虚拟页号
    // 并传入迭代器 vpn_range 中
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
    ) -> Self {
        let start_vpn: VirtPageNum = start_va.floor();
        let end_vpn: VirtPageNum = end_va.ceil();
        Self {
            // 虚拟页号的空间
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            // vpn --> ppn 的映射
            data_frames: BTreeMap::new(),
            map_type,
            map_perm,
        }
    }
    /// 在虚拟页号 vpn 确定的情况下，需要知道将一个怎样的页表项插入多级页表
    /// 页表项的标志位来源于当前逻辑段的类型为 MapPermission 的统一配置，只需将其转换为 PTEFlags；
    /// 而页表项的物理页号则取决于当前逻辑段映射到物理内存的方式：
    ///     当以恒等映射 Identical 方式映射的时候，物理页号就等于虚拟页号；
    ///     当以 Framed 方式映射时，需要分配一个物理页帧让当前的虚拟页面可以映射过去，
    ///     此时页表项中的物理页号自然就是 这个被分配的物理页帧的物理页号。
    ///     此时还需要将这个物理页帧挂在逻辑段的 data_frames 字段下。
    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        // 确定了页表项的标志位和物理页号之后，调用 多级页表 page_table 的 map 接口来插入键值对
        page_table.map(vpn, ppn, pte_flags);
    }
    #[allow(unused)]
    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_type == MapType::Framed {
            self.data_frames.remove(&vpn);
        }
        // 调应 page_table 的 unmap 接口删除以传入的虚拟页号为键的键值对即可。
        // 然而，当以 Framed 映射的时候，不要忘记同时将虚拟页面被映射到的物理页帧 FrameTracker 从 data_frames 中移除，
        // 这样这个物理页帧才能立即被回收以备后续分配。
        page_table.unmap(vpn);
    }
    // 将当前逻辑段到物理内存的映射从传入的该逻辑段所属的地址空间的多级页表中加入
    pub fn map(&mut self, page_table: &mut PageTable) {
        // 遍历逻辑段中的所有虚拟页面
        for vpn in self.vpn_range {
            // 以每个虚拟页面为单位依次在多级页表中进行键值对的插入
            self.map_one(page_table, vpn);
        }
    }

    // 将当前逻辑段到物理内存的映射从传入的该逻辑段所属的地址空间的多级页表中删除
    #[allow(unused)]
    pub fn unmap(&mut self, page_table: &mut PageTable) {
        // 遍历逻辑段中的所有虚拟页面
        for vpn in self.vpn_range {
            // 以每个虚拟页面为单位依次在多级页表中进行键值对的删除
            self.unmap_one(page_table, vpn);
        }
    }
    #[allow(unused)]
    pub fn shrink_to(&mut self, page_table: &mut PageTable, new_end: VirtPageNum) {
        for vpn in VPNRange::new(new_end, self.vpn_range.get_end()) {
            self.unmap_one(page_table, vpn)
        }
        self.vpn_range = VPNRange::new(self.vpn_range.get_start(), new_end);
    }
    #[allow(unused)]
    pub fn append_to(&mut self, page_table: &mut PageTable, new_end: VirtPageNum) {
        for vpn in VPNRange::new(self.vpn_range.get_end(), new_end) {
            self.map_one(page_table, vpn)
        }
        self.vpn_range = VPNRange::new(self.vpn_range.get_start(), new_end);
    }
    /// data: start-aligned but maybe with shorter length
    /// assume that all frames were cleared before
    /// 将切片 data 中的数据拷贝到当前逻辑段实际被内核放置在各个物理页帧上
    /// 从而在第中空间中通过逻辑段就能访问这些数据
    /// 切片 data 中的数据大小不超过当前逻辑段的总大小，且切片中的数据会被对齐到
    /// 逻辑段的开头，然后逐页拷贝到实际的物理页帧
    pub fn copy_data(&mut self, page_table: &PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.get_start();
        let len = data.len();
        // 遍历每一个需要拷贝数据的虚拟页面
        loop {
            // 页面拷贝的数据源 切片
            let src = &data[start..len.min(start + PAGE_SIZE)];
            // 页面拷贝的目标 切片
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .get_bytes_array()[..src.len()];
            // 通过 copy_from_slice() 方法完成复制
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            if start >= len {
                break;
            }
            // 数据拷贝完之后调用该方法
            current_vpn.step();
        }
    }

    pub fn from_another(another: &MapArea) -> Self {
        Self {
            vpn_range: VPNRange::new(another.vpn_range.get_start(), another.vpn_range.get_end()),
            data_frames: BTreeMap::new(),
            map_type: another.map_type,
            map_perm: another.map_perm,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
/// map type for memory set: identical or framed
pub enum MapType {
    // 表示恒等映射
    // 主要是用在启用多级页表之后，内核仍能够在虚存地址空间中访问
    // 一个特定的物理地址指向的物理内存。
    Identical,
    // 表示对于每个虚拟页面都有一个新分配的物理页帧与之对应，
    // 虚地址与物理地址的映射关系是相对随机的
    Framed,
}


bitflags! {
    /// map permission corresponding to that in pte: `R W X U`
    /// 控制该逻辑段的访问方式，它是页表项标志位 PTEFlags 的一个子集
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[allow(unused)]
pub fn remap_test() {
    let mut kernel_space = KERNEL_SPACE.exclusive_access();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert!(!kernel_space
        .page_table
        .translate(mid_text.floor())
        .unwrap()
        .writable(),);
    assert!(!kernel_space
        .page_table
        .translate(mid_rodata.floor())
        .unwrap()
        .writable(),);
    assert!(!kernel_space
        .page_table
        .translate(mid_data.floor())
        .unwrap()
        .executable(),);
    println!("remap_test passed!");
}
