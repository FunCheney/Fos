use crate::Shape::Rectangle;

#[derive(Debug)]
enum  Shape {
    Rectangle,
    Triangle,
    Circle,
}

#[test]
fn test_01() {
    let mut shape_a = Shape::Rectangle;
    let mut i = 0;
    while let Shape::Rectangle = shape_a {
        if i > 9 {
            println!("gather than 9");
            shape_a = Shape::Triangle;
        }else {
            println!("`i` is `{:?}`. Try again.", i);
            i += 1;
        }
    }
}

/// let 本身就支持模式匹配 if let，while let 本身就是使用的模式匹配的能力

enum MyTest {
    Rectangle{x: u32, y: u32},
    Triangle,
    Circle,
}
#[test]
fn test_04() {
    // 创建带负载的，枚举实例
    let shape_a = MyTest::Rectangle {x: 12, y: 32};
    // 模式匹配出负载类容
    // 这种写法是匹配结构体负载获取字段值的方式
    let MyTest::Rectangle {x, y} = shape_a  else{
        panic!("Can't extract rectangle.");
    };
    println!("x: {}, y: {}", x, y);
}

/// 元组匹配
#[test]
fn test_02() {
    let a = (1, 2, 'q');
    let (b, c, d) = a;
    println!("{:?}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
}
/// 元组的析构，常用来从元组的多个返回值里取数据
fn foo() -> (u32, u32, char) {
    (1, 2, 'q')
}

#[test]
fn test_03() {
    let(b, c, d) = foo();
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
}

