fn main() {
    println!("Hello, world!");
    struct User{
        name: String,
        count: String,
        noce: u64,
        active: bool,
    }

    let mut xiaoming = User{
        name: String::from("Fchen"),
        count: String::from("8001000111"),
        noce: 10000,
        active: true,
              
    };
    xiaoming.name = String::from("xiaoming");

    // 修改名字和字段名的简写方法
    let name = String::from("xiaoxio");
    let count = String::from("99999999");
    let noce = 100;
    let active = false;

    //let user1 = User {
    //    name: name,
    //    count: count,
    //    noce: noce,
    //    active: active,
    //};


}
