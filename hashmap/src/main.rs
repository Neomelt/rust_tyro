// hashmap 本质上是一个键值对的集合，类似于 Python 中的字典。
// Rust 中的 HashMap 是一个哈希表实现，提供了高效的键值对存储和查找功能。
// 使用 HashMap 需要引入 std::collections::HashMap 模块。

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Red"), 50);

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
// }

// 还有一种就是使用宏来创建 HashMap，这样可以更简洁地初始化 HashMap。
// fn main() {
//     use std::collections::HashMap;
//     let teams = vec![String::from("Blue"), String::from("Red")];
//     let scores = vec![10, 50];
//     let mut score_map: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

//     // 首先是team上创建iter这个迭代器（迭代器是什么），然后score上也创建一个迭代器，就像两条并行的流水线，然后zip方法像是一个机械臂对应打包起来，形成第三个存储着键值对的迭代器
//     // collect 方法将迭代器打包装箱成 HashMap。

//     for (team, score) in &score_map {
//         println!("{}: {}", team, score);
//     }
// }

// 更新键值对
#![allow(unused)]
fn main() {
    use std::collections::HashMap; // 引入 HashMap 模块

    let mut scores = HashMap::new(); // 创建一个新的 HashMap
    scores.insert(String::from("Blue"), 10); // 插入键值对

    scores.entry(String::from("Yellow")).or_insert(50); // 如果键 "Yellow" 不存在，则插入 50
    scores.entry(String::from("Blue")).or_insert(50); // 如果键 "Blue" 存在，则不插入 50

    println!("{:?}", scores);
}
// 哈希函数牺牲了一定的时效性，但具有安全性
