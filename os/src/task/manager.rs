use alloc::{collections::vec_deque::VecDeque, sync::Arc};

use super::TaskControlBlock;

use lazy_static::*;
use crate::sync::UPSafeCell;


pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task)
    }

    pub fn fetch(&mut self)-> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}



lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> = 
        unsafe {
            UPSafeCell::new(TaskManager::new())
        };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}
