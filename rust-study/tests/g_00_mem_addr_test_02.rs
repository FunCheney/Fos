use std::mem::size_of;
use std::mem::align_of;
/// Rust 在内存中排布数据时，会根据每个域的对齐（aligment）对数据进行重排，使其内存大小和访问效率最好。
/// 比如，一个包含 A、B、C 三个域的 struct，它在内存中的布局可能是 A、C、B：
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
    b: u64,
    c: u16,
}

#[test]
fn test_01() {
    println!("sizeOf s1: {}, s2:{} ", size_of::<S1>(), size_of::<S2>());
    println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
}