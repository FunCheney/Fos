fn foo() -> u32 {
    let i = 100u32;
    i
}

#[test]
fn test_01() {
    let _i = foo();

    // 编译会报错
    // let _i = foo_01();
}
// 这里返回的是引用，函数调用完成后，就被销毁了，堆里的字符串资源也一并被回收了，所以刚刚那段代码当然行不通了
//fn foo_01() -> &u32 {
//    let i = 100u32;
//    &i
//}

struct Point {
    x: u32,
    y: u32,
}

fn foo_02() -> Box<Point> {
    let p = Point { x: 10, y: 20 }; // 这个结构体的实例创建在栈上
    Box::new(p)
}

// 通过 Box::new(p)，把 p 实例强行按位复制了一份，并且放到了堆上，我们记为 p’。
// 然后 foo() 函数返回，把 Box 指针实例 move 给了 _p。之后，_p 拥有了对 p’ 的所有权。
#[test]
fn test_02() {
    let _p = foo_02();
}

// Box<T> 中的所有权分析
// 编译期间已知尺寸的类型实例会默认创建在栈上。Point 有两个字段：x、y，它们的尺寸是固定的，都是 4 个字节，
// 所以 Point 的尺寸就是 8 个字节，它的尺寸也是固定的。所以它的实例会被创建在栈上。
// 第 25 行的 p 拥有这个 Point 实例的所有权。注意 Point 并没有默认实现 Copy，虽然它的尺寸是固定的。
// 在创建 Box 实例的时候会发生所有权转移：资源从栈上 move 到了堆上，原来栈上的那片资源被置为无效状态
// ，因此下面的代码编译不会通过。
fn foo_03() -> Box<Point> {
    let p = Point { x: 19, y: 29 };
    let boxed = Box::new(p);
    // let q = p; // 这里用来检查 p 有没有被 move 走
    boxed
}

#[test]
fn test_03() {
    let _p = foo_03();
}

// 之所以会发生所有权这样的转移，是因为 Point 类型本身就是 move 语义的。作为对照，我们来看一个示例。
fn foo_04() -> Box<u32> {
    let i = 3u32;
    let boxed = Box::new(i);
    let q = i;
    boxed
}

// 在执行 Box::new() 创建 Box 实例时，具有 copy 语义的整数类型和具有 move 语义的 Point 类型行为不一样。
// 整数会 copy 一份自己，Point 实例会把自己 move 到 Box 里面去。
// 一旦创建好 Box 实例后，这个实例就具有了对里面资源的所有权了，它是 move 语义的
#[test]
fn test_04() {
    let _p = foo_04();
}

#[test]
fn test_05() {
    let a = 5;
    let b = Box::new(a);
    println!("{a}");
    println!("{:p}", &b);
    println!("{}", std::mem::size_of_val(&b));
    print!("{}", std::mem::size_of_val(&a));
}
