enum IpAddrKind {
    V4,
    V6
}
fn route(ip_addr_kind: IpAddrKind){

}

#[test]
fn test_enum_01() {
    let four = IpAddrKind::V4;
    route(IpAddrKind::V6);
    route(four);
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[test]
fn test_02() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

