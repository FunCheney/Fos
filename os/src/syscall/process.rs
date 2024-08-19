//! App management syscalls
use log::debug;
use log::info;

// use crate::batch::run_next_app;
use crate::task::{exit_current_run_next};
use crate::task::get_current_task_info;
use crate::task::suspend_current_and_run_next;
use crate::timer::get_time_ms;
use crate::task::TaskInfo;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] syscall exited with code {}", exit_code);
    // run_next_app()
    exit_current_run_next();
    panic!("Unreachable in sys_exit!")
}
pub fn sys_yield() -> isize {
    debug!("[kernel] syscall sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time milliseconds
pub fn sys_get_time() -> isize {
    debug!("[kernel] syscall get_time_ms");
    get_time_ms() as isize
}

pub fn sys_task_info(ts: *mut TaskInfo) -> isize {
    let task_block = get_current_task_info();
    unsafe {
        *ts = TaskInfo {
            status: task_block.task_status,
            call: task_block.syscall_times,
            time: task_block.kernel_time + task_block.user_time, 
        }
    };

    0
}
