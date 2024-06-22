/// Drop trait 允许值在离开作用域的时候执行一段小代码，可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源。
/// 指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。
/// Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。

struct CustomSmartPointer {
    data: String,
}

/// Drop trait 包含在 prelude 中，所以需要导入它
/// 在 CustomSmartPointer 上实现了 Drop Trait，并提供了一个调用 println! 的 drop 方法
/// drop 函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[test]
fn test_01() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // 当实例离开作用域 Rust 会自动调用 drop，并调用我们指定的代码。变量以被创建时相反的顺序被丢弃
    println!("CustomSmartPointers created.");
}


#[test]
fn test_02() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // c.drop();
    //    c.drop();
    //    |       ^^^^ explicit destructor calls not allowed
    //    |
    // help: consider using `drop` function
    //    |
    // 37 |     drop(c);
    //    |     +++++ ~
    // 错误信息表明不允许显式调用 drop。错误信息使用了术语 析构函数（destructor）
    /// 在值离开作用域之前调用 std::mem::drop 显式清理
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}