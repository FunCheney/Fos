pub fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value} {unit_label}");
}
#[test]
fn test_01() {
    print_labeled_measurement(1, 'c');
}

/// 函数 由一系列语句和一个可选的结尾表达式构成
/// Rust 是一门基于表达式的语言，这是一个需要理解的（不同于其他语言）重要区别。
/// 其他语言并没有这样的区别，所以让我们看看语句与表达式有什么区别以及这些区别是如何影响函数体的。
/// 语句（Statements）是执行一些操作但不返回值的指令。 表达式（Expressions）计算并产生一个值。


// 函数定义也是一个语句，这个例子本身就是一个语句。
pub fn function(){
    // 这是一个语句
    let _x = 5;

    // 语句不返回值。因此，不能把 let 语句赋值给另一个变量，比如下面的例子尝试做的，会产生一个错误：
    // let b = 5; 不会返回值, 所以没有可以绑定到 a 上的值
    // let a = (let b = 5);
}

/// 函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是一个表达式
#[test]
fn test_02() {
    // {} 大括号是一个表达式， 它的值是 4
    // 这个值作为 let 语句的一部分被绑定到 y 上
    let y = {
        // let x = 3; 中 3 是一个表达式，它计算出的值是3
        let x = 3;
        // 注意这里没有 分号（;）
        // 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值
        x + 1
    };

    println!("The value of y is: {y}");
}

/// 具有返回值的函数
///
/// 在 five 函数中没有函数调用、宏、甚至没有 let 语句 —— 只有数字 5。
pub fn five() -> i32 {
    5
}

#[test]
fn test_03() {
    // 首先，let ret = five(); 这一行表明我们使用函数的返回值初始化一个变量
    // 其次，five 函数没有参数并定义了返回值类型，不过函数体只有单单一个 5 也没有分号，因为这是一个表达式
    let ret = five();
    print!("ret {ret}");
}

pub fn plus_one(x: i32) -> i32 {
    // x + 1 的行尾加上一个分号，把它从表达式变成语句，将会出现错误
    // expected `i32`, found `()`
    // implicitly returns `()` as its body has no tail or `return` expression
    // x + 1;
    x + 1
}