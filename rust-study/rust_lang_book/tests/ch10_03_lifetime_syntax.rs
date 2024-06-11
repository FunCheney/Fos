use std::fmt::Display;

/// 函数的签名表明：对于某些生命周期 ‘a，会获取两个参数，它们都是与生命周期 'a 存在的一样长的字符串 slice
/// 函数会返回一个同样也与生命周期 'a 存在的一样长的字符串 slice
/// 它的实际含义是 longest 函数返回的引用的生命周期与函数参数所引用的值的生命周期的较小者一致
///
///
/// 记住通过在函数签名中指定生命周期参数时，我们并没有改变任何传入值或返回值的生命周期，
/// 而是指出任何不满足这个约束条件的值都将被借用检查器拒绝。
///
/// 当在函数中使用生命周期注解时，这些注解出现在函数签名中，而不存在于函数体中的任何代码中。
/// 生命周期注解成为了函数约定的一部分，非常像签名中的类型。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


#[test]
fn test_01() {
    /// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。
    /// 换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
    /// 因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
    let string1 = String::from("long String is long");
    {
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

/// 该例子揭示了 result 的引用的生命周期必须是两个参数中较短的那个
#[test]
fn test_02() {
    // 以下代码将 result 变量的声明移动出内部作用域，但是将 result 和 string2 变量的赋值语句一同留在内部作用域中。
    // 接着，使用了变量 result 的 println! 也被移动到内部作用域之外
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {result}");
    // |
    // 41 |         let string2 = String::from("xyz");
    //    |             ------- binding `string2` declared here
    // 42 |         result = longest(string1.as_str(), string2.as_str());
    //    |                                            ^^^^^^^ borrowed value does not live long enough
    // 43 |     }
    //    |     - `string2` dropped here while still borrowed
    // 44 |     println!("The longest string is {result}");
    //    |                                     -------- borrow later used here
}


/// 静态生命周期
/// 'static，其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期，
///
#[test]
fn test_03() {

    let s: &'static str = "I have a static lifetime.";
}

/// 结合泛型类型参数、trait bounds 和生命周期

// ann 的类型是泛型 T，它可以被放入任何实现了 where 从句中指定的 Display trait 的类型
fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where T: Display
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

