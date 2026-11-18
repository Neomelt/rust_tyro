fn run_job<F>(job: F)
where
    F: FnOnce(),
{
    // 函数接收一段代码，之后再调用它。
    job();
}

fn main() {
    // 1. 闭包可以有参数，也可以返回值。
    let add_one = |number| number + 1;
    println!("{}", add_one(41));

    // 2. 闭包可以读取外部变量。
    let base = 10;
    let add_base = |number| number + base;
    println!("{}", add_base(5));

    // 3. move 让闭包取得外部变量的所有权。
    let message = String::from("hello");
    let print_message = move || {
        println!("{message}");
    };

    // message 已经被 move 进闭包，外部不能再使用它。
    run_job(print_message);
}
