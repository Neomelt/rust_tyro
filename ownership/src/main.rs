// Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
// 值在任一时刻有且只有一个所有者。
// 当所有者（变量）离开作用域，这个值将被丢弃。
// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() 在字符串后追加字面值

//     println!("{}", s); // 将打印 `hello, world!`
// }
// String 和字面量的区别
// 字面量是不可变的，String 是可变的
// 字面量是固定大小的，String 是动态大小的
// 字面量在编译时就确定了大小，String 在运行时才确定大小
// 字面量存储在二进制文件中，String 存储在堆中
// 字面量是静态的，String 是动态的

// 二次释放
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1); // 当let s2 = s1 时，s1已经弃用
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// 任何不需要分配内存或某种形式资源的类型都可以实现 Copy 。如下是一些 Copy 的类型：

// 所有整数类型，比如 u32。
// 布尔类型，bool，它的值是 true 和 false。
// 所有浮点数类型，比如 f64。
// 字符类型，char。
// 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

// fn main() {
//     let s = String::from("hello"); // s 进入作用域

//     takes_ownership(s); // s 的值移动到函数里 ...
//     // ... 所以到这里不再有效

//     let x = 5; // x 进入作用域

//     makes_copy(x); // x 应该移动函数里，
//     // 但 i32 是 Copy 的，所以在后面可继续使用 x
// }
// // 变量的销毁顺序遵循后进先出（LIFO）原则。x 先移出作用域，然后是 s。但因为 s 的值已被移走，
// // 所以不会有特殊操作

// fn takes_ownership(some_string: String) {
//     // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) {
//     // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会有特殊操作

// fn main() {
//     let s1 = gives_ownership(); // gives_ownership 将返回值
//     // 移给 s1

//     let s2 = String::from("hello"); // s2 进入作用域

//     let s3 = takes_and_gives_back(s2); // s2 被移动到
//     // takes_and_gives_back 中,
//     // 它也将返回值移给 s3
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
// // 所以什么也不会发生。s1 移出作用域并被丢弃

// fn gives_ownership() -> String {
//     // gives_ownership 将返回值移动给
//     // 调用它的函数

//     let some_string = String::from("yours"); // some_string 进入作用域

//     some_string // 返回 some_string 并移出给调用的函数
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string 进入作用域

//     a_string // 返回 a_string 并移出给调用的函数
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() 返回字符串的长度

//     (s, length)
// }

// fn main() {
//     // let s = "hello";
//     let mut s = String::from("hello"); // 创建一个可变的 String 类型变量 s,使其分配在堆上
//     s.push_str(", world!"); // push_str() 在字符串后追加字面值
//     println!("{}", s);
// }

//

// 克隆（深拷贝）

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone(); // 克隆 s1 的值到 s2

//     println!("s1 = {}, s2 = {}", s1, s2); // 两个变量都有效
// }

fn main() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
