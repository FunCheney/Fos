use alloc::sync::Arc;

use super::{
    block_cache_sync_call, get_block_cache, BitMap, BlockDevice, DiskInode, DiskInodeType,
    SupperBlock, BLOCK_SIZE,
};
use spin::Mutex;

pub struct EasyFileSystem {
    pub block_device: Arc<dyn BlockDevice>,
    pub innode_bitmap: BitMap,
    pub data_bitmap: BitMap,
    innode_area_start_block: u32,
    data_area_start_block: u32,
}

type DataBlock = [u8; BLOCK_SIZE];

impl EasyFileSystem {
    pub fn create(
        block_device: Arc<dyn BlockDevice>,
        total_blocks: u32,
        innode_bitmap_blocks: u32,
    ) -> Arc<Mutex<Self>> {
        let innode_bitmap = BitMap::new(1, innode_bitmap_blocks as usize);
        let innode_num = innode_bitmap.maxmum();
        let innode_area_blocks =
            ((innode_num * core::mem::size_of::<DiskInode>() + BLOCK_SIZE - 1) / BLOCK_SIZE) as u32;

        let innode_total_blocks = innode_bitmap_blocks + innode_area_blocks;
        let data_toal_blocks = total_blocks - 1 - innode_total_blocks;
        let data_bitmap_blocks = (data_toal_blocks + 4096) / 4097;
        let data_area_blocks = data_toal_blocks - data_bitmap_blocks;

        let data_bitmap = BitMap::new(
            (1 + innode_bitmap_blocks + innode_area_blocks) as usize,
            data_bitmap_blocks as usize,
        );

        let mut efs = Self {
            block_device: Arc::clone(&block_device),
            innode_bitmap,
            data_bitmap,
            innode_area_start_block: 1 + innode_bitmap_blocks,
            data_area_start_block: 1 + innode_total_blocks + data_bitmap_blocks,
        };

        for i in 0..total_blocks {
            get_block_cache(i as usize, Arc::clone(&block_device))
                .lock()
                .modify(0, |data_block: &mut DataBlock| {
                    for byte in data_block.iter_mut() {
                        *byte = 0;
                    }
                });
        }

        get_block_cache(0, Arc::clone(&block_device)).lock().modify(
            0,
            |supper_block: &mut SupperBlock| {
                supper_block.initialize(
                    total_blocks,
                    innode_bitmap_blocks,
                    innode_area_blocks,
                    data_bitmap_blocks,
                    data_area_blocks,
                );
            },
        );

        assert_eq!(efs.alloc_inode(), 0);
        let (root_innode_block_id, root_innode_offset) = efs.get_disk_inode_ops(0);
        get_block_cache(root_innode_block_id as usize, Arc::clone(&block_device))
            .lock()
            .modify(root_innode_offset, |disk_inode: &mut DiskInode| {
                disk_inode.initialize(DiskInodeType::Directory);
            });

        block_cache_sync_call();
        Arc::new(Mutex::new(efs))
    }

    pub fn open(block_device: Arc<dyn BlockDevice>) -> Arc<Mutex<Self>> {
        get_block_cache(0, Arc::clone(&block_device)).lock().read(
            0,
            |supper_block: &SupperBlock| {
                assert!(supper_block.is_validate(), "Error loading EFS!");
                let innode_total_blocks =
                    supper_block.inode_bitmap_blocks + supper_block.inode_area_blocks;

                let efs = Self {
                    block_device,
                    innode_bitmap: BitMap::new(1, supper_block.inode_bitmap_blocks as usize),
                    data_bitmap: BitMap::new(
                        (1 + innode_total_blocks) as usize,
                        supper_block.data_bit_map_blocks as usize,
                    ),
                    innode_area_start_block: 1 + supper_block.inode_bitmap_blocks,
                    data_area_start_block: 1
                        + innode_total_blocks
                        + supper_block.data_bit_map_blocks,
                };

                Arc::new(Mutex::new(efs))
            },
        )
    }

    pub fn get_disk_inode_ops(&self, inode_id: u32) -> (u32, usize) {
        let inode_size = core::mem::size_of::<DiskInode>();
        let inode_per_block = (BLOCK_SIZE / inode_size) as u32;
        let block_id = self.innode_area_start_block + inode_id / inode_per_block;
        (block_id, (inode_id % inode_per_block) as usize * inode_size)
    }

    pub fn get_date_block_id(&self, data_block_id: u32) -> u32 {
        self.data_area_start_block + data_block_id
    }

    pub fn alloc_inode(&mut self) -> u32 {
        self.innode_bitmap.alloc(&self.block_device).unwrap() as u32
    }

    pub fn alloc_data(&mut self) -> u32 {
        self.data_bitmap.alloc(&self.block_device).unwrap() as u32 + self.data_area_start_block
    }

    pub fn dealloc_data(&mut self, block_id: u32) {
        get_block_cache(block_id as usize, Arc::clone(&self.block_device))
            .lock()
            .modify(0, |data_block: &mut DataBlock| {
                data_block.iter_mut().for_each(|p| {
                    *p = 0;
                })
            });

        self.data_bitmap.delloc(
            &self.block_device,
            (block_id - self.data_area_start_block) as usize,
        );
    }
}
