#[test]
fn test_01() {
    let v : Vec<i32> = Vec::new();

    // 新建一个拥有值 1、2 和 3 的 Vec<i32>。推断为 i32 是因为这是默认整型类型
    let v1 = vec![1, 2, 3];

    // 对于新建一个 vector 对象，并向其增加元素，可以使用 push 方法
    let mut v2 = Vec::new();
    v2.push(2);
    v2.push(3);
}

#[test]
fn test_02() {
    // 读取 vector 元素
    let v = vec![1, 2, 4];
    let third = &v[2];
    println!("The third element is {third}");

    // 索引 2 用来获取第三个元素，因为索引是从数字 0 开始的
    // 使用 & 和 [] 会得到一个索引位置元素的引用。
    // 当使用索引作为参数调用 get 方法时，会得到一个可以用于 match 的 Option<&T>。
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    if let Some(4) = third {
        println!("The third element is {:?}", third);
    }


    let v = vec![1, 2, 3, 4, 5];
    // 当通过  [] 方法 引用一个不存在的元素时，ust 会造成 panic
    // let does_not_exist = &v[100];
    // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None
    let does_not_exist = v.get(100);

}

#[test]
fn test_03() {
    // 一旦程序获取了一个有效的引用，借用检查器将会执行所有权和借用规则（第四章讲到）
    // 来确保 vector 内容的这个引用和任何其他引用保持有效
    let mut v = vec![1, 2, 3, 4, 5];

    // 根据不能在相同作用域中同时存在可变和不可变引用的规则
    // 当我们获取了 vector 的第一个元素的不可变引用并尝试在 vector 末尾增加一个元素的时候
    // ，如果尝试在函数的后面引用这个元素是行不通的。
    let first = &v[0];

    v.push(6);

    // println!("The first element is: {first}");
    // 为什么第一个元素的引用会关心 vector 结尾的变化？
    // 不能这么做的原因是由于 vector 的工作方式：
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
}

#[test]
fn test_04() {
    let v = vec![1, 2, 3, 4];

    for o in &v  {
        println!("o: {o}");
    }

    for i in v {
        println!("i: {i}");
    }

    // 这里继续打印会报错，因为经过上一个遍历后，所有权被移动

    // for i in v {
    //     println!("i: {i}");
    // }

}

#[test]
fn test_05() {
    let mut v = vec![100, 32, 57];
    // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值。
    for i in &mut v {
        *i += 50;
    }
    for o in &v  {
        println!("o: {o}");
        // for 循环中获取的 vector 引用阻止了同时对 vector 整体的修改。
        // v.push(12);
    }
}

#[test]
fn test_06() {
    //枚举的成员都被定义为相同的枚举类型，
    // 所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存。
    // 第二个好处是可以准确的知道这个 vector 中允许什么类型。
    // 如果 Rust 允许 vector 存放任意类型，那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造成错误。
    // 使用枚举外加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}