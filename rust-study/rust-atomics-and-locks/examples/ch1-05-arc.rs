use std::sync::Arc;
use std::thread;

fn main() {
    // 引用计数 为 1
    let a = Arc::new([1, 2, 3]);
    // clone 之后引用计数 + 1 为 2
    let b = a.clone();

    // 每一个线程都有自己的 Arc，通过它可以访问共享数据
    // 当线程执行结束，引用计数减一
    // 最后一个放弃其Arc的线程将看到计数器降为零，并且将是那个放弃并释放数组的线程
    thread::spawn(move || {
        dbg!(a)
    });
    thread::spawn(move || {
        dbg!(b)
    });


    // 引用计数 为 1
    let a = Arc::new([1, 2, 3]);
    // clone 之后引用计数 + 1 为 2
    let b = a.clone();
    thread::spawn(move || {
        dbg!(b);
    });
    dbg!(a);


    // 定义变量
    let a = Arc::new([1, 2, 3]);

    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });

    dbg!(a);


}