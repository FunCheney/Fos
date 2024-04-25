/// 枚举中的类型参数
/// 枚举的变体可以挂载任意其他类型作为负载。因此每个负载的位置，都可以出现类型参数。
/// 比如最常见的两个枚举，Option<T> 与 Result<T, E>，就是泛型。
/// Option<T> 表示有 或 无
enum Option<T> {
    Some(T),
    None,
}
/// Result<T, E> 表示结果正确 或者 错误
enum Result<T, E> {
    Ok(T),
    Err(E),
}

/// 更复杂的枚举中带类型参数的例子
struct Point<T>{
    x: T,
    y: T,
}
enum Arr<T, U>{
    V1(Point<T>),
    V2(Vec<U>),
}


/// 函数中的类型参数
/// 这里 T: std::fmt::Display 的意思是要求 T 满足某些条件 / 约束
fn point<T: std::fmt::Display> (p: Point<T>) {
    println!("{}, {}", p.x, p.y);
}

#[test]
fn test_01() {
    let p1 = Point {x: 1, y: 3};
    point(p1);
    let p2 = Point {x: 2.4, y: 3.5};
    point(p2);
}