// 第 22 节：TCP 异步通信——回声服务端。
// 在 async_demo 目录打开两个终端：
// 终端 A 先运行：cargo run --example 22_tcp_server
// 终端 B 再运行：cargo run --example 22_tcp_client
// 本例只服务一个连接，客户端发送完毕后，服务端也会退出。
// 可选：在命令后加 -- 127.0.0.1:7879 更换地址，客户端也要使用同一个地址。

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 不提供参数时，只监听本机的 7878 端口。
    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("127.0.0.1:7878"));

    // ① 绑定地址并等待一个客户端连接。
    // ? 的意思是：成功就取出值，失败就把错误返回给调用者，不在这里 unwrap。
    let listener = TcpListener::bind(&address).await?;
    println!("服务端监听：{}", listener.local_addr()?);
    let (mut stream, peer) = listener.accept().await?;
    println!("客户端已连接：{peer}");

    let mut buffer = [0_u8; 1024];
    let mut total = 0;

    loop {
        // ② read 把字节读进 buffer，返回这次真正读到了多少字节。
        let count = stream.read(&mut buffer).await?;
        if count == 0 {
            // 客户端关闭发送方向，而且已发送的字节都读完了，才会读到 0。
            break;
        }

        // ③ 只发送本次读到的那一段，不能把整个 buffer 都写回去。
        // write_all 会把这一段全部写出，不要求一次底层写入就完成。
        stream.write_all(&buffer[..count]).await?;
        total += count;
        println!("本次回送 {count} 字节");
    }

    // ④ 关闭发送方向，让客户端知道回送的数据已经全部发完。
    stream.shutdown().await?;
    println!("本次共回送 {total} 字节，服务端结束");
    Ok(())
}

// TCP 传递的是字节流，一次 read 不等于对方的一次 write，也不等于一条完整消息。
// 本节直接逐段回送字节，不需要判断消息边界；第 23 节再练习如何拼成完整消息。
