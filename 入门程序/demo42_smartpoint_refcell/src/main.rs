/*
RefCell<T> 和内部可变性模式

内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。
为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则。
不安全代码表明我们在手动检查这些规则而不是让编译器替我们检查。
*/

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    //内部可变性：不可变值的可变借用
    let x = 5;
    //let y = &mut x; //error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable -> 当有一个不可变值时，不能可变地借用它

    //结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者c
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
