use std::thread;
use std::thread::JoinHandle;

fn main() {
    let numbers = Vec::from_iter(0..=1000);
    let t: JoinHandle<usize> = thread::Builder::new().name("test".to_string())
        .spawn(move || {
        let len = numbers.len();
        let sum: usize= numbers.iter().sum();
        sum / len
    }).unwrap();
    // 通过 join 方法得到返回值
    let average = t.join().unwrap();
    println!("average {}", average);
}