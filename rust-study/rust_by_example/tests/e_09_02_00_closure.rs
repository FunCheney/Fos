

#[test]
fn test_01() {
    let i = 0;
    // 使用函数实现自增
    fn function(i: i32) -> i32 { i + 1}
    println!("function: {}", function(i));
    // 使用闭包实现自增方式1
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    println!("1st closing annotated: {}", closure_annotated(i));
    // 方式 2
    let closure_inferred = |i| i + 1;
    println!("2nd closing inferred: {}", closure_inferred(i));
}

