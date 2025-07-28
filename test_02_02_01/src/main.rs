// 1.

// 移除某个部分让代码工作
// fn main() {
//     let x: i32 = 5;
//     let mut y = 5; // 由于不同类型的变量不能互相赋值，这里 y 的类型是 i32

//     y = x;

//     let z = 10; // 这里 z 的类型是 i32
// }

// rust默认的类型
// 整型：i32
// 浮点型：f64
// 布尔型：bool
// 字符型：char
// 字符串：String
// 数组：数组的类型是 [T; N]，其中 T 是数组元素的类型，N 是数组的长度
// 元组：元组的类型是 (T1, T2, ..., TN)，其中 T1, T2, ..., TN 是元组元素的类型
// 结构体：结构体的类型是 StructName，其中 StructName 是结构体的名称
// 枚举：枚举的类型是 EnumName，其中 EnumName 是枚举的名称
// 切片：切片的类型是 &[T]，其中 T 是切片元素的类型
// 函数：函数的类型是 fn(参数类型) -> 返回值类型
// 闭包：闭包的类型是 Fn(参数类型) -> 返回值类型
// 引用：引用的类型是 &T，其中 T 是引用的类型
// 指针：指针的类型是 *const T 或 *mut T，其中 T 是指针指向的类型
// 泛型：泛型的类型是 T，其中 T 是泛型类型参数
// 其他类型：如 Vec<T>、HashMap<K, V> 等集合类型，T 是元素类型，K 是键类型，V 是值类型

// 2.

// 填空

// #![allow(unused)]
// fn main() {
//     let v: u16 = 38_u8 as u16;
// }

// 3.

// 修改 `assert_eq!` 让代码工作
// fn main() {
//     let x = 5;
//     assert_eq!("i32".to_string(), type_of(&x));
// }

// // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// 4.

// 填空，让代码工作
// fn main() {
//     assert_eq!(i8::MAX, 127);
//     assert_eq!(u8::MAX, 255);
// }

// 5.

// 解决代码中的错误和 `panic`
// fn main() {
//     let v1 = 247_u8 + 8;
//     let v2 = i8::checked_add(119, 8).unwrap();
//     println!("{},{}", v1, v2);
// }

// 6.

// 修改 `assert!` 让代码工作
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     // 0xff 是十六进制的 255
//     // 0o77 是八进制的 63
//     // 0b1111_1111 是二进制的 255
//     // 1_024 是十进制的 1024
//     // 所以 v 的值是 1024 + 255 + 63 + 255 = 1597

//     assert!(v == 1597);
// }

// 7.

// 将 ? 替换成你的答案
// fn main() {
//     let x = 1_000.000_1; // f64,默认为 f64
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64
// }

// 8.

// fn main() {
//     assert!(0.1f32 + 0.2f32 == 0.3f32);
//     // 断言成功原因是这只是巧合
// }

// fn main() {
//     assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.001);
// }

// 更规范的写法？
// fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
//     (a - b).abs() < epsilon
// }

// fn main() {
//     assert!(approx_equal(0.1 + 0.2, 0.3, 1e-9));
// }

// 9.

// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -5);

//     for c in 'a'..='z' {
//         println!("{}", c as u8);
//     }
// }

// 10.

// 填空
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range { start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));
//     // Range 和 RangeInclusive 是 Rust 中的两个范围类型
//     // Range 表示一个不包含结束值的范围，而 RangeInclusive 表示一个包含结束值的范围
//     // 例如，1..5 表示从 1 到 5（不包含 5），而 1..=5 表示从 1 到 5（包含 5）
//     // 这两个范围类型都可以用于迭代、切片等操作
// }

// 11.

// 填空，并解决错误
fn main() {
    // 整数加法
    assert!(1u32 + 2 == 3);

    // 整数减法
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 / 3.2 - 3.0 < 1e-3); // error ! 修改它让代码工作

    assert!(24 % 5 == 4);

    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // 0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // 0010
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
