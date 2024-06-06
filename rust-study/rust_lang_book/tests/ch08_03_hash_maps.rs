use std::collections::HashMap;

/// HashMap<K,V> 类型存储了一个键类型 K 对应一个值类型 V 的映射
/// 它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中

#[test]
fn test_01() {
    let mut socre = HashMap::new();
    socre.insert(String::from("Blue"), 20);
    socre.insert(String::from("Yellow"), 50);

    let team = String::from("Blue");
    let s = socre.get(&team).copied().unwrap_or(0);
    println!("s: {s}");

    for (k, v) in socre {
        println!("{k} : {v}");

    }
}

/// hashMap 所有权
#[test]
fn test_02() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{map:?}");

    // 这里 field_name 和 field_value 不再有效，使用他们将会报错
    // println!("{field_name} ：{field_value}");


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
}

#[test]
fn test_03() {
    // 如果哈希 map 中键已经存在则不做任何操作。如果不存在则连同值一块插入。
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    println!("{map:?}");
    // 使用 entry api
    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，
    // 如果不存在则将参数作为新值插入并返回新值的可变引用。
    map.entry(String::from("Blue")).or_insert(17);
    map.entry(String::from("YELLOW")).or_insert(20);
    println!("{map:?}");
}

#[test]
fn test_04() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // split_whitespace 方法返回一个由空格分隔 text 值子 slice 的迭代器
    for world in text.split_whitespace() {
        // or_insert 方法返回这个键的值的一个可变引用（&mut V）
        let count = map.entry(world).or_insert(0);
        // 将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count。
        // 这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
        *count += 1;
    }
    println!("{map:?}");
}