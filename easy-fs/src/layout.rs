/// Magic number for sanity check
const EFS_MAGIC: u32 = 0x3b800001;

#[repr(C)]
pub struct SupperBlock {
    magic: u32,
    pub total_blocks: u32,
    pub inode_bitmap_blocks: u32,
    pub inode_area_blocks: u32,
    pub data_bit_map_blocks: u32,
    pub data_area_blocks: u32,
}

impl SupperBlock {
    pub fn initialize(
        &mut self,
        total_blocks: u32,
        inode_bitmap_blocks: u32,
        inode_area_blocks: u32,
        data_bit_map_blocks: u32,
        data_area_blocks: u32,
    ) {
        *self = self {
            magic: EFS_MAGIC,
            total_blocks,
            inode_bitmap_blocks,
            inode_area_blocks,
            data_bit_map_blocks,
            data_area_blocks,
        }
    }

    pub fn is_validate(&self) -> bool {
        self.magic == EFS_MAGIC
    }
}
