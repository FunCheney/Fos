use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd=> println!("unknown command: {cmd:?}"),
        }
    }

    STOP.store(true, Relaxed);

    background_thread.join().unwrap();
}

fn some_work() {
    println!("do something....");
    thread::sleep(Duration::from_millis(100));
}