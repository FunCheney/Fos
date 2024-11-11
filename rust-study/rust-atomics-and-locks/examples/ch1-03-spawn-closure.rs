use std::thread;

fn main() {
    let numbers = vec![1,2,3];
    // 这里使用 move 关键字。如果不使用 move 关键字
    // 会造成编译错误
    thread::spawn(move || {
        for n in numbers {
            println!("n: {}", n);
        }
    }).join().unwrap();
}