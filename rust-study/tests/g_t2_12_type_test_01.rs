use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read, Result};
///  按照类型的检查时机，在编译时检查还是在运行是检查可以将系统分为静态类型系统还是动态类型系统
///  对于静态类型系统可以进一步分为显示静态和隐式静态、rust/java 都属于显示静态语言
///
///  在类型系统中多态是一个很重要的思想他是指在使用相同的接口时，不同类型的对象可以采用不同的实现
///
///  静态类型多态系统可以分为
///     参数多态:  代码的操作类型是一个满足某些约束的参数
///     特设多态:  同一个行为有多个不同的实现多态，比如加法，可以 1+1，也可以是 “abc” + “cde”、matrix1 + matrix2、甚至 matrix1 + vector1。在面向对象编程语言中，特设多态一般指函数的重载。
///     子类型多态: 在运行时子类型可以被当做父类型使用
// 在 rust 中参数多态和特是个多态 通过 trait 来支持
// 子类型多态通过使用 trait object 来实现

/// 类型安全
/// 从内存的角度来看，类型安全是指代码，只能按照被允许的方法，访问它被授权访问的内存
///在此基础上，Rust 还进一步对内存的访问进行了读 / 写分开的授权。所以
///，Rust 下的内存安全更严格：代码只能按照被允许的方法和被允许的权限，访问它被授权访问的内存。
///
///rust 类型系统提供了类型推导

// 在一个作用域内，rust 可以通过上下文，推导出变量的类型
#[test]
fn test_01() {
    let mut map = BTreeMap::new();
    map.insert("hello", "world");
    println!("map: {:?}", map)
}

#[test]
fn test_02() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // 这里没有名曲类型是无法通过类型推导来推导出类型
    let nums: Vec<_> = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect::<Vec<_>>(); // 在泛型函数后使用 :: 来强制使用类型 T，这种写法被称为 turbofish
    println!("{:?}", nums);
}

/// 用泛型实现参数多态
/// 参数多态，包含泛型数据结构和泛型函数
// 泛型数据结构
// rust 对数据结构的泛型或参数化类型有着完整的支持
/// pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
///{
///    // 借用的数据
///    Borrowed(&'a B),
///    // 拥有的数据
///    Owned(<B as ToOwned>::Owned),
///}
//  它就像 Option 一样，在返回数据的时候，提供了一种可能：要么返回一个借用的数据（只读），要么返回一个拥有所有权的数据（可写）。
//  对于拥有所有权的数据 B ，第一个是生命周期约束。这里 B 的生命周期是 'a，所以 B 需要满足 'a，这里和泛型约束一样
//  ，也是用 B: 'a 来表示。当 Cow 内部的类型 B 生命周期为 'a 时，Cow 自己的生命周期也是 'a。
//   B 还有两个约束：?Sized 和 “where B: ToOwned”。
//   1. 在表述泛型参数的约束时，Rust 允许两种方式，一种类似函数参数的类型声明，
//   用 “:” 来表明约束，多个约束之间用 + 来表示；另一种是使用 where 子句，在定义的结尾来表明参数的约束。两种方法都可以，且可以共存。
//   2. ?Sized 是一种特殊的约束写法，? 代表可以放松问号之后的约束。由于 Rust 默认的泛型参数都需要是 Sized，也就是固定大小的类型，所以这里 ?Sized 代表用可变大小的类型。
//


// 定义一个带有泛型参数 R 的 reader，此处我们不限制 R
struct MyReader<R> {
    reader: R,
    buf: String,
}

// 实现 new 函数不需要限制 R

impl<R> MyReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

// 定义 process 时，我们需要用到 R 的方法，此时我们限制 R 必须为 Read Trait

impl<R> MyReader<R>
where
    R: Read,
{
    pub fn process(&mut self) -> Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}

#[test]
fn test_03() {
    let f = File::open("/etc/hosts").unwrap();
    let mut reader = MyReader::new(BufReader::new(f));
    let size = reader.process().unwrap();
    println!("total size read: {}", size);
}

/// 泛型函数

fn id<T>(x: T) -> T {
    x
}

/// 对于泛型函数的处理，rust 会进行单态化处理，也就是在编译时，把所有用到的泛型函数的泛型参数展开，
/// 生成若干个函数。这样的好处是，泛型函数的调用是静态分派。在编译时就一一对应，即保证了 多态的灵活，有没有效率的损失
/// 单态化有很明显的坏处，就是编译速度很慢，一个泛型函数，编译器需要找到所有用到的不同类型，
/// 一个个编译，所以 Rust 编译代码的速度总被人吐槽，这和单态化脱不开干系（另一个重要因素是宏）。
/// 这样编出来的二进制会比较大，因为泛型函数的二进制代码实际存在 N 份。
///
/// 因为单态化，代码以二进制分发会损失泛型的信息。 /////
#[test]
fn test_04() {
    let int = id(10);
    let string = id("Tyr");
    println!("{}, {}", int, string);
}

