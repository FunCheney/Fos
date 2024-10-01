#![no_std]
#![deny(missing_docs)]
extern crate alloc;

mod block_cache;
mod block_dev;

pub use block_dev::BlockDevice;

pub const BLOCK_SIZE: usize = 512;
