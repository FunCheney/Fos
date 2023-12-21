use std::fmt::{Debug, Display};

/// trait 类型参数的默认实现
/// 定义带类型参数的 trait 的时候，可以为类型参数指定一个默认类型，比如 trait TraitA {}。
/// 这样使用时，impl TraitA for SomeType {} 就等价于 impl TraitA for SomeType {}。
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
    T: Debug // 定义TraitA的时候，对T作了约束
{
    fn play(&self, _t: T) {}
}

struct OneType;

impl<T> TraitOne<T> for OneType
where
    T: Debug + PartialEq // 将TraitA实现到类型 OneType 上时，加强了约束
{}

#[test]
fn test_01() {
    let a = OneType;
    a.play(10);
}






