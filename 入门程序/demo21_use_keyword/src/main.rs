use std::collections::HashMap;
use std::fmt;
use std::io;

//使用 as 关键字提供新的名称
use std::io::Result as IoResult;

//使用外用包
use rand::Rng;

//嵌套路径来消除大量的 use 行
use std::{cmp::Ordering, io};

//使用 glob 运算符将所有公有定义引入作用域
use std::collections::*;

/*
以下导入方式等价
use std::io;
use std::io::Write;

use std::io::{self, Write};
*/

//将 HashMap 引入作用域的习惯用法
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);

    let secret_number = rand::thread_rng().gen_range(1..=100);
}

//使用父模块将两个具有相同名称的类型引入同一作用域
fn function1() -> fmt::Result {
    // --snip--
}
fn function2() -> io::Result<()> {
    // --snip--
}
fn function3() -> IoResult<()> {
    // --snip--
}