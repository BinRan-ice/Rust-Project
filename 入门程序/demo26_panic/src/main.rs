fn main() {
    //在一个简单的程序中调用 panic!
    //panic!("crash and burn");

    /*
    thread 'main' panicked at src/main.rs:17:6:
    index out of bounds: the len is 3 but the index is 99
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    错误指向 main.rs 的第 17 行，这里我们尝试访问索引 99。
    下面的说明（note）行提醒我们可以设置 RUST_BACKTRACE 环境变量来得到一个 backtrace。
    backtrace 是一个执行到目前位置所有被调用的函数的列表。
    Rust 的 backtrace 跟其他语言中的一样：阅读 backtrace 的关键是从头开始读直到发现你编写的文件。
    这就是问题的发源地。这一行往上是你的代码所调用的代码；往下则是调用你的代码的代码。
    这些行可能包含核心 Rust 代码，标准库代码或用到的 crate 代码。
     */
    let v = vec![1, 2, 3];
    v[99];
}
