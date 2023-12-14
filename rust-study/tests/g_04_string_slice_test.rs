
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

#[test]
fn slice_02() {
    // [u8] 是字节串切片，大小是可以动态变化的
    // &[u8] 是对字节串切片的引用, 即 切片引用，与 &str 是类似的
    // &[u8; N] 是对数组 u8 长度为 N  的引用
    // Vec<u8> 是 u8 类型的动态数组, 与 String 类似，这是一种具有所有权的类型

    let a_vec: Vec<u8> = vec![1, 2, 4, 3, 5, 6, 7, 8];

    let a_slice: &[u8] = &a_vec[0..6];
    println!(" a_slice {:?}", a_slice);

    let another_vec = a_slice.to_vec();
    println!(" another {:?}", another_vec);
    let another_vec_2 = a_slice.to_owned();
    println!(" another_vec_2 {:?}", another_vec_2);

}