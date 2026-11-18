use std::sync::Arc;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    // 这里用 Arc，是因为子任务和 main 都要继续使用同一份字符串。
    // Arc::clone 只增加共享所有者，不会复制字符串内容。
    let message = Arc::new(String::from("这份数据可以跟着任务换线程"));
    let task_message = Arc::clone(&message);

    // move 把这个 Arc 的所有权移进 Future。
    // spawn 要求 Future 满足 Send：它可以被安全地转移到另一个线程。
    let handle = tokio::spawn(async move {
        // 等待后还要使用 task_message，所以 Future 需要把它保留下来。
        // 如果等待时任务让出了执行机会，下次可能由另一个线程继续执行。
        sleep(Duration::from_millis(100)).await;
        println!("任务读取：{task_message}");
    });

    handle.await.unwrap();
    println!("main 也能读取：{message}");
}

// 上一节的 'static：任务不能依赖可能提前失效的借用。
// 这一节的 Send：任务被转移到另一个线程时，携带的数据也必须允许安全地转移。
// move 负责捕获变量的所有权，不会让一个原本不满足 Send 的类型自动变成 Send。
//
// 可以试着把 Arc 全部换成 Rc，并把导入改成 use std::rc::Rc;，再运行这个例子。
// Rc 也能共享所有权，但它的引用计数不使用原子操作，不允许在线程之间转移。
// 此时 Future 携带着 Rc，无法满足 spawn 的 Send 要求，因此编译器会报错。
// Arc<String> 满足这里的要求；Arc 并不能让任意内部类型都变得线程安全。

// 跨线程传递数据并不是必须用 Arc，关键是被传递的类型满足 Send。
// 如果把数据交给任务后，外面就不用了：普通 String 直接通过 async move 移进去即可。
// 如果任务和外面要共同拥有同一份数据：可以像本例一样用 Arc。
// 如果各自需要独立的一份数据：可以克隆内容，比如 String::clone()，会复制字符串内容。
// Send 关心能不能安全地跨线程转移，Arc 解决共享所有权的问题，两者不是一回事。
