//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

use address::VPNRange;
pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{kernel_token, MapPermission, MemorySet, KERNEL_SPACE};
use page_table::PTEFlags;

#[allow(unused)]
pub use page_table::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, PageTable,
    PageTableEntry, UserBuffer, UserBufferIterator,
};

/// initiate heap allocator, frame allocator and kernel space
/// 内存管理子系统的初始化
pub fn init() {
    // 初始化全局动态内存分配器
    heap_allocator::init_heap();
    // 初始化物理页帧管理器
    frame_allocator::init_frame_allocator();
    // 创建内核地址，并让 CPU 开启分页模式
    // 首先，我们引用 KERNEL_SPACE ，这是它第一次被使用，就在此时它会被初始化，
    // 调用 MemorySet::new_kernel 创建一个内核地址空间并使用 Arc<Mutex<T>> 包裹起来；
    // 接着使用 .exclusive_access() 获取一个可变引用 &mut MemorySet 。
    // 需要注意的是这里发生了两次隐式类型转换：我们知道 exclusive_access 是 UPSafeCell<T> 的方法而不是 Arc<T> 的方法，
    // 由于 Arc<T> 实现了 Deref Trait ， 当 exclusive_access 需要一个 &UPSafeCell<T> 类型的参数的时候，
    // 编译器会自动将传入的 Arc<UPSafeCell<T>> 转换为 &UPSafeCell<T> 这样就实现了类型匹配；
    // 事实上 UPSafeCell<T>::exclusive_access 返回的是一个 RefMut<'_, T> ，这同样是 RAII 的思想，当这个类型生命周期结束后互斥锁就会被释放。
    // 而该类型实现了 DerefMut Trait，因此当一个函数接受类型为 &mut T 的参数却被传入一个类型为 &mut RefMut<'_, T> 的参数的时候，编译器会自动进行类型转换使参数匹配。
    // 最后，我们调用 MemorySet::activate
    KERNEL_SPACE.exclusive_access().activate();
}
