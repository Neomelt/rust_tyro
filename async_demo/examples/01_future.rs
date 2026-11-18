async fn say_hello() {
    println!("hello");
}

fn main() {
    // 只创建 Future，不执行函数体，所以不会输出 hello。
    let _future = say_hello();
}
// 因为异步函数是惰性的，是lazy的
