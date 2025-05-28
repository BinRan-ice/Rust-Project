use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    //监听传入的流并在接收到流时打印信息
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //bind 函数返回 Result<T, E>，这表明绑定可能会失败。

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

//读取 TcpStream 并打印数据
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    /* let http_request: Vec<_> = buf_reader   //收集浏览器发送给服务端的请求行
    .lines()    //遇到换行符（newline）字节就切分数据流的方式返回一个 Result<String, std::io::Error> 的迭代器
    .map(|line| line.unwrap())  //将 Result<String, std::io::Error> 转换为 String
    .take_while(|line| !line.is_empty())    //take_while 方法获取迭代器中的元素，直到闭包返回 false
    .collect(); //将迭代器中的元素收集到一个集合中 */
    let request_line = buf_reader.lines().next().unwrap().unwrap(); //获取请求行

    /* if request_line == "GET / HTTP/1.1" {
        //如果请求行是 GET / HTTP/1.1
        //一个微型成功 HTTP 响应写入流
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("pages/hello.html").unwrap(); //读取 hello.html 文件的内容
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); //将 status_line 和 contents 字符串连接到 response 字符串

        stream.write(response.as_bytes()).unwrap(); //将 response 字符串的字节写入流
    } else { //对于任何不是 / 的请求返回 404 状态码的响应和错误页面
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("pages/404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } */
    //重构使得 if 和 else 块中只包含两个情况所不同的代码
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "pages/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
