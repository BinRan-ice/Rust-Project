/*
Cargo 有两个主要的配置：
(1)运行 cargo build 时采用的 dev 配置
(2)运行 cargo build --release 的 release 配置。
dev 配置为开发定义了良好的默认配置，release 配置则为发布构建定义了良好的默认配置。
*/

/*
如下是 dev 和 release 配置的 opt-level 设置的默认值(文件名：Cargo.toml)：
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

opt-level 设置控制 Rust 会对代码进行何种程度的优化。这个配置的值从 0 到 3。越高的优化级别需要更多的时间编译，但是生成的代码会更快运行。
*/

fn main() {
    println!("Hello, world!");
}
