// rust中引用分为可变引用和不可变引用
// 1. 同一作用域内可变引用和不可变引用不能同时存在
// 2. 可变引用只能有一个
// 3. 可变引用必须在使用完后才能使用

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// 🍀，rust好难

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // 没问题
//     let r2 = &s; // 没问题
//     println!("{} and {}", r1, r2);
//     // 此位置之后 r1 和 r2 不再使用

//     let r3 = &mut s; // 没问题
//     // println!("{}", r2); // 不能使用 r2
//     println!("{}", r3);
// }

// 悬垂引用

// 下面的代码会报错，原因是返回了一个悬垂引用（dangling reference）
// 也就是说，函数返回了一个指向已经被释放的内存的引用

fn main() {
    let reference_to_nothing = dangle();
    println!("This will not print: {}", reference_to_nothing);
}

// 这里尝试返回一个局部变量 s 的引用
fn dangle() -> String {
    let s = String::from("hello");
    // &s
    s
} // s 离开作用域被释放，&s 就变成了悬垂引用，编译器会报错

// fn main() {
//     let x = 5;
//     let y = &x; // y 是 x 的不可变引用
//     assert_eq!(x, 5);
//     assert_eq!(*y, 5); //  引用数据在调用的时候需要解引用
//     println!("{}", *y); // 打印引用的值
// }

// fn main() {
//     let x = 5;
//     let y = &x;
//     println!("x: {}", x);
//     println!("y: {:p}", y);
//     println!("y: {}", y); // 自动解引用
//     println!("y: {}", *y); // 手动解引用
// }

// {}这玩意叫Display
// {:?}这玩意叫Debug

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
