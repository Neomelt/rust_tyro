// 第 21 节：Semaphore，限制同时进行的操作数量。
// 学习进度：已学到 20_join_set.rs，下次从本节继续。
// 在 async_demo 目录运行：cargo run --example 21_semaphore

use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    // ① 准备两张“工作许可证”，任务要先领到一张，才能进入下面的工作部分。
    // 多个任务需要访问同一个 Semaphore，所以用 Arc 共享它。
    let semaphore = Arc::new(Semaphore::new(2));
    let mut tasks = JoinSet::new();

    // ② 创建六个任务。六个任务都被调度，但最多两个能同时持证工作。
    for number in 1..=6 {
        let semaphore = Arc::clone(&semaphore);

        tasks.spawn(async move {
            // 有空余许可证就领取，没有就异步等待，不占着线程干等。
            // acquire_owned 得到的许可证自己持有 Arc，不需要借用外面的 Semaphore。
            let _permit = semaphore.acquire_owned().await.unwrap();

            println!("任务 {number} 开始工作");
            sleep(Duration::from_millis(300)).await;
            println!("任务 {number} 完成工作");

            // ③ 任务结束，_permit 被释放，许可证自动归还，其他任务就能领取。
            // _permit 是一个真正的变量，开头的下划线只是避免“未使用”的警告。
            // 这里不能改成 let _ = ...，那样许可证会在这条语句结束时就被释放。
        });
    }

    // ④ 等待全部任务结束；JoinSet 管理任务，Semaphore 控制工作部分的并发数量。
    while let Some(result) = tasks.join_next().await {
        result.unwrap();
    }

    println!("六个任务全部完成");
}

// 可以把 Semaphore::new(2) 改成 1 或 3，观察每批能同时工作多少个任务。
// 这个数量不是线程数；等待许可证的任务也已经创建，只是暂时不能进入工作部分。
// 例子里的任务很少，可以全部创建；海量任务还需要控制创建数量，不能只依赖许可证。
