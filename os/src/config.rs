//! const used in os

pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
//pub const MAX_APP_SIZE: usize = 4;
//pub const APP_BASE_ADDRESS: usize = 0x80400000;
//pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

/// 页面内偏移位宽
pub const PAGE_SIZE_BITS: usize = 0xc;
/// 页面大小 4096
pub const PAGE_SIZE: usize = 0x1000;

/// 物理内存的终止物理地址
pub const MEMORY_END: usize = 0x8800_0000;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;

pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE;

pub use crate::board::CLOCK_FREQ;
#[allow(unused)]
pub use crate::board::MMIO;
