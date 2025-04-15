/*
方法（method）与函数类似：
它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
不过方法与函数是不同的，因为它们在结构体的上下文中被定义，并且它们第一个参数总是 self，它代表调用该方法的结构体实例。
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
在一个 impl 块中，Self 类型是 impl 块的类型的别名。
方法的第一个参数必须有一个名为 self 的Self 类型的参数，所以 Rust 让你在第一个参数位置上只用 self 这个名字来简化。
注意，我们仍然需要在 self 前面使用 & 来表示这个方法借用了 Self 实例，就像我们在 rectangle: &Rectangle 中做的那样。
方法可以选择获得 self 的所有权，或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。
*/
impl Rectangle {
    //所有在 impl 块中定义的函数被称为 关联函数（associated functions）。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //在 Rectangle 上实现 can_hold 方法，它获取另一个 Rectangle 实例作为参数，返回一个布尔值。
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*
可以选择将方法的名称与结构中的一个字段相同。
在 main 中，当我们在 rect1.width 后面加上括号时。Rust 知道我们指的是方法 width。
当我们不使用圆括号时，Rust 知道我们指的是字段 width。
*/
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

/*
不是方法的关联函数经常被用作返回一个结构体新实例的构造函数。
这些函数的名称通常为 new ，但 new 并不是一个关键字。
例如我们可以提供一个叫做 square 关联函数，它接受一个维度参数并且同时作为宽和高.
这样可以更轻松的创建一个正方形 Rectangle 而不必指定两次同样的值
*/
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a non-zero width; it is {}", rect1.width);
    }
    println!("-----------------------------------------");

    let rect2 = Rectangle {
        width: 50,
        height: 70,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("-----------------------------------------");

    //使用结构体名和 :: 语法来调用这个关联函数
    let sq = Rectangle::square(3);
}
