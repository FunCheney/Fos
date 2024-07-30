//! Types related to task manager

use super::TaskContext;
use crate::{
    config::{kernel_stack_position, TRAP_CONTEXT},
    mm::{MapPermission, MemorySet, PhyPageNum, VirtAddr, KERNEL_SPACE}, 
    trap::{trap_handler, TrapContext}};


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

impl TaskControlBlock {
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set.translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap().ppn();

        let task_status = TaskStatus::Ready;
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        KERNEL_SPACE
            .exclusive_access()
            .inset_framed_area(kernel_stack_bottom.into(), 
                               kernel_stack_top.into(),
                               MapPermission::R | MapPermission::W);
        let task_control_block = Self {
            task_status,
            task_cx: TaskContext::goto_trap_return(kernel_stack_top),
            user_time: 0,
            kernel_time: 0,
            memory_set,
            trap_cx_ppn,
            base_size: user_sp,
        };

        let trap_cx = task_control_block.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            kernel_stack_top,
            trap_handler as usize
        );
        task_control_block
    }

    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
}
