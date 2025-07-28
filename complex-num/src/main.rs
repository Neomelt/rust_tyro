use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    // 使用 Complex { re: 2.1, im: -1.2 } 创建复数
    let b = Complex::new(11.1, 22.2);
    // 使用 Complex::new() 创建复数
    //  这两种方式是等价的
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}
