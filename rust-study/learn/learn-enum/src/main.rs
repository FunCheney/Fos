
    enum  IpAddrKind{
        v4,
        v6,
    };

    struct IpAddr{
        kind: IpAddrKind,
        address: String,
    };

fn main() {

    let i1 = IpAddr{
        kind: IpAddrKind::v4,
        address: String::from("::1"),
    };

    let i2 = IpAddr{
        kind: IpAddrKind::v6,
        address: String::from("127.0.0.1"),
    };

    enum IpAddr2{
        v4(String),
        v6(String),
    };

    let i3 = IpAddr2::v4(String::from("127.0.0.1"));

    // 可以是不同的类型
    enum IpAddr3{
        v4(u8, u8, u8, u8),
        v6(String),
    };

    let i1 = IpAddr3::v4(127, 0, 0, 1);
    let i2 = IpAddr3::v6(String::from("::1"));

    // 4. 经典用法
    enum Message {
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        Change(i32, i32, i32),
    };
    // 等同于
    // struct QuitMessage 结构体
    // struct MoveMessage{
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage()
    // struct Change(i32, i32, i32)
    
    
    //枚举类型方法






    println!("Hello, world!");
}
