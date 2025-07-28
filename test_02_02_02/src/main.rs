// 1.

// 修改2处 `assert_eq!` 让代码工作

// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1), 4);

//     let c2 = '中';
//     assert_eq!(size_of_val(&c2), 4);

//     println!("Success!")
// }

// 2.

// 修改一行让代码正常打印
// fn main() {
//     let c1 = "中"; // 错误原因是由于在rust中，字符只能用'',字符串才是"",所以既可以修改上面，也可以修改下面使得其变成字符
//     print_char(c1);
// }

// fn print_char(c: &str) {
//     println!("{}", c);
// }

// 3.

// // 使成功打印
// fn main() {
//     let _f: bool = false;

//     let t = true;
//     if t {
//         println!("Success!")
//     }
// }

// 4.

// fn main() {
//     let f = true;
//     let t = true && false;
//     assert_eq!(t, !f);

//     println!("Success!")
// }

// 5.

// 让代码工作，但不要修改 `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();

//     let v = (2, 3);
//     assert_eq!(_v, implicitly_ret_unit());
//     // 这里感觉还是将_v改成v0更好，因为_v实际上已经被使用了

//     println!("Success!")
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()")
// }

// // 不要使用下面的函数，它只用于演示！
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }

// 6.

// 让代码工作：修改 `assert!` 中的 `4`
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}

// 单元类型的大小为0字节，因为它不包含任何数据。`size_of_val` 返回的是值的大小，而单元类型没有实际的数据内容，所以它的大小是0。
// 单元类型在Rust中通常用于表示函数没有返回值的情况，或者作为占位符类型。
// 在这个例子中，`unit` 是一个单元类型的实例，`size_of_val(&unit)` 返回的是它的大小，应该是0。
// 因此，`assert!(size_of_val(&unit) == 0);`
// 断言会通过，程序会打印 "Success!"。
