/// 引用
/// 引用也是一种值，是固定尺寸的值
/// 1. 所有权变量的作用域：从定义开始直到所属那层花括号结束
/// 2. 引用类型的作用域：从定义开始直到最后一次使用结束
/// 3. 引用类型变脸的作用域不会长于 所有权类型变量的作用域
/// 4. 一个所有权型变量的不可变引用可以同时存在多个，可以复制多份。
/// 5. 一个所有权型变量的可变引用与不可变引用的作用域不能交叠，也可以说不能同时存在。
/// 6. 某个时刻对某个所有权型变量只能存在一个可变引用，不能有超过一个可变借用同时存在，也可以说，对同一个所有权型变量的可变借用之间的作用域不能交叠。
/// 7. 在有借用存在的情况下，不能通过原所有权型变量对值进行更新。当借用完成后（借用的作用域结束后），物归原主，又可以使用所有权型变量对值做更新操作了。
#[test]
fn test_01() {
    let a: u32 = 10;
    // b 是 a 的一级引用
    let b = &a;
    // c 是变量 a 的多级引用
    let c = &&&&&&a;
    // d 是遍两 a 的多级引用
    let d = &b;
    // 引用b再赋值给e
    let e = d;
    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
    println!("{e}");
}

#[test]
fn test_02() {
    let s1 = String::from("I am a superman.");
    let s2 = &s1;
    let s3 = &&&&&s1;
    let s4 = &s2;
    let s5 = s2;
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s4}");
    println!("{s5}");
}

#[test]
fn test_03() {
    // 不可变引用
    // 如果要对一个变量内容进行修改，必须要拥有改变量的所有权
    // 很多时候，我们没有办法拥有那个资源的所有权，比如你引用一个别人的库，它没有把所有权类型暴露出来，
    // 但是确实又有更新其内部状态的需求。
    // 因此需要一个东西，它既是引用，又能修改指向资源的内容，于是就引入了可变引用。
    // case 1
    // let a: u32 = 10;
    // let b = &mut a;  这里 a 必须是 可变类型，如 case 2

    // case 2
    let mut a: u32 = 10;
    let b = &mut a;
    *b = 20;
    println!("{b}");
}
#[test]
fn test_04() {
    let mut a: u32 = 10;
    let b = &mut a;
    *b = 39;
    println!("{b}");
    println!("{a}");
}

#[test]
fn test_05() {
    let mut a: u32 = 10;
    let b = &mut a;
    *b = 39;
    // println!("{a}"); // 这里先答应 a 变量会报错
    /// 打印语句 println！ 不管传入所有权变量类型还是引用类型都能正确打印出 预期1
    /// 实际上 println! 中默认会对所有权变量做不可变借用操作
    println!("{b}");
}

#[test]
fn test_06() {
    let mut a: u32 = 10;
    // ①：可变借用发生在这里
    let b = &mut a;
    *b = 20;
    // 利用 b 更新了 a 的之后, c 再次借用 a
    // ②：不可变借用发生在这里
    let c = &a;
    // ③：这里打印会报错，原因是使用了可变借用
    // println!("{b}");
    println!("{c}");
}

#[test]
fn test_07() {
    let mut a = 10u32;
    // 不可变借用
    let c = &a; // c的定义移到这里来了
                // 可变借用
    let b = &mut a;
    *b = 20;
    // 使用了 不可变借用 故 报错了
    // println!("{c}");
}

#[test]
fn test_08() {
    let mut a = 10u32;
    let c = &a; // c的定义移到这里来了
    let b = &mut a;
    *b = 20;
    println!("{b}"); // 这里打印的变量换成b
}

/// 同一个所有权变量的 可变类型与不可变类型之间的作用域不能交叠
#[test]
fn test_09() {
    let mut a: u32 = 10;
    let b = &mut a;
    *b = 39;
    // let d = &mut a;
    println!("{b}");
}
/// 在有借用的情况下，不能对所有权变量进行更改值的操作
#[test]
fn test_10() {
    let mut a: u32 = 10;
    let r1 = &mut a;
    a = 20;
    println!("{r1}");
}
