use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

/// 内部可变性 （Interior mutability）是 rust 的一种设计模式，它允许你在有不可变引用时也可以改变数据，这通常是借用规则不允许的。
/// 为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则
/// 不安全代码表明，我们在手动检查这些规则，而不是让编译器帮我们检查

/// 当可以确保代码在运行时会遵守借用规则，即使编译器不能保证的情况，可以选择使用那些运用内部可变性模式的类型。
/// 所涉及的 unsafe 代码将被封装进安全的 API 中，而外部类型仍然是不可变的。

/// 通过 RefCell<T> 在运行时检查借用规则
/*
借用规则：
不同于 Rc<T>，RefCell<T> 代表其数据的唯一的所有权。那么是什么让 RefCell<T> 不同于像 Box<T> 这样的类型呢？
    1. 在任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用 之一（而不是两者）。
    2. 引用必须总是有效的。
    对于引用和 Box<T>，借用规则的不可变性作用于 ⚠️ 编译时
    对于 RefCell<T>，这些不可变性作用于 ⚠️ 运行时

    在编译时检查借用规则的优势是这些错误将在开发过程的早期被捕获，同时对运行时没有性能影响，因为所有的分析都提前完成了。
    为此，在编译时检查借用规则是大部分情况的最佳选择，这也正是其为何是 Rust 的默认行为。
    相反在运行时检查借用规则的好处则是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的。
*/

/// 如果 Rust 编译器不能通过所有权规则编译，它可能会拒绝一个正确的程序；从这种角度考虑它是保守的。
/// 如果 Rust 接受不正确的程序，那么用户也就不会相信 Rust 所做的保证了。
/// 然而，如果 Rust 拒绝正确的程序，虽然会给程序员带来不便，但不会带来灾难。
/// RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。

/// 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
///     Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
///     Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
///     因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
///
/*
    RefCell<T> 是一个获得内部可变性的方法。RefCell<T> 并没有完全绕开借用规则，
    编译器中的借用检查器允许内部可变性并相应地在运行时检查借用规则。如果违反了这些规则，会出现 panic 而不是编译错误。
 */

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    struct MockMessenger {
        // 这中写法编译不通过
        // sent_message: Vec<String>,
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_message.borrow_mut().push(String::from(msg));
        }

    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_message.borrow().len(), 1);
    }
}

/// 当我们创建不可变和可变引用时分别采用 &, &mut
/// 对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法
/// borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut<T> 类型的智能指针。
/// 这两个类型都实现了 Deref，所以可以当作常规引用对待。
// RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。
// 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。
// 当 Ref<T> 值离开作用域时，不可变借用计数减一。
// 就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[test]
fn test_03() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
