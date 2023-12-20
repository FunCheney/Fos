/// trait 是一种约束
struct Point<T> {
    x: T,
    y: T,
}

/// fn print<T: std::fmt::Display>(p: Point<T>) {
/// 这里的 Display 就是一个 trait，用来对类型参数 T 进行约束。
/// 它表示必须要实现了 Display 的类型才能被代入类型参数 T，也就是限定了 T 可能的类型范围。
/// std::fmt::Display 这个 trait 主要是定义成配合格式化参数 "{}" 使用的，
/// 和它相对的还有 std::fmt::Debug，用来定义成配合格式化参数 "{:?}" 使用。
/// 而例子里的整数和浮点数，都默认实现了这个 Display trait，
/// 因此整数和浮点数这两种类型能够代入函数 print() 的类型参数 T，从而执行打印的功能。
fn print<T: std::fmt::Display> (p: Point<T>){
    println!("Point {}, {}", p.x, p.y);
}

#[test]
fn test_01() {
    let p = Point {x: 10.2, y: 20.4};
    print(p);
}

/// 语法上 T: TraitA 意思就是我们对类型参数 T 施加了 TraitA 这个约束标记
/// 一个 trait 在一个类型上只能被实现一次
/// trait 对类型参数实施约束的同时，也对具体的类型提供了能力。
/// 让我们看到类型参数后面的约束，就知道到时候代入这其中的类型会具有哪些能力。

/// T: TraitA + TraitB + TraitC + TraitD
///
/// trait 里面可以包含关联函数、关联类型和关联常量。
trait Sport {
    // 关联方法
    fn play(&self);

    fn play_mut(&mut self);

    fn play_own(self);

    // 关联函数
    fn play_some() -> Self;
}

struct FootBall;

impl Sport for FootBall {
    fn play(&self) {
        todo!()
    }

    fn play_mut(&mut self) {
        todo!()
    }

    fn play_own(self) {
        todo!()
    }

    fn play_some() -> Self {
        todo!()
    }
}

