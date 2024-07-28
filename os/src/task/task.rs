//! Types related to task manager

use super::TaskContext;
use crate::mm::{MemorySet, PhyPageNum};

pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub user_time: usize,
    pub kernel_time: usize,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhyPageNum,
    pub base_size: usize,

}


#[derive(Copy,Clone,PartialEq)]
pub enum TaskStatus {
    UnInit, // 未初始化
    Ready, // 准备运行
    Running,// 正在运行
    Exited, // 已退出 
}
