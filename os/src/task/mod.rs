//! os/src/task/mod.rs
/// 应用的执行与切换
mod switch;
#[allow(clippy::rodule_inception)]
mod task;
mod task_context;

use crate::sync::UPSafeCell;
use lazy_static::*;
use log::{debug, info};
use switch::_switch;
use task::{TaskControlBlock, TaskStatus};

use crate::config::MAX_APP_SIZE;
use crate::loader::{get_num_app, init_app_cx};
pub use task_context::TaskContext;
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
    tasks: [TaskControlBlock; MAX_APP_SIZE],
    // 当前正在运行的任务Id
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        debug!("TaskManager init get user apps {}", num_app);
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_SIZE];

        for (i, task) in tasks.iter_mut().enumerate().take(num_app) {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_status = TaskStatus::Ready;
        }

        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

impl TaskManager {
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        debug!("run_first_task task0");
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            debug!("_switch next_task_cx_ptr start");
            _switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
            info!("_switch next_task_cx_ptr end");
        }

        panic!("unreachable in run_first_task!");
    }

    pub fn run_next_task(&self) {
        debug!("run_next_task  into...");
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            unsafe {
                debug!("_switch run_next_task cur next ");
                _switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            panic!("All application completed");
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }
}

pub fn exit_current_run_next() {
    TASK_MANAGER.mark_current_exited();
    run_next_task();
}

pub fn suspend_current_and_run_next() {
    info!("task mod call suspend_current_and_run_next");
    TASK_MANAGER.mark_current_suspended();
    run_next_task();
}
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}
pub fn run_next_task() {
    info!("task mod call run_next_task");
    TASK_MANAGER.run_next_task();
}
