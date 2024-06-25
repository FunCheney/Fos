use core::cell::{RefMut, RefCell};

/// 允许在单核上安全的使用可变全局变量
/// 和 RefCell 一样提供内部可变性和借用时检查
pub struct UPSafeCell<T>{
    /// inner data
    /// 对 RefCell 进行封装
    inner: RefCell<T>,
}

/// unsafe 标记 Sync 全局变量
unsafe impl<T> Sync for UPSafeCell<T>{

}

impl<T> UPSafeCell<T>{
    // new 被申明一个 unsafe 函数，使用者在创建一个 UPSafeCell 的时候保证
    // 访问 UPSafeCell 包裹的数据时，访问之前调用 exclusive_access，访问之后
    // 销毁借用标记再进行下一次访问。只能依靠调用者自己保证。
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
