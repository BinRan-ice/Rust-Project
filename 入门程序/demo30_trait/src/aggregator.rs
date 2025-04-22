use std::fmt::Display;
use std::fmt::Debug;

//Summary trait 定义，它包含由 summarize 方法提供的行为
pub trait Summary {
    fn summarize(&self) -> String;
}

//NewsArticle 结构体
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//在 NewsArticle类型上实现 Summary trait
impl Summary for NewsArticle {  //在类型上实现 trait 类似于实现常规方法。区别在于 impl 关键字之后，我们提供需要实现 trait 的名称，接着是 for 和需要实现 trait 的类型的名称。
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//Tweet 结构体
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//在 Tweet 类型上实现 Summary trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


//默认实现：Summarys trait 的定义，带有一个 summarizes 方法的默认实现
pub trait Summarys {
    fn summarizes(&self) -> String {
        String::from("(Read more...)")
    }
}

//如果想要对 NewsArticle 实例使用这个默认实现，可以通过 impl Summary for NewsArticle {} 指定一个空的 impl 块。
impl Summarys for NewsArticle {}

//默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。
pub trait Summarys2 {
    fn summarizes2_author(&self) -> String;
    fn summarizes2(&self) -> String {
        format!("(Read more from {}...)", self.summarizes2_author())
    }
}

//在实现 trait 时定义 summarize2_author
impl Summarys2 for Tweet {
    fn summarizes2_author(&self) -> String {
        format!("@{}", self.username)
    }
}

//trait 作为参数
pub fn notify1(item: &impl Summary) {    //在 notify 函数体中，可以调用任何来自 Summary trait 的方法，比如 summarize
    println!("Breaking news! {}", item.summarize());
}

//Trait Bound 语法
pub fn notify2<T: Summary>(item: &T) {   //notify2 函数签名中的 item 参数是一个泛型 T 的实例，而 T 必须实现 Summary trait
    println!("Breaking news! {}", item.summarize());
}

/*
impl Trait:
pub fn notify(item1: &impl Summary, item2: &impl Summary) {

trait bound :
pub fn notify<T: Summary>(item1: &T, item2: &T) {
*/

//通过 + 指定多个 trait bound(以下两个函数等价)
//如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//通过 where 简化 trait bound（以下两个函数等价）
fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}
fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    0
}

//返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

//使用 trait bound 有条件地实现方法
/* use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
} */