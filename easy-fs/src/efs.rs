//! 实现 easy-fs的整体磁盘布局，将各段区域及上面的磁盘数据构结整合起来就是简易文件系统
//! EasyFileSystem 的职责。它知道每个布局区域所在的位置，磁盘块的分配和回收也需要经过它才能完成，
//! 因此某种意义上讲它还可以看成一个磁盘块管理器

use super::{
    block_cache_sync_all, get_block_cache, Bitmap, BlockDevice, DiskInode, DiskInodeType, Inode,
    SuperBlock,
};
use crate::BLOCK_SZ;
use alloc::sync::Arc;
use spin::Mutex;
///An easy file system on block
pub struct EasyFileSystem {
    ///Real device
    /// 块设备指针
    pub block_device: Arc<dyn BlockDevice>,
    ///Inode bitmap
    /// 索引节点位图
    pub inode_bitmap: Bitmap,
    ///Data bitmap
    /// 数据块位图
    pub data_bitmap: Bitmap,
    /// 索引节点区域起始块编号，方便确定每个索引节点在磁盘上的位置
    inode_area_start_block: u32,
    /// 数据节点区域起始块编号，方便确定每个数据块在磁盘上的位置
    data_area_start_block: u32,
}

type DataBlock = [u8; BLOCK_SZ];
/// An easy fs over a block device
impl EasyFileSystem {
    /// A data block of block size
    /// 在块设备上创建并初始化一个文件系统
    pub fn create(
        block_device: Arc<dyn BlockDevice>,
        total_blocks: u32,
        inode_bitmap_blocks: u32,
    ) -> Arc<Mutex<Self>> {
        // calculate block size of areas & create bitmaps
        // 计算并初始化索引节点位图
        let inode_bitmap = Bitmap::new(1, inode_bitmap_blocks as usize);
        // 计算索引节点数量
        let inode_num = inode_bitmap.maximum();
        // 计算并初始化数据块
        let inode_area_blocks =
            ((inode_num * core::mem::size_of::<DiskInode>() + BLOCK_SZ - 1) / BLOCK_SZ) as u32;
        // 计算索引节点所占数据块
        let inode_total_blocks = inode_bitmap_blocks + inode_area_blocks;
        // 计数数据块
        let data_total_blocks = total_blocks - 1 - inode_total_blocks;
        // 计算数据块位图所占数据块
        let data_bitmap_blocks = (data_total_blocks + 4096) / 4097;
        // 计算数据块区域所占数据块
        let data_area_blocks = data_total_blocks - data_bitmap_blocks;
        // 数据块位图
        let data_bitmap = Bitmap::new(
            (1 + inode_bitmap_blocks + inode_area_blocks) as usize,
            data_bitmap_blocks as usize,
        );
        // 初始化 efs
        let mut efs = Self {
            block_device: Arc::clone(&block_device),
            inode_bitmap,
            data_bitmap,
            inode_area_start_block: 1 + inode_bitmap_blocks,
            data_area_start_block: 1 + inode_total_blocks + data_bitmap_blocks,
        };
        // clear all blocks
        // 首先将块设备的前 total_blocks 个块清零，因为 easy-fs 要用到它们，这也是为初始化做准备
        for i in 0..total_blocks {
            get_block_cache(i as usize, Arc::clone(&block_device))
                .lock()
                .modify(0, |data_block: &mut DataBlock| {
                    for byte in data_block.iter_mut() {
                        *byte = 0;
                    }
                });
        }
        // initialize SuperBlock
        // 将位于块设备编号为 0 块上的超级块进行初始化，只需传入之前计算得到的每个区域的块数就行了
        get_block_cache(0, Arc::clone(&block_device)).lock().modify(
            0,
            |super_block: &mut SuperBlock| {
                super_block.initialize(
                    total_blocks,
                    inode_bitmap_blocks,
                    inode_area_blocks,
                    data_bitmap_blocks,
                    data_area_blocks,
                );
            },
        );
        // write back immediately
        // create a inode for root node "/"
        // 调用 alloc_inode 在 inode 位图中分配一个 inode
        // 由于这是第一次分配，它的编号固定是 0
        assert_eq!(efs.alloc_inode(), 0);
        // 将分配到的 inode 初始化为 easy-fs 中的唯一一个目录
        // 调用 get_disk_inode_pos 来根据 inode 编号获取该 inode 所在的块的编号以及块内偏移
        // 之后就可以将它们传给 get_block_cache 和 modify 了
        let (root_inode_block_id, root_inode_offset) = efs.get_disk_inode_pos(0);
        get_block_cache(root_inode_block_id as usize, Arc::clone(&block_device))
            .lock()
            .modify(root_inode_offset, |disk_inode: &mut DiskInode| {
                disk_inode.initialize(DiskInodeType::Directory);
            });
        block_cache_sync_all();
        Arc::new(Mutex::new(efs))
    }
    /// Open a block device as a filesystem
    /// 从一个已写入了 easy-fs 镜像的块设备上打开 easy-fs
    pub fn open(block_device: Arc<dyn BlockDevice>) -> Arc<Mutex<Self>> {
        // read SuperBlock
        // 读取设备号为 0 的块作为超级块，就可以从中知道 easy-fs 的磁盘布局
        // ，由此可以构造 efs实例
        get_block_cache(0, Arc::clone(&block_device))
            .lock()
            .read(0, |super_block: &SuperBlock| {
                assert!(super_block.is_valid(), "Error loading EFS!");
                let inode_total_blocks =
                    super_block.inode_bitmap_blocks + super_block.inode_area_blocks;
                let efs = Self {
                    block_device,
                    inode_bitmap: Bitmap::new(1, super_block.inode_bitmap_blocks as usize),
                    data_bitmap: Bitmap::new(
                        (1 + inode_total_blocks) as usize,
                        super_block.data_bitmap_blocks as usize,
                    ),
                    inode_area_start_block: 1 + super_block.inode_bitmap_blocks,
                    data_area_start_block: 1 + inode_total_blocks + super_block.data_bitmap_blocks,
                };
                Arc::new(Mutex::new(efs))
            })
    }
    /// Get the root inode of the filesystem
    pub fn root_inode(efs: &Arc<Mutex<Self>>) -> Inode {
        let block_device = Arc::clone(&efs.lock().block_device);
        // acquire efs lock temporarily
        let (block_id, block_offset) = efs.lock().get_disk_inode_pos(0);
        // release efs lock
        Inode::new(block_id, block_offset, Arc::clone(efs), block_device)
    }
    /// Get inode by id
    /// 可以从 inode 位图或数据块位图上分配的 bit 编号，来算出各个存储 inode 和数据块的磁盘块在磁盘上的实际位置
    pub fn get_disk_inode_pos(&self, inode_id: u32) -> (u32, usize) {
        let inode_size = core::mem::size_of::<DiskInode>();
        let inodes_per_block = (BLOCK_SZ / inode_size) as u32;
        let block_id = self.inode_area_start_block + inode_id / inodes_per_block;
        (
            block_id,
            (inode_id % inodes_per_block) as usize * inode_size,
        )
    }
    /// Get data block by id
    pub fn get_data_block_id(&self, data_block_id: u32) -> u32 {
        self.data_area_start_block + data_block_id
    }
    /// Allocate a new inode
    /// inode 的分配
    pub fn alloc_inode(&mut self) -> u32 {
        self.inode_bitmap.alloc(&self.block_device).unwrap() as u32
    }

    /// Allocate a data block
    /// 数据块的分配
    pub fn alloc_data(&mut self) -> u32 {
        self.data_bitmap.alloc(&self.block_device).unwrap() as u32 + self.data_area_start_block
    }
    /// Deallocate a data block
    pub fn dealloc_data(&mut self, block_id: u32) {
        get_block_cache(block_id as usize, Arc::clone(&self.block_device))
            .lock()
            .modify(0, |data_block: &mut DataBlock| {
                data_block.iter_mut().for_each(|p| {
                    *p = 0;
                })
            });
        self.data_bitmap.dealloc(
            &self.block_device,
            (block_id - self.data_area_start_block) as usize,
        )
    }
}
