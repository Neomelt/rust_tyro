// 第 24 节：综合练习——模拟设备数据采集器。
// 在 async_demo 目录运行：cargo run --example 24_device_collector
// 数据流：模拟采集任务 -> mpsc 通道 -> 批量保存任务 -> CSV 文件。
// 输出放在 async_demo/target/device_samples/ 下，每次生成一个新文件，不覆盖已有数据。

use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{self, AsyncWriteExt};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{Duration, sleep};

struct Sample {
    sequence: u32,
    value: u32,
}

// 采集任务拥有发送端，停止采集后释放它，保存任务才能知道“不会再有数据了”。
async fn collect_samples(
    tx: mpsc::Sender<Sample>,
    mut shutdown: oneshot::Receiver<()>,
) -> io::Result<()> {
    let mut sequence = 0;

    loop {
        tokio::select! {
            _ = &mut shutdown => {
                println!("采集任务：收到停止通知，不再产生新数据");
                break;
            }
            _ = sleep(Duration::from_millis(150)) => {
                sequence += 1;
                let sample = Sample {
                    sequence,
                    value: 20 + sequence, // 模拟设备读数，不需要真实串口。
                };

                // 通道满了就异步等待，给保存任务消化数据的机会。
                // 已经生成的数据先发完，再在下一轮检查停止信号。
                tx.send(sample).await.map_err(|_| {
                    io::Error::new(io::ErrorKind::BrokenPipe, "保存任务已退出，无法继续发送")
                })?;
                println!("采集任务：已发送第 {sequence} 条数据");
            }
        }
    }

    Ok(()) // tx 在这里释放，通道发送端关闭。
}

// 把内存中的一批数据写进文件，写成功后再清空这批数据。
async fn write_batch(file: &mut File, batch: &mut Vec<Sample>) -> io::Result<usize> {
    if batch.is_empty() {
        return Ok(0);
    }

    for sample in batch.iter() {
        let line = format!("{},{}\n", sample.sequence, sample.value);
        file.write_all(line.as_bytes()).await?;
    }
    file.flush().await?;

    let count = batch.len();
    batch.clear();
    println!("保存任务：已写入一批，共 {count} 条");
    Ok(count)
}

async fn save_samples(mut rx: mpsc::Receiver<Sample>, mut file: File) -> io::Result<usize> {
    file.write_all(b"sequence,value\n").await?;
    let mut batch = Vec::new();
    let mut total = 0;

    // 发送端关闭后，recv 仍会先取完通道中剩余的数据，最后才返回 None。
    while let Some(sample) = rx.recv().await {
        batch.push(sample);
        if batch.len() == 3 {
            total += write_batch(&mut file, &mut batch).await?;
        }
    }

    // 收工时不足三条的也要保存，不能等不到下一批就直接丢掉。
    println!("保存任务：通道已读完，补存剩余 {} 条", batch.len());
    total += write_batch(&mut file, &mut batch).await?;
    file.flush().await?;
    file.sync_all().await?; // 请求操作系统把文件数据和元数据同步到存储设备。
    println!("保存任务：完成收尾，共保存 {total} 条数据");
    Ok(total)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // ① 先准备输出文件；准备失败就返回错误，不启动采集。
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("device_samples");
    fs::create_dir_all(&directory).await?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?
        .as_nanos();
    let path = directory.join(format!("samples-{timestamp}.csv"));
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await?;
    println!("输出文件：{}", path.display());

    // ② 两个任务各自拥有所需的数据；只有保存任务拥有文件，不需要 Mutex。
    let (tx, rx) = mpsc::channel(4);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let collector = tokio::spawn(collect_samples(tx, shutdown_rx));
    let saver = tokio::spawn(save_samples(rx, file));

    // ③ 让例子自动运行一小会儿，然后模拟“用户准备退出”。
    sleep(Duration::from_millis(1100)).await;
    println!("主流程：通知采集任务收工");
    let _ = shutdown_tx.send(());

    // ④ 先等采集结束，再等保存任务读完通道并完成收尾，不在这里 abort。
    // 先取得两个任务的结果，再检查错误，给另一个任务正常退出的机会。
    let collector_result = collector.await;
    let saver_result = saver.await;

    // 外层 Result 表示任务有没有 panic / 被取消，内层 Result 表示操作有没有失败。
    // 把任务错误转成 io::Error，统一通过 main 的返回值报告。
    match collector_result {
        Ok(result) => result?,
        Err(error) => return Err(io::Error::other(error)),
    }
    let saved = match saver_result {
        Ok(result) => result?,
        Err(error) => return Err(io::Error::other(error)),
    };

    println!("程序正常退出，本次保存 {saved} 条数据");
    Ok(())
}

// 采集条数取决于实际调度，不保证每次完全一样。
// “停止采集”和“停止保存”是不同阶段：先不产生新数据，再处理完已经产生的数据。
// 优雅关闭提供正常收尾的机会；磁盘写入失败会返回错误，断电或强杀也不能靠停止通知兜底。
