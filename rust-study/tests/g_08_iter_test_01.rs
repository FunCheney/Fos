/// 迭代器其实很简单，就是对一个集合类型进行遍历。比如对 Vec<T>、对 HashMap<K, V> 等进行遍历

#[test]
fn test_01() {
    let a: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut iter = a.into_iter();
    while let Some (i) = iter.next() {
        println!("{i}")
    }
}

/// Rust 中的迭代器根据所有权三态可以分成三种。
/// 1.获取集合元素不可变引用的迭代器，对应方法为 iter()。
/// 2.获取集合元素可变引用的迭代器，对应方法为 iter_mut()。
/// 3.获取集合元素所有权的迭代器，对应方法为 into_iter()。
#[test]
fn test_02() {
    let mut a = [1, 2, 3];    // 一个整数数组

    let mut an_iter = a.iter();  // 转换成第一种迭代器

    assert_eq!(Some(&1), an_iter.next());
    assert_eq!(Some(&2), an_iter.next());
    assert_eq!(Some(&3), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.iter_mut();  // 转换成第二种迭代器

    assert_eq!(Some(&mut 1), an_iter.next());
    assert_eq!(Some(&mut 2), an_iter.next());
    assert_eq!(Some(&mut 3), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.into_iter();  // 转换成第三种迭代器，并消耗掉a

    assert_eq!(Some(1), an_iter.next());
    assert_eq!(Some(2), an_iter.next());
    assert_eq!(Some(3), an_iter.next());
    assert_eq!(None, an_iter.next());

    // 对于整数数组 [1,2,3] 而言，调用 into_iter() 实际会复制一份这个数组，
    // 再将复制后的数组转换成迭代器，并消耗掉这个复制后的数组，因此最后的打印语句能把原来那个 a 打印出来。
    // u32 类型复制
    println!("{:?}", a);
}

#[test]
fn test_03() {
    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];
    let mut an_iter = a.iter();

    assert_eq!(Some(&"1".to_string()), an_iter.next());
    assert_eq!(Some(&"2".to_string()), an_iter.next());
    assert_eq!(Some(&"3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.iter_mut();

    assert_eq!(Some(&mut "1".to_string()), an_iter.next());
    assert_eq!(Some(&mut "2".to_string()), an_iter.next());
    assert_eq!(Some(&mut "3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.into_iter();

    assert_eq!(Some("1".to_string()), an_iter.next());
    assert_eq!(Some("2".to_string()), an_iter.next());
    assert_eq!(Some("3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());
    // 对于字符串数组 ["1".to_string(), "2".to_string(), "3".to_string()] 而言，
    // 调用 into_iter() 会直接消耗掉这个字符串数组，因此最后的打印语句不能把原来那个 a 打印出来。
    // String 是移动
    // println!("{:?}", a);    // 请你试试这一行有没有问题？
}
