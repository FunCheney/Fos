use crate::sync::{Mutex, UPSafeCell};
use crate::task::{block_current_and_run_next, current_task, wakeup_task, TaskControlBlock};
use alloc::{collections::VecDeque, sync::Arc};

pub struct Condvar {
    pub inner: UPSafeCell<CondvarInner>,
}

pub struct CondvarInner {
    /// 只有一个阻塞队列
    pub wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl Condvar {
    pub fn new() -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(CondvarInner {
                    // 创建一个空阻塞队列
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }

    pub fn signal(&self) {
        let mut inner = self.inner.exclusive_access();
        // 从阻塞队列中移除一个
        if let Some(task) = inner.wait_queue.pop_front() {
            // 将其唤醒
            wakeup_task(task);
        }
    }

    /// 接收一个当前线程持有的锁作为参数
    pub fn wait(&self, mutex: Arc<dyn Mutex>) {
        // 先释放锁
        mutex.unlock();
        let mut inner = self.inner.exclusive_access();
        // 将当前线程挂在条件阻塞队列中
        inner.wait_queue.push_back(current_task().unwrap());
        drop(inner);
        // 阻塞当前线程
        block_current_and_run_next();
        // 被唤醒之后还需要获取锁，wait 才能返回
        mutex.lock();
    }
}
