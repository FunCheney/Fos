use std::sync::Mutex;
use std::thread;

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10  {
            s.spawn(|| {
               let mut gurad = n.lock().unwrap();
                for _ in 0..100 {
                    *gurad += 1;
                }
            });
        }
    });

    assert_eq!(n.into_inner().unwrap(), 1000);
}