use std::env;
use std::process;
use demo33_io_project::Config;

fn main() {
    //读取参数值
    //let args: Vec<String> = env::args().collect(); //env::args 函数返回一个迭代器，其第一个元素是调用 Rust 程序的名称，其余元素是传递给程序的每个参数

    //将参数值保存进变量
    /*
    当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。
    当 Result 是 Err 时，unwrap_or_else 会将 Err 的内部值(not enough arguments)，传递给闭包中位于两道竖线间的参数 err。
     */
    let config = Config::build(env::args()).unwrap_or_else(|err| {    //|err| 是一个闭包参数
        //将错误打印到标准错误
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);   //立即停止程序并将传递给它的数字作为退出状态码
    });

    //读取第二个参数所指定的文件内容,处理 main 中 run 返回的错误
    if let Err(e) = demo33_io_project::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    //let args: &[String] = &["hello".to_string(), "world".to_string()];
}