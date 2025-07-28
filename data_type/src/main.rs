// //#![allow(unused)]
// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number!");
// }
// // rust有4个标量类型:整型、浮点型、布尔型和字符

// fn main() {
//     let _x = 2.0; // f64
//     // _x表示有意不使用的变量
//     let _y: f32 = 3.0; // f32
// }

// fn main() {
//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';
//     println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);
// }
// // rust有4个复合类型:元组和数组

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {}", y);
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
//     println!(
//         "five_hundred: {}, six_point_four: {}, one: {}",
//         five_hundred, six_point_four, one
//     );
// }

// // 一眼看下去全是宏
// #![allow(unused_variables)] // 允许未使用的变量
// type File = String; // 定义一个类型别名 File，实际上是 String 的别名

// // 接受一个可变引用的 File 类型参数
// fn open(f: &mut File) -> bool {
//     true
// }

// // 接受一个可变引用的 File 类型参数，并返回一个布尔值
// fn close(f: &mut File) -> bool {
//     true
// }

// #[allow(dead_code)] // 允许未使用的函数
// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!() // ❗ 用来标记某个函数或代码块还没写好。
// }

// fn main() {
//     let mut f1 = File::from("f1.txt");
//     open(&mut f1);
//     //read(&mut f1, &mut vec![]);
//     close(&mut f1);
//     // 上面这种写法是原型设计，后续可以逐步实现具体功能
//     // 这种写法的好处是可以先定义接口，后续再实现具体功能
//     // 这样可以让代码更清晰，易于维护和扩展
//     // 也可以让其他人更容易理解代码的意图
//     // 这种写法在 Rust 中很常见，尤其是在库的设计中
// }

// 开始字符串

// fn main() {
//     let my_name: String = String::from("Pascal");
//     // 也就是说他默认是字符串切片，是定义在栈上的，而如果    let my_name: String = String::from("Pascal");这样写或者to_string则是定义在堆上的
//     greet(my_name);
// }

// fn greet(name: String) {
//     println!("Hello, {}!", name);
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     // s.clear(); // error! // .clear()方法会清空字符串,属于可变借用

//     println!("the first word is: {}", word);
// }
// fn first_word(s: &String) -> &str {
//     &s[..1]
// }

//  字符串相关类型有好多
// 除了 String 类型的字符串，Rust 的标准库还提供了其他类型的字符串，例如 OsString， OsStr， CsString 和 CsStr 等，注意到这些名字都以 String 或者 Str 结尾了吗？它们分别对应的是具有所有权和被借用的变量。

// 字符串操作符
// 追加
// fn main() {
//     let mut s = String::from("hello");
//     s.push_str(", world!");
//     println!("{}", s);
// }

/// 连接
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = String::from(", world!");
//     let s3 = s1 + &s2;
//     println!("{}", s3);
// }

// // 插入
// fn main() {
//     let mut s = String::from("Hello rust!");
//     s.insert(5, ',');
//     println!("插入字符 insert() -> {}", s);
//     s.insert_str(6, " I like");
//     println!("插入字符串 insert_str() -> {}", s);
// }

// // 元组
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {},{},{}", x, y, z);
// }
// fn main() {
//     // 结构体
//     struct User {
//         active: bool,
//         username: String,
//         email: String,
//         sign_in_count: u64,
//     }

//     // 创建一个实例
//     let user1 = User {
//         active: true,
//         username: String::from("user1"),
//         email: String::from("user1@example.com"),
//         sign_in_count: 1,
//     };

//     // let user2 = User {
//     //     active: user1.active,
//     //     username: user1.username,
//     //     email: String::from("another@example.com"),
//     //     sign_in_count: user1.sign_in_count,
//     // };

//     // 简单的写法

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };

//     println!("user1: {}, user2: {}", user1.active, user2.active);
// }
// 以上的写法全都是有所有权的，还可以在结构体中使用引用，但是需要注意生命周期问题
// struct User<'a> {
//     username: &'a str,
//     email: &'a str,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         email: "someone@example.com",
//         username: "someusername123",
//         active: true,
//         sign_in_count: 1,
//     };
//     println!(
//         "User: {}, Email: {}, Active: {}, Sign In Count: {}",
//         user1.username, user1.email, user1.active, user1.sign_in_count
//     );
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:?}", rect1);
//     dbg!(&rect1); // 使用 dbg! 宏打印调试信息
// }

struct QuitMessage; // 单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

enum Message {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit(_) => println!("Quit"),
        Message::Move(m) => println!("Move to ({}, {})", m.x, m.y),
        Message::Write(WriteMessage(text)) => println!("Write: {}", text),
        Message::ChangeColor(ChangeColorMessage(r, g, b)) => {
            println!("Change color to ({}, {}, {})", r, g, b)
        }
    }
}
fn main() {
    let quit_msg = Message::Quit(QuitMessage);
    let move_msg = Message::Move(MoveMessage { x: 10, y: 20 });
    let write_msg = Message::Write(WriteMessage(String::from("Hello, World!")));
    let change_color_msg = Message::ChangeColor(ChangeColorMessage(255, 0, 0));

    process_message(quit_msg);
    process_message(move_msg);
    process_message(write_msg);
    process_message(change_color_msg);
}
