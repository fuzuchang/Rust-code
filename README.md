# Rust-code
Rust是Mozilla开发的注重安全、性能和并发性的系统级编程语言

#### 使用Cargo 新建一个Rust二进制项目 传递 --bin
> cargo new project-name --bin
***
####基于cargo的rust项目组织结构
* benches/   存放基准测试源代码
* examples/  存放示例程序源代码
* src/       存放源代码
* src/main.rs 默认的可执行程序入口文件
* src/lib.rs  默认的库入口文件
* src/bin/*.rs 其它可执行文件，一个rs文件均对应一个可执行文件
* target/    Cargo生成文件目录
* tests/     存放外部测试源代码
