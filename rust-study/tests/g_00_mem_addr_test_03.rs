// 理论上，编译时可以确定大小的值都会放在栈上
// 包括 rust 提供的原生类型，比如字符串、数组、元组、以及开发者自定义的固定大小的结构体（struct）、枚举（enum） 等。
// 如果数据结构的大小无法确定，或者它的大小确定但是在使用时需要更长的生命周期，就最好放在堆上。

use std::collections::HashMap;
use std::mem::size_of;

/// enum 是一个标签联合体，它的大小是标签的大小，加上最大类型的长度。

enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}
// 这是一个声明宏它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4} {} {}",
            "Type", "T", "Option", "Result"
        );
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4} {:8} {:12}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>()
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

// Vec<T> 是三个 word 的胖指针，一个指向堆内存的指针 pointer、分配的堆内存的容量 capacity，以及数据在堆内存的长度 length
#[test]
fn test_01() {
    show_size!(header);
    show_size!(u8);
    show_size!(f64);
    show_size!(&u8);
    show_size!(&Box<u8>);
    show_size!(&[u8]);

    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
}