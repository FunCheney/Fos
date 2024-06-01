#[test]
fn test_01() {
    // 使用 parse 将 String 转换为数字时，必须增加类型注解
    let guess:u32 = "42".parse().expect("value is not number");
    print!("the value {:?}", guess);
}


#[test]
fn test_02() {
    //浮点类型
    // Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64
    let a = 1.0;
    let b :f32 = 2.0;

    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");
    // multiplication
    let product = 4 * 30;
    println!("product {product}");
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient {quotient}");
    let truncated = -5 / 3; // 结果为 -1
    println!("truncated {truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("remainder {remainder}");
}

#[test]
fn test_03() {

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("x.0 {five_hundred}, x.1 {six_point_four}, x.2 {one}");
}