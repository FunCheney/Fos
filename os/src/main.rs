#![no_main]
#![no_std]
mod lang_items;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#![no_mangle]
fn rust_main() -> !{
    clear_bss();
    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
