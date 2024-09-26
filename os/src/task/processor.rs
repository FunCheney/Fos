use super::{fetch_task, TaskStatus};
use super::{TaskContext, TaskControlBlock, __switch};
use crate::{sync::UPSafeCell, trap::TrapContext};
use alloc::sync::Arc;
use lazy_static::*;

/// 处理器管理结构，描述 CPU 执行状态
pub struct Processor {
    // 在当前处理器上正在执行的任务
    current: Option<Arc<TaskControlBlock>>,
    // 空闲任务
    idle_task_cx: TaskContext,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }

    // 获取空闲任务
    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }

    // 获取当前进程 TCB
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
}

// 初始化一个 PROCESSOR 第一此调用的时候会被加载
lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

pub fn run_tasks() {
    // 循环
    loop {
        // 在这里完成 PROCESSOR 的 初始化
        let mut processor = PROCESSOR.exclusive_access();
        // 选择一个用来切换的进程
        // 这里第一个进程是 initproc 程序，
        // 在切换到改进程执行的时候，会做一些其他操作，具体实现
        // user/src/bin 目录下
        if let Some(task) = fetch_task() {
            // 获取空闲的进程
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();

            let mut task_inner = task.inner_exclusive_access();

            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            // 修改进程状态
            task_inner.task_status = TaskStatus::Running;
            drop(task_inner);
            processor.current = Some(task);
            drop(processor);

            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        }
    }
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().current()
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.inner_exclusive_access().get_user_token();
    token
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}

/// 进程调度的方法
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.exclusive_access();
    let idle_task_cx = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx);
    }
}
