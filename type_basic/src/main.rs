// fn main() {
//     // This is a simple example of parsing a string into an integer
//     // and handling the potential error if the string is not a valid number.
//     let guess = "42".parse::<i32>().expect("Not a number!");
//     println!("The guess is: {}", guess);
// }

// let语句只能在函数内部或代码块内部使用

// 不能在全局使用的语句
// let语句（变量声明）
// 赋值语句
// 控制流语句（if, loop, while等）
// fn main() {
//     // This is a simple example of parsing a string into an integer
//     // and handling the potential error if the string is not a valid number.
//     let guess: i32 = "42".parse().expect("Not a number!");
//     println!("The guess is: {}", guess);
// }

// 补课：
// 补码：
// 计算机中负数的表示方法
// 1. 原码：符号位 + 绝对值
// 2. 反码：符号位不变，绝对值取反
// 3. 补码：反码 + 1
// 比如：
// 5的原码：0000 0101
// 5的反码：1111 1010
// 5的补码：1111 1011
// -5的原码：1000 0101
// -5的反码：1111 1010
// -5的补码：1111 1011
// i8 就有8 bit，i32 就有32 bit

// 有符号数字以补码形式存储。

// 每个有符号类型规定的数字范围是 -(2n - 1) ~ 2n - 1 - 1，其中 n 是该定义形式的位长度。因此 i8 可存储数字范围是 -(27) ~ 27 - 1，即 -128 ~ 127。无符号类型可以存储的数字范围是 0 ~ 2n - 1，所以 u8 能够存储的数字为 0 ~ 28 - 1，即 0 ~ 255。

// fn main() {
//     let x: i32 = -5;
//     println!("{:032b}", x as u32); // 打印成无符号看补码
// }

// 整型溢出
// fn main() {
//     let x: i8 = 127;
//     // let y: i8 = x + 1; // 会报错：attempt to add with overflow
//     let y = x.wrapping_add(1); // 不会报错，还能测试溢出

//     println!("y = {}", y);
// }

// 新名词：HashMap
// HashMap 是一个键值对集合，提供了高效的查找、插入和删除操作。它使用哈希函数将键映射到存储位置，从而实现快速访问。

// 浮点数两个准则：
// 避免在浮点数上测试相等性
// 当结果在数学上可能存在未定义时，需要格外的小心

// fn main() {
//     // 断言0.1 + 0.2与0.3相等
//     assert!(0.1 + 0.2 == 0.3);
// }

// fn main() {
//     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
//     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

//     println!("abc (f32)");
//     println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//     println!("         0.3: {:x}", (abc.2).to_bits());
//     println!();

//     // .to_bits() 方法将浮点数转换为其二进制表示的整数形式
//     // 用于分析和调试浮点数的精度问题
//     println!("xyz (f64)");
//     println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//     println!("         0.3: {:x}", (xyz.2).to_bits());
//     println!();

//     assert!(abc.0 + abc.1 == abc.2);
//     assert!(xyz.0 + xyz.1 == xyz.2);
// }

// fn main() {
//     let x = (-42.0_f32).sqrt();
//     assert_eq!(x, x);
// }

// fn main() {
//     let x = (-42.0_f32).sqrt();
//     // 这个灰色的f32就是编译器推断出来的类型
//     // 一种好玩的变量定义方式
//     if x.is_nan() {
//         println!("未定义的数学行为")
//     }
// }

// fn main() {
//     // 编译器会进行自动推导，给予twenty i32的类型
//     let twenty = 20;
//     // 类型标注
//     let twenty_one: i32 = 21;
//     // 通过类型后缀的方式进行类型标注：22是i32类型
//     let twenty_two = 22i32;

//     let twenty_three = 23_i32; // 通过下划线后缀进行类型标注

//     // 只有同样类型，才能运算
//     let addition = twenty + twenty_one + twenty_two + twenty_three;
//     println!(
//         "{} + {} + {} + {} = {}",
//         twenty, twenty_one, twenty_two, twenty_three, addition
//     );

//     // 对于较长的数字，可以用_进行分割，提升可读性
//     let one_million: i64 = 1_000_000;

//     println!("{}", one_million.pow(2));
//     // .pow(n) 方法用于计算数字的n次方

//     // 定义一个f32数组，其中42.0会自动被推导为f32类型
//     let forty_twos = [42.0, 42f32, 42.0_f32];

//     // 打印数组中第一个值，并控制小数位为2位
//     println!("{:.2}", forty_twos[0]);
//     // 打印浮点数控制其精度的方式为 {:.n}，其中n为小数位数
// }

// 位运算

// 位运算仅对整数类型有效

// 对数值进行二进制逻辑操作得到的二进制然后转成十进制就是结果

// fn main() {
//     // 无符号8位整数，二进制为00000010
//     let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;

//     // 二进制为00000011
//     let b: u8 = 3;

//     // {:08b}：左高右低输出二进制01，不足8位则高位补0
//     println!("a value is        {:08b}", a);

//     println!("b value is        {:08b}", b);

//     println!("(a & b) value is  {:08b}", a & b);

//     println!("(a | b) value is  {:08b}", a | b);

//     println!("(a ^ b) value is  {:08b}", a ^ b);
//     // 异或运算
//     println!("(!b) value is     {:08b}", !b);

//     println!("(a << b) value is {:08b}", a << b);

//     println!("(a >> b) value is {:08b}", a >> b);

//     let mut a = a;
//     // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
//     a <<= b;
//     println!("(a << b) value is {:08b}", a);
// }

// fn main() {
//     let a: u8 = 255;

//     // 0 ≤ 位移位数 < 类型位数

//     let b = a >> 7; // ok
//     let b = a << 7; // ok
//     let b = a >> 8; // overflow
//     let b = a << 8; // overflow
// }

// 序列
// fn main() {
//     // 使用for循环打印1到5
//     for i in 1..=5 {
//         println!("{}", i);
//         // println!会自动添加换行符
//     }

//     for i in 1..=5 {
//         print!("{} ", i);
//     }
//     // print!不会自动添加换行符
//     // 但是发现一个有意思的现象：用print!会在末尾打印一个%符号
//     // 这是因为print!宏在输出时会在末尾添加一个换行符
//     print!("\n[END]");
// }

// as转换

fn main() {
    let a: i32 = 10;
    let b = 100_i8 as i32; // 使用as进行类型转换
    //  一般来说，as转换是安全的，但如果转换后值超出目标类型的范围，则会发生截断。
    // 例如，将100_i8转换为i32是安全的，因为100在i32的范围内，但如果将200_i8转换为i32，则会发生截断。
    if a < b {
        println!("Ten is less than one hundred.");
    }
}

// Rust数值库：num

// 类型转换必须是显式的
// Rust 的数值上可以使用方法，例如你可以用以下方法来将 13.14 取整：13.14_f32.round()，在这里我们使用了类型后缀，因为编译器需要知道 13.14 的具体类型
