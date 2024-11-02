//! bitMap 位图抽象
//! 在 easy-fs 布局中存在两类不同的位图，分别对索引节点和数据块进行管理。
//! 每个位图都由若干个块组成，每个块大小为 512 bytes，即 4096 bits

use super::{get_block_cache, BlockDevice, BLOCK_SZ};
use alloc::sync::Arc;
/// A bitmap block
/// BitmapBlock 是一个磁盘数据结构，它将位图区域中的一个磁盘块解释为长度为 64 的一个 u64 数组
/// 数组包含 64 × 64 = 4096 bits
type BitmapBlock = [u64; 64];
/// Number of bits in a block
const BLOCK_BITS: usize = BLOCK_SZ * 8;
/// A bitmap
/// 保存了它所在区域的起始块编号以及区域的长度为多少个块。
pub struct Bitmap {
    /// 起始编号
    start_block_id: usize,
    /// 所占块数
    blocks: usize,
}

/// Decompose bits into (block_pos, bits64_pos, inner_pos)
/// 将 bit 编号分解为区域中的块编号 block_pos, 块内的组编号 bits64_pos，
/// 以及组内编号 inner_pos 三元组
fn decomposition(mut bit: usize) -> (usize, usize, usize) {
    let block_pos = bit / BLOCK_BITS;
    bit %= BLOCK_BITS;
    (block_pos, bit / 64, bit % 64)
}

impl Bitmap {
    /// A new bitmap from start block id and number of blocks
    /// 创建一个位图
    pub fn new(start_block_id: usize, blocks: usize) -> Self {
        Self {
            start_block_id,
            blocks,
        }
    }
    /// Allocate a new block from a block device
    /// 分配一个 bit
    pub fn alloc(&self, block_device: &Arc<dyn BlockDevice>) -> Option<usize> {
        // 枚举区域中的每一个块，在每个块中以 bit 组（每组64bits）为单位进行遍历
        for block_id in 0..self.blocks {
            // 获取缓块缓存
            let pos = get_block_cache(
                // block_id + 加上区域起始块编号
                block_id + self.start_block_id as usize,
                Arc::clone(block_device),
            )
            // 获取缓存块的互斥锁，对缓存块进行访问
            .lock()
            .modify(0, |bitmap_block: &mut BitmapBlock| {
                // offset 为 0，这是因为整个块上只有一个 BitmapBlock，他的大小正好是 512 字节
                // 传入的类型是 &mut BitmapBlock
                //
                // 尝试在 bitmap_block 中找到一个空闲的 bit 并返回其位置
                //遍历每 64 bits 构成的组（一个 u64 ），如果它并没有达到 u64::MAX（即 2^64 − 1 ）
                //则通过 u64::trailing_ones 找到最低的一个 0 并置为 1 。如果能够找到的话，
                //bit 组的编号将保存在变量 bits64_pos 中，而分配的 bit 在组内的位置将保存在变量 inner_pos 中
                if let Some((bits64_pos, inner_pos)) = bitmap_block
                    .iter()
                    .enumerate()
                    .find(|(_, bits64)| **bits64 != u64::MAX)
                    .map(|(bits64_pos, bits64)| (bits64_pos, bits64.trailing_ones() as usize))
                {
                    // modify cache
                    bitmap_block[bits64_pos] |= 1u64 << inner_pos;
                    Some(block_id * BLOCK_BITS + bits64_pos * 64 + inner_pos as usize)
                } else {
                    None
                }
            });
            if pos.is_some() {
                return pos;
            }
        }
        None
    }
    /// Deallocate a block
    pub fn dealloc(&self, block_device: &Arc<dyn BlockDevice>, bit: usize) {
        let (block_pos, bits64_pos, inner_pos) = decomposition(bit);
        get_block_cache(block_pos + self.start_block_id, Arc::clone(block_device))
            .lock()
            .modify(0, |bitmap_block: &mut BitmapBlock| {
                assert!(bitmap_block[bits64_pos] & (1u64 << inner_pos) > 0);
                bitmap_block[bits64_pos] -= 1u64 << inner_pos;
            });
    }
    /// Get the max number of allocatable blocks
    pub fn maximum(&self) -> usize {
        self.blocks * BLOCK_BITS
    }
}
