use tokio::sync::oneshot;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    // oneshot 通道只能发送一次，适合发送“请停止”的信号。
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let worker = tokio::spawn(async move {
        let mut shutdown_rx = shutdown_rx;
        let mut count = 0;

        loop {
            tokio::select! {
                // 没收到停止信号时，继续做工作。
                _ = sleep(Duration::from_millis(500)) => {
                    count += 1;
                    println!("任务工作中：第 {count} 次");
                }

                // 收到停止信号后，任务自己跳出循环。
                _ = &mut shutdown_rx => {
                    println!("收到停止信号");
                    break;
                }
            }
        }

        // 任务自己结束前，可以在这里释放资源、保存状态等。
        println!("任务完成清理");
    });

    sleep(Duration::from_secs(2)).await;

    // 发送停止信号，不直接强制取消任务。
    let _ = shutdown_tx.send(());

    // 等待任务处理信号并完成清理。
    worker.await.unwrap();
    println!("程序安全退出");
}

// 这个例子演示的是“优雅关闭”：准备关程序时，先通知任务“该收工了”，
// 让任务自己完成善后，再等待它退出。
// 比如任务平时不断接收传感器数据，先存在内存里，攒够一批才写进文件；
// 收到停止信号后，就可以停止接收新数据，把剩下的数据写完，再结束任务。
//
// send(()) 只发送一个通知，“停止”的含义来自接收端收到通知后执行 break。
// 保存数据的逻辑需要自己写，当前例子只是用 println! 表示清理过程。
// worker.await 则让主程序等这个任务收拾完，避免主程序提前退出。
// 如果有多个任务需要善后，就需要通知到它们，并等待它们分别完成。
// 这种方式适用于程序主动关闭，不能保证断电或进程被强杀时也能完成保存。
