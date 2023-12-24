#[test]
fn test() {
    let data = vec![1, 2, 3, 4];
    let data1 = data;
    println!("sum of data1: {}", sum(data1));
    // 所有权已经被转移
    //println!("data1 {:?}", data1);
    // 所有权被转以后 又被使用
    // println!("sum of data:{} ", sum(data));
}

fn sum(data: Vec<i32>) -> i32 {
    data.iter().fold(0, |acc, x| acc + x)
}
/// 进行变量赋值、传参和函数返回时，如果涉及的数据结构没有实现 Copy trait
/// ，就会默认使用 Move 语义转移值的所有权，失去所有权的变量将无法继续访问原来的数据.
/// borrow 语义，允许一个值的所有权在不发生转移的情况下，被其他上下文使用
/// Borrow 语义通过引用语法（& 或者 &mut）来实现。

#[test]
fn test_01() {
    let data = vec![1, 2, 3, 5];
    let data1 = &data;
    // 值的地址是什么，引用的地址是什么
    println!(
        "addr of value {:p} {:p}, addr of data {:p}, addr of data1 {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1 {}", sum_01(data1));

    // 堆上的地址是什么
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}
fn sum_01(data: &Vec<i32>) -> i32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}
