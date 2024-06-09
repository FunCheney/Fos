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

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl <X1, Y1> Point3<X1, Y1> {
    fn mixup <X2,Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3{
            x: self.x,
            y: other.y,
        }
    }
}

#[test]
fn test_03() {
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
/// 泛型代码的性能
/// 泛型并不会比具体的类型运行慢
/// Rust 会在编译时对泛型代码进行单态化来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

#[test]
fn test_04() {
    let integer = Some(5);
    let float = Some(5.0);
    // 当 Rust 编译这些代码时会进行单态化，编译器会读取传递给 Option<T> 的值并发现有两种Option<T>：一个对应 i32 另一个对应 f64。
    // 它会将泛型定义 Option<T> 展开为两个针对 i32 和 f64 的定义，接着将泛型定义替换为这两个具体的定义。
}
/// 编译器生成的单态化版本的代码看起来像这样（编译器会使用不同于如下假想的名字）：
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}