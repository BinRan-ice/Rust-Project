use std::ops::Add;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {    //实现 Add trait 重载 Point 实例的 + 运算符
    type Output = Point;    //定义关联类型 Output，用于指定 + 运算符的返回类型
    fn add(self, other: Point) -> Point {    //定义 add 方法
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

//完全限定语法与消歧义：调用相同名称的方法
trait Pilot {   //两个 trait 定义为拥有 fly 方法，并在直接定义有 fly 方法的 Human 类型上实现这两个 trait
    fn fly(&self);
}

trait Wizard {  //两个 trait 定义为拥有 fly 方法，并在直接定义有 fly 方法的 Human 类型上实现这两个 trait
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {  //定义一个 Animal trait，其中包含一个关联函数 baby_name
    fn baby_name() -> String;
}

struct Dog;

impl Dog {  
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {   //为 Dog 结构体实现 Animal trait
    fn baby_name() -> String {
        String::from("puppy")
    }
}

//实现 OutlinePrint trait，它要求来自 Display 的功能
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// 在 Point 上实现 OutlinePrint
impl OutlinePrint for Point {}

impl fmt::Display for Point {    //在 Point 上实现 Display 并满足 OutlinePrint 要求的限制
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//创建 Wrapper 类型封装 Vec<String> 以便能够实现 Display
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("-----------------------------------------");

    let person = Human; //调用 Human 实例的 fly
    Pilot::fly(&person);    //指定我们希望调用哪一个 trait 的 fly 方法
    Wizard::fly(&person);   //指定我们希望调用哪一个 trait 的 fly 方法
    person.fly();
    println!("-----------------------------------------");

    /*
    不是方法的关联函数没有 self 参数。
    当存在多个类型或者 trait 定义了相同函数名的非方法函数时，Rust 就不总是能计算出我们期望的是哪一个类型。
    除非使用 完全限定语法（fully qualified syntax）。
    <Type as Trait>::function(receiver_if_method, next_arg, ...);
     */
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());    //使用完全限定语法来指定我们希望调用的是 Dog 上 Animal trait 实现中的 baby_name 函数
    println!("-----------------------------------------");

    //在 Point 实例上调用 outline_print 来显示位于星号框中的点的值
    let p = Point { x: 1, y: 2 };
    p.outline_print();
    println!("-----------------------------------------");

    //在 Wrapper 实例上调用 Display 来显示包含两个字符串的 Wrapper 的值
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    println!("-----------------------------------------");
}