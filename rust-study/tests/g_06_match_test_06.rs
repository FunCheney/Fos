#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[derive(Debug)]
enum Shape {
    Rectangle(Rectangle),
    Triangle((u32, u32), (u32, u32), (u32, u32)),
    Circle {origin: (u32, u32), radius: u32},
}

#[test]
fn test_01() {
    let a_rec = Rectangle {
        width: 10,
        height: 20,
    };
    // let shape_a = Shape::Rectangle(a_rec);
    // let shape_a = Shape::Triangle((0, 1), (3,4), (3, 0));
    let shape_a = Shape::Circle {origin: (1,2), radius: 5};
    match shape_a {
        Shape::Rectangle(a_rec) => { //解析出一个结构体
            println!("Rectangle {}, {}", a_rec.width, a_rec.height);
        }
        Shape::Triangle(x, y, z) => {
            // 解析出一个元组
            println!("Triangle {:?}, {:?}, {:?}", x, y, z);
        }
        Shape::Circle {origin, radius } => {
            // 解析出一个结构体
            println!("Circle {:?}, {:?}", origin, radius);
        }
    }

}