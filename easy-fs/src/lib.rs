//! file system
#![no_std]
#![deny(missing_docs)]
extern crate alloc;
mod bitmap;
mod block_cache;
mod block_dev;
mod efs;
mod layout;
mod vfs;
/// use a block size
pub const BLOCK_SIZE: usize = 512;
use bitmap::BitMap;
use block_cache::{block_cache_sync_call, get_block_cache};
pub use block_dev::BlockDevice;
pub use efs::EasyFileSystem;
use layout::*;
pub use vfs::Inode;
