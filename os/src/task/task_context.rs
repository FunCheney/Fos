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
            ra: _restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
