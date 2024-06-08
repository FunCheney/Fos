/// largest 有泛型类型 T。它有个参数 list，其类型是元素为 T 的 slice。
/// largest 函数会返回一个与 T 相同类型的引用。

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[test]
fn test_01() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

/// 结构体中定义泛型
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U>{
    x: T,
    y: U,
}

/// 方法定义中的泛型
///
/// 通过在 impl 之后声明泛型 T，Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型
impl <T> Point<T> {
    fn x(&self) ->&T {
        &self.x
    }
}