use crate::{block_cache::get_block_cache, BlockDevice, BLOCK_SIZE};

type BitMapBlock = [u64; 64];
const BLOCKS_BITS: usize = BLOCK_SIZE * 8;


pub struct BitMap {
    start_block_id: usize,
    blocks: usize,
}

impl BitMap {
    pub fn new(start_block_id: usize, blocks: usize) -> Self {
        Self {
            start_block_id,
            blocks,
        }
    }

    pub fn alloc(&self, block_device: &Arc<dyn BlockDevice>) -> Option<usize> {
        for block_id in 0..self.blocks  {
            let pos = get_block_cache(
                block_id + self.start_block_id as usize,
                Arc::clone(block_device),
                ).lock().modify(offset, f)
        }
    }
}
