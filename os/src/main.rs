#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(negative_impls)]

extern crate alloc;

#[macro_use]
mod console;

mod board;
pub mod config;
mod lang_items;
mod loader;
mod logging;
mod mm;
pub mod mutex;
mod sbi;
mod syscall;
mod task;
mod timer;
mod trap;
mod utils;

use core::{
    arch::{asm, global_asm},
    ffi::c_void,
    ptr,
};

use utils::entry_anim::draw_entry_animation;

use crate::config::KERNEL_BASE;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        ptr::write_bytes(sbss as *mut c_void, 0, ebss as usize - sbss as usize);
    }
}

#[no_mangle]
pub extern "C" fn jump_to_app() {
    unsafe {
        asm!("la t0, app_1_start");
        asm!("jalr zero, 0(t0)");
    }
}

#[no_mangle]
pub fn fake_main(hart_id: usize) {
    unsafe {
        asm!("add sp, sp, {}", in(reg) KERNEL_BASE);
        asm!("la t0, rust_main");
        asm!("add t0, t0, {}", in(reg) KERNEL_BASE);
        asm!("mv a0, {}", in(reg) hart_id);
        asm!("jalr zero, 0(t0)");
    }
}

#[no_mangle]
pub fn rust_main(_hart_id: usize) -> ! {
    clear_bss();
    draw_entry_animation(false);
    logging::init();
    #[cfg(feature = "test")]
    logging::test();
    mm::init();
    loader::list_apps();
    panic!("shutdown machine");
}
