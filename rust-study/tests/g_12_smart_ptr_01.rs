use std::alloc::{GlobalAlloc, Layout, System};

#[test]
fn test_01() {
    let a = Box::new(1);
    *a;
    println!("{}", a);

    let b = Box::new(String::from("hello"));
    let c = *b;
    println!("{c}");
    //println!("{}", b)
}
// 智能指正和胖指针的区别
// 智能指正对堆上的值有所有权，胖指针没有所有权。
// 在 rust 中凡是要做再远回收的数据结构，且实现了 Deref、DerefMut、Drop 都是智能指针
//
// BOX<T> 他是 rust 基本的在堆上分配内存的方式，绝大多数其他包含内存分配的数据类型，内部都是
// 通过 Box<T> 来完成的，如 Vec<T>
