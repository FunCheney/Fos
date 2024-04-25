use std::fmt::{Debug, Display};

/// trait 类型参数的默认实现
/// 定义带类型参数的 trait 的时候，可以为类型参数指定一个默认类型，比如 trait TraitA<T = u64> {}。
/// 这样使用时，impl TraitA for SomeType {} 就等价于 impl TraitA<u64> for SomeType {}。
trait  TraitA<T = Self> {
    fn func(t: T) {}
}

// 这个默认类型为i32
trait TraitB<T = i32> {
    fn func(t: T) {}
}

struct  SomeType;
// 这里省略了类型参数，所以这里的T为Self
// 进而T就是SomeType本身
impl TraitA for SomeType {
    fn func(t: SomeType) {}
}
// 这里省略了类型参数，使用默认类型i32
impl TraitB for SomeType {
    fn func(t: i32) {
    }
}
// 这里不省略类型参数，明确指定类型参数为String
impl TraitA<String> for SomeType {
    fn func(t: String) {
    }
}

// 这里不省略类型参数，明确指定类型参数为String
impl TraitB<String> for SomeType {
    fn func(t: String) {}
}

/// Rust 中的关联类型
trait TraitC {
    type Item;
}

struct Foo<T: TraitC<Item = String>> {
    x: T,
}

/// 关联类型的具化是在应用约束时，类型参数的默认类型指定是在定义 trait 时
/// 1. 类型参数可以在 impl 类型的时候具化，也可以延迟到使用的时候具化。而关联类型在被 impl 时就必须具化。
/// 2. 由于类型参数和 trait 名一起组成了完整的 trait 名字，不同的具化类型会构成不同的 trait，
///     所以看起来同一个定义可以在目标类型上实现“多次”。而关联类型没有这个作用。

/// 对于第一点，请看下面的示例：

trait TraitOne<T>
where
    T: Debug // 定义 TraitOne 的时候，对 T 作了约束
{
    fn play(&self, _t: T) {}
}

struct OneType;

impl<T> TraitOne<T> for OneType
where
    T: Debug + PartialEq // 将TraitA实现到类型 OneType 上时，加强了约束
{}

// impl TraitOne<u32> for OneType { // 这里具化成了 u32 类型
// }

#[test]
fn test_01() {
    let a = OneType;
    a.play(10);
}

/// 所以这个模型仅用关联类型来实现，是写不出来的。
/// 好像带类型参数的 trait 功能更强大，那用这个不就够了？但关联类型也有它的优点，
/// 比如关联类型没有类型参数，不存在多引入了一个参数的问题，而类型参数是具有传染性的，
/// 特别是在一个调用层次很深的系统中，增删一个类型参数可能会导致整个项目文件到处都需要改，非常头疼。

trait Add {
    type ToAdd;  // 多定义一个关联类型
    type OutPut;

    fn add(self, rhs: Self::ToAdd) -> Self::OutPut;
}

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type ToAdd = Point;
    type OutPut = Point;

    fn add(self, rhs: Self::ToAdd) -> Self::OutPut {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Point {  // 这里重复impl了同一个trait，无法编译通过
    type ToAdd = i32;
    type OutPut = Point;

    fn add(self, rhs: Self::ToAdd) -> Self::OutPut {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

