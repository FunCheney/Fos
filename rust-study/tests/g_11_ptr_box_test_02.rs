/// Box<T> 解引用
#[test]
fn test_01() {
    let val: u8 = 4;
    let boxed: Box<u8> = Box::new(val); // 这里 boxed 里面那个u8就是堆上的值
    println!("{}", boxed);
    // 解引用是 Box::new() 的逆操作，可以看到整个过程是相反的。
    let val = *boxed;
    println!("{}", val);
}

/// 对于具有 move 语义的类型来说，情况就不一样了
#[derive(Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[test]
fn test_02() {
    let p = Point {x: 10, y: 20};
    let boxed: Box<Point> = Box::new(p);
    let val = *boxed;  // 这里做了解引用，Point实例回到栈上
    println!("{:?}", val);
    // println!("{:?}", boxed); // 解引用后想把boxed再打印出来
}

/// Box<T> 实现了 trait
// Box<T> 的好处在于它的明确性，它里面的资源一定在堆上，所以我们就不用再去关心资源是在栈上还是堆上这种细节问题了。
// 一种类型，被 Box<> 包起来的过程就叫作这个类型的盒化（boxed）。
// Rust 在标准库里为 Box<T> 实现了 Deref、Drop、AsRef<T> 等 trait，
// 所以 Box<T> 可以直接调用 T 实例的方法，访问 T 实例的值。

impl Point {
    fn play(&self) {
        println!("I'am a method of Point.");
    }
}

#[test]
fn test_03() {
    let boxed: Box<Point> = Box::new(Point{x: 10, y: 20});
    boxed.play(); // 点操作符触发deref
    println!("{:?}", boxed);
}

// Box 拥有对 T 实例的所有权，所以可以对 T 实例进行写操作。
#[test]
fn test_04() {
    let mut boxed: Box<Point> = Box::new(Point {x: 10, y: 20});
    *boxed = Point {  // 这一行进行了解引用操作
        x: 20,
        y: 30,
    };
    println!("{:?}", boxed);
}

// Box<T> 的 Clone
#[test]
fn test_05() {
    let mut boxed: Box<Point> = Box::new(Point{x: 10, y: 20});
    let mut another_boxed = boxed.clone(); // 克隆
    *another_boxed = Point{x: 100, y: 200}; // 修改新的一份值
    println!("{:?}", boxed); // 打印原来一份值
    println!("{:?}", another_boxed); // 打印新的一份值
}