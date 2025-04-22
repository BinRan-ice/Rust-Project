/*
当编写一个先会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，
还可以选择让调用者知道这个错误并决定该如何处理。这被称为 传播错误，这样能更好的控制代码调用
*/
use std::fs::File;
use std::error::Error;
use std::io::{self,Read};

//修改 main 返回 Result<(), E> 允许对 Result 值使用 ? 运算符
fn main() -> Result<(), Box<dyn Error>> {   //Box<dyn Error> 类型是一个 trait 对象,它可以表示任何实现 Error trait 的类型
    let username = read_username_from_file1();
    match username {
        Ok(username) => println!("Username: {}", username),
        Err(e) => panic!("Error reading username: {:?}", e),
    }

    //? 运算符只能被用于返回值与 ? 作用的值相兼容的函数。
    let greeting_file = File::open("hello.txt")?;  //尝试在返回 () 的 main 函数中使用 ? 的代码不能编译
    Ok(())
}

//一个函数使用 match 将错误返回给代码调用者
fn read_username_from_file1() -> Result<String, io::Error> { //函数返回一个 Result<T, E> 类型的值，其中泛型参数 T 的具体类型是 String，而 E 的具体类型是 io::Error。
    let username_file_result = File::open("username.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), //如果 File::open 返回 Err，我们就会立即返回这个 Err 作为 read_username_from_file 函数的返回值
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//传播错误的简写：? 运算符
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;    //如果 File::open 返回 Err，? 运算符会立即返回 Err 并将 Err 的值传递给 from 函数,它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

//问号运算符之后的链式方法调用
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?; //? 运算符只能被用于返回 Result 的函数
    Ok(username)
}

//? 也可用于 Option<T> 值
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}