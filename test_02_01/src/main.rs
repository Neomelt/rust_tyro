// 1.
// fn main() {
//     let x: i32 = 5; // uninitialized but using, ERROR !
//     let _y: i32; // uninitialized but also unusing, only warning
//     println!("{} is equal to 5", x);
// }

// 2.

// fn main() {
//     let mut __ = 1;
//     // mut 是因为后面的 += 需要对变量 __ 进行可变借用
//     __ += 2;
//     // 使用了 += 的方式对变量 __ 进行赋值操作
//     // 这会将 __ 的值从 1 增加到 3
//     // 注意：在 Rust 中，+= 是一个语句而不是表达式，所以它不会返回值
//     // 因此不能直接在 println! 中使用 __ += 2
//     // 而是需要先执行 __ += 2，然后再打印 __ 的值
//     println!("x = {}", __);
// }

// 3.

// fn main() {
//     let x: i32 = 10;
//     let y: i32 = 20;
//     {
//         let y: i32 = 5;
//         println!("x 的值是 {}, y 的值是 {}", x, y);
//     }
//     println!("x 的值是 {}, y 的值是 {}", x, y);
// }

// 4.
// 修复错误
// fn main() {
//     define_x();
//     // println!("x = {}", x); // 这里会报错，因为 x 在 define_x 函数中定义的作用域内
//     // 不能直接打印输出，只能通过调用函数的方法来获取
// }

// fn define_x() {
//     let x = "hello";
// }

// 5.

// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;

//         assert_eq!(x, 12);
//         // 这里的断言是对当前作用域内的 x 进行的。

//         //assert_eq!是一个宏，用于断言两个值相等，如果不相等则会导致程序 panic。
//         // 这里的 x 是在当前作用域内重新定义的变量，它的值是 12，而外部的 x 的值是 5。
//         // 因此这个断言会失败，导致程序 panic。
//         // panic 是指程序在运行时遇到错误而中止执行。
//         // 要让这个断言通过，可以将 x 的值改为 12。
//         // 但是这样会导致后面的 println! 输出的值不是 42，而是 12。
//         // 所以我们需要在后面的 println!
//         // 中使用外部的 x 的值，而不是当前作用域内的 x 的值。
//     }

//     assert_eq!(x, 5);
//     // 这里的断言是对外部作用域内的 x 进行的，它的值是 5。

//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }

// 6.

// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let mut x = x;
//     x += 3;

//     let y = 4;
//     // 遮蔽
//     let y = "I can also be bound to text!";
// }

// 7.

// fn main() {
//     let _x = 1;
// }

// // compiler warning: unused variable: `x`

// #![allow(unused_variables)]
// // 允许未使用的变量警告
// fn main() {
//     let x = 1;
//     // println!("The value of x is: {}", x);
//     // 原因是因为在 main 函数中，变量 x 被声明但没有被使用。
//     // 可以让他输出以调用，也可以用宏
// }

// // compiler warning: unused variable: `x`

// 8.

// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }
// // 这里的错误是因为在 Rust 中，元组解构赋值需要使用 let 关键字来声明变量。
// // 因此需要将 let (mut x, y) = (1, 2); 修改为 let (mut x, y) = (1, 2);。
// // 另外，元组解构赋值的语法是正确的，不需要修改。
// // 但是需要注意的是，元组解构赋值的变量必须是可变的，否则会报错。
// // 因此需要将 x 声明为可变的，即使用 mut 关键字。

// 另一种做法是使用模式匹配来解构元组：
fn main() {
    let (_x, y) = (1, 2);
    // 使用 _x 来忽略第一个元素，因为我们不需要它
    let x = 3;
    // 直接声明 x 的值为 3,而不是 x += 2;
    // 这样就不需要使用 mut 关键字了
    // += 会导致 x 需要是可变的，但我们可以直接赋值
    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

// 9.

// fn main() {
//     let (x, y);
//     (x, ..) = (3, 4);
//     [.., y] = [1, 2];
//     // 填空，让代码工作
//     assert_eq!([x, y], [3, 2]);
// }
