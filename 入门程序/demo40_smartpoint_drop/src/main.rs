/*
使用 Drop Trait 运行清理代码

对于智能指针模式来说第二个重要的 trait 是 Drop，其允许我们在值要离开作用域时执行一些代码。
可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源。
*/

use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

//结构体 CustomSmartPointer，其实现了放置清理代码的 Drop trait
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    //尝试手动调用 Drop trait 的 drop 方法提早清理
    //c.drop(); //error[E0040]: explicit use of destructor method -> 错误信息表明不允许显式调用 drop 方法

    //如果我们需要强制提早清理值，可以使用 std::mem::drop 函数
    drop(c); //调用 drop 函数时，Rust 不会在 c 离开作用域时再次调用 drop 方法
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}