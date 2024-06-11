use std::fmt::{Debug, Display};
use std::slice::IterMut;

/// trait 定义了与某个特定类型拥有可能与其他类型共享的功能
/// 可以通过 trait 以一种抽象的方式定义共同行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

/// 定义 trait
// 一个类型的方法可由其可供调用的方法构成，如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
// trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[test]
fn test_01() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

/// trait 做为参数
/// 如何使用 trait 来接受多种不同类型的参数
pub fn notify(item : &impl Summary){
    // 对于 item 参数，我们指定了 impl 关键字和 trait 名称，而不是具体的类型
    // 该参数支持任何实现了指定 trait 的类型
    println!("Breaking news! {}", item.summarize());
}

/// trait Bound 语法
pub fn notify2<T: Summary> (item: &T){
    /// trait bound 与泛型参数声明在一起，位于尖括号中的冒号后面。
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {

}

pub fn notify4<T: Summary>(item: &T, item2: &T){

}
/// 通过 + 指定多个 trait bound
// 如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，
// 那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现：
pub fn notify5(item: &impl Summary + Display) {

}

pub fn notify6<T: Summary + Display> (item: &T){

}

/// 通过 where 简化 trait bound
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    10
}

fn some_function2<T,U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Display {

    10
}

/// 返回实现了 trait 的类型
fn returns_summarize_able() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn returns_summarize_able1(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

/// 使用 trait bound 有条件地实现方法
/// 通过使用带有 trait bound 的泛型参数的 impl 块。可以有条件地只为那些实现了特定 trait 的类型实现方法

struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self { x, y }
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


/// 也可以对任何实现了特定 trait 的类型有条件地实现 trait。
/// 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations，它们被广泛的用于 Rust 标准库中。
// impl<T: Display> ToString for T {
//     // --snip--
// }
//


