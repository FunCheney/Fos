use std::thread;

fn f (){
    println!("Hello from another thread");

    let id = thread::current().id();

    println!("my thread id: {id:?}");
}

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread");

    t1.join().unwrap();
    t2.join().unwrap();


}