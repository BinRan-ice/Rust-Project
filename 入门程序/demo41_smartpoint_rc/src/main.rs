/*
Rc<T> 引用计数智能指针

为了启用多所有权需要显式地使用 Rust 类型 Rc<T>，其为 引用计数（reference counting）的缩写。
引用计数意味着记录一个值的引用数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
*/

use std::rc::Rc;    //使用 use 语句将 Rc<T> 引入作用域
use crate::List::{Cons, Nil};

/* enum List {
    Cons(i32, Box<List>),
    Nil,
} */

//使用 Rc<T> 定义的 List
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    //不能用两个 Box<T> 的列表尝试共享第三个列表的所有权
    //let a = Cons(5,Box::new(Cons(10,Box::new(Nil))));
    //let b = Cons(3,Box::new(a));
    //let c = Cons(4,Box::new(a)); //error[E0382]: use of moved value: `a` -> 值 `a` 已经被移动了，不能再次使用

    //使用 Rc<T> 允许多个所有者共享数据
    let a = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3,Rc::clone(&a));  //克隆 Rc<T> 会增加引用计数
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4,Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
