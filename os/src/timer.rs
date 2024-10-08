
//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MESC_PER_SEC: usize = 1000;

#[allow(unused)]
const USEC_PER_SEC: usize = 1000000;

pub fn get_time() -> usize {
    time::read()  
}

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MESC_PER_SEC)
}

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

/// 获取当前时间 微秒
#[allow(unused)]
pub fn get_time_us() -> usize{
    time::read() / (CLOCK_FREQ / USEC_PER_SEC)
}
