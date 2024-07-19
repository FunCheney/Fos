mod heap_allocator;

mod address;
mod frame_allocator;
mod page_table;

pub fn init() {
    heap_allocator::init_heap();
}

pub fn heap_test() {
    heap_allocator::heap_test();
}
