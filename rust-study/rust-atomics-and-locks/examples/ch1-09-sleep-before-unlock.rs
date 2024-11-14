use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {

        s.spawn(|| {
            for _ in 0..10 {
                let mut gurad = n.lock().unwrap();
                for _ in 0..100  {
                    *gurad += 1;
                }
                thread::sleep(Duration::from_secs(1));
            }
        });
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}