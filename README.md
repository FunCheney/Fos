
# 本项目模块介绍

| 模块         |                       | 功能介绍                                           |
|:-----------|:----------------------|:-----------------------------------------------|
| os         | 操作系统                  | rust 实现                                        |
| 1️⃣        | bootloader            | qemu 启动依赖                                      |
| 2️⃣        | easy-fs               | 文件系统模块                                         |
| 3️⃣        | easy-fs-fuse          | 文件系统测试                                         |
| 4️⃣        | user                  | os 用户态程序                                       |
| code       | risc-v 指令             |                                                |
| 1️⃣        | asm                   | 汇编代码                                           |
| 2️⃣        | os                    | c实现测试                                          |
| rust-study | rust 学习项目             | rust 代码                                        |
|            | rust-atomics-and-locks | https://marabos.nl/atomics/ 代码示例               |
|            | rust_by_example       | https://rustwiki.org/zh-CN/rust-by-example/ 代码 |
|            | rust_lang_book        | https://doc.rust-lang.org/book/ 代码             |
| tests 目录下  |                       | 极客时间课程示例与笔记                                    |
| 几个小工具      |
| 1️⃣        | getinfo               | 网络服务客户端&服务端                                    |
| 2️⃣        | guessing_game         | 猜字游戏                                           |
| 3️⃣        | httpie                | 模拟 curl                                        |
| 4️⃣        | minigrep              | 模拟 grep                                        |
| 5️⃣        | myallocator           | 模拟内存分配                                         |


# os 快速启动

### rust 环境安装
通过下述命令安装 rust 环境
```
curl https://sh.rustup.rs -sSf | sh
```
通过下述命令校验是否安装成功
```
rustc --version
```
通过如下命令安装 rustc 的 nightly 版本，并把该版本设置为 rustc 的默认版本。
```
rustup install nightly
rustup default nightly
```
再次确认安装版本
```
rustc --version
```
接下来安装一些Rust相关的软件包
```
rustup target add riscv64gc-unknown-none-elf
rustup component add llvm-tools-preview
rustup component add rust-src

```

### QEMU 模拟器安装
```
# for Debian/Ubuntu
sudo apt-get install qemu-system
```
注：mac m1 也可使用
```
# for macos
brew install qemu
```

### 快速体验

```
git checkout feature/step-1
```
运行
```
make run
```
可看到对应的输出


