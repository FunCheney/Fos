#[test]
fn string_01() {
    let s = String::from("hello, world");
    println!("{}", s);
    let aa = s.to_string();
    println!("{}", aa);
    let a = &"hello"[0..1];

    let b = &"";

    let c = "";
}
/// ① : String 是 字符串的所有权形式，在堆中分配内存。String 的类型是动态可变的
/// ② : str 是字符串的切片类型，通常以 &str 的形式出现; &str 是字符串视图的借用形式
/// ③ : 字符串字面量默认放在静态数据区里，而静态数据区中的字符串总是贯穿程序运行的整
///     个生命周期，知道程序结束才会被释放。
///
/// ④ : &String 仅是对 String 类型字符串的 普通引用
/// ⑤ : 对 String 做字符串切面操作后会得到 &str。&str 就是指向由 String 管理的内存资源的切片引用。
///     是目标字符串的借用形式，不会再把字符串内容复制一份
///      &str 即可以引用堆中的字符串，也可以引用静态数据区域内的字符串 &'static str 是 &str 的一
///     种特殊形式
#[test]
fn string_02() {
    // hello 字符串的字面量，存放在静态数据区
    // s1 是指向静态数据中的这个字符串的切片引用
    let s1: &'static str = "hello";
    // to_string() rust 将静态数据区中的数据拷贝了一份保存在堆内存
    let s2: String = s1.to_string();
    // 对 s2 的不可变引用, 类型为 &String
    let s3: &String = &s2;
    // s4 为对 s2 切片引用，类型是 &str 切片就是一块连续内存的某种视图
    // 它可以提取目标对象的全部或一部分，s4 就是全部
    let s4: &str = &s2[..];
    // s5 是对 s2 的另一个切片引用，类型也是 &str
    let s5: &str = &s2[0..5];
}

#[test]
fn str_to_string(){

    let s: String= "hello".to_string();

    let a_slice: &str = &s[0..3];

    let another_string = a_slice.to_string();

    println!("{}", another_string);

}


#[test]
fn string_error_01(){
    let s = String::from("hello");
    // 这一句执行后 s 不可用，s1 可用
    // Java 默认做了引用的拷贝，并且新旧两个变量同时指向原来那个对象。
    // 而 Rust 不一样，Rust 虽然也是把字符串的引用由 s 拷贝到了 s1，
    // 但是只保留了最新的 s1 到字符串的指向，同时却把 ❗️s 到字符串的指向给“抹去”了
    let s1 = s;
    println!("{s}");
    println!("{s1}");
}


