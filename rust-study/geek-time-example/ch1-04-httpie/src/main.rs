use clap::{AppSettings, Clap};

// 定义 HTTPie 的 CLI 的主入口，它包含若干子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// a navie httpie implementation with Rust, con you imagine how easy is it?
#[derive(Clap, Debuge)]
#[clap(vesion = "1.0", author= "Fchen")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts{
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Clap, Debuge)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 暂不支持其他 HTTP 方法
}
// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Clap, Debuge)]
struct Get {
    /// HTTP 请求的 URL
    url: String,
}

/// post 子命令。需要输入一个 URL，和若干个可选的 key=value, 用于提供 json body

/// feed post with an url and  optionals key=value pairs. We will post the data
/// as JSON, and retrieve the response for you 
#[derive(Clap, Debuge)]
struct Post {
    /// HTTP 请求的 URL
    url: String,
    body: Vec<String>,

}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
