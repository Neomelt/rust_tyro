use tokio::time::{Duration, sleep};

async fn calculate_number() -> Result<i32, &'static str> {
    sleep(Duration::from_secs(1)).await;

    // 模拟计算本身失败
    Err("暂时算不出来")
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(calculate_number());

    // 返回结果为：Ok(Err("暂时算不出来"))
    // 外层 Result 表示 Tokio 任务是否正常结束，内层 Result 表示计算是否成功。
    // 返回 Err 是正常返回错误值，不等于 panic。
    match handle.await {
        Ok(Ok(number)) => println!("结果：{number}"),
        Ok(Err(error)) => println!("计算失败：{error}"),
        Err(join_error) => println!("任务异常结束：{join_error}"),
    }
}
