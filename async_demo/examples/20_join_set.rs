use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    // ① 创建一个任务集合，不用再给每个任务单独保存一个 JoinHandle。
    let mut tasks = JoinSet::new();

    // ② 按 1、2、3 的顺序创建任务，但让它们等待不同的时间。
    // 每轮循环中的 number 和 delay 来自一个元组。
    for (number, delay) in [(1, 300), (2, 100), (3, 200)] {
        // 和 tokio::spawn 一样，交给 Tokio 调度后，主流程继续往下走。
        tasks.spawn(async move {
            sleep(Duration::from_millis(delay)).await;
            number // 这个任务的返回值是自己的编号。
        });
    }

    println!("三个任务已经交给 Tokio，开始收取结果");

    // ③ 每次 join_next().await 收取一个已完成任务的结果，并把它移出集合。
    // 如果集合里还有任务，但暂时没有完成的，就等待；集合空了才返回 None。
    // Some 里仍然包着 Result，用来区分任务正常完成，还是 panic / 被取消。
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(number) => println!("收到任务 {number} 的结果"),
            Err(error) => println!("有任务未正常完成：{error}"),
        }
    }

    // ④ 所有任务的结果都已经取走，循环结束。
    println!("所有任务都处理完了");
}

// JoinSet 按任务完成的顺序交付结果，不按创建任务的顺序等待。
// 本例通常会按 2、3、1 的顺序收到结果；具体先后仍取决于实际调度。
// 接收循环是一轮轮取结果，但各个子任务早已被 spawn 出去，可以并发推进。
// 如果 JoinSet 提前被释放，里面剩余的任务会被请求取消，所以需要等待完成时要保留它。
