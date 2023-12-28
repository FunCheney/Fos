use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".to_string());
    println!("addr {}", addr);
    let listener = TcpListener::bind(&addr).await?;
    // 这里是一个循环，表明始终处于服务的状态
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move{
            // 创建一个缓冲区
            let mut buf = [0;1024];
            let mut offset = 0;
            // 循环读, 因为不能保证一次性从网络上读取到完整数据
            loop {
                let n =socket
                    .read(&mut buf[offset..])
                    .await
                    .expect("failed to read data from socket");

                // 返回 n = 0 的情况。是碰到了 EOF，表明远端写操作已经断开，这里要判断
                if n == 0 {
                    // 碰到了EOF就直接返回结束此任务，因为后面的操作没了意义
                    return;
                }
                println!("offset {} n {}", offset, n);
                let end = offset + n;
                // 转换指令为字符串
                if let Ok(directive) = std::str::from_utf8(&buf[..end]){
                    println!("{directive}");
                    let output = process(directive).await;
                    println!("{output}");
                    // 向客户端返回处理结果
                    socket.write_all(&output.as_bytes()).await.expect("failed to write data to socket");
                } else {
                    // 判断是否转换失败，如果失败，就有可能是网络上的数据还没读完
                    // 要继续loop读下一波数据
                    offset = end;
                }

            }
        });

    }
}

async fn process(directive: &str) -> String {
    if directive == "gettime" {
        // 这里我们用了unwrap()是因为我们一般确信执行date命令不会失败
        // 更可靠的做法是对返回的Result作处理
        let output = Command::new("date").output().await.unwrap();
        String::from_utf8(output.stdout).unwrap()
    }else{
        // 如果是其他指令，我们目前返回 无效指令
        "invalid command".to_owned()
    }
}
