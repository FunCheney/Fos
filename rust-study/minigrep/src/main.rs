use std::{env, fs, process};
use std::error::Error;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // // dbg!(args);
    // // let config = parse_config(&args);
    // let config = Config::build(&args)
    //     // unwrap_or_else 它定义于标准库的 Result<T, E> 上。
    //     // 使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理。
    //     // 当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。
    //     // 然而，当其值是 Err 时，该方法会调用一个 闭包（closure），
    //     // 也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数
    //     .unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
    // println!("Searching for: {}", config.query);
    // println!("In file: {}", config.file_path);
    //
    // // if let 来检查 run 是否返回一个 Err 值，不同于 unwrap_or_else，并在出错时调用 process::exit(1)
    // // run 并不返回像 Config::build 返回的 Config 实例那样需要 unwrap 的值。
    // // 因为 run 在成功时返回 ()，而我们只关心检测错误，所以并不需要 unwrap_or_else 来返回未封装的值，因为它只会是 ()。
    // if let Err(e) = run(config){
    //     println!("Application error: {e}");
    //     process::exit(1);
    // };
    let args: Vec<String> = env::args().collect();
    let  config = Config::build(&args).unwrap_or_else( |err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {

    }
}

// fn run(config: Config) {
    // 读取指定的文件内容
    // read_to_string() 接受 file_path, 打开文件，接着返回其包含内容的  std::io::Result<String>
    // let contents = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");


    // println!("With text:\n{contents}");
// }

// 将 run 函数的返回类型变为 Result<(), Box<dyn Error>>
// 之前这个函数返回 unit 类型 ()，现在它仍然保持作为 Ok 时的返回值。
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     // 去掉了 expect 调用并替换为  ?。不同于遇到错误就 panic!，? 会从函数中返回错误值并让调用者来处理它。
//     let contents = fs::read_to_string(config.file_path)?;
//     println!("With text:\n{contents}");
//     // 现在成功时这个函数会返回一个 Ok 值。因为 run 函数签名中声明成功类型返回值是 ()，这意味着需要将 unit 类型值包装进 Ok 值中
//     Ok(())
// }

// struct Config {
//     query: String,
//     file_path: String,
// }
//
// fn parse_config(args: &Vec<String>) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     Config {query, file_path}
// }

// impl Config {
//     // 在成功时带有一个 Config 实例而在出现错误时带有一个 &'static str
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         Ok(Config{query, file_path})
//     }
// }
