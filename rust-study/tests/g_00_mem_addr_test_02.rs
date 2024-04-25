use std::mem::size_of;
use std::mem::align_of;
/// Rust 在内存中排布数据时，会根据每个域的对齐（aligment）对数据进行重排，使其内存大小和访问效率最好。
/// 比如，一个包含 A、B、C 三个域的 struct，它在内存中的布局可能是 A、C、B：
/// 理论上，编译时可以确定大小的值都会放在栈上
// 包括 rust 提供的原生类型，比如字符串、数组、元组、以及开发者自定义的固定大小的结构体（struct）、枚举（enum） 等。
// 如果数据结构的大小无法确定，或者它的大小确定但是在使用时需要更长的生命周期，就最好放在堆上。

/// 内存对齐规则：
/// 首先确定每个域的长度和对齐长度，原始类型的对齐长度和类型的长度一致。
/// 每个域的起始位置要和其对齐长度对齐，如果无法对齐，则添加 padding 直至对齐。
/// 结构体的对齐大小和其最大域的对齐大小相同，而结构体的长度则四舍五入到其对齐的倍数。
struct S1{
    a: u8,
    b: u16,
    c: u8,
}

// align_of::<T>() 返回的是 T 类型的实例在内存中的对齐要求，以字节为单位。
// 这个对齐要求是硬件平台和编译器相关的，通常是类型中最严格对齐字段的对齐要求。
//#[repr()]
struct S2 {
    a: u8,
    c: u8,
    b: u16,
}

#[test]
fn test_01() {
    println!("sizeOf s1: {}, s2:{} ", size_of::<S1>(), size_of::<S2>());
    println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
    println!("sizeOf i32: {}, i32:{} ", size_of::<i32>(), align_of::<i32>());
    println!("sizeOf bool: {}, bool:{} ", size_of::<bool>(), align_of::<bool>());
    println!("sizeOf f64: {}, f64:{} ", size_of::<f64>(), align_of::<f64>());
    println!("sizeOf Sting: {}, String:{} ", size_of::<String>(), align_of::<String>());
}