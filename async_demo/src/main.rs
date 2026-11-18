async fn say_hello() {
    println!("hello");
}

#[tokio::main]
// rust的异步函数不能直接作为程序入口
// 这个宏会在编译期生成普通入口，创建 Tokio 运行时，
// 自动在编译期实现一个程序入口的转换
// 再通过 block_on 执行我们写的异步代码。

// // 如果不这样写的话：
// fn main() {
//     // 我们需要先创建一个异步运行时
//     let runtime = tokio::runtime::Runtime::new().unwrap();
//     // 然后这个运行时调用方法block_on
//     // 通过运行时执行这个 Future，当前线程等待它完成
//     runtime.block_on(async {
//         say_hello().await;
//     });
// }

async fn main() {
    say_hello().await;
    // say_hello() 创建一个 Future，单独调用它不会执行函数体
    // 接着 .await 会推动它执行，并等待它完成
}
