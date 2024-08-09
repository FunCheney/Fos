//! Types related to task manager

use super::TaskContext;

/// 保存任务的状态
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    // 任务状态
    pub task_status: TaskStatus,
    // 任务上下文
    pub task_cx: TaskContext,
    // 用户态时间
    pub user_time: usize,
    //  内核态时间
    pub kernel_time: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}
