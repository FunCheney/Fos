
#[test]
fn slice_test() {
    /// 切片背景知识：
    ///     切片是一段连续内存的视图；在 Rust 中 用 [T] 来表示，T 表示元素类型
    ///     这个视图可以是连续内存的全部 或 一部分
    ///     切片一般通过切片的引用来访问
    let s = String::from("hello");
    println!("s {}", s);
    let s1 = &s[..];
    println!("s1 {}", s1);
    let s2 = &s1[1..2];
    println!("s2 {}", s2);
}

#[test]
fn slice_to_string() {
    // 这里的 s 指向的是 静态数据区域的引用、还是指向 堆上的引用
    let s: &str = "hello";
    let s1 = s.to_string();
    println!("s1 {}", s1);
    let s2 = String::from(s);
    println!("s2 {}", s2);
    let s3 = s.to_owned();
    println!("s3 {}", s3);
}