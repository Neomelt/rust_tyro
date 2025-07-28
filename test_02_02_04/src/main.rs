// 1.

// fn main() {
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);

//     assert_eq!(s, 3);
// }

// // 必须得给定返回值类型
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// 2.

// fn main() {
//     print();
// }

// // 使用另一个类型来替代 i32
// fn print() -> () {
//     println!("hello,world");
// }

// 3.

// fn main() {
//     never_return();
// }

// fn never_return() -> ! {
//     // implement this function, don't modify fn signatures
//     panic!("I return nothing!")

//     // Rust 中 ! 可以自动“转化”为任何其他类型，比如这里可以返回为()
// }

// 4.

// 下来再看看
// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };

//     // 这里与其返回一个 None，不如使用发散函数替代
//     never_return_fn()
// }

// // 使用三种方法实现以下发散函数
// fn never_return_fn() -> ! {
//     // 1. 使用 panic! 宏
//     // panic!("This function never returns!");
//     // 2. 使用 loop 循环
//     // loop {
//     //     // 永远不会结束的循环
//     // }
//     // 3. 使用 unreachable! 宏
//     // unreachable!("This function never returns!");
//     // 4. 使用 std::process::abort() 终止程序
//     // std::process::abort();
//     // 5. 使用 unimplemented! 宏
//     // unimplemented!("This function never returns!");
//     // 6. 使用 todo! 宏
//     todo!("This function never returns!");
//     // 7. 使用 std::process::exit(0) 退出程序
//     // std::process::exit(0);
// }

// 5.

fn main() {
    // 填空
    let b = false;

    let _v = match b {
        // match 用来模式匹配
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
