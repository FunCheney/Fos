mod heap_allocator;

mod address;
mod frame_allocator;
mod memory_set;
mod page_table;

pub use memory_set::KERNEL_SPACE;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

pub fn heap_test() {
    heap_allocator::heap_test();
}
