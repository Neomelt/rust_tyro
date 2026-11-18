use tokio::time::{Duration, sleep};

async fn calculate_number() -> i32 {
    sleep(Duration::from_secs(1)).await;
    42
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(calculate_number());

    let result = handle.await;
    println!("{result:?}");

    let number = result.unwrap();
    println!("{number:?}");
}
