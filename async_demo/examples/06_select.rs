use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_secs(2)) => {
            println!("两秒的等待先完成");
        }
        _ = sleep(Duration::from_secs(1)) => {
            println!("一秒的等待先完成");
        }
    }
}

// _ 是通配符，标识无论当前值是什么形式均匹配
// 但是却不保存到任何变量，可以理解为一个占位符

// select! 会在当前tokio task第一个future完成时返回
// select!返回值并且丢弃其他还没跑完的future
