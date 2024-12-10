use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    TcpStream::connect("").await?;

    Ok(())
}