use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Arc::new(String::from("串口波特率：115200"));

    let config_a = Arc::clone(&config);
    let config_b = Arc::clone(&config);

    let task_a = tokio::spawn(async move {
        println!("任务 A 读取：{config_a}");
    });

    let task_b = tokio::spawn(async move {
        println!("任务 B 读取：{config_b}");
    });

    task_a.await.unwrap();
    task_b.await.unwrap();

    println!("main 也能读取：{config}");
}
