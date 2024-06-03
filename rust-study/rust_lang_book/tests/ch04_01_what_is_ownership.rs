/// 所有权规则
/// 1. Rust 中的每一个值都有一个 所有者（owner）。
/// 2. 值在任一时刻有且只有一个所有者。
/// 3. 当所有者（变量）离开作用域时，这个值将被丢弃

#[test]
fn test_01() {
    // 变量 s 绑定到了一个字符串字面值，这个字符串值是硬编码进程序代码中的。
    // 这个变量从声明的点开始直到当前 作用域 结束时都是有效的。
    let _s = "hello world";
}

#[test]
fn test_02() {
    {
        // s 在这里无效，它尚未声明
        let  _s = "hello"; // 从此处起，s 是有效的
        // 使用 s
    } // 此作用域已结束，s 不再有效
}

/// 字符串字面值，在编译时期就知道内容，所以文本被直接硬编码进最终的可执行文件中。
///     这使得字符串字面值快速且高效。
///     这些特性都只得益于字符串字面值的不可变性
/// 对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容
///     必须在运行时向内存分配器（memory allocator）请求内存。
///     需要一个当我们处理完 String 时将内存返回给分配器的方法。
#[test]
fn test_03() {
    // 使用 from 函数基于字符串字面值来创建 String
    // 当调用 String::from 时，它的实现 (implementation) 请求其所需的内存
    // 在 rust 种，内存在拥有它的变量离开作用域后就被自动释放。
    let s = String::from("hello");
    // 这里不能使用 基于字符串字面值来创建 String
    // s.push(" world");
    println!("s: {s}");
    let mut s1 = String::from("hello");
    s1.push_str("world");
    println!("s1: {s1}");

    {
        let _s2 = String::from("hello"); // 从此处起，s 是有效的
        // 使用 _s2
    } // 作用域结束， _s2 不再有效。这是一个将 String 需要的内存返回给分配器的很自然的位置：
    // 当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。
    // 这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。
}

#[test]
fn test_04_move() {
    // “将 5 绑定到 x；接着生成一个值 x 的拷贝并绑定到 y”。
    // 现在有了两个变量，x 和 y，都等于 5。
    // 这也正是事实上发生了的，因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中。
    let x = 5;
    let y = x;
    println!("x {x}, y {y}");
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("s1: {s1}, s2: {s2}");
    //  let s1 = String::from("hello");
    //    |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    //    |     let s2 = s1;
    //    |              -- value moved here
    //    |     println!("s1: {s1}, s2: {s2}");
    //    |                   ^^^^ value borrowed here after move
}