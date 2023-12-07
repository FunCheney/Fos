struct Point {
    x: f64,
    y: f64,
}


impl Point {
    fn origin() -> Point {
        Point{x: 0.0, y: 0.0}
    }

    fn new(x: f64, y: f64) -> Point {
        Point{x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Rectangle {
    // 这是一个方法类
    // &self 是 self: &Self 的语法糖
    // Self 是当前调用对象的类型，对于本例来说 Self = Rectangle
    fn area(&self) -> f64{

        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;
        
        ((x1 - x2) * (y1 - y2)).abs()

    }
    fn preimeter(&self) -> f64{

        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        2 * ((x1 - x2) * (y1 - y2)).abs()
    }

}
fn main() {

    println!("Hello, world!");
}
