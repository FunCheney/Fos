use alloc::sync::Arc;

use crate::{bitmap::BitMap, BlockDevice};

pub struct EasyFileSystem {
    pub block_device: Arc<dyn BlockDevice>,
    pub innode_bitmap: BitMap,
    pub data_bitmap: BitMap,
    innode_area_start_block: u32,
    data_area_start_block: u32,
}
