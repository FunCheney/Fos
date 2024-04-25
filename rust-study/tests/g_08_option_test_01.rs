/// Option<T> 定义
/// 定义为包含两个变体的枚举。一个是不发负载的 None，一个是带类型参数作为负载的 Some
/// Rust 做了两件事情
/// 1. Rust 定义完变量之后必须初始化，所以不存在未定义值的情况
/// 2. Rust 把空值单独提出来做 Option<T>::None 定义，在标准库层面做好了规范
// enum Option<T> {
//     Some(T),
//     None
// }
/// Result 被定义为包含两个变体的枚举。这两个变体各自带一个类型参数作为其负载。
/// Ok(T) 用来表示结果正确，Err(E) 用来表示结果有错误。
// enum Result<T, E>{
//     Ok(T),
//     Err(E),
// }

/// 解包
/// Option<u32>::Some(10) 和 u32 明显是不同的类型，如何得到我们真正想要的值
/// ：：解包：：

/// expect()
/// Option<T> 用来消解外面的包裹层，取出里面类型的值，带一个错误提示，如果 Option 实例为 None，就会 panic 并打印出提示信息
/// Rest<T, E> 用来消解外面的包裹层，取出里面类型的值，带一个错误提示，如果 Result 实例为 Err，就会 panic 并打印出提示信息
#[test]
fn test_01() {
    let x = Some("value");
    assert_eq!(x.expect("fruits are healthy"), "value");
    let path = std::env::var("IMPORTANT_PATH").
        expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
}

/// unwrap() 方法
///  Option<T> 用来消解外面的包裹层，取出里面类型的值，带一个错误提示，如果 Option 实例为 None，就会 panic。
///     与 expect 不同地方在于 panic 时，unwrap 时不带提示信息
/// Rest<T, E> 用来消解外面的包裹层，取出里面类型的值，带一个错误提示，如果 Result 实例为 Err，就会 panic。
///     与 expect 不同的地方在于 panic 时，unwrap 时不带提示信息
#[test]
fn test_02() {
    let x = Some("air");
    assert_eq!(x.unwrap(), "air");
    let x: Result<u32, &str> = Ok(4);
    assert_eq!(x.unwrap(), 4);
}
/// unwrap_or() 方法
/// Option<T> 用来消除外面的包裹层，取出里面的类型值，如果 Option 实例为 None，不会 panic，而是取由这个函数提供的参考值，
///     由于这个特性， unwrap_or 常备用来提供默认参数
/// Rest<T, E> 用来消除外面的包裹层，取出里面的类型值，如果 Result 值为 Err，不会 panic，而是取由这个函数提供的参考值，
///     由于这个特性，unwrap_or 常备用来提供默认参数
#[test]
fn test_03() {
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    let default = 2u32;
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap_or(default),2);

    let x: Result<u32, &str> = Err("error");
    assert_eq!(x.unwrap_or(default), default);
}

/// unwrap_or_default()
/// Option<T> 用来消除外面的包裹层，取出里面的类型值，如果 Option 实例为 None，不会 panic，而是取包裹类型的默认值
/// Rest<T, E> 用来消除外面的包裹层，取出里面的类型值，如果 Result 值为 Err，不会 panic，而是去包裹类型的默认值

#[test]
fn test_04() {
    let x: Option<u32> = None;
    let c: Option<u32> = Some(10);
    assert_eq!(x.unwrap_or_default(), 0);
    assert_eq!(c.unwrap_or_default(), 10);

    // Result
    let good_year_from_input = "1909";
    let bad_year_from_input = "190blarg";
    let good_year = good_year_from_input.parse().unwrap_or_default();
    let bad_year = bad_year_from_input.parse().unwrap_or_default();
    assert_eq!(1909, good_year);
    assert_eq!(0, bad_year);
}