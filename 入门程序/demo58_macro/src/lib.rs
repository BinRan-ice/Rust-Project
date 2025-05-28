//一个 vec! 宏定义的简化版本
#[macro_export] //#[macro_export] 注解表明只要导入了定义这个宏的 crate，该宏就应该是可用的。如果没有该注解，这个宏不能被引入作用域。
macro_rules! vec {  //开始宏定义
    ( $( $x:expr ),* ) => { //$x:expr匹配一个表达式，而$( $x:expr ),*允许匹配多个表达式，并用逗号分隔。
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/* /*
第二种形式的宏被称为 过程宏（procedural macros），因为它们更像函数（一种过程类型）
过程宏接收 Rust 代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出，
而非像声明式宏那样匹配对应模式然后以另一部分代码替换当前代码。
*/

//一个定义过程宏的例子
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {   //定义过程宏的函数接收一个 TokenStream 作为输入并生成 TokenStream 作为输出
    // 一些操作
} */

// 将宏库的内容导入并重新导出
pub use hello_macro_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}

/*
类属性宏
类属性宏允许创建属性来定义函数或结构体。这些宏看起来像函数调用，不过不同于函数调用，它们作用于整个项而不是项的部分。

#[route(GET, "/")]
fn index() {}

#[route]宏定义的函数签名看起来像这样：
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
*/

/*
类函数宏
类函数（Function-like）宏的定义看起来像函数调用的宏。类似于 macro_rules!，它们比函数更灵活

let sql = sql!(SELECT * FROM posts WHERE id=1);

sql! 宏应该被定义为如此：
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
*/