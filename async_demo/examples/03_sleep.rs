use tokio::time::{Duration, sleep};

async fn task(name: &str, seconds: u64) {
    println!("{name} 开始");

    // 异步等待：暂停当前任务，期间不阻塞线程
    sleep(Duration::from_secs(seconds)).await;

    println!("{name} 完成");
}

#[tokio::main]
async fn main() {
    task("A", 2).await;
    task("B", 1).await;
}

// 异步在解决某些时间阻塞而带来的线程资源的浪费，高效使用线程
//  这也是一种沏茶问题
