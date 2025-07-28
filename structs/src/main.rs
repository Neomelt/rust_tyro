// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");

//     user1.sign_in_count = 2;
//     println!("User1: {:?}", user1.username);
//     println!("User1: {:?}", user1.email);
//     println!("User1: {:?}", user1.sign_in_count);
//     println!("User1: {:?}", user1.active);

//     let user2 = build_user(String::from("someone"), String::from("someusername123"));
//     println!("User2: {:?}", user2.username);
//     println!("User2: {:?}", user2.email);
//     println!("User2: {:?}", user2.sign_in_count);
//     println!("User2: {:?}", user2.active);
// }

// // 元组结构体
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("black: ({}, {}, {})", black.0, black.1, black.2);
//     println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);
// }

// 单元结构体
// 单元结构体是没有任何字段的结构体，通常用于标记类型或实现 trait
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
