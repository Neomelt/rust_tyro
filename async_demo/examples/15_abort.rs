use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        for i in 1..=10 {
            println!(
                "任务运行中：第 {i} 次，线程：{:?}",
                std::thread::current().id()
            );
            sleep(Duration::from_millis(500)).await;
        }
    });

    // main 自己等待一段时间，然后取消另一个异步任务。
    sleep(Duration::from_millis(1200)).await;
    handle.abort();

    // 被 abort 的任务不会返回正常结果，而是得到 JoinError。
    match handle.await {
        Ok(()) => println!("任务正常完成"),
        Err(error) if error.is_cancelled() => println!("任务已取消"),
        Err(error) => println!("任务异常结束：{error}"),
    }

    println!("main 当前线程：{:?}", std::thread::current().id());
}
