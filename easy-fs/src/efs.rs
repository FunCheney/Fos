use alloc::sync::Arc;

use super::{
    block_cache_sync_call, get_block_cache, BitMap, BlockDevice, DiskInode, DiskInodeType, Inode,
    SuperBlock, BLOCK_SIZE,
};
use spin::Mutex;

/// 简易文件系统
pub struct EasyFileSystem {
    /// read deice
    pub block_device: Arc<dyn BlockDevice>,
    /// Inode bitmap
    pub inode_bitmap: BitMap,
    /// data bitmap
    pub data_bitmap: BitMap,
    inode_area_start_block: u32,
    data_area_start_block: u32,
}

type DataBlock = [u8; BLOCK_SIZE];

impl EasyFileSystem {
    /// 创建 data_block
    pub fn create(
        block_device: Arc<dyn BlockDevice>,
        total_blocks: u32,
        inode_bitmap_blocks: u32,
    ) -> Arc<Mutex<Self>> {
        let inode_bitmap = BitMap::new(1, inode_bitmap_blocks as usize);
        let inode_num = inode_bitmap.maxmum();
        let inode_area_blocks =
            ((inode_num * core::mem::size_of::<DiskInode>() + BLOCK_SIZE - 1) / BLOCK_SIZE) as u32;

        let inode_total_blocks = inode_bitmap_blocks + inode_area_blocks;
        let data_toal_blocks = total_blocks - 1 - inode_total_blocks;
        let data_bitmap_blocks = (data_toal_blocks + 4096) / 4097;
        let data_area_blocks = data_toal_blocks - data_bitmap_blocks;

        let data_bitmap = BitMap::new(
            (1 + inode_bitmap_blocks + inode_area_blocks) as usize,
            data_bitmap_blocks as usize,
        );

        let mut efs = Self {
            block_device: Arc::clone(&block_device),
            inode_bitmap,
            data_bitmap,
            inode_area_start_block: 1 + inode_bitmap_blocks,
            data_area_start_block: 1 + inode_total_blocks + data_bitmap_blocks,
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
            |supper_block: &mut SuperBlock| {
                supper_block.initialize(
                    total_blocks,
                    inode_bitmap_blocks,
                    inode_area_blocks,
                    data_bitmap_blocks,
                    data_area_blocks,
                );
            },
        );

        assert_eq!(efs.alloc_inode(), 0);
        let (root_inode_block_id, root_inode_offset) = efs.get_disk_inode_ops(0);
        get_block_cache(root_inode_block_id as usize, Arc::clone(&block_device))
            .lock()
            .modify(root_inode_offset, |disk_inode: &mut DiskInode| {
                disk_inode.initialize(DiskInodeType::Directory);
            });

        block_cache_sync_call();
        Arc::new(Mutex::new(efs))
    }

    /// open
    pub fn open(block_device: Arc<dyn BlockDevice>) -> Arc<Mutex<Self>> {
        get_block_cache(0, Arc::clone(&block_device))
            .lock()
            .read(0, |supper_block: &SuperBlock| {
                assert!(supper_block.is_validate(), "Error loading EFS!");
                let inode_total_blocks =
                    supper_block.inode_bitmap_blocks + supper_block.inode_area_blocks;

                let efs = Self {
                    block_device,
                    inode_bitmap: BitMap::new(1, supper_block.inode_bitmap_blocks as usize),
                    data_bitmap: BitMap::new(
                        (1 + inode_total_blocks) as usize,
                        supper_block.data_bitmap_blocks as usize,
                    ),
                    inode_area_start_block: 1 + supper_block.inode_bitmap_blocks,
                    data_area_start_block: 1 + inode_total_blocks + supper_block.data_bitmap_blocks,
                };

                Arc::new(Mutex::new(efs))
            })
    }

    /// get inode by id
    pub fn get_disk_inode_ops(&self, inode_id: u32) -> (u32, usize) {
        let inode_size = core::mem::size_of::<DiskInode>();
        let inode_per_block = (BLOCK_SIZE / inode_size) as u32;
        let block_id = self.inode_area_start_block + inode_id / inode_per_block;
        (block_id, (inode_id % inode_per_block) as usize * inode_size)
    }
    /// get data by data_block_id
    pub fn get_data_block_id(&self, data_block_id: u32) -> u32 {
        self.data_area_start_block + data_block_id
    }

    /// alloc new inode
    pub fn alloc_inode(&mut self) -> u32 {
        self.inode_bitmap.alloc(&self.block_device).unwrap() as u32
    }

    /// alloc_data
    pub fn alloc_data(&mut self) -> u32 {
        self.data_bitmap.alloc(&self.block_device).unwrap() as u32 + self.data_area_start_block
    }

    /// dealloc_data
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

    /// root_inode
    pub fn root_inode(efs: &Arc<Mutex<Self>>) -> Inode {
        let block_device = Arc::clone(&efs.lock().block_device);

        let (block_id, block_offset) = efs.lock().get_disk_inode_ops(0);

        Inode::new(block_id, block_offset, Arc::clone(efs), block_device)
    }
}
