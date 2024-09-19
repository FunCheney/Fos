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

/// 退出当前的进程
/// 1. 当前进程从控制块 PROCESSOR 中取出
/// 2. 把 exit_code 写入到进程控制块中
/// 3. 把自己挂到 initproc 的子进程集合值中
/// 4. 释放应用地址空间
/// 5. 接着调度 schedule 来触发函数调度并切换任务
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
    /// 初始化进程管理
    /// 第一个用户进程
    /// 内嵌 initproc 在操作系统中                                            
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(
        // 解析 elf 文件，并建立应用的地址空间，内核栈，形成一个就绪的进程控制块
        TaskControlBlock::new(
            get_app_data_by_name("initproc").unwrap()
        ));
}


pub fn add_initproc() {
    // 添加第一个进程，它是唯一一个不是通过 fork 创建的进程
    // 添加到就绪队列中
    add_task(INITPROC.clone())
}


