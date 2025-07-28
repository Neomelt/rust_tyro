// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// // 参数传递
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }

// 语句和表达式, 表达式的结尾没有分号，表达式的值会被返回，语句的结尾有分号，语句没有值
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {}", y);
// }

// 有返回值的函数
// fn five() -> i32 {
//     5 // 此处表达式的返回值与指定的返回值类型应相同
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }

fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 此处表达式的返回值与指定的返回值类型应相同
}

// 函数名和变量名使用蛇形命名法

// 在函数中，语句会隐式地返回一个值，而表达式则会显式地返回一个值。
// 所以在rust中，我们可以利用assert_eq!宏来验证函数的返回值是否正确。
// 也可以通过下面的方式来显式返回()
// fn clear() -> () {
// 这里的 () 是一个空元组，表示没有返回值
// 也可以写成 return ();
//}

// 永不返回的发散函数 !

// fn dead_end() -> ! {
//     panic!("你已经到了穷途末路，崩溃吧！");
// }
