// fn main() {
//     let c = 'z';
//     let z = 'ℤ';
//     let g = '国';
//     let heart_eyed_cat = '😻';

//     println!(
//         "c: {}, z: {}, g: {}, heart_eyed_cat: {}",
//         c, z, g, heart_eyed_cat
//     );
// }

// fn main() {
//     let x = '中';
//     println!("字符'中'占用了{}字节的内存大小", size_of_val(&x));

//     // Rust 的字符只能用 '' 来表示， "" 是留给字符串的。
// }

// fn main() {
//     let t = true;

//     let f: bool = false; // 使用类型标注,显式指定f的类型

//     if f {
//         println!("这是段毫无意义的代码");
//     } else {
//         println!("这是段有意义的代码");
//     }
// }

// 没有返回值的函数在 Rust 中是有单独的定义的：发散函数( diverging functions )，顾名思义，无法收敛的函数。

// | 名称   | 写法   | 含义                   |
// | ---- | ---- | -------------------- |
// | 单元类型 | `()` | 表示“无返回值”或“无意义的值”     |
// | 单元值  | `()` | 是单元类型唯一的值，字面量也是 `()` |

// 学到一个数据结构：键值对
// 键值对是一个包含两个元素的元组，第一个元素是键，第二个元素是值。
// 键值对的键可以是任意类型，但值必须是同一种类型

fn main() {
    let mut map = std::collections::HashMap::new();
    map.insert("name", "Alice");
    map.insert("age", "30");
    map.insert("city", "New York");

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 访问键值对
    if let Some(name) = map.get("name") {
        println!("Name: {}", name);
    }
}
