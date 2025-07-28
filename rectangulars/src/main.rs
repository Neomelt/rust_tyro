// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // 用元组重构
// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1 //dimensions是一种元组类型的参数
//                                 // 不太确定，后面查查
// }

// method syntax
// 这种写法是为了更好的面向对象
// 类比cpp中的类
// Rust 的方法只能在 impl 块里写，C++ 的成员函数写在类里。

// class Rectangle {
// public:
//     int width;
//     int height;
//     int area() const {
//         return width * height;
//     }
// };

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area() // 调用方法
//                      // 没有括号的是字段，有括号的是方法
//     );
//     println!("{:?}", rect1.width);
// }

// 关联函数
// 所有在 impl 块中定义的函数被称为关联函数（associated function），因为它们与 impl 后面命名的类型相关。我们可以定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例。我们已经使用了一个这样的函数，String::from 函数，它是在 String 类型上定义的。

// 关联函数经常被用作返回一个结构体新实例的构造函数。例如我们可以提供一个关联函数，它接受一个维度参数并且同时作为宽和高，这样可
// impl Rectangle {
//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size,
//         }
//     }
// }
// 使用结构体名和 :: 语法来调用这个关联函数：比如 let sq = Rectangle::square(3);。这个方法位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间。第 7 章会讲到模块。

// 多个 impl 块
// 每个结构体都允许拥有多个 impl 块，但每个方法有其自己的 impl 块。

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
