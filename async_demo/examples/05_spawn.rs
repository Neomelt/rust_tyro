use tokio::time::{Duration, sleep};

async fn boil_water() {
    println!("开始烧水");
    sleep(Duration::from_secs(3)).await;
    println!("水烧好了");
}

async fn wash_cup() {
    println!("开始洗杯子");
    sleep(Duration::from_secs(1)).await;
    println!("杯子洗好了");
}

#[tokio::main]
async fn main() {
    let water = tokio::spawn(boil_water());
    let cup = tokio::spawn(wash_cup());

    water.await.unwrap();
    cup.await.unwrap();

    println!("水和杯子都准备好了，可以泡茶了！");
}

// spawn 把 Future 交给 Tokio，登记为独立 task，并立即返回 JoinHandle。
// 对 JoinHandle 使用 .await，才会等待对应 task 完成。
//
// join! 则要等所有传入的 Future 都完成后才返回。
