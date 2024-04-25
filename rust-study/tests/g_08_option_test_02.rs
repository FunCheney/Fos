/// 不解包的情况
/// Option<T> 的情况
/// map()：在 Option 是 Some 的情况下，通过 map 中提供的函数或闭包把 Option 里的类型转换成另一种类型。
///     在 Option 是 None 的情况下，保持 None 不变。map() 会消耗原类型，也就是获取所有权。
#[test]
fn test_01() {
    let may_be_some_string = Some(String::from("hello world"));
    let may_be_len = may_be_some_string.map(|s| s.len());
    assert_eq!(may_be_len, Some(11));

    let x: Option<&str> = None;
    assert_eq!(x.map(|s| s.len()), None);
}

/// cloned() 通过克隆 Option<&T> 转化为 Option<T>
#[test]
fn test_02() {
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));
}
/// is_some()：如果 Option 是 Some 值，返回 true。
#[test]
fn test_03() {
    let x:Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);
}
/// is_none()：如果 Option 是 None 值，返回 true。
#[test]
fn test_04() {
    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);
}

/// as_ref()：把 Option<T> 或 &Option<T> 转换成 Option<&T>。创建一个新 Option，
/// 里面的类型是原来类型的引用，就是从 Option<T> 到 Option<&T>。原来那个 Option<T> 实例保持不变。
#[test]
fn test_05() {
    let text: Option<String> = Some(String::from("hello"));
    let _text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {text:?}");
}

/// as_mut()：把 Option<T> 或 &mut Option<T> 转换成 Option<&mut T>。
#[test]
fn test_06() {
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {},
    }

    assert_eq!(x, Some(42))
}
/// take()：把 Option 的值拿出去，在原地留下一个 None 值。这个非常有用。相当于把值拿出来用，但是却没有消解原来那个 Option。
#[test]
fn test_07() {
    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    let mut x: Option<u32> = None;
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, None);
}
/// replace()：在原地替换新值，同时把原来那个值抛出来。
#[test]
fn test_09() {
    let mut x = Some(2);
    let old = x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    let mut x = None;
    let old = x.replace(3);
    assert_eq!(x, Some(3));
    assert_eq!(old, None);
}
/// and_then()：如果 Option 是 None，返回 None；如果 Option 是 Some，
/// 就把参数里面提供的函数或闭包应用到被包裹的内容上，并返回运算后的结果。
#[test]
fn test_10() {
    fn sq_then_to_string(x: u32) -> Option<String> {
        x.checked_mul(x).map(|sq| sq.to_string())
    }

    assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));
    assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
    assert_eq!(None.and_then(sq_then_to_string), None);
}