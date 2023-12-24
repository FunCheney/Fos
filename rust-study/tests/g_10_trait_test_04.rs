/// 利用 trait object 传参
/// impl trait 和 dyn trait 也可以用于函数传参。

struct Atype;
struct Btype;
struct Ctype;

trait TraitA {}

impl TraitA for Atype {}
impl TraitA for Btype {}
impl TraitA for Ctype {}

fn doit(x: impl TraitA) {}
// 等价于
// fn doit<T: TraitA>(x: T) {}

#[test]
fn test_01() {
    let a = Atype;
    doit(a);
    let b = Btype;
    doit(b);
    let c = Ctype;
    doit(c);
}

// 这里引用的形式是 &dyn TraitA
fn doit_02(x: &dyn TraitA) {}

#[test]
fn test_02() {
    let a = Atype;
    doit_02(&a);
}
/// 两种都可以。那么它们的区别是什么呢？
// impl trait 用的是编译器静态展开，也就是编译时具化（单态化）。上面那个 impl trait 示例展开后类似于下面这个样子。
// struct Atype;
// struct Btype;
// struct Ctype;
//
// trait TraitA {}
//
// impl TraitA for Atype {}
// impl TraitA for Btype {}
// impl TraitA for Ctype {}
//
// fn doit_a(x: Atype) {}
// fn doit_b(x: Btype) {}
// fn doit_c(x: Ctype) {}
//
// fn main() {
//     let a = Atype;
//     doit_a(a);
//     let b = Btype;
//     doit_b(b);
//     let c = Ctype;
//     doit_c(c);
// }
/// 而 dyn trait 的版本不会在编译期间做任何展开，dyn TraitA 自己就是一个类型，这个类型相当于一个代理类型，
/// 用于在运行时代理相关类型及调用对应方法。既然是代理，也就是调用方法的时候需要多跳转一次，从性能上来说，
/// 当然要比在编译期直接展开一步到位调用对应函数要慢一点。

// 那它们和 enum 相比呢？
// enum 是封闭类型集，可以把没有任何关系的任意类型包裹成一个统一的单一类型。
// 后续的任何变动，都需要改这个统一类型，以及基于这个 enum 的模式匹配等相关代码。
// 而 impl trait 和 dyn trait 是开放类型集。只要对新的类型实现 trait，
// 就可以传入使用了 impl trait 或 dyn trait 的函数，其函数签名不用变。

/// 利用 trait obj 将不同的类型装进集合里
#[test]
fn test_03() {
    let a = Atype;
    let b = Btype;
    let c = Ctype;
    // let v = vec![a, b, c];
    // 因为 Vec 中要求每一个元素是同一种类型，不能将不同的类型实例放入同一个 Vec。
    // 而利用 trait object，我们可以“绕”过这个限制。
    let v: Vec<&dyn TraitA> = vec![&a, &b, &c];
}

/// 哪些 trait 能用作 trait object？
/// 只有满足对象安全（object safety）的 trait 才能被用作 trait object
// 安全的 trait object：
trait TraitSafety {
    fn foo(&self) {}
    fn foo_mut(&mut self) {}
    fn foo_box(self: Box<Self>) {}
}

trait NotObjectSafe {
    const CONST: i32 = 1;  // 不能包含关联常量

    fn foo() {}  // 不能包含这样的关联函数
    fn selfin(self); // 不能将Self所有权传入
    fn returns(&self) -> Self; // 不能返回Self
    fn typed<T>(&self, x: T) {} // 方法中不能有类型参数
}


