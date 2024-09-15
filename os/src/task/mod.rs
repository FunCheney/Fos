//! os/src/task/mod.rs
/// 应用的执行与切换
mod context;
mod switch;
mod pid;
mod manager;
mod processor;

#[allow(clippy::rodule_inception)]
mod task;

use crate::loader:: get_app_data_by_name;
use crate::sbi::shutdown;
use alloc::sync::Arc;
use lazy_static::*;
pub use  manager::{add_task, fetch_task};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};
pub use context::TaskContext;
pub use processor::{
    current_task,current_trap_cx,current_user_token,
    run_tasks,schedule,take_current_task,
};

pub fn suspend_current_and_run_next() {
    let task = take_current_task().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    add_task(task);
    schedule(task_cx_ptr);
}

pub const IDEL_PID: usize = 0;


pub fn exit_current_and_run_next(exit_code: i32) {

    let task = take_current_task().unwrap();
    let pid = task.get_pid();

    if pid == IDEL_PID {
        
        println!(
            "kernel Idle processor exit with exit_code {}", exit_code
        );

        if exit_code != 0 {
            shutdown(true)
        }else {
            shutdown(false)
        }
    }

    let mut inner = task.inner_exclusive_access();
    inner.task_status = TaskStatus::Zombie;
    inner.exit_code = exit_code;

    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for children in inner.children.iter()  {
            children.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(children.clone());
        }
    }

    inner.children.clear();
    inner.memory_set.recycle_data_pages();
    drop(inner);
    drop(task);
    let mut unused = TaskContext::zero_init();
    schedule(&mut unused as *mut _);
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
            get_app_data_by_name("initproc").unwrap()
        ));
}


pub fn add_initproc() {
    add_task(INITPROC.clone())
}


