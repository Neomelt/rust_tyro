// // 切片就像数组一样，
// // 但是它们并不拥有数据。
// // 切片是对数组的引用
// // 可以把切片想象成一个指向数组的指针
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes(); // 将字符串转换为字节数组 // 这里的as_bytes()方法是String类型的方法
//     for (i, &item) in bytes.iter().enumerate() {
//         // iter()方法返回一个迭代器，enumerate()方法返回一个包含索引和元素的元组
//         // 遍历字节数组
//         if item == b' ' {
//             // 如果遇到空格
//             return i; // 返回当前索引
//         }
//     }
//     s.len() // 如果没有空格，返回字符串的长度
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]; // 如果没有空格，返回整个字符串
                   // 此处可以不加return
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // 传入字符串的引用
    println!("The first word is: {}", word);
    s.clear(); // 清空字符串
               // println!("The first word is: {}", word); // 这里会报错，因为 word 是一个悬垂引用,只需要加个mut
               // 也就是说，word 指向的内存已经被释放了
}
