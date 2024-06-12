use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let config = parse_config(&args);

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    // 读取指定的文件内容
    // read_to_string() 接受 file_path, 打开文件，接着返回其包含内容的  std::io::Result<String>
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config {query, file_path}
}
