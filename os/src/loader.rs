//! os/src/loader.rs
/// 实现应用的加载
use log::debug;

use crate::config::*;
use crate::trap::TrapContext;
use core::arch::asm;

/// 内核栈
#[repr(align(4096))]
#[derive(Clone, Copy)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

/// 用户栈
#[repr(align(4096))]
#[derive(Clone, Copy)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: [KernelStack; MAX_APP_SIZE] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_SIZE];

static USER_STACK: [UserStack; MAX_APP_SIZE] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_SIZE];

impl KernelStack {
    /// 获取栈顶的位置，在 risc-v 中，栈向下生长
    fn get_sp(&self) -> usize {
        // 返回数结尾的地址
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> usize {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }

        cx_ptr as usize
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

/// load nth user app as
/// APP_BASE_ADDRESS + n * APP_SIZE_LIMIT, APP_BASE_ADDRESS + (n + 1) * APP_SIZE_LIMIT
pub fn load_app() {
    extern "C" {
        fn _num_app();
    }
    debug!("loader app start");
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    unsafe {
        asm!("fence.i");
    }
    // load apps
    for i in 0..num_app {
        let base_i = get_base_i(i);
        debug!("loader app base_i {}", base_i);
        // clear region
        (base_i..base_i + APP_SIZE_LIMIT)
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });

        // load app from data section to memory
        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };

        let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };

        dst.copy_from_slice(src);
    }
}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }

    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

/// 初始化每个任务的内核栈 
pub fn init_app_cx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(TrapContext::app_init_context(
        get_base_i(app_id),
        USER_STACK[app_id].get_sp(),
    ))
}
