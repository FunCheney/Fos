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
#[test]
fn test_owner_ship_fun() {
    let s = String::from("hello");  // s 进入作用域

    take_owner_ship(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    // 所以在后面可继续使用 x
}
pub fn take_owner_ship(some_thing :String){ // some_thing 进入作用域
    println!("some_thing {some_thing}"); // 使用
} // 这里 some_string 移出作用域并调用 drop 方法，占用的内存被释放

pub fn makes_copy(some_integer : i32){ // some_integer 进入作用域
    println!("{}", some_integer);
}// 这里，some_integer 移出作用域。没有特殊之处

#[test]
fn test_given_owner_ship() {

    let s1 = gives_ownership();         // gives_ownership 将返回值转移给 s1

    let s2 = String::from("hello");  // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中，它也将返回值移给 s3
}// 这里，s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {   // gives_ownership 会将返回值移动给调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string    // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}

/// 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据的变量离开作用域时，
/// 其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

#[test]
fn test_06() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    // 这一行会报错，s1 的所有权已经被转移
    // println!("s1 {s1}");
    println!("The length of '{}' is {}.", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}