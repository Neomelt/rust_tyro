use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(4);

    // 复制一个发送端，两个发送端连接的是同一个通道
    let tx2 = tx.clone();

    let task_a = tokio::spawn(async move {
        tx.send("来自 A 的消息").await.unwrap();
    });

    let task_b = tokio::spawn(async move {
        tx2.send("来自 B 的消息").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("收到：{message}");
    }

    task_a.await.unwrap();
    task_b.await.unwrap();
}
