use super::UPSafeCell;
use crate::task::TaskControlBlock;
use crate::task::{block_current_and_run_next, suspend_current_and_run_next};
use crate::task::{current_task, wakeup_task};
use alloc::{collections::VecDeque, sync::Arc};

pub trait Mutex: Sync + Send {
    /// 加锁
    fn lock(&self);
    /// 释放锁
    fn unlock(&self);
}

/// 基于 yield 机制
pub struct MutexSpin {
    locked: UPSafeCell<bool>,
}

impl MutexSpin {
    pub fn new() -> Self {
        Self {
            locked: unsafe { UPSafeCell::new(false) },
        }
    }
}

impl Mutex for MutexSpin {
    fn lock(&self) {
        loop {
            let mut locked = self.locked.exclusive_access();
            if *locked {
                drop(locked);
                suspend_current_and_run_next();
                continue;
            } else {
                *locked = true;
                return;
            }
        }
    }

    fn unlock(&self) {
        let mut locked = self.locked.exclusive_access();
        *locked = false;
    }
}

/// 基于阻塞机制
pub struct MutexBlocking {
    inner: UPSafeCell<MutexBlockingInner>,
}

pub struct MutexBlockingInner {
    /// 表示是否有线程进入临界区
    /// 线程通过 sys_mutex_lock 系统调用尝试获取锁的时候，发现这个值为 true
    /// 就需要等待该值变为 false，在此之前都需要被阻塞
    locked: bool,
    /// 阻塞（等待）队列，记录blocked 变为false 而被阻塞的线程
    wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl MutexBlocking {
    pub fn new() -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(MutexBlockingInner {
                    locked: false,
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }
}

impl Mutex for MutexBlocking {
    fn lock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        // 首先检测是否已经有线程在临界区
        // 如果为 locked 为 true，将当前线程复制一份加入到阻塞队列中
        if mutex_inner.locked {
            mutex_inner.wait_queue.push_back(current_task().unwrap());
            drop(mutex_inner);
            // 阻塞当前线程
            block_current_and_run_next();
        } else {
            // 当前线程可以进入临界区。
            // locked  改为 true
            mutex_inner.locked = true;
        }
    }

    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        // 假定持有锁
        assert!(mutex_inner.locked);
        // 从阻塞队列中取出一个线程
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            // 如果存在，将该线程唤醒
            wakeup_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }
}
