
// 该函数将闭包作为参数并调用它。
fn apply<F>(f: F) where
// 闭包没有输入值和返回值。
    F: FnOnce() {
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

    f();
}

// 输入闭包，返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32 where
// 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32 {

    f(3)
}

#[test]
fn test_01() {
    use std::mem;

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

/// 不加move关键字，闭包将不会获取a、b和c的所有权，而是捕获它们的引用。
/// 这意味着闭包将依赖于a、b和c在make_quadratic函数作用域中的生命周期
pub fn make_quadratic(a: f64, b: f64, c: f64) -> impl Fn(f64) -> f64 {
     move |x| a*x*x + b*x + c
}

#[test]
fn test_03() {
    let quadratic = make_quadratic(1.0, -3.0, 2.0);
    let result = quadratic(4.0); // 计算 f(4) = 1*4^2 - 3*4 + 2
    println!("{}", result); // 输出结果
}


/// 闭包在堆上分配，并且返回 Box
pub fn make_quadratic_box(a: f64, b: f64, c: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(make_quadratic(a, b, c))
}
