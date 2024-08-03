use crate::{sbi::shutdown, stack_trace::print_stack_trace};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        println!(
            "PanicInfo at {}:{} {}",
            location.file(),
            location.line(),
            _info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", _info.message().unwrap());
    }
    unsafe {
        print_stack_trace();
    }
    shutdown(true)
}
