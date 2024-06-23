use std::sync::mpsc;
use std::thread;

#[test]
fn test_01() {
    // 第一个元素是发送端，发送着
    // 第二和元素是接收端，接受者
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let rec = rx.recv().unwrap();

    println!("got {rec}");
}