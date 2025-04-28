/*
通过 Deref trait 将智能指针当作常规引用处理

实现 Deref trait 允许我们重载 解引用运算符（dereference operator）*（不要与乘法运算符或通配符相混淆）。
通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。
*/

use std::ops::Deref;

struct MyBox<T>(T); //定义一个包含单个成员的元组结构体 MyBox，这个成员的类型是泛型 T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//通过实现 Deref trait 将某类型像引用一样处理
impl<T> Deref for MyBox<T> {
    type Target = T;  //定义关联类型 Target，它将被用于关联类型的类型声明

    fn deref(&self) -> &T {
        &self.0 //deref 返回了 * 运算符访问的值的引用
    }
}

fn main() {
    //追踪指针的值
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);  //使用解引用运算符来跟踪 i32 值的引用

    //在 Box<i32> 上使用解引用运算符
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);  //使用解引用运算符来跟踪 i32 值的 Box

    //因为 Deref 强制转换，使用 MyBox<String> 的引用调用 hello 是可行的
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    /* //如果 Rust 没有 Deref 强制转换则必须编写的代码
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
     */
}

//hello 函数有着 &str 类型的参数 name
fn hello(name: &str) {
    println!("Hello, {}!", name);
}