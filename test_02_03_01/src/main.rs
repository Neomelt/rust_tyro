// 1.

// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = String::from("hello, world");
//     let y = &x; // 通过引用来借用x,将y绑定到x的值
//                 // y是x的一个不可变引用，存储在栈上
//     println!("{},{}", x, y);
// }

// 第二种改法
// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x: &str = "hello, world"; // 直接将x定义为静态内存量
//     let y = x;
//     println!("{},{}", x, y);
// }

// 第三种方法：直接对x深拷贝

// fn main() {
//     let x = String::from("hello, world");
//     let y = x.clone();
//     println!("{},{}",x,y);
// }

// 第四种方法：引用的赋值不会移动所有权，而是借用，因为引用本身就是借用

// fn main() {
//     let x = &String::from("hello, world");
//     let y = x;
//     println!("{},{}", x, y);
// }

// 第五种方法：切片借用，比copy更加安全，高效,可以理解为浅拷贝

// fn main() {
//     let x = String::from("hello, world");
//     let y = x.as_str(); // 等效于 let y = &x[..];
//     println!("{},{}",x,y);
// }

// 2.

// 不要修改 main 中的代码
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);
//     println!("{}", s2);
// }

// // 只能修改下面的代码!
// // 这个函数的传进来的数据是在堆上，
// // 这个函数的返回值也是在堆上
// // 但是由于语句没有返回值，所以这个函数刚开始并没有返回值
// // 所以需要修改函数以返回一个值
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

// 3.

// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// // 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     // Vec是rust的动态数组类型，可以自动扩展
//     // 传输方便
//     let _s = s.as_bytes();
//     // as_bytes() 方法将 String 转换为字节数组
//     // into_bytes() 方法将 String 转换为字节数组并消耗 String
//     // 这里的_s是一个字节数组，存储在栈上
//     // 但是_s的生命周期是_s的作用域结束时
//     println!("{:?}", _s);
//     s
// }

// 4.

// 修复错误，不要删除任何代码行
// fn main() {
//     let s = String::from("hello, world");

//     print_str(s.clone()); // 已经被moved,所以需要深拷贝一个
//     // String需要clone来深拷贝，因为它会被moved
//     println!("{}", s);
// }

// fn print_str(s: String) {
//     println!("{}", s);
// }

// 引用的改法

// fn main() {
//     let s = String::from("hello, world");
//     print_str(&s);
//     println!("{}", s);
// }
// fn print_str(s: &String) {
//     println!("{}", s)
// }

// 5.

// 不要使用 clone，使用 copy 的方式替代
// fn main() {
//     let x = (1, 2, (), "hello"); // to_string基本等价于String:from，但是前者更通用，后者只是适用于&str
//     // 那既然如此这个数据也是在堆上了
//     // 如果不用clone的话，估计就得把x变为一个可以copy的量了
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }

// 6.

// fn main() {
//     let s = String::from("hello, ");

//     // 只修改下面这行代码 !
//     let mut s1 = s; // 所有权 move 了
//     // 对的，因为后面要给s1加东西，所以需要可变
//     s1.push_str("world");
//     println!("{}", s1)
// }

// 7.

// fn main() {
//     let x = Box::new(5);
//     // Box是堆内存智能指针
//     let mut y = Box::new(3); // 完成该行代码，不要修改其它行！   // wcnmd,这是什么狗屎题，我还以为y和后面的断言有关系呢

//     *y = 4; // 这里是解引用

//     assert_eq!(*x, 5);
//     // 这里要让*x是5
// }

// 8.

// about move

//example

// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }

//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };

//     // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
//     // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
//     let Person { name, ref age } = person;

//     println!("The person's age is {}", age);

//     println!("The person's name is {}", name);

//     // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
//     //println!("The person struct is {:?}", person);

//     // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
//     println!("The person's age from person struct is {}", person.age);
//     // println!("The person's name from person struct is {}", person.name);
//     // Error!这里可以看到，因为 `name` 的所有权被转移了，所以 `person.name` 不可以使用
// }

// 8.

// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//     let _s = t.0;

//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t.1); // 这里的t.1是可以使用的，因为t.0的所有权已经被转移了
// }

// 9.

// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//     // 填空，不要修改其它代码
//     let (s1, s2) = t.clone(); // 使用 clone 来深拷贝 t 的值

//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
//     // 由于这里的需求是需要输出t,所以需要t的所有权保留
// }

fn main() {
    let t = (String::from("hello"), String::from("world"));

    // fill the blanks
    let (ref s1, ref s2) = t;
    // ref是引用的意思，这样s1和s2就不会move t的所有权
    // ref是直接在模式匹配中使用的，不是运行时，也就是他天生就是引用的意思

    // 好好好，原来ref是语法糖啊

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

// 今天先学到这里吧，去锻炼会
