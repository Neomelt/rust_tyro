// fn main() {
//     let number = 7;

//     // if value must be a boolean
//     // if number { is not allowed }
//     // if number == 5 { is allowed }
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// // else if
// // every branch must be a same type
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     println!("The value of number is: {number}");
// }

// fn main() {
//     loop {
//         println!("again");
//         break;
//     }
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }

// fn main() {
//     // rev change the range to 3, 2, 1
//     for number in (1..4).rev() {
//         println!("{}!", number);
//     }
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [4, 3, 2, 1];
//     // `.iter()` 方法把 `a` 数组变成一个迭代器
//     for (i, v) in a.iter().enumerate() {
//         println!("第{}个元素是{}", i + 1, v);
//     }
// }

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
