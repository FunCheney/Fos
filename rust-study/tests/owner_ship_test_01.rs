
#[test]
fn test_01() {
    let s1 = String::from("hello");
    // 在这一行之后 s2 持有 s1 的所有权
    // s1 处于无效的状态
    let s2 = s1;
    // 花括号结束 s2 及 s2 所拥有的字符串内存，就被回收掉了，
    // s1 所对应的那个局部变量的内存空间也一并被回收了。
    println!("{s2}");
}

#[test]
fn test_02() {
    let s1 = String::from("hello");
    // 这里可以正常打印
    foo(s1);
}


#[test]
fn test_03() {
    let s1 = String::from("hello");
    foo(s1);
    // 由于在 foo 函数中发生了所有权转移，所以这里打印报错
    // println!("{s1}")
}

fn foo(s: String){
    // 这里会获取到字符串 s 的所有权
    println!("{s}")
}

#[test]
fn test_04() {
    let s1 = String::from("hello");
    let s1 = foo_01(s1);
    println!("{s1}")
}

fn foo_01(s: String) -> String {
    println!("{s}");
    // 这里 把所有权转移出来
    s
}
/// 总结：那些变量类型做在赋值的时候做 移动所有权，那些做复制所有权
/// 默认做赋值所有权的类型
///     所有整数类型、布尔类型、浮点类型、字符类型、以上类型组成的元组类型、数组类型；不可变应用类型
///
/// 其他类型默认做 移动所有权操作
///
/// 至于为什么：留以后解答



