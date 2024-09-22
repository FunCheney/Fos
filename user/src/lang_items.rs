#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let err = panic_info.message().unwrap();
    if let Some(location) = panic_info.location() {
        println!(
            "Paniced at {}:{}, {}",
            location.file(),
            location.line(),
            err
        );
    } else {
        println!("PanicInfo: {}", err);
    }

    loop {}
}
