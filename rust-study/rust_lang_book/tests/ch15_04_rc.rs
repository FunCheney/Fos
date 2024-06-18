/// 大部分情况下所有权是非常明确的：可以准确的知道那个变量拥有某个值，然而，某些情况单个值可能会有多个所有者
/// 为了启用多所有权需要显式地使用 Rust 类型 Rc<T>，其为 引用计数（reference counting）的缩写
/// 引用计数意味着记录一个值的引用数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
/// Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。
/// 如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效。

enum List{
    Coin(i32, Box<List>),
    Nil,
}

use std::rc::Rc;
use crate::List::{Coin, Nil};
use crate::List1::{Cons, Nll};

#[test]
fn test_01() {
    let a = Coin(5, Box::new(Coin(10, Box::new(Nil))));
    // Cons 成员拥有其储存的数据，所以当创建 b 列表时，a 被移动进了 b 这样 b 就拥有了 a。
    // 接着当再次尝试使用 a 创建 c 时，这不被允许，因为 a 的所有权已经被移动。
    let b = Coin(4, Box::new(a));
    // let c = Coin(3, Box::new(a));
}

enum List1 {
    // 修改 List 的定义为使用 Rc<T> 代替 Box<T>
    Cons(i32, Rc<List1>),
    Nll,
}

#[test]
fn test_02() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nll)))));
    // 当创建 b 时，不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc<List>，
    // 这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc<List> 中数据的所有权。
    let b = Cons(3, Rc::clone(&a));
    // 创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3。
    // 每次调用 Rc::clone，Rc<List> 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理。
    let c = Cons(4, Rc::clone(&a));
    // 也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。
    // Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
    // Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。
    // 通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。
    // 当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
}

#[test]
fn test_03() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nll)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}