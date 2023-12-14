struct User {
    active: bool,
    user_name: String,
    email: String,
    sing_in_count: u64
}

#[test]
fn test_01() {
    let user = User {
        active: true,
        user_name: "F.chen".to_string(),
        email: "111111".to_string(),
        sing_in_count: 1,
    };
}

struct Class {
    serial_number: u32,
    grander_number: u32,
    entry_year: String,
    members: Vec<User>,
}

/// 结构体的类型
/// 命名结构体
///    每个字段都有名字的结构体，比如前面提到的 User 每个字段都有明确的名字和类型
#[test]
fn test_02() {

    let active = true;
    let user_name = String::from("F.chen");
    let email = "111111".to_string();
    let sing_in_count = 22;

    let user1 = User {
        active, // 这里本来应该是 active: active,
        user_name,
        email,
        sing_in_count,
    };
    // 若想修改结构体中字段的的值。结构体必须是 mut 类型。
    let mut user2 = User{
        active: true,
        user_name: String::from("fanchen"),
        email: String::from("sdkdkkdk"),
        sing_in_count: 12,
    }
    
    user2.email = String::from("fc_dsg@123.com");
}

/// 元组结构体
/// 就是元组和结构体的结合体

struct Color(u32, u32, u32);
struct Point(u32, u32, u32);
#[test]
fn test_03() {
    let col = Color(1, 32, 44);
    let point = Point(2, 2, 2);
}



/// 单元结构体
/// 单元结构体只有一个类型名字，没有任何字段
/// 在定义和创建的时候可以省略 花括号

struct ArticalModel;

#[test]
fn test_04() {
    let moudle = ArticalModel;
    
}
