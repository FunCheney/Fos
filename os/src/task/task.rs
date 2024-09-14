//! Types related to task manager

use core::cell::RefMut;

use alloc::{rc::Weak, sync::Arc, vec::Vec};

use super::{pid::{pid_alloc, KernelStack, PidHandle}, TaskContext};
use crate::{
    config::{kernel_stack_position, TRAP_CONTEXT},
    mm::{MapPermission, MemorySet, PhysPageNum, VirtAddr, KERNEL_SPACE}, 
    sync::UPSafeCell, 
    trap::{trap_handler, TrapContext}
};

/// 第四部分 TaskControlBlock 备份
pub struct TaskControlBlockBak {
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
    // 应用数据只有可能出现在应用地址空间低于 base_size 的字节区域中
    pub base_size: usize,
    #[allow(unused)]
    pub heap_bottom: usize,
    #[allow(unused)]
    pub program_bak: usize,
}


/// 第五部分: 重构 TaskControlBlock
pub struct TaskControlBlock {
    // 不可变便变量，初始化之后就不再变化的数据
    // 进程标识符
    pub pid: PidHandle,
    // 内核栈
    pub kernel_stack: KernelStack,
    // 可变变量
    // 在运行过程中可能发生变化的数据
    // 包裹在 UPSafeCell 中，在外层只能获取到任务控制块的不可变引用
    // 要想修改里面部分内容，就需要使用 UPSafeCell<T> 提供内部可变性
    inner: UPSafeCell<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
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
    // 应用数据只有可能出现在应用地址空间低于 base_size 的字节区域中
    pub base_size: usize,
    #[allow(unused)]
    pub heap_bottom: usize,
    #[allow(unused)]
    pub program_bak: usize,
    // 指向当前进程的父进程
    pub parent: Option<Weak<TaskControlBlock>>,
    // 将当前进程的所有子进程的任务控制块，以 Arc 的方式保存在一个向量中
    pub children: Vec<Arc<TaskControlBlock>>,
    // 当进程调用 exit 系统调用，或者执行出错，由内核终止的时候，保存 exit_code 在
    // 它的任务块中，并等待它的父进程通过 waitpid 的方式回收它的资源，收集它的 pid 以及退出码
    pub exit_code: i32,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    #[allow(unused)]
    UnInit, // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Zombie,  // 僵尸
    Exited,  // 已退出
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext{
        self.trap_cx_ppn.get_mut()
    }

    pub fn get_user_token(&self) -> usize {
        self.get_user_token()
    }

    fn get_status(&self) -> TaskStatus {
        self.task_status
    }

    pub fn is_zombie(&self) -> bool {
        self.get_status() == TaskStatus.Zombie
    }

}

impl TaskControlBlockBak {
    /// 通过 app_id, app_id 对应的 elf 文件创建 任务控制块
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

impl TaskControlBlock {
     pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }

    pub fn get_pid(&self) -> usize {
        self.pid.0
    }

    pub fn new(elf_data: &[u8]) -> Self {
        // 解析传入的 elf 格式数据结构，构造应用的地址空间 memory_set 并获取其他信息
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        // 从地址空间 memory_set 中查多级页表找到应用地址空间中的 Trap 上下文实际被放在哪个物理页帧
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // alloc a pid and kernel stack in kernel spcae
        let pid_handle = pid_alloc();
        let kernel_stack = KernelStack::new(&pid_handle);
        let kernel_stack_top = kernel_stack.get_top();
        // push a task context which gose to trap_return to the top of kernel stack
        let task_control_block = Self {
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner{
                    task_status: TaskStatus::Ready,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    user_time: 0,
                    kernel_time: 0,
                    memory_set,
                    trap_cx_ppn,
                    base_size: user_sp,
                    heap_bottom: user_sp,
                    program_bak: user_sp,
                    parent: None,
                    children: Vec::new(),
                    exit_code: 0
                })
            },
        };
        // prepare TrapContext in user spcae
        let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            kernel_stack_top, 
            trap_handler as usize,
        );

        task_control_block
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set.translate(VirtAddr::from(TRAP_CONTEXT))
            .into().unwrap().ppn();

        // **** access inner exclusively
        let mut inner = self.inner.exclusive_access();
        inner.memory_set = memory_set;
        inner.trap_cx_ppn = trap_cx_ppn;
        inner.base_size = user_sp;
        let trap_cx = inner.get_trap_cx();

        *trap_cx = TRAP_CONTEXT::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            self.kernel_stack.get_top(),
            trap_handler as usize,
        );

        // **** release inner automatically
    }

    pub fn fork(self: &Arc<Self>) -> &Arc<Self> {
        // --- access parent PCB exclusively
        let mut parent_inner = self.inner_exclusive_access();

        let memory_set = MemorySet::from_existed_user(&parent_inner.memory_set);

        let trap_cx_ppn = memory_set.translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap().ppn();
        
        let pid_handle = pid_alloc();
        let kernel_stack = KernelStack::new(&pid_handle);
        let kernel_stack_top = kernel_stack.get_top();

        let task_control_block = Arc::new(TaskControlBlock{
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner{
                    task_status: TaskStatus::Ready,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    user_time: 0,
                    kernel_time: 0,
                    memory_set,
                    trap_cx_ppn,
                    base_size: parent_inner.base_size,
                    heap_bottom: 0,
                    program_bak: 0,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    exit_code: 0,
                })
            },
        });

        // add children
        parent_inner.children.push(task_control_block.clone());
        // modify kernel_sp in trap_cx
        // **** access children PCB exclusively
        let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
        trap_cx.kernel_sp = kernel_stack_top;
        // return
        task_control_block
        // ---- release parent PCB automatically
        // **** release children PCB automatically
    }

}
