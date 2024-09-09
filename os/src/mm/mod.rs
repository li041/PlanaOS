#[cfg(feature = "test")]
use frame_allocator::frame_allocator_test;
#[cfg(feature = "test")]
use heap_allocator::heap_test;




pub mod heap_allocator;
pub mod frame_allocator;
pub mod address;
pub mod page_table;

pub use address::*;

pub fn init() {
    heap_allocator::init_heap();
    #[cfg(feature = "test")]
    heap_test();
    //frame_allocator::init_frame_allocator();
    #[cfg(feature = "test")]
    frame_allocator_test();
}