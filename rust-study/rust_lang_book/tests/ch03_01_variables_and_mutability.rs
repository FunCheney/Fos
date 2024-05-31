#[test]
fn test_01() {
    let x = 5;
    println!("the value is: {x}");
    // 不可以对 不可变变量 二次赋值
    // x = 6;
    // print!("the value is: {x}");

    // 变量设置为可变 （mut）变量，可进行 二次赋值
    let mut y = 5;
    println!("the value is: {y}");
    y = 6;
    print!("the value is: {y}");
}

#[test]
fn test_02() {
    let x = 5;
    println!("the value is: {x}");
    // 通过 let x 创建一个新变量 x， 获取初始值 + 1
    let x = x + 1;
    println!("the value is: {x}");

}

