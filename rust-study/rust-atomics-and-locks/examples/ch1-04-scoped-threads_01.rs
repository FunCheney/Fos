use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=1000);
    // 这里如果是 空数组会 panic
    // let numbers = Vec::new();

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        // 将闭包中的返回给发送给主函数
        sum / len
    });
    // 通过 join 方法得到返回值
    let average = t.join().unwrap();
    println!("average {}", average);
    // 闭包会转移所有权
    // println!("numbers {:?}", numbers);
}