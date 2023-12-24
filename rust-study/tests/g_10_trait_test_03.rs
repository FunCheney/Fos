/// 函数要返回不同的类型说起。比如一个常见的需求，要在一个 Rust 函数中返回可能的多种类型，应该怎么写？
// 写成 返回固定类型的函数签名，那么他只能返回那个类型
fn return_type() -> Atype {
    let a = Atype;
    a
}

// 想到的第一个办法是 使用枚举
enum TotalType {
    A(Atype),
    B(Btype),
    C(Ctype),
}

// enum 常用于聚合类型。这些类型之间可以没有任何关系，用 enum 可以无脑 + 强行把它们揉在一起
// 。enum 聚合类型是编码时已知的类型，也就是说在聚合前，需要知道待聚合类型的边界，
// 一旦定义完成，之后运行时就不能改动了，它是封闭类型集。
fn return_type_01(i: i32) -> TotalType { // 返回枚举类型
    if i == 0 {
        let a = Atype;
        TotalType::A(a) // 在这个分支中返回变体A
    } else if i == 1 {
        let b = Btype;
        TotalType::B(b) // 在这个分支中返回变体B
    } else {
        let c = Ctype;
        TotalType::C(c) // 在这个分支中返回变体C
    }
}

// 第二种办法是利用类型参数，我们试着引入一个类型参数，改写一下。
// fn return_type_02<T>() -> T {
//     let a = Atype;
//     a
// }
// 这里编译是无法通过的
// 因为这里这个类型参数 T 是在这个函数调用时指定，而不是在这个函数定义时指定的。
// 所以针对我们的需求，你没法在这里直接返回一个具体的类型代入 T。只能尝试用 T 来返回，于是我们改出第二个版本。

/// 第二个版本
// impl Atype {
//     fn new() -> Atype {
//         Atype
//     }
// }
//
// impl Btype {
//     fn new() -> Btype {
//         Btype
//     }
// }
//
// impl Ctype {
//     fn new() -> Ctype {
//         Ctype
//     }
// }
//
// fn return_type_02<T>() -> T {
//     T::new()
// }
// 这里编译还是无法通过的
// Rustc 小助手并不知道我们定义这个类型参数 T 里面有 new 这个关联函数。
// 联想到我们前面学过的，可以用 trait 来定义这个协议，让 Rust 认识它。
/// 第三个版本：
/// 这个版本顺利编译通过了。
/// 在这个示例中，我们认识到了引入 trait 的必要性，就是让 Rustc 小助手知道我们在协议层面有一个 new() 函数，
/// 一旦类型参数被 trait 约束后，它就可以去 trait 中寻找协议定义的函数和方法。
trait TraitD {
    fn new() -> Self;    // TraitA中定义了new()函数
}

impl TraitD for Atype {
    fn new() -> Atype {
        Atype
    }
}

impl TraitD for Btype {
    fn new() -> Btype {
        Btype
    }
}

impl TraitD for Ctype {
    fn new() -> Ctype {
        Ctype
    }
}

fn return_type_02<T: TraitD>() -> T {
    T::new()
}

#[test]
fn test() {
    let a: Atype = return_type_02::<Atype>();
    let b: Btype = return_type_02::<Btype>();
    let c: Ctype = return_type_02::<Ctype>();
}

/// 为了解决上面那个问题，我们真的是费了不少力气。
/// 实际上，Rust 提供了更优雅的方案来解决这个需求。
/// Rust 利用 trait 提供了一种特殊语法 impl trait，你可以看一下示例。
struct Atype;

struct Btype;

struct Ctype;

trait TraitA {}

impl TraitA for Atype {}

impl TraitA for Btype {}

impl TraitA for Ctype {}

fn return_type_03() -> impl TraitA { // 注意这一行的函数返回类型
    let a = Atype;
    a
    // 或
    // let b = Btype;
    // b
    // 或
    //
    // let c = Ctype;
    // c
}
// 可以看到，这种表达非常简洁，同一个函数签名可以返回多种不同的类型，
// 并且在函数定义时就可以返回具体的类型的实例。更重要的是消除了类型参数 T。
// 上述代码已经很有用了，但是还是不够灵活，比如我们要用 if 逻辑选择不同的分支返回不同的类型，就会遇到问题。

