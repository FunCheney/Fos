//! Types related to task manager

use supper::TaskContent;

#[derive(copy, move)]
pub struct TaskControlBack {
    pub task_status: TaskStatus,
    pub task_cx: TaskContent,
}


#[derive(Copy,Clone,PartialEq)]
pub enum TaskStatus {
    UnInit, // 未初始化
    Ready, // 准备运行
    Running,// 正在运行
    Exited, // 已退出 
}
