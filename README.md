# rust_tyro

这是我个人学习 Rust 的仓库，记录练习源码和中文注释笔记。按知识点拆成独立的 Cargo 项目，边写代码边记录自己的理解、疑问和踩过的坑，方便之后回顾。

主要跟着 The Rust Programming Language（The Book）和《Rust 语言圣经》学习，后面计划配合 Rustlings 继续练习。

## 已学内容

| 内容 | 对应目录 |
| --- | --- |
| Cargo 项目的创建、编译与运行 | [hello_cargo](hello_cargo/) |
| 变量、可变性、遮蔽、基本类型、数组、解构、函数与控制流 | [variable](variable/)、[shadwow](shadwow/)、[data_type](data_type/)、[array](array/)、[variable_deconstruction](variable_deconstruction/)、[function](function/)、[control_flow](control_flow/) |
| 所有权、移动与复制、引用与借用、切片 | [ownership](ownership/)、[references](references/)、[slice_type](slice_type/) |
| 结构体、方法与关联函数 | [structs](structs/)、[rectangulars](rectangulars/) |
| 枚举、模式匹配、`if let`，以及用 `Box` 构建链表的练习 | [enums](enums/)、[match_control_flow](match_control_flow/)、[if_let](if_let/)、[enum_list](enum_list/) |
| `Vec`、字符串与 `HashMap` | [collections](collections/)、[hashmap](hashmap/) |
| 错误处理：`Result`、`panic!`、`unwrap`、`?` | [Error_Handling](Error_Handling/)、[async_demo](async_demo/) |
| 闭包、异步编程与 Tokio | [async_demo](async_demo/) |

还有一些小练习：

- [guessing_game](guessing_game/)：猜数字。
- [complex-num](complex-num/)：复数类型与运算。
- [find_color](find_color/)：使用 OpenCV 转换颜色空间、提取蓝色区域并显示结果，需要本机配置 OpenCV 和图形环境。
- `test_02_*` 等目录：学习过程中的语法练习。

## 异步学习进度

[async_demo/examples](async_demo/examples/) 按学习顺序保存示例，一个主题一个文件，保留每次学习的源码和注释。

**目前学到 `20_join_set.rs`。**

| 示例编号 | 学习内容 |
| --- | --- |
| 01～03 | Future 的惰性执行、运行时与 `block_on`、异步 `sleep` |
| 04～06 | `join!`、`spawn`、`JoinHandle`、`select!` |
| 07～09 | 任务返回值、任务错误与业务错误、嵌套 `Result`、超时 |
| 10～13 | `mpsc` 通道、发送端克隆、`Arc` 共享所有权、`Mutex` 互斥锁 |
| 14～17 | 阻塞任务、闭包与 `move`、`abort`、通过 `oneshot` 通知任务优雅关闭 |
| 18～20 | `'static`、`Send`、`JoinSet`，以及任务的执行顺序 |

后续示例已经写好，留着继续学习：

- [21_semaphore.rs](async_demo/examples/21_semaphore.rs)：限制同时进行的操作数量。
- [22_tcp_server.rs](async_demo/examples/22_tcp_server.rs) / [22_tcp_client.rs](async_demo/examples/22_tcp_client.rs)：TCP 回声通信。
- [23_framing.rs](async_demo/examples/23_framing.rs)：消息边界与缓冲。
- [24_device_collector.rs](async_demo/examples/24_device_collector.rs)：模拟采集、通道通信、批量保存 CSV 与优雅关闭的综合练习。

## 运行方式

各个学习目录是独立的 Cargo 项目。在仓库根目录可以指定项目运行：

```bash
cargo run --manifest-path guessing_game/Cargo.toml
```

运行某个异步示例：

```bash
cargo run --manifest-path async_demo/Cargo.toml --example 20_join_set
```

也可以先进入 `async_demo` 目录，再运行：

```bash
cargo run --example 20_join_set
```

TCP 服务端和客户端需要在两个终端中分别运行，具体命令写在对应文件开头。
