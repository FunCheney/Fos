
/// Rust 的核心语言中只有一种字符串类型：字符串 slice str，它通常以被借用的形式出现，&str。
/// 第四章讲到了 字符串 slices：它们是一些对储存在别处的 UTF-8 编码字符串数据的引用。
/// 举例来说，由于字符串字面值被储存在程序的二进制输出中，因此字符串字面值也是字符串 slices。




#[test]
fn test_01() {
    let mut s = String::from("foo");
    // push_str 方法采用字符串 slice 因为我们并不需要获取参数的所有权
    s.push_str("bar");
    println!("s {s}");
}
