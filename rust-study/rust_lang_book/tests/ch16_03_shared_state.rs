use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

/// 信道 类似与但所有权，一旦将数据传入信道，将无法再使用这个值
/// 共享内存类似与多所有权，多个线程可以同时访问相同的内存位置


#[test]
fn test_01() {
    // 是用 new 来创建一个 Mutex<T>
    let m = Mutex::new(5);
    {
        // 使用 lock 方法获取锁，已访问互斥器中的数据，这个调用会阻塞当前线程，直到获取到锁
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    // Mutex<T> 是一个智能指针。更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。
    // 这个智能指针实现了 Deref 来指向其内部数据；其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁
    println!("m = {m:?}");
}


#[test]
fn test_02() {
    //    |
    // 25 |     let counter = Mutex::new(0);
    //    |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
    // ...
    // 29 |         let handle = thread::spawn(move || {
    //    |                                    ------- value moved into closure here, in previous iteration of loop
    // ...
    // 41 |     println!("Result: {}", *counter.lock().unwrap());
    //    |                             ^^^^^^^ value borrowed here after move
    // let counter = Mutex::new(0);


    // Rc<T> 并不能安全的在线程间共享。当 Rc<T> 管理引用计数时，它必须在每一个 clone 调用时增加计数，
    // 并在每一个克隆被丢弃时减少计数。Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断。
    // 在计数出错时可能会导致诡异的 bug，比如可能会造成内存泄漏，或在使用结束之前就丢弃一个值
    // let counter = Rc::new(Mutex::new(0));

    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
           let mut  num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handle);
    }

    for handle in handlers{
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}