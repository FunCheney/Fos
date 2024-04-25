/// 关联类型
/// 在 trait 中，可以带一个或多个关联类型。关联类型起一种类型占位功能，定义 trait 时声明，
/// 在把 trait 实现到类型上的时候为其指定具体的类型
trait Sport {
    type SportType;
    fn play(&self, st: Self::SportType);
}

struct FootBall;

pub enum SportType {
    Land,
    Water,
}

impl Sport for FootBall {
    type SportType = SportType;

    fn play(&self, st: Self::SportType) {
    }
}

#[test]
fn test_01() {
    let f = FootBall;
    f.play(SportType::Land)
}

struct Foo; // 定义一个新类型
struct Point<T> {
    x: T,
    y: T,
}

fn print<T:std::fmt::Display> (p: Point<T>){
    println!("Point {}, {}", p.x, p.y);
}
#[test]
fn test_02() {

    let p = Point {x: Foo, y: Foo}; // 初始化一个Point 实例
    // Foo 为实现 disPlay Trait
    // print(p);
}

/// 在 T 上使用关联类型
/// 标准库中的 迭代器 Iterator trait 的定义。
/// Iterator 定义了一个关联类型 Item。注意这里的 Self::Item 实际是 ::Item 的简写
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
/// 一般来说，如果一个类型参数被 TraitA 约束，而 TraitA 里有关联类型 MyType，
/// 那么可以用 T::Mytype 这种形式来表示路由到这个关联类型。
trait TraitA {
    type MyType;
}

fn doit<T:TraitA>(a: T::MyType) { // 这里使用了关联类型

}

struct A;

impl TraitA for A {
    type MyType = String;
}

#[test]
fn test_03() {
    doit::<A>("abc".to_string());  // 给Rustc小助手喂信息：T具化为TypeA
}

