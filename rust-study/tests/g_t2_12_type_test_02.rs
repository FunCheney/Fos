use std::fmt::{Debug, Formatter};
use std::io::Write;
use regex::Regex;

/// 特设多态：包括运算符重载，是指同一种行为有很多不同的实现；
/// 子类型多态：而把子类型当成父类型使用，比如 Cat 当成 Animal 使用，属于子类型多态。
///
/// trait 是 rust 中的接口，它定义了类型使用这个接口的行为。
///
// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize>;
//     fn flush(&mut self) -> Result<()>;
//     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize> { ... }
//     fn is_write_vectored(&self) -> bool { ... }
//     fn write_all(&mut self, buf: &[u8]) -> Result<()> { ... }
//     fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> Result<()> { ... }
//     fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()> { ... }
//     fn by_ref(&mut self) -> &mut Self where Self: Sized { ... }
// }
// # Self 代表当前的类型，比如 File 类型实现了 Write，那么实现过程中使用到的 Self 就指代 File
// # Self 代表当前的类型，比如 File 类型实现了 Write，那么实现过程中使用到的 Self 就指代 File

struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl Debug for BufBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {

    // 实现 write 方法
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
       // 把 buf 添加到 BufBuilder 的尾部
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }
    // 实现 flush 方法
    fn flush(&mut self) -> std::io::Result<()> {
        // 内存操作不需要 flush
        Ok(())
    }
}

#[test]
fn test_01() {
    let mut builder = BufBuilder::new();
    builder.write(b"hello world").unwrap();
    println!("{:?}", builder);
}

pub trait Parse{
   // parse 方法是一个静态方法，第一个参数与 self 无关，所以在调用时需要使用 T::parse(str) 。
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re:Regex = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = re.captures(s) {
            captures.get(0).map_or(0, |s| s.as_str().parse().unwrap_or(0))
        }else{
            0
        }
    }
}

#[test]
fn test_02() {
    assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(u8::parse("abcd"), 0);
}





