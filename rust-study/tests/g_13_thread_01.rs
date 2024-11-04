#[test]
fn test_01() {
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        println!("Task 1 Start");
        let id = 1;
        for i in 0..10 {
            println!("task {} count {}", id, i);
            yield_now();
        }
        println!("Task 1 finish");
    });
}
