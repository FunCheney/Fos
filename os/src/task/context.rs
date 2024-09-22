//! implemention of ['TaskContext']

use crate::trap::trap_return;

/// TaskContext
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TaskContext {
    // 记录了 _switch 函数返回之后，程序应该跳到那里执行
    // 从而在任务完成切换并 ret 之后能到正确的位置
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        Self {
            ra: trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
