/// 枚举是一种类型，它容纳选项的可能性，每一个可能得选项都是一个变体
/// 在 Rust 中枚举的条目被叫做这个枚举的变体
/// 枚举与结构体不同，结构体的实例化需要所有的字段一起起作用，枚举的实例化只需要而且只能是其中一个变体起作用
enum Shape {
    Rectangle,
    Triangle,
    Circle
}

/// 负载
/// enum 中的变体可以挂载各种形式的类型
/// 所有其他类型，比如字符串，元组，结构体等都可以作为枚举的负载，挂载到日中一个变体上
enum Shape_01 {
    // 挂载结构体负载，表示宽、高 的属性
    Rectangle {width: u32, height: u32},
    // 变体挂载了一个元组负载 ((u32, u32), (u32, u32), (u32, u32))，表示三个顶点
    Triangle ((u32, u32), (u32, u32), (u32, u32)),
    // 变体挂载了一个结构体负载 { origin: (u32, u32), radius: u32 }，表示一个原点加半径长度。
    Circle {origin:(u32, u32), radius:(u32)},
}

enum WebEvent {
    PageLoad,
    PageUnLoad,
    KeyPass(char),
    Paste(String),
    Click{x: u64, y: u64},
}

/// 枚举实例化
#[test]
fn test_01() {
    let a = WebEvent::PageLoad;
    let b = WebEvent::PageUnLoad;
    let c = WebEvent::KeyPass('v');
    let d = WebEvent::Paste(String::from("hello"));
    let e = WebEvent::Click {x: 1, y:2};
}

/// 类 c 枚举符
// 给枚举值一个初始变量
enum Number {
    Zero = 0,
    One,
    Two,
}
// 给枚举值的每个变量赋不同的值

enum Color {
    Read = 0xfff000,
    Green = 0xffff12,
    Blue = 0x00ff00,
}

#[test]
fn test_02() {
    // 用 as 进行类型的转化
    println!("zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);
    // 代码中的 println! 里的 {:06x} 是格式化参数，这里表示打印出值的 16 进制形式，占位 6 个宽度，不足的用 0 补齐
    println!("rose are #{:06x}", Color::Read as i32);
    println!("rose are #{:06x}", Color::Green as i32);
}

/// 空枚举
/// 它与单元结构体一样，表示一个类型
/// 但是它不能被初始化
enum Foo {}

#[test]
fn test_03() {
    //
    let a = Foo {};
}
