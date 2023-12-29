use std::collections::HashMap;
use std::mem::size_of;
use std::mem::align_of;
/// enum 是一个标签联合体，它的大小是标签的大小，加上最大类型的长度。
/// Option<T>    Option 是有值、无值 简单类型
///
/// Result<T, E> Result 包括成功返回数据和错误返回数据的枚举类型
///

enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}
// 这是一个声明宏它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4} {:>10} {:>16} {:>22}  {:>28} {:>34}",
            "Type", "T", "align_of(T)", "Option", "align_of(Option)","Result","align_of(Result)"
        );
        println!("{}", "-".repeat(128));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4} {:10} {:16} {:22}  {:28} {:34}",
            stringify!($t),
            size_of::<$t>(),
            align_of::<$t>(),
            size_of::<Option<$t>>(),
            align_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>(),
            align_of::<Result<$t, std::io::Error>>(),
        )
    }
}
// Type                        T Option Result
// ----------------------------------------------------------------
// u8                          1        2           16
// f64                         8       16           16
// &u8                         8        8           16
// &Box<u8>                    8        8           16
// &[u8]                      16       16           16
// String                     24       24           24
// Vec<u8>                    24       24           24
// HashMap<String, String>    48       48           48
// E                          56       56           56
/// Option 配合带有引用类型的数据结构，比如 &u8、Box、Vec、HashMap ，没有额外占用空间。
/// 对于 Option 结构而言，它的 tag 只有两种情况：0 或 1， tag 为 0 时，表示 None，tag 为 1 时，表示 Some。
/// 正常来说，当我们把它和一个引用放在一起时，虽然 tag 只占 1 个 bit，
/// 但 64 位 CPU 下，引用结构的对齐是 8，所以它自己加上额外的 padding，会占据 8 个字节，一共 16 字节，这非常浪费内存。怎么办呢？
/// Rust 是这么处理的，我们知道，引用类型的第一个域是个指针，而指针是不可能等于 0 的，
/// 但是我们可以复用这个指针：当其为 0 时，表示 None，否则是 Some，减少了内存占用，这是个非常巧妙的优化，我们可以学习。

// Vec<T> 是三个 word 的胖指针，一个指向堆内存的指针 pointer、分配的堆内存的容量 capacity，以及数据在堆内存的长度 length
#[test]
fn test_01() {
    show_size!(header);
    show_size!(u8);
    show_size!(u32);
    show_size!(f64);
    show_size!(&u8);
    show_size!(&Box<u8>);
    show_size!(&[u8]);

    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
}