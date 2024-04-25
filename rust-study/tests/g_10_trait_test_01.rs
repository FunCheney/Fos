use std::fmt::{Debug, Display};

/// T: TraitA 里的 T 表示类型参数，强调“参数”，使用 TraitA 来削减它的类型空间。
/// impl<T: TraitB> TraitA for T {} 末尾的 T 更强调类型参数的“类型”部分，为某些类型实现 TraitA。
/// doit<T>(a: T) {} 中第二个 T 表示某种类型，更强调类型参数的“类型”部分。

/// trait 上也是可以带类型参数的
/// trait TraitA<T> {}
/// 表示这个 trait 里面的函数或方法，可能会用到这个类型参数。
/// 在定义 trait 的时候，还没确定这个类型参数的具体类型。要等到 impl 甚至使用类型方法的时候，才会具体化这个 T 的具体类型。
/// TraitA<T> 是一个整体
trait TraitA<T> {}

struct Atype;
// impl<T> TraitA<T> for Atype {}

impl TraitA<u32> for Atype {}

/// 而如果被实现的类型上自身也带类型参数，那么情况会更复杂。
struct BType<R> {
    a: R,
}

// impl <T, R> TraitA<T> for BType<R> {}
/// 这些类型参数都是可以在 impl 时被约束的，像下面这样：

impl <T, R> TraitA<T> for BType<R>
where
    T: Debug, // 在 impl 时添加了约束
    R: Display, // 在 impl 时添加了约束
{}

/// impl 示例
/// 1. 平面上的一个点与平面上的另一个点相加，形成一个新的点。算法是两个点的 x 分量和 y 分量分别相加。
/// 2. 平面上的一个点加一个整数 i32，形成一个新的点。算法是分别在 x 分量和 y 分量上面加这个 i32 参数。
trait Add<T> {
    type Output; // 关联类型
    fn add(self, rhs: T) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}
/// 为 Point 实现 add<Point> 这个 Trait
impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

#[test]
fn test_01() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };

    let p3 = p1.add(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p4 = p3.add(2);
    assert_eq!(p4.y, 5);
}
