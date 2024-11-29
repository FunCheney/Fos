use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    time_ready: Condvar,
}

impl<T> Channel<T> {

    pub fn new() -> Channel<T> {
        Channel {
            queue: Mutex::new(VecDeque::new()),
            time_ready: Condvar::new(),
        }
    }

    pub fn send(&self, val: T) {
        self.queue.lock().unwrap().push_back(val);
        self.time_ready.notify_one();
    }

    pub fn recv(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        loop {
            if let Some(val) = queue.pop_front() {
                return val;
            }

            queue = self.time_ready.wait(queue).unwrap();
        }
    }
}