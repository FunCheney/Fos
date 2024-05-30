/// _ 占位符
#[derive(Debug)]
enum Shape {
    Rectangle,
    Triangle,
    Circle,
}
/// 想测试一些东西，或者就是不想处理一些分支，可以用 _ 偷懒。
#[test]
fn test_01() {
    let shape_a = Shape::Rectangle;
    let ret = match shape_a {
        Shape::Rectangle => 1,
        _ => 10,
    };
    println!("{}", ret);
}
/// match 除了和枚举进行分支管理外，还可以和其他类型结合进行分支分派
#[test]
fn test_03() {
    let number = 13;
    match number {
        // 匹配单个数值
        1 => println!("1"),
        2 | 3 | 4 | 5 => println!("12344fiif"),
        6..=10 => println!("(6,10]"),
        _ => println!("other"),
    }
}

/// 模式匹配
/// match 是模式匹配的入口
/// 模式匹配就是按对象值的结构进行匹配，并且可以取出符合模式的值
/// match 不限于在模式匹配中使用，除了 match 以外，Rust 还给模式匹配提供了其他一些语法层面的支持

#[test]
fn test_04() {
    // 使用 if let 改写 test_01
    let shape_a = Shape::Rectangle;
    if let Shape::Rectangle = shape_a {
        println!("1")
    } else {
        println!("10")
    }
}

// ref 关键词
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    student: bool,
}

#[test]
fn test_02() {
    let a = User {
        name: String::from("mike"),
        age: 12,
        student: true,
    };
    let User {
        ref name, // 这里是一个 ref
        age,
        student,
    } = a;

    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
}
