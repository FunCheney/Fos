/// 结构体所有权
/// 结构体有一种与所有权相关的特性，部分移动
/// 也就是说：结构体中的部分字段是可以被移出去的

#[derive(Debug)]
struct User {
    active: bool,
    user_name: String,
    email: String,
    sing_in_count: u32,
}

/// #derive(Debug) 语法在 rust 中叫做属性标注
/// 具体来说就是派生宏属性，派生宏作用于紧跟着的下面的结构体，可以为结构体自动添加一些功能
/// 派生 Debug 这个宏 可以在 println("{:?}") 中把结构体打印出来，方便调试

#[test]
fn test_01() {
    let active = true;
    let user_name = String::from("FanChen");
    let email = String::from("fc_2012@163.com");
    let user1 = User {
        active,
        user_name,
        email,
        sing_in_count: 1,
    };
    // 这里发生了所有权转移
    let email = user1.email;
    // 这里打印 user1 会报错
    // println!("{:?}", user1);

    // 这些字段涉及可以打印的
    println!("{}", user1.active); // 分别打印另外3个字段
    println!("{}", user1.user_name);
    println!("{}", user1.sing_in_count);

    // 这里无法打印，因为 值已经被移动了
    // println!("{}", user1.email);
}

/// 字段类型为引用类型
/// 但是要加生命周期标注
/// 生命周期标注后面再研究
struct Student<'a>{
    name: &'a str,
}


/// Rust 不是面相对象的语言，但是它有面向对象的特性
/// Rust 承载面向对象特性的主要类型就是结构体
/// Rust 中有个关键字 impl 可以用来给结构体或者其他类型实现方法，也就是关联在某个类型上的函数

/// 实例方法

struct Rectangle {
    width: u32,
    length: u32
}

impl Rectangle {
    fn area(self) -> u32{
        self.width * self.length
    }
}

#[test]
fn test_05() {
    let rec1 = Rectangle {
        width: 12,
        length: 34,
    };

    // 使用 . 号调用 ara() 方法
    println!("rec are is {}", rec1.area())
}

/// 上述的 area 方法中有一个 self，是一个 单 self。
/// self 是 Rust 中的一个语法糖, 完整语法是 self: Self
/// Self 是 Rust 里面一个特殊的类型名，它表示正在被实现的那个类型
/// Rust 中 所有权和借用形式总是成对出现的，在 impl 方法中也是如此

struct Area {
    width: u32,
    length: u32
}
/// 方法是实现在类型上的特殊函数
impl Area {
    // self: Self 传入的是实例的所有权类型
    fn area1(self) -> u32 {
        self.width * self.length
    }
    // self: &Self 传入的是实例的不可变引用
    fn area2(&self) -> u32 {
        self.width * self.length
    }
    // self: &mut Self 传入实例的可变引用
    fn area3(&mut self) -> u32 {
        self.width * self.length
    }

    fn numbers(row: u32, cols: u32) -> u32{
        row * cols
    }

}

/// 关联函数
/// 方法的第一个参数是 self。从函数定义上来说，第一个参数可以不是 self。
/// 如果实现在类型上，且第一个参数不是 self，那么他就叫做类型的关联函数
/// 调用的时候是用 类型::方法名
/// 关联函数与 java 的静态方法有着类似的作用

#[test]
fn test_06() {
    Area::numbers(1,10);
}
/// 结构体实力化
/// ① 构造函数
/// Rust 社区一般约定使用 new() 这个名字的关联函数，像下面这样把类型的实例化包起来。
///
impl Rectangle {
    pub fn new(width: u32, length: u32) -> Self {
        Rectangle {
            width,
            length,
        }
    }
}

#[test]
fn test_07() {
    let rec = Rectangle::new(23, 32);
}

/// ②：Default 实例化
#[derive(Debug, Default)]
struct Test {
    i: u32,
    j: u32,
}

#[test]
fn test_09() {
    let rec1: Test = Default::default();
    let rec2: Test = Test::default();

    println!("{:?}", rec1);
    println!("{:?}", rec2);


}




