use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let a = AtomicU32::new(100);
    let b = a.fetch_add(23, Relaxed);
    let c = a.load(Relaxed);
    println!("a is {:?}", a);
    assert_eq!(b, 100);
    assert_eq!(c, 123);
}