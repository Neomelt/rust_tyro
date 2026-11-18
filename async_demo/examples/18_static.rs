use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    // 主流程①：进入这个代码块，先执行里面的代码，最后才把它产生的值赋给 handle。
    let handle = {
        // 为什么这里可以将{}里的代码块直接赋给handle，是因为rust的{}可以返回一个值，就像前面的
        // example 里面的函数末尾可以直接写一个 42,代码块最后的表达式没有分号，它的值就成为了
        // 整个代码块的值
        // 主流程②：创建字符串 message。
        let message = String::from("这份数据交给任务保管");

        // async move 把 message 的所有权移进 Future，任务自己拥有这个 String。
        // 如果删掉 move，这里就会借用局部变量 message，无法满足 spawn 的 'static 要求。
        // 主流程③：创建 Future，把它交给 Tokio 调度；spawn 返回 JoinHandle，不等任务做完。
        tokio::spawn(async move {
            // 子任务被调度后：先等待 100 毫秒，再打印字符串，最后结束。
            // 等待期间，主流程可以继续运行。
            sleep(Duration::from_millis(100)).await;
            println!("任务收到：{message}");
        }) // 这里也没有分号，所以spawn 的返回值就是外面这个代码块的值，赋给 handle 的，是 tokio::spawn(...) 返回的值
    }; // 创建任务的代码块结束，但 String 已经移进任务，可以继续使用。

    // 主流程④：拿到 handle 后，继续执行这里的打印；此时结束的是代码块，main 还没结束。
    println!("创建任务的代码块已经结束");
    // 主流程⑤：通过 handle.await 等待子任务；如果它还没完成，main 就在这里暂停。
    // 子任务已经由 spawn 交给 Tokio，不需要等到这里 await 才开始执行。
    // await 得到 Result：正常完成是 Ok(())；子任务 panic 或被取消则是 Err(JoinError)。
    // unwrap 遇到 Ok 就取出里面的值，遇到 Err 就在当前的 main 任务里触发 panic。
    // 所以子任务 panic 后，这里的 unwrap 会让外面也 panic；换成 match 就能自行处理错误。
    handle.await.unwrap();
    // 主流程⑥：等待结束，unwrap 成功后执行到这里，main 才结束。
}

// 这里的 'static 是对类型的生命周期约束，不是要求变量永远不释放。
// 任务不能依赖可能提前失效的借用；自己拥有 String 就能满足这个要求。
// 这里 message 会在任务结束时随 Future 一起释放，不会因为 'static 就一直占着内存。
// 注意：move 不会延长引用所指数据的生命周期；移入一个短期引用仍然不能满足 'static。

// 主流程和子任务可以交错执行，异步不规定“外面一定先执行，里面一定后执行”。
// 这个例子中，子任务有 100 毫秒的等待，所以通常先看到外面的打印，再看到任务里的打印；
// 但两次打印的先后不是严格保证，不能依赖 sleep 来保证执行顺序。
