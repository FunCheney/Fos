use std::fmt::Debug;

/// 在约束中具化关联类型
trait TraitA {
    type Item;
}

struct Foo<T: TraitA<Item=String>>{ // 在约束表达式中对关联类型做了具化
    x: T,
}

struct A;

impl TraitA for A {
    type Item = String;
    // 这里会报错
    // type Item = i32;
}

#[test]
fn test_01() {
    let a = Foo {
        x: A,
    };
}

trait TraitB {
    type Item: Debug; // 这里对关联类型添加了Debug约束
}

#[derive(Debug)]
struct B;

struct C;

impl TraitB for  C {
    type Item = B; // 这里这个类型B已满足Debug约束
}

fn doit<T> () // 定义参数类型 T
where
    T: TraitA, // 使用where语句将T的约束表达放在后面来
    T::Item: Debug + PartialEq // 注意这一句，直接对TraitA的关联类型Item添加了更多一个约束 PartialEq
{}

