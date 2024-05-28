use core::panic::PanicInfo;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    if let Some(location) = _info.location(){
        println!(
            "PanicInfo at {}:{} {}",
            location.file(), location.line(),
            _info.message().unwrap()
            );
    }else {
        println!("Panicked: {}", _info.message().unwrap());
    }
    shutdown(true);
}
