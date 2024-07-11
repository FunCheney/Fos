use crate::{sbi::shutdown, stack_trace::print_stack_trace};
use core::panic::PanicInfo;
use log::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        error!(
            "PanicInfo at {}:{} {}",
            location.file(),
            location.line(),
            _info.message().unwrap()
        );
    } else {
        error!("Panicked: {}", _info.message().unwrap());
    }
    unsafe {
        print_stack_trace();
    }
    shutdown(true)
}
