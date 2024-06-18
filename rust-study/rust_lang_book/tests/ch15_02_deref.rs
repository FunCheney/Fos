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
    // 定义了用于此 trait 的关联类型
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
    // *y 在 rust 的底层实际上是 *(y.deref())
    // rust 将 * 运算符替换为先调用 deref 方法，在进行普通的解引用操作
    // deref 方法返回值的引用，以及 *(y.deref()) 括号外面的普通解引用仍为必须的原因是所有权
    // 如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self
    // 在这里以及大部分使用解引用运算符的情况下我们并不希望获取 MyBox<T> 内部值的所有权。
    assert_eq!(5, *y);
}


/// Deref 强制转换
/// Deref 强制转换（deref coercions）将实现了 Deref trait 的类型的引用转换为另一种类型的引用
/// Deref 强制转换可以将 &String 转换为 &str，因为 String 实现了 Deref trait 因此可以返回 &str。


// Deref 强制转换是 Rust 在函数或方法传参上的一种便利操作，并且只能作用于实现了 Deref trait 的类型。
// 当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时将自动进行。
// 这时会有一系列的 deref 方法被调用，把我们提供的类型转换成了参数所需的类型。
fn hello(name: &str){
    println!("Hello, {name}!");
}
#[test]
fn test_05() {
    let m = MyBox::new(String::from("FChen"));
    // 使用字符串 slice 作为参数调用 hello 函数
    // &m 调用 hello 函数，其为 MyBox<String> 值的引用
    // MyBox<T> 上实现了 Deref trait，Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String
    //     标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice
    // Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了。
    hello(&m);
}


/// 如果 Rust 没有实现 Deref 强制转换，为了使用 &MyBox<String> 类型的值调用 hello，要做如下代码编写
#[test]
fn test_06() {
    let m = MyBox::new(String::from("hello"));
    // (*m) 将 MyBox<String> 解引用为 String
    // 接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配 hello 的签名
    hello(&(*m)[..]);
}


