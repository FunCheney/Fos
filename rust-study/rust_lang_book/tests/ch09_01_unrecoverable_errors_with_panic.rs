/// 在实践中有两中方法造成 panic！
/// 执行会造成代码 panic 的操作（比如访问超过数组结尾的内容）或者显式调用 panic!


#[test]
fn test_01() {
    // 调用 panic!
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    // 这里尝试访问 vector 的第一百个元素（这里的索引是 99 因为索引从 0 开始），
    // 不过它只有三个元素。这种情况下 Rust 会 panic
    // v[99];
}

