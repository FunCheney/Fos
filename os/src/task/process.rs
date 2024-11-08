use super::id::RecycleAllocator;
use super::manager::insert_into_pid2process;
use super::TaskControlBlock;
use super::{add_task, SignalFlags};
use super::{pid_alloc, PidHandle};
use crate::fs::{File, Stdin, Stdout};
use crate::mm::{translated_refmut, MemorySet, KERNEL_SPACE};
use crate::sync::{Condvar, Mutex, Semaphore, UPSafeCell};
use crate::trap::{trap_handler, TrapContext};
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefMut;

pub struct ProcessControlBlock {
    // immutable
    pub pid: PidHandle,
    // mutable
    inner: UPSafeCell<ProcessControlBlockInner>,
}

pub struct ProcessControlBlockInner {
    /// 进程状态是否为僵尸进程
    pub is_zombie: bool,
    /// 进程的地址空间
    pub memory_set: MemorySet,
    /// 指向当前进程的父进程
    pub parent: Option<Weak<ProcessControlBlock>>,
    // 将当前进程的所有子进程的任务控制块，以 Arc 的方式保存在一个向量中
    pub children: Vec<Arc<ProcessControlBlock>>,
    // 当进程调用 exit 系统调用，或者执行出错，由内核终止的时候，保存 exit_code 在
    // 它的任务块中，并等待它的父进程通过 waitpid 的方式回收它的资源，收集它的 pid 以及退出码
    pub exit_code: i32,
    // 文件描述符表
    // 保存了若干实现了 File Trait 的文件，由于采用 Rust 的 Trait Object 动态分发
    // Vec 的动态长度特性使得我们无需设置一个固定的文件描述符数量上限，我们可以更加灵活的使用内存，而不必操心内存管理问题；
    // Option 使得我们可以区分一个文件描述符当前是否空闲，当它是 None 的时候是空闲的，而 Some 则代表它已被占用；
    // Arc 首先提供了共享引用能力。可能会有多个进程共享同一个文件对它进行读写。
    //     此外被它包裹的内容会被放到内核堆而不是栈上，于是它便不需要在编译期有着确定的大小；
    // dyn 关键字表明 Arc 里面的类型实现了 File/Send/Sync 三个 Trait ，但是编译期无法知道它具体是
    // 哪个类型（可能是任何实现了 File Trait 的类型如 Stdin/Stdout ，故而它所占的空间大小自然也无
    // 法确定），需要等到运行时才能知道它的具体类型，对于一些抽象方法的调用也是在那个时候才能找到
    // 该类型实现的方法并跳转过去。
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
    pub signals: SignalFlags,
    /// 进程控制块中设置一个向量保存进程下所有线程的任务控制块
    pub tasks: Vec<Option<Arc<TaskControlBlock>>>,
    /// 进程为进程内的线程分配资源的通用资源分配器
    pub task_res_allocator: RecycleAllocator,
    // 使用 Vec<Option<T>> 构造一个可空槽位且槽位数可以扩展的互斥锁表
    // 表中每个元素都实现了 Mutex Trait, 是一种互斥锁
    pub mutex_list: Vec<Option<Arc<dyn Mutex>>>,
    /// 信号量也是一种进程内的资源
    /// 一个进程内也有多个不同的信号量
    /// 将信号量表加入到进程控制块中
    pub semaphore_list: Vec<Option<Arc<Semaphore>>>,
    /// 条件变量也是一种资源
    pub condvar_list: Vec<Option<Arc<Condvar>>>,
}

impl ProcessControlBlockInner {
    #[allow(unused)]
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }

    pub fn alloc_fd(&mut self) -> usize {
        if let Some(fd) = (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none()) {
            fd
        } else {
            self.fd_table.push(None);
            self.fd_table.len() - 1
        }
    }

    pub fn alloc_tid(&mut self) -> usize {
        self.task_res_allocator.alloc()
    }

    pub fn dealloc_tid(&mut self, tid: usize) {
        self.task_res_allocator.dealloc(tid)
    }

    pub fn thread_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn get_task(&self, tid: usize) -> Arc<TaskControlBlock> {
        self.tasks[tid].as_ref().unwrap().clone()
    }
}

