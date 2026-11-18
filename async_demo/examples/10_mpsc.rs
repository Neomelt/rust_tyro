use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(4);

    // async { ... } 是异步代码块：不用定义函数，也能直接创建一个 Future。
    // 创建时不会立刻执行里面的代码，spawn 再把这个 Future 交给 Tokio 调度。
    // move 把用到的外部变量捕获进来；这里是把 tx 的所有权移进 Future，
    // 让新任务自己拥有发送端，外面的代码就不能再使用这个 tx 了。
    // 这样可以保证任务运行期间数据一直有效，避免类似 C++ 悬空指针或悬空引用的问题。
    let sender = tokio::spawn(async move {
        tx.send("第一条消息").await.unwrap();
        tx.send("第二条消息").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("收到：{message}");
    }

    sender.await.unwrap();
}
