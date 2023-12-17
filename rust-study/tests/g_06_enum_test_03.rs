use crate::Shape::Rectangle;

/// match 语句
/// 它的作用是判断或匹配 值是哪一个枚举的变体
#[derive(Debug)]
enum Shape {
    Rectangle,
    Triangle,
    Circle,
}

#[test]
fn test_01() {
    let shape_a = Shape::Rectangle; // 创建枚举
    match shape_a {
        Shape::Rectangle => {
            println!("{:?}", Shape::Rectangle); // 进了这个分支
        }
        Shape::Triangle => {
            println!("{:?}", Shape::Triangle); // 进了这个分支
        }
        Shape::Circle => {
            println!("{:?}", Shape::Circle); // 进了这个分支
        }
    }
}
/// match 可返回值
/// 每一个分支里面返回的值必须相关
#[test]
fn test_02() {
    let shape_2 = Shape::Circle;
    let rec =  match shape_2 {
        Shape::Rectangle => {
            1
        }
        Shape::Triangle => {
            2
        }
        Shape::Circle => {
            3
        }
    };
    println!("rec {}", rec)
}
/// 枚举的所有分支都必须处理
#[test]
fn test_03() {
    let shape_3 = Shape::Rectangle;
    let ret = match shape_3 {
        Rectangle => {
            1
        }
        Shape::Triangle => {
            1
        }
    };
    println!("{}", ret)
}
