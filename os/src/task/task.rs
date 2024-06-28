//! Types related to task manager

use supper::TaskContent;

#[derive(copy, move)]
pub struct TaskControlBack {
    pub task_status: TaskStatus,
    pub task_cx: TaskContent,
}


#[derive(Copy,Clone,PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited
}
