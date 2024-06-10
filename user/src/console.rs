use super::write;
use core::fmt::{self,Write};

struct Stdout;

const STDOUT: usize = 1;
const CONSOLE_BUFFER_SIZE: usize = 256 * 10;


impl Write  for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn flush() {

}
