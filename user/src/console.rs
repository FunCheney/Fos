/// console.rs
use super::write; // 指向 lib.rs 中的 write 方法
use core::fmt::{self,Write};

struct Stdout;

const STDOUT: usize = 1;

impl Write  for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 基于 lib.rs 中的 write 方法实现
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn flush() {

}

pub fn print(args: fmt::Arguments) {
    //  基于 core 中的 write_fmt 实现
    Stdout.write_fmt(args).unwrap();
}
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
     ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}
