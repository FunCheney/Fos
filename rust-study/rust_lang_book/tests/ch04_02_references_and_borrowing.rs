
// 在上一节中将 String 返回给调用函数，以便在调用 calculate_length 后仍能使用 String，
// 因为 String 被移动到了 calculate_length 内。
/// 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。
/// 与指针不同，引用确保指向某个特定类型的有效值。

#[test]
fn test_01() {
    let s = String::from("hello");
    // 将创建一个引用的行为称为 借用（borrowing）。
    let len = calculate_length(&s);
    println!("s {s}, len {len}");
}
// 变量 s 有效的作用域与函数参数的作用域一样，不过当 s 停止使用时并不丢弃引用指向的数据，因为 s 并没有所有权。
// 当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。
fn calculate_length(s: &String) -> usize { // s 是 String 的引用
    s.len()
}// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么都不会发生


#[test]
fn test_mut_ref() {
    let mut s = String::from("hello");
    // 调用 change 函数的地方创建一个可变引用 &mut s
    change(&mut s);

    println!("s {s}");
}
/// change 函数将改变它所借用的值。
fn change(s : &mut String){
    s.push_str(", world");
}

#[test]
fn test_dangling_pointer() {
    // dangle();
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用
//     let  s = String::from("hello"); // s 是一个新字符串
//
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
