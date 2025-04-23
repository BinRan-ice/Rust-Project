/* fn main() {
    //生命周期避免了悬垂引用
    let r;
    {
        let x = 5;
        r = &x;
    }
    //println!("r: {}", r); //尝试使用离开作用域的值的引用。error: `x` does not live long enough
                          // 生命周期注解语法
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+ */

/* //main 函数调用 longest 函数来寻找两个字符串 slice 中较长的一个
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

//以下代码会报错，因为 Rust 并不知道将要返回的引用是指向 x 或 y。
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */

/* /*
生命周期注解语法:
    生命周期参数名称必须以撇号（'）开头，其名称通常全是小写，类似于泛型其名称非常短。
    大多数人使用 'a 作为第一个生命周期注解。生命周期参数注解位于引用的 & 之后，并有一个空格来将引用类型与生命周期注解分隔开。
*/
fn main() {
    &i32       // 引用
    &'a i32    // 带有显式生命周期的引用
    &'a mut i32// 带有显式生命周期的可变引用
}
 */

/* //main 函数调用 longest 函数来寻找两个字符串 slice 中较长的一个
fn main() {
    //通过拥有不同的具体生命周期的 String 值调用 longest 函数
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}

//以下代码不会报错，因为 Rust 知道返回的引用的生命周期与传入的引用的生命周期有关。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { //longest 函数返回的引用的生命周期与函数参数所引用的值的生命周期的较小者一致
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */

/*
深入理解生命周期

如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
*/

//结构体定义中的生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,  //一个存放引用的结构体，所以其定义需要生命周期注解
}

//方法定义中的声明周期注解
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

//适用于第三条生命周期省略规则的例子
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt { part: first_sentence };

    //静态生命周期：'static 这个生命周期可以存活于整个程序期间
    let s: &'static str = "I have a static lifetime.";
}

/*
编译器采用三条规则来判断引用何时不需要明确的注解。
第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。
如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。

第一条规则是编译器为每一个引用参数都分配一个生命周期参数。
换句话说就是，函数有一个引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32)，
有两个引用参数的函数就有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。

第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。

第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
说明是个对象的方法 (method)，那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。
*/

/* //结合泛型类型参数、trait bounds 和生命周期
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */