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
    tokio::join!(boil_water(), wash_cup(),);

    println!("水和杯子都准备好了，可以泡茶了！");
}

// 来思考一下join方法的具体实现：
// join是tokio的一个宏，负责推进多个 Future，等待其全部完成
// 它使用了rust提供的trait ： future
// future 的核心方法是poll
// 当写下：
// async fn boil_water() {
//     // ...
// }
// rust编译器实现了两个东西：
// 1.保存当前任务的数据类型（虽然我还是没明白）
// 2.为这个类型实现 Future trait，包含poll方法
// 看了下源码，感觉有点难懂，反正大致就是tokio构造了一个遵循rust future trait的
// sleep类型，然后对其进行处理，其实似乎没有涉及到优先级排序的算法什么的
// 备注： ready! 不太理解
