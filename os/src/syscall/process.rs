//! App management syscalls

// use crate::batch::run_next_app;
use crate::task::exit_current_run_next;
use crate::task::suspend_current_and_run_next;
use crate::timer::get_time_ms;

/// task exits and submit an exit code
#[allow(unused)]
pub fn sys_exit(exit_code: i32) -> ! {
    // run_next_app()
    exit_current_run_next();
    panic!("Unreachable in sys_exit!")
}
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time milliseconds
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}
