#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
mod console;
mod sbi;
mod lang_items;
mod trap;
mod loader;
mod mm;
mod logging;

pub mod config;


use core::{arch::{global_asm, asm}, ffi::c_void, ptr};


global_asm!(include_str!("entry.S"));
//global_asm!(include_str!("link_app.S"));

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

// #[no_mangle]
// pub extern "C" fn jump_to_app() {
//     unsafe {
//         asm!("la t0, app_0_start");
//         asm!("jalr zero, 0(t0)");
//     }
// }


#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("hello world");
    logging::init();
    #[cfg(feature = "test")]
    logging::test();
    mm::init();
    // loader::list_apps();
    //jump_to_app();
    panic!("shutdown machine");
}