impl ProcessControlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, ProcessControlBlockInner> {
        self.inner.exclusive_access()
    }

    pub fn new(elf_data: &[u8]) -> Arc<Self> {
        // memory_set with elf program headers/trampoline/trap context/user stack
        // 解析传入的 elf 格式数据结构，构造应用的地址空间 memory_set 并获取其他信息
        let (memory_set, ustack_base, entry_point) = MemorySet::from_elf(elf_data);
        // allocate a pid
        // 分配进程id
        let pid_handle = pid_alloc();
        // 创建进程控制块 PCB
        let process = Arc::new(Self {
            pid: pid_handle,
            inner: unsafe {
                UPSafeCell::new(ProcessControlBlockInner {
                    is_zombie: false,
                    memory_set,
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
                    tasks: Vec::new(),
                    task_res_allocator: RecycleAllocator::new(),
                    mutex_list: Vec::new(),
                    semaphore_list: Vec::new(),
                    condvar_list: Vec::new(),
                })
            },
        });
        // create a main thread, we should allocate ustack and trap_cx here
        // 创建主线程的 TaskControlBlock
        let task = Arc::new(TaskControlBlock::new(
            Arc::clone(&process),
            ustack_base,
            true,
        ));
        // prepare trap_cx of main thread
        // 获取所有信息并填充主线程的 Trap 上下文
        let task_inner = task.inner_exclusive_access();
        let trap_cx = task_inner.get_trap_cx();
        let ustack_top = task_inner.res.as_ref().unwrap().ustack_top();
        let kstack_top = task.kstack.get_top();
        drop(task_inner);
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            ustack_top,
            KERNEL_SPACE.exclusive_access().token(),
            kstack_top,
            trap_handler as usize,
        );
        // add main thread to the process
        // 将主线程插入到进程的线程列表中，此时列表为空，可直接插入
        let mut process_inner = process.inner_exclusive_access();
        process_inner.tasks.push(Some(Arc::clone(&task)));
        drop(process_inner);
        // 维护 pid 与 ProcessControlBlock 之间的映射关系
        insert_into_pid2process(process.getpid(), Arc::clone(&process));
        // add main thread to scheduler
        // 将主线程加入到任务管理器使它可以被调度
        add_task(task);
        process
    }

    /// Only support processes with a single thread.
    pub fn exec(self: &Arc<Self>, elf_data: &[u8], args: Vec<String>) {
        assert_eq!(self.inner_exclusive_access().thread_count(), 1);
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, ustack_base, entry_point) = MemorySet::from_elf(elf_data);
        let new_token = memory_set.token();
        // substitute memory_set
        self.inner_exclusive_access().memory_set = memory_set;
        // then we alloc user resource for main thread again
        // since memory_set has been changed
        let task = self.inner_exclusive_access().get_task(0);
        let mut task_inner = task.inner_exclusive_access();
        task_inner.res.as_mut().unwrap().ustack_base = ustack_base;
        task_inner.res.as_mut().unwrap().alloc_user_res();
        task_inner.trap_cx_ppn = task_inner.res.as_mut().unwrap().trap_cx_ppn();
        // push arguments on user stack
        let mut user_sp = task_inner.res.as_mut().unwrap().ustack_top();

        // push arguments on user stack
        // 将命令行参数 压栈
        // 数组中的每个元素都指向一个用户栈更低处的命令行参数字符串的起始地址
        user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
        let argv_base = user_sp;
        // 最开始我们只是分配空间，具体的值要等到字符串被放到用户栈上之后才能确定更新
        let mut argv: Vec<_> = (0..=args.len())
            .map(|arg| {
                translated_refmut(
                    new_token,
                    (argv_base + arg * core::mem::size_of::<usize>()) as *mut usize,
                )
            })
            .collect();

        // 将传入的 args 中的字符串压入到用户栈中
        // 我们在用户栈上预留空间之后逐字节进行复制
        *argv[args.len()] = 0;
        for i in 0..args.len() {
            user_sp -= args[i].len() + 1;
            *argv[i] = user_sp;
            let mut p = user_sp;
            for c in args[i].as_bytes() {
                // translated_str 从应用地址空间取出的，它的末尾不包含 \0 。
                // 为了应用能知道每个字符串的长度，我们需要手动在末尾加入 \0
                *translated_refmut(new_token, p as *mut u8) = *c;
                p += 1;
            }
            *translated_refmut(new_token, p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        // 将 user_sp 以 8 字节对齐
        // 这是因为命令行参数的长度不一，很有可能压入之后 user_sp 没有对齐到 8 字节，
        // 那么在 K210 平台上在访问用户栈的时候就会触发访存不对齐的异常。在 Qemu 平台上则并不存在这个问题
        user_sp -= user_sp % core::mem::size_of::<usize>();
        // initialize trap_cx
        let mut trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            task.kstack.get_top(),
            trap_handler as usize,
        );
        trap_cx.x[10] = args.len();
        trap_cx.x[11] = argv_base;
        *task_inner.get_trap_cx() = trap_cx;
    }

    /// Only support processes with a single thread.
    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        let mut parent = self.inner_exclusive_access();
        assert_eq!(parent.thread_count(), 1);
        // clone parent's memory_set completely including trampoline/ustacks/trap_cxs
        let memory_set = MemorySet::from_existed_user(&parent.memory_set);
        // alloc a pid
        let pid = pid_alloc();
        // copy fd table
        let mut new_fd_table: Vec<Option<Arc<dyn File + Send + Sync>>> = Vec::new();
        for fd in parent.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }
        // create child process pcb
        // 创建子进程的 PCB
        let child = Arc::new(Self {
            pid,
            inner: unsafe {
                UPSafeCell::new(ProcessControlBlockInner {
                    is_zombie: false,
                    memory_set,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    exit_code: 0,
                    fd_table: new_fd_table,
                    signals: SignalFlags::empty(),
                    tasks: Vec::new(),
                    task_res_allocator: RecycleAllocator::new(),
                    mutex_list: Vec::new(),
                    semaphore_list: Vec::new(),
                    condvar_list: Vec::new(),
                })
            },
        });
        // add child
        parent.children.push(Arc::clone(&child));
        // create main thread of child process
        //创建子进程的主线程控制块，注意它继承了父进程的 ustack_base ，
        //并且不用重新分配用户栈和 Trap 上下文。将主线程加入到子进程中
        let task = Arc::new(TaskControlBlock::new(
            Arc::clone(&child),
            parent
                .get_task(0)
                .inner_exclusive_access()
                .res
                .as_ref()
                .unwrap()
                .ustack_base(),
            // here we do not allocate trap_cx or ustack again
            // but mention that we allocate a new kstack here
            false,
        ));
        // attach task to child process
        let mut child_inner = child.inner_exclusive_access();
        child_inner.tasks.push(Some(Arc::clone(&task)));
        drop(child_inner);
        // modify kstack_top in trap_cx of this thread
        let task_inner = task.inner_exclusive_access();
        let trap_cx = task_inner.get_trap_cx();
        //子进程基本上继承父进程的主线程Trap 上下文，但是其中的内核地址需修改
        trap_cx.kernel_sp = task.kstack.get_top();
        drop(task_inner);
        // 添加 pid - pcb 之间映射
        insert_into_pid2process(child.getpid(), Arc::clone(&child));
        // add this thread to scheduler
        // 将子进程的主线程加入到任务调度器
        add_task(task);
        child
    }

    pub fn getpid(&self) -> usize {
        self.pid.0
    }
}
