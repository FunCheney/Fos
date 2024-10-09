use core::u64;

use alloc::sync::Arc;

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
        for block_id in 0..self.blocks {
            let pos = get_block_cache(
                block_id + self.start_block_id as usize,
                Arc::clone(block_device),
            )
            .lock()
            .modify(0, |bitmap_block: &mut BitMapBlock| {
                if let Some((bits64_pos, inner_pos)) = bitmap_block
                    .iter()
                    .enumerate()
                    .find(|(_, bits64)| **bits64 != u64::MAX)
                    .map(|(bits64_pos, bits64)| (bits64_pos, bits64.trailing_ones() as usize))
                {
                    bitmap_block[bits64_pos] |= 1u64 << inner_pos;
                    Some(block_id * BLOCKS_BITS + bits64_pos * 64 + inner_pos as usize)
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

    pub fn delloc(&self, block_device: &Arc<dyn BlockDevice>, bit: usize) {
        let (block_pos, bits64_pos, inner_pos) = decomposition(bit);
        get_block_cache(block_pos + self.start_block_id, Arc::clone(block_device))
            .lock()
            .modify(0, |bitmap_block: &mut BitMapBlock| {
                assert!(bitmap_block[bits64_pos] & (1u64 << inner_pos) > 0);
                bitmap_block[bits64_pos] -= 1u64 << inner_pos;
            });
    }
}

pub fn decomposition(mut bit: usize) -> (usize, usize, usize) {
    let block_pos = bit / BLOCKS_BITS;
    bit = bit % BLOCKS_BITS;
    (block_pos, bit / 64, bit % 64)
}
