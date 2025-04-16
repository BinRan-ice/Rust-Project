fn main() {
/*
    $ cargo new my-project
        Created binary (application) `my-project` package
    $ ls my-project
    Cargo.toml
    src
    $ ls my-project/src
    main.rs
运行了这条命令后，我们先用 ls 来看看 Cargo 给我们创建了什么，
Cargo 会给我们的包创建一个 Cargo.toml 文件。查看 Cargo.toml 的内容，
会发现并没有提到 src/main.rs，因为 Cargo 遵循的一个约定：
src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。
同样的，Cargo 知道如果包目录中包含 src/lib.rs，
则包带有与其同名的库 crate，且 src/lib.rs 是 crate 根。
crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。

在此，我们有了一个只包含 src/main.rs 的包，
意味着它只含有一个名为 my-project 的二进制 crate。
如果一个包同时含有 src/main.rs 和 src/lib.rs，
则它有两个 crate：一个二进制的和一个库的，且名字都与包相同。
通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：
每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。 
 */
}
