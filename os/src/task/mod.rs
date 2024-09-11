//! os/src/task/mod.rs
/// 应用的执行与切换
mod context;
mod switch;

#[allow(clippy::rodule_inception)]
mod task;

use crate::loader::{get_app_data, get_num_app};
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::timer::{get_time_ms, get_time_us};
use crate::trap::TrapContext;
use alloc::vec::Vec;
use lazy_static::*;
use log::{info, warn};
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

pub struct TaskManager {
    // 任务管理器管理的任务数目，TaskManager 初始化之后就不会在变化
    num_app: usize,

    // 包裹在 TaskManagerInner 中的任务控制块数组tasks 以及 正在运行的
    // 任务编号 currnet_task 会在执行应用的过程中发生变化
    // 每个应用的运行状态都会发生变化，而 CPU 执行的应用也在不断切换，
    // 因此需要将 TaskManagerInner 包裹在 UPSafeCell 内以获取内部可变性以及单核上安全的运行时借用检
    // 查能力
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    // 任务列表
    tasks: Vec<TaskControlBlock>,
    // 当前正在运行的任务Id
    current_task: usize,
    // 停表
    stop_watch: usize,
}

impl TaskManagerInner {
    fn refresh_stop_watch(&mut self) -> usize {
        let start_time = self.stop_watch;
        self.stop_watch = get_time_ms();
        self.stop_watch - start_time
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        info!("init TASK_MANAGER");
        // 获取任务数量
        let num_app = get_num_app();
        let mut tasks: Vec<TaskControlBlock> = Vec::new();

        for i in 0..num_app {
            // 为每个应用创建 TCB
            tasks.push(TaskControlBlock::new(
                   // 获取对应程序的 elf 文件
                    get_app_data(i),
                    i
            ));
        }
        warn!("TASK_MANAGER loader apps = {}", num_app);
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                    stop_watch: 0,
                })
            },
        }
    };
}

impl TaskManager {
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        // 统计内核时间
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        // 统计内核时间并输出
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
        println!(
            "[task {} exited. user_time {} ms, kernel_time {} ms]",
            current, inner.tasks[current].user_time, inner.tasks[current].kernel_time
        );
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        // 开始记录时间
        inner.refresh_stop_watch();
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    pub fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            if current != next {
                warn!("[kernel] task switch from {} to {}", current, next);
            }
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            println!("All application completed");
            println!("task switch time: {} us", self::get_switch_time_count());
            shutdown(false);
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    /// 统计内核时间，从现在开始算的是用户时间
    fn user_time_start(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
    }

    /// 统计用户时间，从现在开始算的时内核时间
    fn user_time_end(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].user_time += inner.refresh_stop_watch();
    }

    fn get_current_token(&self) -> usize {
        let inner = self.inner.exclusive_access();
        let currnet = inner.current_task;
        inner.tasks[currnet].get_user_token()
    }

    fn get_current_trap_cx(&self) -> &mut TrapContext {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].get_trap_cx()
    }
}

pub fn current_uset_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}

/// 切换的开始时间
static mut SWITCH_TIME_START: usize = 0;

/// 切换的总时间
static mut SWITCH_TIME_COUNT: usize = 0;

/// 包装 __switch 函数，所有的任务切换都会经过 __switch, 统计它的运行开销
unsafe fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext) {
    // 切换开始时间
    SWITCH_TIME_START = get_time_us();
    crate::task::switch::__switch(current_task_cx_ptr, next_task_cx_ptr);
    // 切换结束总时间
    SWITCH_TIME_COUNT += get_time_us() - SWITCH_TIME_START;
}

/// 获取总的切换时间
fn get_switch_time_count() -> usize {
    unsafe { SWITCH_TIME_COUNT }
}

pub fn exit_current_run_next() {
    // 标记当前任务退出
    mark_current_exited();
    // 运行下一个任务
    run_next_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}
pub fn run_first_task() {
    // 第一次 调用 该方法是，触发 TaskManager 初始化
    TASK_MANAGER.run_first_task();
}
pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

pub fn user_time_start() {
    TASK_MANAGER.user_time_start()
}

pub fn user_time_end() {
    TASK_MANAGER.user_time_end()
}
