use std::{thread, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // spawn_blocking 用来运行会阻塞线程的同步代码。
    // 这个闭包不是 async 闭包，里面可以直接调用 thread::sleep。
    let blocking_task = tokio::task::spawn_blocking(|| {
        println!("阻塞任务开始");
        thread::sleep(Duration::from_secs(2));
        println!("阻塞任务完成");
        42
    });

    // 这个异步任务仍然可以在阻塞任务等待时继续运行。
    let async_task = tokio::spawn(async {
        for i in 1..=3 {
            sleep(Duration::from_millis(500)).await;
            println!("异步任务：第 {i} 次运行");
        }
    });

    let result = blocking_task.await.unwrap();
    async_task.await.unwrap();

    println!("阻塞任务返回：{result}");
}
