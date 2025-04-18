mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//使用 use 关键字将路径引入作用域
use crate::front_of_house::hosting;

/*
pub use crate::front_of_house::hosting; //重导出（re-exporting）
在这个修改之前，外部代码需要使用路径 front_of_house::hosting::add_to_waitlist() 来调用 add_to_waitlist 函数。
现在这个 pub use 从根模块重导出了 hosting 模块，外部代码现在可以使用路径 hosting::add_to_waitlist()。
*/

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    pub fn eat_restaurant() {
        // use 只能创建 use 所在的特定作用域内的短路径,以下代码会报错,这是一个不同于 use 语句的作用域，所以函数体不能编译
        //hosting::add_to_waitlist();
    }

/*
使用 use 将 add_to_waitlist 函数引入作用域，这并不符合习惯.
我们必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
*/
}