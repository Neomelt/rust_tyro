use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let counter_a = Arc::clone(&counter);
    let counter_b = Arc::clone(&counter);

    let task_a = tokio::spawn(async move {
        let mut number = counter_a.lock().await;
        *number += 1;
    }); // number 离开作用域，自动释放锁

    let task_b = tokio::spawn(async move {
        let mut number = counter_b.lock().await;
        *number += 1;
    });

    task_a.await.unwrap();
    task_b.await.unwrap();

    let number = counter.lock().await;
    println!("最终计数：{}", *number);
}
