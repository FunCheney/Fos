use std::thread;

fn main() {
    // 通过 spawn 创建一个新的线程
    // 参数是一个函数，线程将会执行该函数。函数返回，线程停止执行
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
