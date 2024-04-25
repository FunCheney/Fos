use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;
use clap::Parser;
use anyhow::{anyhow, Result};
use colored::Colorize;
use mime::Mime;
use reqwest::{Client, header, Response, Url};
/// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
/// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

///  A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "FChen")]
struct Opts {
    #[clap(subcommand)]
    sub_cmd: SubCommand
}
// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}
// get 子命令
#[derive(Parser, Debug)]
struct Get {
    // http 请求的 url
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    // 这里们检查一下 URL 是否合法
    let _url: Url = s.parse()?;
    Ok(s.into())
}


// post 子命令 需要输入一个 URL，和若干个可选的 key=value，用于提供 json body
#[derive(Parser, Debug)]
struct Post {
    // post 请求 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    // 请求 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}
/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
struct KvPair {
    key: String,
    value: String,
}
// 为 KvPair 实现 FromStr 后可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // 使用 = 号进行 split 会的到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取出第一个结果作为 Key， 返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            key: (split.next().ok_or_else(err)?).to_string(),
            // 第二个结果作为 value
            value: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}
/// 处理 get 子命令
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}
/// 处理 post 子命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.key, &pair.value);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}

// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}
// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}
/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        // 其它 mime type，我们就直接输出
         _ => println!("{}", body), }
}
/// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp); print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}
/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime>{
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<()>{
    let opts: Opts = Opts::parse();
    // 生成一个 http 客户端
    let client = Client::new();

    let result = match opts.sub_cmd {
        SubCommand::Get(ref args) =>  get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

// 仅在 cargo test 时才编译
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }
    #[test] fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());

        assert_eq!( parse_kv_pair("a=1").unwrap(),
                    KvPair {
                        key: "a".into(),
                        value: "1".into()
                    }
        );
        assert_eq!( parse_kv_pair("b=").unwrap(),
                    KvPair {
                        key: "b".into(),
                        value: "".into()
                    }
        );
    }
}

