### cargo
rust 中的包管理工具


cargo 中常用的命令
- `cargo run (--release)` 编译+运行rust项目  --release 是高性能的模式
- `cargo build (--release)` 编译项目
- `cargo check` 快速检查代码是否能够通过 该命令很快 节省很多的编译时间

#### Cargo.toml 和 Cargo.lock

Cargo.toml 是项目数据描述文件。
> Cargo.toml 类似于 package.json

Cargo.lock 是 Cargo 工具根据 toml文件生成的项目依赖详细清单，一般不需要修改。

##### 项目依赖
toml中一共可以定义三种依赖：
- rust 官方仓库 crates.io, 通过版本说明来描述。
- 项目的源代码git的仓库地址，通过 URL 来描述。
- 通过本地的绝对地址和相对地址，通过 Unix 模式来描述。


基本语法

变量的可变性与不变性(variables)

```rust
let a = 5
```
a 就是一个不可变的变量

```rust
let mut a = 5
a = 6
```
用了mut关键字后，a此时是个可变的变量

对于大对象处理中，使用不可变变量来定义。每次只在一个变量上面来修改。
对于小的数据结构来说，使用可变变量来修饰，更容易来理解。


使用下划线开头忽略未使用的变量

创建了一个变量没有使用，rust会给一个警告。
```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```
y这边rust编译时候会报一个warning，但_x却不会。





