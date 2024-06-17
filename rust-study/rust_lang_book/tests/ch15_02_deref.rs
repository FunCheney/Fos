use std::fmt::Debug;
use std::ops::Deref;

/// 实现 Deref trait 允许我们重载 解引用运算符 *
/// 通过这种方式实现 Deref trait 的智能指针可以当作常规引用来对待，可以编写操作引用的代码并用于智能指针。

#[test]
fn test_01() {
    // 变量 x 存放了一个 i32 值 5
    let x = 5;
    // y 等于 x 的一个引用
    let y = &x;
    // 可以断言 x 等于 5
    assert_eq!(5, x);
    // 如果希望对 y 的值做出断言，必须使用 *y 来追踪引用所指向的值（也就是 解引用），
    // 这样编译器就可以比较实际的值了
    // assert_eq!(5, y);
}


#[test]
fn test_02() {
    let x = 5;
    let y = Box::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

#[test]
fn test_03() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y);
}

impl <T> Deref  for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_04() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

