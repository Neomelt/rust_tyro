// 第 22 节：TCP 异步通信——客户端。
// 先在另一个终端运行 22_tcp_server，再运行本例：
// cargo run --example 22_tcp_client
// 可选地址：cargo run --example 22_tcp_client -- 127.0.0.1:7879

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("127.0.0.1:7878"));

    // ① 连接服务端；如果对方还没启动，这里会返回连接错误。
    let mut stream = TcpStream::connect(&address).await?;
    let message = "你好，Tokio！这是客户端发来的消息。";

    // ② 网络收发的是字节，as_bytes() 把字符串借用为一段字节。
    stream.write_all(message.as_bytes()).await?;
    println!("已发送：{message}");

    // ③ 只关闭发送方向，表示“我不会再发了”，仍然可以继续接收服务端的回送。
    // 服务端读完后会读到 0，结束回送并关闭它的发送方向。
    stream.shutdown().await?;

    // ④ 持续接收，直到服务端不再发送；不会假设一次 read 就能读完所有内容。
    let mut reply = Vec::new();
    stream.read_to_end(&mut reply).await?;
    let reply = String::from_utf8(reply).map_err(io::Error::other)?;
    println!("收到回声：{reply}");

    // 只在成功解码后展示文本；收到无效的 UTF-8 时，上面会返回错误。
    Ok(())
}

// 这里用“发送方向关闭”表示本次数据已经发完。
// 如果双方都不关闭发送方向，却都在 read_to_end，就可能一直等待对方结束。
// 长连接通常要约定消息边界，而不是每发送一条消息就关闭发送方向。
