use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

#[test]
fn test_01() {
    // File::open 的返回值是 Result<T, E>
    // 泛型参数 T 会被 File::open 的实现放入成功返回值的类型 std::fs::File，这是一个文件句柄。
    // 错误返回值使用的 E 的类型是 std::io::Error
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

}

#[test]
fn test_02() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {e:?}"),
        },
        other_error => {
            panic!("Problem opening the file: {other_error:?}");
        }
    });
}

fn read_user_name_for_file() -> Result<String, io::Error> {
    let read_file_result = File::open("hello.txt");

    let mut read_file = match read_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut user_name = String::new();
    match read_file.read_to_string(&mut user_name) {
        Ok(_) => Ok(user_name),
        Err(e) => Err(e),
    }
}

#[test]
fn test_03() {
    read_user_name_for_file().expect("error");
}


fn read_user_name_for_file2() -> Result<String, io::Error> {
    // Result 值之后的 ? 被定义为与示例 read_user_name_for_file 中定义的处理 Result 值的 match 表达式有着完全相同的工作方式。
    // 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
    // 如果值是 Err，Err 将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。
    let mut read_file_result = File::open("hello.txt")?;
    let mut user_name = String::new();
    read_file_result.read_to_string(&mut user_name)?;
    Ok(user_name)
}