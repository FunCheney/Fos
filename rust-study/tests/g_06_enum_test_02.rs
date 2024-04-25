/// impl 枚举. 不能对枚举的变体进行 impl

enum MyEnum {
    Add,
    Subtract,
}

impl MyEnum {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => {x + y},
            Self::Subtract => {x - y},
        }
    }
}
#[test]
fn test_01() {
    // 实例化枚举
    let add = MyEnum::Add;
    // 执行枚举的方法
    add.run(14, 34);
}

/// 枚举的变体不能实例化
enum Foo {
    AAA,
    BBB,
    CCC,
}

impl Foo::AAA {}