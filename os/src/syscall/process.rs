//! App management syscalls

use crate::{
    loader::get_app_data_by_name,
    mm::{translated_refmut, translated_str},
    task::{
        add_task, current_task, current_user_token, exit_current_and_run_next,
        suspend_current_and_run_next,
    },
    timer::get_time_ms,
};
use alloc::sync::Arc;

/// task exits and submit an exit code
#[allow(unused)]
pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
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

pub fn sys_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    trap_cx.x[10] = 0;
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(_path: *const u8) -> isize {
    let token = current_user_token();
    let _path = translated_str(token, _path);

    if let Some(data) = get_app_data_by_name(_path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();

    let mut inner = task.inner_exclusive_access();

    if !inner
        .children
        .iter()
        .any(|p| pid == -1 || pid as usize == p.get_pid())
    {
        return -1;
    }

    let pair = inner.children.iter().enumerate().find(|(_, p)| {
        p.inner_exclusive_access().is_zombie() && (pid == -1 || pid as usize == p.get_pid())
    });

    if let Some((idx, _)) = pair {
        let children = inner.children.remove(idx);
        assert_eq!(Arc::strong_count(&children), 1);
        let found_pid = children.get_pid();

        let exit_code = children.inner_exclusive_access().exit_code;
        *translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;

        found_pid as isize
    } else {
        -2
    }
}
