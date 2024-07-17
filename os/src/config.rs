//! const used in os


pub const USER_STACK_SIZE: usize = 4096;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const MAX_APP_SIZE: usize = 4;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

/// 页面内偏移位宽
pub const PAGE_SIZE_BITS: usize = 0xc;
/// 页面大小 4096
pub const PAGE_SIZE: usize = 0x1000;


pub use crate::board::CLOCK_FREQ;

