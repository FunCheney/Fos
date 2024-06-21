use core::cell::{RefMut, RefCell};

pub struct UPSafeCell<T>{
    /// inner data
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T>{

}

impl<T> UPSafeCell<T>{
    // new 被申明一个 unsafe 函数，
    pub unsafe fn new(value: T) -> Self {
        Self{
            inner: RefCell::new(value),
        }
    }

    // 调用 exclusive_access 可以得到它包裹的数据的独占访问权
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