// fn return_type_04(i: i32) -> impl TraitA {
//     if i == 0 {
//         let a = Atype;
//         a // 在这个分支中返回类型a
//     } else if i == 1 {
//         let b = Btype;
//         b // 在这个分支中返回类型b
//     } else {
//         let c = Ctype;
//         c // 在这个分支中返回类型c
//     }
// }
// 这里编译会报错
// if else 要求返回同一种类型，Rust 检查确实严格。不过我们可以通过加 return 跳过 if else 的限制。

// fn return_type_05(i: i32) -> impl TraitA {
//     if i == 0 {
//         let a = Atype;
//         return a; // 在这个分支中返回类型a
//     } else if i == 1 {
//         let b = Btype;
//         return b; // 在这个分支中返回类型b
//     } else {
//         let c = Ctype;
//         return c; // 在这个分支中返回类型c
//     }
// }
// 编译报错
// 它说期望 Atype，却得到了 Btype。这个报错其实有点奇怪，它们不是都满足 impl TraitA 吗？
// impl TraitA 作为函数返回值这种语法，其实也只是指代某一种类型而已，
// 而这种类型是在函数体中由返回值的类型来自动推导出来的。
// 例子中，Rustc 小助手遇到 Atype 这个分支时，就已经确定了函数返回类型为 Atype，
// 因此当它分析到后面的 Btype 分支时，就发现类型不匹配了。问题就在这里。你可以将条件分支顺序换一下，看一下报错的提示，加深印象。

// Rust 还给我们提供了进一步的措施：trait object。形式上，
/// trait Object
/// 形式上，就是在 trait 名前加 dyn 关键字修饰，在这个例子里就是 dyn TraitA。
/// dyn TraitName 本身就是一种类型，它和 TraitName 这个 trait 相关，但是它们不同，dyn TraitName 是一个独立的类型。

// fn return_type_06(i: u32) -> dyn TraitA { // 注意这里的返回类型换成了 dyn TraitA
//     if i == 0 {
//         let a = Atype;
//         return a
//     } else if i == 1 {
//         let b = Btype;
//         return b
//     } else {
//         let c = Ctype;
//         return c
//     }
// }
// 编译报错
/// ----------------------------------------------------------------------------------------
// fn return_type_06(i: u32) -> dyn TraitA { // 注意这里的返回类型换成了 dyn TraitA
//     |                              ^^^^^^^^^^ doesn't have a size known at compile-time
//     |
// help: return an `impl Trait` instead of a `dyn Trait`, if all returned values are the same type
//     |
// 173 | fn return_type_06(i: u32) -> impl TraitA { // 注意这里的返回类型换成了 dyn TraitA
//     |                              ~~~~
// help: box the return type, and wrap all of the returned values in `Box::new`
//     |
// 173 ~ fn return_type_06(i: u32) -> Box<dyn TraitA> { // 注意这里的返回类型换成了 dyn TraitA
// 174 |     if i == 0 {
// 175 |         let a = Atype;
// 176 ~         return Box::new(a)
// 177 |     } else if i == 1 {
// 178 |         let b = Btype;
// 179 ~         return Box::new(b)
// 180 |     } else {
// 181 |         let c = Ctype;
// 182 ~         return Box::new(c)
//     |
///------------------------------------------------------------------------------------------------
/// 然后给出了第一个建议：
///    你可以用 impl TraitA 来解决，前提是所有分支返回同一类型。 return_type_03 中已经测试过了 ✅
///    随后给出了第二个建议，你可以用 Box 把 dyn TraitA 包起来。
///
/// ==============================================================================================

/// 这里我们引入了一个新的东西 Box。Box 的作用是可以保证获得里面值的所有权，必要的时候会进行内存的复制，
/// 比如把栈上的值复制到堆中去。一旦值到了堆中，就很容易掌握到它的所有权。
/// 具体到这个示例中，因为 a、b、c 都是函数中的局部变量，这里如果返回引用 &dyn TraitA 的话是万万不能的，因为违反了所有权规则。
/// 而 Box 就能满足这里的要求。后续我们在智能指针一讲中会继续讲解 Box。
fn doit(i: u32) -> Box<dyn TraitA> {
    if i == 0 {
        let a = Atype;
        Box::new(a)
    } else if i == 1 {
        let b = Btype;
        Box::new(b)
    } else {
        let c = Ctype;
        Box::new(c)
    }
}
