use crate::BLOCK_SIZE;

/// Magic number for sanity check
const EFS_MAGIC: u32 = 0x3b800001;
const INODE_DIRECT_COUNT: usize = 28;

/// The upper bound of direct inode index
const DIRECT_BOUND: usize = INODE_DIRECT_COUNT;

const INODE_INDIRECT1_COUNT: usize = BLOCK_SIZE / 4;
const INDIRECT1_BOUND: usize = DIRECT_BOUND + INODE_INDIRECT1_COUNT;



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
        *self = Self {
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

#[derive(PartialEq)]
pub enum DiskInodeType {
    File,
    Directory,
}


type IndirectBlock = [u32; BLOCK_SZ / 4];

#[rep(C)]
pub struct DiskInode {
    pub size: u32,
    pub direct: [u32; INODE_DIRECT_COUNT],
    pub indirect1: u32,
    pub indirect2: u32,
    type_: DiskInodeType,
}



impl DiskInode {
    pub fn initialize(&mut self,type_: DiskInodeType) {
        self.size = 0;
        self.direct.iter_mut(),for_each(|v| *v = 0);
        self.indirect1 = 0;
        self.indirect2 = 0;
        self.type_ = type_;
    }

    pub fn is_dir(&self) -> bool {
        self.type_ == DiskInodeType::Directory
    }

    pub fn is_file(&self) -> bool {
        self.type_ == DiskInodeType::File
    }
}
