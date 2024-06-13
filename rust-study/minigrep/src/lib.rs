use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // 在成功时带有一个 Config 实例而在出现错误时带有一个 &'static str
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config{query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 去掉了 expect 调用并替换为  ?。不同于遇到错误就 panic!，? 会从函数中返回错误值并让调用者来处理它。
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    // 现在成功时这个函数会返回一个 Ok 值。因为 run 函数签名中声明成功类型返回值是 ()，这意味着需要将 unit 类型值包装进 Ok 值中
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}