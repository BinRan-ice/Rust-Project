use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;

fn main() {
    //监听传入的流并在接收到流时打印信息
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //bind 函数返回 Result<T, E>，这表明绑定可能会失败。
    let pool = ThreadPool::new(4);  //创建有限数量的线程

    for stream in listener.incoming().take(2) { //调用 incoming 方法来获取流并在 for 循环中处理它们,在处理两个请求之后通过退出循环来停止 server
        let stream = stream.unwrap();

        //为每一个流新建一个线程
        pool.execute(|| {   //调用 execute 方法并传递一个闭包，这个闭包会在新线程中运行
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

//读取 TcpStream 并打印数据
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); //获取请求行

    //重构使得 if 和 else 块中只包含两个情况所不同的代码
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "pages/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "pages/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "pages/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
