//! os/src/task/mod.rs

mod context;
mod switch;

use task::{TaskControlBlock, TaskStatus}

pub struct TaskManager{
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks:[TaskControlBlock; MAX_APP_NUM],
    currnet_task: usize,
}
