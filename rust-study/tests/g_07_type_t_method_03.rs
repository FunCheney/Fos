
/// 方法上的类型参数
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> { // 在impl后定义impl block中要用到的类型参数
    // 该方法的返回类型是 &T 使用到了 impl<T> 里面的参数类型 T
    fn x(&self) -> &T {// 这里，在方法的返回值上使用了这个类型参数
        &self.x
    }
}

#[test]
fn test_01() {
    let p = Point{x: 1.3, y: 2.4};
    println!("p.x = {}", p.x);
}



