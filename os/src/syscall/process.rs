//! App management syscalls
use crate::batch::run_next_app;
use crate::task::suspend_current_and_run_next;
use crate::task::exit_current_run_next;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    // run_next_app()
    exit_current_run_next()
}
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}
