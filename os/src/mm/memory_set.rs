use alloc::{collections::btree_map::BTreeMap, vec::Vec};

use crate::config::PAGE_SIZE;

use super::{address::{PhyPageNum, VirtPageNum}, frame_allocator::{frame_alloc, FrameTracker}, page_table::{self, PTEFlags, PageTable}};


pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1<<1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    pub fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut  self.page_table);

        if let Some(data) = data {
            map_area.copy_data(&self.page_table, data);
            self.areas.push(map_area);
        }
    }

    pub fn inset_framed_area(
        &mut self,
        strat_va: VirtAddr, end_va: VirtAddr, permission: MapPermission
    ) {
        self.push(MapArea::new(
                strat_va,
                end_va,
                MapType::Framed,
                permission,
        ), None);
        
    }

    pub fn new_kernel() ->Self;

    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize);    
}

impl MapArea {
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission
    ) -> Self{
        let start_vpn: VirtPageNum = start_va.floor();
        let end_vpn: VirtPageNum = end_va.ceil();
        Self {
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            data_frames: BTreeMap::new(),
            map_type,
            map_perm,
        }  
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
        }
    }

    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range  {
            self.unmap_one(page_table, vpn);
        }
    }

    pub fn copy_data(&mut self, page_table: &PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.get_start();
        let len = data.len();

        loop {
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwarp()
                .ppn()
                .get_bytes_array()[..src.len()];
            dst.copy_from_slice(src);
            src += PAGE_SIZE;
            if start > len {
                break;
            }
            current_vpn.step();
        }
    }

    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhyPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = PhyPageNum(vpn.0);
            }

            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }

        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        match self.map_type {
            MapType::Framed => {
                self.data_frames.remove(&vpn);
            }

            _=> {}
        }
        page_table.unmap(vpn);
    }
}
