//! os/src/loader.rs 
use core::arch::asm;


const APP_SIZE_LIMIT: usize = 0x20000;


pub fn load_app(){
    extern "C" {
        fn _num_app();
    }

    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe {
        core::slice::from_raw_parts(num_app_ptr.add[1], num_app + 1)
    };
    // load apps 
    for i in 0..num_app {
        let base_i = get_base_i(i);
        // clear region
        (base_i..base_i+APP_SIZE_LIMIT).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(0);
        });
        // load app from data section to memory
        let src = unsafe {
            core::slice::from_raw_partsP(
                app_start[i] as *const u8,
                app_start[i + 1] - app_start[i]
            )
        };

        let dst = unsafe {
            core::slice::from_raw_parts_mut(base_i as *mut u8, src.len())
        };

        dst.copy_from_slice(src);

        unsafe {
            asm!("fence.i")
        }
    }

}
