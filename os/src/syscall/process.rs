//! App management syscalls
use log::info;

// use crate::batch::run_next_app;
use crate::task::exit_current_run_next;
use crate::task::suspend_current_and_run_next;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    // run_next_app()
    exit_current_run_next();
    panic!("Unreachable in sys_exit!")
}
pub fn sys_yield() -> isize {
    info!("syscall sys_yield");
    suspend_current_and_run_next();
    0
}
