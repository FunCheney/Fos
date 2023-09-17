// 1.字符串silce 是 String 中一部分值的引用 
// 2.字面值就是 silce
// 3.其他类 slice
fn main() {

    let s = String::from("hello world");
    let h = &s[0..=7];
    println!("h = {}", h);
    println!("Hello, world!");

}
