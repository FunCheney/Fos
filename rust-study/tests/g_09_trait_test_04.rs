/// trait 里有可选的关联函数、关联类型、关联常量这三项内容。一旦 trait 定义好，
/// 它就相当于一条法律或协议，在实现它的各个类型之间，在团队协作中不同的开发者之间，
/// 都必须按照它定义的规范实施。这是强制性的，而且这种强制性是由 Rust 编译器来执行的。
/// 也就是说，如果你不想按这套协议来实施，那么你注定无法编译通过。
trait TraitA {
    const LEN: u32 = 10;
}

struct A;

impl TraitA for A {
    const LEN: u32 = 12;
}

#[test]
fn test_01() {
    println!("{:?}", A::LEN);
    print!("{:?}", <A as TraitA>::LEN);
}

struct B;

impl TraitA for B {

}

#[test]
fn test_02() {
    println!("{:?}", B::LEN);
    print!("{:?}", <B as TraitA>::LEN);
}

/// trait TraitA: TraitB {}
/// 个语法的意思是如果某种类型要实现 TraitA，那么它也要同时实现 TraitB。反过来不成立。
// trait Shape {
//     fn area(&self) -> f64;
// }
// trait Circle : Shape {
//     fn radius(&self) -> f64;
// }
/// 上面这两行等价与下面这两行

// trait Shape {
//     fn area(&self) -> f64;
// }
// trait Circle
//     where Self: Shape {
//     fn radius(&self) -> f64;
// }

/// 一个 trait 依赖多个 trait 也是可以的。
/// trait TraitA: TraitB + TraitC {}
/// 这个例子里面，T: TraitA 实际表 T: TraitA + TraitB + TraitC。因此可以少写不少代码。
/// 在约束依赖中，冒号后面的叫 supertrait，冒号前面的叫 subtrait。可以理解为 subtrait 在 supertrait 的约束之上，
/// 又多了一套新的约束。这些不同约束的地位是平等的。

trait Shape {
    fn play(&self) {    // 定义了play()方法
        println!("1");
    }
}
trait Circle : Shape {
    fn play(&self) {    // 也定义了play()方法
        println!("2");
    }
}
impl Shape for A {}
impl Circle for A {}

impl A {
    fn play(&self) {    // 又直接在A上实现了play()方法
        println!("3");
    }
}

#[test]
fn test_04() {
    let a = A;
    a.play();    // 调用类型A上实现的play()方法
    <A as Circle>::play(&a);  // 调用trait Circle上定义的play()方法
    <A as Shape>::play(&a);   // 调用trait Shape上定义的play()方法
}