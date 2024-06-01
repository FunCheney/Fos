
mod my{
    // 一个公有的结构体，带有一个共有字段（类型为泛型 T）
    pub struct OpenBox<T> {
        pub context: T,
    }
    // 一个公共的结构体，带有一个私有字段（类型为泛型 T）
    #[allow(dead_code)]
    pub struct ClosedBox<T>{
        context: T,
    }

    impl <T> ClosedBox<T> {
        // 构造一个公有的构造方法
        pub fn new(context: T) -> ClosedBox<T>{
            ClosedBox{
                context: context,
            }
        }
    }
}

#[test]
fn test_01() {
    // 带有公有字段的公有结构体，可以像平常一样构造
    let open_box = my::OpenBox{context: "pub context" };
    // 并且它们的字段可以正常访问到。
    println!("The open box context: {}", open_box.context);
    // 带有私有字段的公有结构体，不能通过类名来访问
    // let close_boc = my::ClosedBox{context: "private context"};

    // 不过带有私有字段的公有结构体，可以使用公有的构造器来访问
    let _closed_box = my::ClosedBox::new("classified information ");
    // 但是，一个结构体的私有字段不能访问到
    // println!("_closed_box filed {}", _closed_box.context);
}