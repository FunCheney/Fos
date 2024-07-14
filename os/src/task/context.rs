//! implemention of ['TaskContext']

use log::debug;

/// TaskContext
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        debug!("TaskContext zero_init...");
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        debug!("TaskContext goto_restore...");
        extern "C" {
            fn _restore();
        }

        Self {
            // ra 初始化为 _restore 地址
            // 当 __switch 完成后，ret 就能直接进入 trap.S 的 _restore 恢复到用户状态继续执行
            ra: _restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
