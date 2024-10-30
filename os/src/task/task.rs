//! Types related to task manager

use super::{
    pid::{pid_alloc, KernelStack, PidHandle},
    SignalActions, SignalFlags, TaskContext,
};
use crate::fs::{File, Stdin, Stdout};
use crate::{
    config::TRAP_CONTEXT,
    mm::{translated_refmut, MemorySet, PhysPageNum, VirtAddr, KERNEL_SPACE},
    sync::UPSafeCell,
    trap::{trap_handler, TrapContext},
};
use alloc::string::String;
use alloc::vec;
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};
use core::cell::RefMut;

/// 第五部分: 重构 TaskControlBlock
pub struct TaskControlBlock {
    // 不可变便变量，初始化之后就不再变化的数据
    // 进程标识符, 进程创建完成之后会有一个自己的标识符
    pub pid: PidHandle,
    // 进程内核栈
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
    #[allow(unused)]
    pub user_time: usize,
    #[allow(unused)]
    pub kernel_time: usize,
    // 拥有独立的页表
    // 应用的地址空间 （虚拟地址空间）
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
    // 文件描述符表
    // 保存了若干实现了 File Trait 的文件，由于采用 Rust 的 Trait Object 动态分发
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,

    pub signals: SignalFlags,
    pub signal_mask: SignalFlags,
    // the signal which is being handling
    pub handling_sig: isize,
    // Signal actions
    pub signal_actions: SignalActions,
    // if the task is killed
    pub killed: bool,
    // if the task is frozen by a signal
    pub frozen: bool,

    pub trap_ctx_backup: Option<TrapContext>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    #[allow(unused)]
    UnInit, // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Zombie,  // 僵尸
    #[allow(unused)]
    Exited, // 已退出
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
    pub fn is_zombie(&self) -> bool {
        self.get_status() == TaskStatus::Zombie
    }

    /// 在进程控制块中分配一个最小的空闲文件描述符来访问一个新打开的文件夹。
    pub fn alloc_fd(&mut self) -> usize {
        // 从小到大遍历所有曾经分配过的文件描述符尝试找到一个空闲的
        if let Some(fd) = (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none()) {
            fd
        } else {
            // 没有找到，拓展文件描述符表的长度并新分配一个
            self.fd_table.push(None);
            self.fd_table.len() - 1
        }
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
                UPSafeCell::new(TaskControlBlockInner {
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
                    exit_code: 0,
                    // 当一个进程被创建的时候，内核会默认为其打开三个缺省就存在的文件：
                    fd_table: vec![
                        // 0 -> stdin 文件描述符 0； 标准输入
                        Some(Arc::new(Stdin)),
                        // 1 -> stdout 文件描述符 1: 标准输出
                        Some(Arc::new(Stdout)),
                        // 2 -> stderr 文件描述符 2: 标准错误的输出
                        Some(Arc::new(Stdout)),
                    ],
                    signals: SignalFlags::empty(),
                    signal_mask: SignalFlags::empty(),
                    handling_sig: -1,
                    signal_actions: SignalActions::default(),
                    killed: false,
                    frozen: false,
                    trap_ctx_backup: None,
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

    pub fn exec(&self, elf_data: &[u8], args: Vec<String>) {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, mut user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // push arguments on user stack
        // 将命令行参数 压栈
        // 数组中的每个元素都指向一个用户栈更低处的命令行参数字符串的起始地址
        user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
        let argv_base = user_sp;
        // 最开始我们只是分配空间，具体的值要等到字符串被放到用户栈上之后才能确定更新
        let mut argv: Vec<_> = (0..=args.len())
            .map(|arg| {
                translated_refmut(
                    memory_set.token(),
                    (argv_base + arg * core::mem::size_of::<usize>()) as *mut usize,
                )
            })
            .collect();
        *argv[args.len()] = 0;
        // 将传入的 args 中的字符串压入到用户栈中
        // 我们在用户栈上预留空间之后逐字节进行复制
        for i in 0..args.len() {
            user_sp -= args[i].len() + 1;
            *argv[i] = user_sp;
            let mut p = user_sp;
            for c in args[i].as_bytes() {
                // translated_str 从应用地址空间取出的，它的末尾不包含 \0 。
                // 为了应用能知道每个字符串的长度，我们需要手动在末尾加入 \0
                *translated_refmut(memory_set.token(), p as *mut u8) = *c;
                p += 1;
            }
            *translated_refmut(memory_set.token(), p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        // 将 user_sp 以 8 字节对齐
        // 这是因为命令行参数的长度不一，很有可能压入之后 user_sp 没有对齐到 8 字节，
        // 那么在 K210 平台上在访问用户栈的时候就会触发访存不对齐的异常。在 Qemu 平台上则并不存在这个问题
        user_sp -= user_sp % core::mem::size_of::<usize>();

        // **** access current TCB exclusively
        let mut inner = self.inner_exclusive_access();
        // substitute memory_set
        inner.memory_set = memory_set;
        // update trap_cx ppn
        inner.trap_cx_ppn = trap_cx_ppn;
        // initialize trap_cx
        let mut trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            self.kernel_stack.get_top(),
            trap_handler as usize,
        );
        trap_cx.x[10] = args.len();
        trap_cx.x[11] = argv_base;
        *inner.get_trap_cx() = trap_cx;
        // **** release current PCB
    }

    /// TCB 的构建过程，复制父进程的内容，并构造新的进程控制块
    /// 1. 建立新页表
    /// 2. 创建新的陷入上下文
    /// 3, 创建新的应用内核栈
    /// 4. 创建任务上下文
    /// 5. 建立父子关系
    /// 6. 设置 0 为 fork 返回码
    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        // --- access parent PCB exclusively
        let mut parent_inner = self.inner_exclusive_access();
        // 复制一份地址空间
        let memory_set = MemorySet::from_existed_user(&parent_inner.memory_set);
        // 分配 物理页
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();

        let pid_handle = pid_alloc();
        let kernel_stack = KernelStack::new(&pid_handle);
        let kernel_stack_top = kernel_stack.get_top();
        let mut new_fd_table: Vec<Option<Arc<dyn File + Send + Sync>>> = Vec::new();
        // 子进程需要完全继承父进程的文件描述符表来和父进程共享所有文件
        for fd in parent_inner.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }
        let task_control_block = Arc::new(TaskControlBlock {
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
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
                    fd_table: new_fd_table,
                    signals: SignalFlags::empty(),
                    // inherit the signal_mask and signal_action
                    signal_mask: parent_inner.signal_mask,
                    handling_sig: -1,
                    signal_actions: parent_inner.signal_actions.clone(),
                    killed: false,
                    frozen: false,
                    trap_ctx_backup: None,
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
