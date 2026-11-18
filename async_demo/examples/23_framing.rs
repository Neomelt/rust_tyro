// 第 23 节：消息边界与缓冲——用换行符分隔消息。
// 在 async_demo 目录运行：cargo run --example 23_framing
// 本例可以独立运行，不需要先启动第 22 节的服务端。

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> io::Result<()> {
    // ① 在内存里创建一对连接着的字节流，模拟通信两端。
    // 8 是每个方向的缓冲容量，不是一条消息的长度。
    // 用小容量练习分段读取，避免依赖操作系统碰巧怎样拆分 TCP 数据。
    let (mut writer, reader) = io::duplex(8);

    let sender = tokio::spawn(async move {
        // ② 先发送一条消息的前半部分，没有换行符，消息还不完整。
        writer.write_all("温度：".as_bytes()).await.unwrap();
        println!("发送端：已发出半条消息");
        sleep(Duration::from_millis(200)).await;

        // 补上第一条的后半部分，同时再发一条完整消息。
        // 我们自己约定：每遇到一个 \n，才表示一条消息结束。
        writer
            .write_all("25℃\n湿度：60%\n".as_bytes())
            .await
            .unwrap();
        writer.shutdown().await.unwrap();
    });

    // ③ BufReader 保存还没消费完的字节，lines() 按换行符逐条取出文本。
    // next_line() 会等待一整行，而不是把每次读到的零散字节直接当成消息。
    let mut lines = BufReader::new(reader).lines();
    let mut count = 0;

    while let Some(message) = lines.next_line().await? {
        count += 1;
        // 取出的字符串已经去掉行尾的换行符。
        println!("接收端得到第 {count} 条完整消息：{message}");
    }

    sender.await.unwrap();
    println!("发送端已结束，共收到 {count} 条消息");
    Ok(())
}

// 输出应包含两条完整消息：“温度：25℃”和“湿度：60%”。
// 这里的等待只是方便观察；判断消息结束靠换行符，不靠 sleep 的时间。
// lines() 也可以配合 BufReader<TcpStream> 使用，规则由通信双方事先约定。
// 注意：lines() 会把 EOF 前最后一段没有换行符的文本也作为最后一行返回。
// 如果协议要求每条消息必须以换行结束，就需要额外检查；本例发送的两条都有换行。
// 按行读取可能持续累积数据，面向不可信的通信对象时还要设置消息长度上限。
