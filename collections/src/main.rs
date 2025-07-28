// fn main() {
//     let mut v: Vec<i32> = Vec::new();
//     // 如果创建的数组是空的，需要指定类型
//     v.push(1);
//     v.push(2);
//     v.push(3);
//     v.push(4);
//     // 使用 Vec 的 push 方法添加元素
//     // 需要保证 Vec 的可变性
//     // 同时数组上的数据是并列的

//     let a = v.get(0);

//     // 有两种方式获取元素：
//     // 1. 使用索引访问
//     // 2. 使用 get 方法，get 方法返回 Option 类型，可以安全地处理越界访问
//     // 使用索引访问元素，可能会导致越界错误
//     let b = &v[0];
//     // v.push(5);
//     // 如果在借用 `v` 的过程中修改了 `v`，会导致编译错误
//     // 因为这还是由于v是分配在堆上，如果修改堆上的数据需要拷贝到一个新的堆上，但是由于前面的借用还没有结束，就形成了一个悬垂引用，所以会导致编译错误
//     // 解决方法是使用 get 方法来获取元素，这样就不会发生越界
//     // 使用 get 方法获取元素，返回 Option 类型
//     let c = v.get(7);

//     for i in &v {
//         println!("The value is: {}", i);
//     }

//     println!("Hello, world!{:?}", v);
//     println!("The first element is: {:?}", a);
//     println!("The eighth element is: {:?}", b);
//     println!("The ninth element is: {:?}", c);

//     // 可以使用 for 循环遍历可变 Vec 中的元素

//     let mut s = vec![100, 32, 57];
//     for i in &mut s {
//         *i += 50;
//         println!("The value is: {}", i);
//     }
//     #[derive(Debug)]
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];

//     println!("The row is: {:?}", row);
// }

// 集合用于存储多个值的容器类型，Rust 提供了多种集合类型，如 `Vec`、`HashMap`、`HashSet` 等。它们各自有不同的特性和用途。
// 特点
// - 动态大小：`Vec` 是一个动态数组，可以在运行时改变大小。
// - 键值对：`HashMap` 存储键值对，允许快速查找。
// - 唯一性：`HashSet` 存储唯一值，自动去重。
// - 有序性：`BTreeMap` 和 `BTreeSet` 保持元素的有序性。
// - 性能：不同集合类型在性能上有不同的权衡，选择合适的集合类型可以提高程序的效率。

// Osstring 是 Rust 中用于处理操作系统字符串的类型，它是一个可增长的、UTF-8 编码的字符串类型。它提供了许多方法来处理字符串，如拼接、切片、查找等。OsString 与 String 的区别在于，OsString 可以处理非 UTF-8 编码的字符串，而 String 只能处理 UTF-8 编码的字符串。
// Cstring 是 Rust 中用于处理 C 风格字符串的类型，它是一个不可增长的、UTF-8 编码的字符串类型。它提供了许多方法来处理字符串，如拼接、切片、查找等。CString 与 String 的区别在于，CString 可以处理 C 风格字符串，而 String 只能处理 Rust 风格的字符串。

// #[allow(unused)]
// fn main() {
//     let data = "initial contents";

//     let s = data.to_string();

//     // 该方法也可直接用于字符串字面量：
//     let s = "initial contents".to_string();
//     println!("String: {}", s);

//     // 或者使用 String::from 函数：
//     let s = String::from("initial contents");
//     println!("String: {}", s);
//     // rust的字符串不支持索引
// }

#![allow(unused)]
fn main() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
