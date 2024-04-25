trait TraitA {}
trait TraitB {}
trait TraitC {}

struct A;
struct B;
struct C;

impl TraitA for A {}
impl TraitB for A {}
impl TraitC for A {}  // 对类型A实现了TraitA, TraitB, TraitC
impl TraitB for B {}
impl TraitC for B {}  // 对类型B实现了TraitB, TraitC
impl TraitC for C {}  // 对类型C实现了TraitC

// 7个版本的doit() 函数
fn doit1<T: TraitA + TraitB + TraitC>(t: T) {}
fn doit2<T: TraitA + TraitB>(t: T) {}
fn doit3<T: TraitA + TraitC>(t: T) {}
fn doit4<T: TraitB + TraitC>(t: T) {}
fn doit5<T: TraitA>(t: T) {}
fn doit6<T: TraitB>(t: T) {}
fn doit7<T: TraitC>(t: T) {}
#[test]
fn test() {
    doit1(A);
    doit2(A);
    doit3(A);
    doit4(A);
    doit5(A);
    doit6(A);
    doit7(A);  // A的实例能用在所有7个函数版本中

    doit4(B);
    doit6(B);
    doit7(B);  // B的实例只能用在3个函数版本中

    doit7(C);  // C的实例只能用在1个函数版本中
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {    // 第一次 impl
fn new(x: T, y: T) -> Self {
    Self { x, y }
}
}

impl<T: Display + PartialOrd> Pair<T> {  // 第二次 impl
fn cmp_display(&self) {
    if self.x >= self.y {
        println!("The largest member is x = {}", self.x);
    } else {
        println!("The largest member is y = {}", self.y);
    }
}
}