/*
使用Box<T>指向堆上的数据

box 允许你将一个值放在堆上而不是栈上.使用场景：
1. 当有一个在编译时未知大小的类型,而又需要在确切大小的上下文中使用这个类型值的时候
2. 当有一个大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
3. 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型时
*/

use crate::List::{Cons, Nil};

//cons list 的每一项都包含两个元素：当前项的值和下一项。其最后一项值包含一个叫做 Nil 的值且没有下一项。
#[derive(Debug)]
enum List { //定义一个代表 i32 值的 cons list 数据结构的枚举
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    //使用 Box<T> 在堆上储存数据
    let b = Box::new(5);
    println!("b = {}", b);
    println!("-------------------------------");

    //List 枚举储存列表 1, 2, 3
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("{:?}", list);
    println!("-------------------------------");
}