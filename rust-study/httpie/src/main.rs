extern crate clap;
use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use reqwest::Url;
use std::str::FromStr;

// 定义 HTTPie 的 CLI 主入口，包含多个命令
// 下面 /// 的注释是文档， clap 会将其当成是 CLI 的帮助

/// A naive httpie implementation wite Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// 子命令分别对应不同的 HTTP 方法，暂时只支持 GET / POST 方法
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(crate::SubCommand),
}

#[derive(Parser, Debug)]
struct Get {
    #[arg(value_parser=parse_url)]
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    #[arg(value_parser=parse_url)]
    url: String,
    #[arg(value_parser=parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Debug, Clone, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

fn parse_url(s: &str) -> Result<String> {
    // check url
    let _url: Url = s.parse()?;

    Ok(s.into())
}

fn main() {
    let opts = Opts::parse();

    println!("{:?}", opts);
}