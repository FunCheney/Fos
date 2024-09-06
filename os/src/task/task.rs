//! Types related to task manager

use super::TaskContext;
use crate::{
    config::{kernel_stack_position, TRAP_CONTEXT},
    mm::{MapPermission, MemorySet, PhysPageNum, VirtAddr, KERNEL_SPACE},
    trap::{trap_handler, TrapContext},
};

pub struct TaskControlBlock {
    // 任务状态
    pub task_status: TaskStatus,
    // 任务上下文
    pub task_cx: TaskContext,
    // 用户态时间
    pub user_time: usize,
    pub kernel_time: usize,
    // 应用的地址空间
    pub memory_set: MemorySet,
    // 位于应用地址空间次高页的 Trap 上下文被实际存放在物理页帧的物理页号
    pub trap_cx_ppn: PhysPageNum,
    // 统计应用数据的大小，也就是在应用地址空间中从 0x0 开始到用户栈结束一共包含多少字节
    #[allow(unused)]
    pub base_size: usize,
    #[allow(unused)]
    pub heap_bottom: usize,
    #[allow(unused)]
    pub program_bak: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    #[allow(unused)]
    UnInit, // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

impl TaskControlBlock {
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        // 解析传入的 elf 格式数据结构，构造应用的地址空间 memory_set 并获取其他信息
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        // 从地址空间 memory_set 中查多级页表找到应用地址空间中的 Trap 上下文实际被放在哪个物理页帧
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();

        // 设置任务的状态为 Ready
        let task_status = TaskStatus::Ready;
        // 根据传入的应用 ID app_id 调用在 config 子模块中定义的 kernel_stack_position
        // 找到应用的内核栈预计放在内核地址空间 KERNEL_SPACE 中的哪个位置，
        // 并通过 insert_framed_area 实际将这个逻辑段 加入到内核地址空间中；
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        KERNEL_SPACE.exclusive_access().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        let task_control_block = Self {
            // 在应用的内核栈顶压入一个跳转到 trap_return 而不是 __restore 的任务上下文，
            // 这主要是为了能够支持对该应用的启动并顺利切换到用户地址空间执行。
            // 在构造方式上，只是将 ra 寄存器的值设置为 trap_return 的地址。 trap_return 是后面要介绍的新版的 Trap 处理的一部分。
            // 这里对裸指针解引用成立的原因在于：当前已经进入了内核地址空间，而要操作的内核栈也是在内核地址空间中的
            task_status,
            task_cx: TaskContext::goto_trap_return(kernel_stack_top),
            user_time: 0,
            kernel_time: 0,
            memory_set,
            trap_cx_ppn,
            base_size: user_sp,
            heap_bottom: user_sp,
            program_bak: user_sp,
        };

        // 查找该应用的 Trap 上下文的内核虚地址
        let trap_cx = task_control_block.get_trap_cx();
        // 调用 TrapContext::app_init_context 函数，通过应用的 Trap 上下文的可变引用来对其进行初始化
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }

    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    /// 获取页表的起始位置
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }

    #[allow(unused)]
    pub fn change_program_brk(&mut self, size: isize) -> Option<isize> {
        let _old_brk = self.program_bak;
        let new_brk = self.program_bak as isize + size as isize;
        if new_brk < self.heap_bottom as isize {
            return None;
        }
        None
    }
}
