async fn say_hello() {
    println!("hello");
}
// 这是正常的创建一个异步运行时的方法
fn main() {
    // 创建 Tokio 异步运行时
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // 调用异步函数，创建 Future，此时还没打印
    let future = say_hello();

    // 运行时推动 Future 执行，
    // 当前线程在这里等待它完成
    runtime.block_on(future);

    println!("异步任务执行完了");
}
