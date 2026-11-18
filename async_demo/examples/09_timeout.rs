use tokio::time::{Duration, sleep, timeout};

async fn slow_task() -> &'static str {
    sleep(Duration::from_secs(2)).await;
    "任务完成"
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_secs(1), slow_task()).await {
        Ok(message) => println!("{message}"),
        Err(_) => println!("超时了"),
    }
}

// slow_task 的等待时间短于 timeout 限制时，会返回函数的 message
// 超时时，timeout 会丢弃它持有的 Future；这里的 sleep 也随之取消
