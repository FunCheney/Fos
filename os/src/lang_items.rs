use core::panic::PanicInfo;

#[panic_handler]
fn PanicInfo(_info: &PanicInfo) -> ! {
    loop{}
}
