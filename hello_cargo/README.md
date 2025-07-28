# Cargo

## cargo new

创建一个项目

## cargo run

编译并运行项目

## cargo build

构建/编译一个项目

当在后面加上release参数时会编译出release版本

``` bash
xlqmu@xlqmu-Predator-PHN16-71:~/code_learning/rust/hello_cargo$ cargo build --release 
   Compiling hello_cargo v0.1.0 (/home/xlqmu/code_learning/rust/hello_cargo)
    Finished `release` profile [optimized] target(s) in 0.06s

```

同时会在target目录下生成一个release文件夹

## cargo check

检查代码是否可能编译，但是不生成二进制文件
