// fn main() {
//     let s: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
//     println!("Hello, world! {:?}", s);
// }

// 由于复杂类型的数据无法直接去用[type;num]的方式去定义
// 需要使用std::array::from_fn来生成数组

fn main() {
    let s: [String; 5] = std::array::from_fn(|i| format!("String {}", i));
    println!("{:?}", s);
}
