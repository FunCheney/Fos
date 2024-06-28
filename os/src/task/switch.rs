
//! os/src/task/switch.rs
use super::TaskContext;
use core::{arch::global_asm};

core::arch::global_asm!(includer_str!("switch.S"))


extern "C" {
    pub fn _switch (
        current_task_cx_ptr: *mut TaskContext,
        next_task_cx_ptr: *const TaskContext
    )
}
