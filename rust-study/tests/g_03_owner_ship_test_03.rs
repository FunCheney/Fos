/// 多级引用

#[test]
fn test_01() {
    let mut a1 = 10u32;
    let mut b = &mut a1;
    *b = 20;
    let c = &mut b;
    // *c 这里对二级可变引用只使用一级解引用的操作
    // 这里会报错的 中间引用的类型为 &mut u32，而我们却要给它赋值为 u32
    // *c = 30;
    **c = 30;
    print!("{c}");
}
/// 多级可变引用， 要利用可变引用 去修改目标资源的时候。需要做多级引用解引用
/// 只有全是 多级可变引用的情况下，才能修改目标值的资源
/// 对于多级引用（可变& 不可变）打印语句中，可以自动解引用正确的层数，直到访问到目标值的资源
#[test]
fn test_02() {
    let mut a = 10u32;
    // b 是可变引用
    let b = &mut a;
    // c 是对可变引用 的引用
    let mut c = &b;
    // d 是对可变引用的不可变引用的 可变引用
    let d = &mut c;
    // a <-- b <-- c <-- d
    // 这里会报错
    // ***d = 20;
    println!("{d}")

}

// 将字符串的不可变引用 传进 函数参数
fn foo(s: &String) {
    println!("in fn foo: {s}")
}

#[test]
fn test_03() {
    let s1 = String::from("hello rust");
    // 这里传的是字符串的引用 &s1
    foo(&s1);
    // 这里可以打印 s1 的值
    println!("{s1}");
}

// 将字符串的可变引用传进函数参数
fn foo_01(s: &mut String) {
    s.push_str(" study");
}

#[test]
fn test_04() {
    let mut s1 = String::from("hello rust");
    println!("{s1}");
    foo_01(&mut s1); // 这里传入字符串的可变引用
    println!("{s1}");

}