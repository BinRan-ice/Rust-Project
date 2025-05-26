/*
所有的模式语法
*/

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },    //一个匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32), //元组结构体
    Hsv(i32, i32, i32), //元组结构体
}

enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color), //枚举成员包含了一个结构体
}

struct Points {
    x: i32,
    y: i32,
    z: i32,
}

enum Messagess {
    Hello { id: i32 },
}

fn main() {
    //匹配字面值
    let x = 1;
    match x {   //这段代码会打印 one 因为 x 的值是 1。如果希望代码获得特定的具体值，则该语法很有用。
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!("-------------------------------");

    //匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {   //一个 match 语句其中一个分支引入了覆盖变量 y
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),    
        _ => println!("Default case, x = {:?}", x), //如果 x 的值是 None 而不是 Some(5)，头两个分支的模式不会匹配，所以会匹配下划线
    }
    println!("At the end: x = {:?}, y = {:?}", x, y);
    println!("-------------------------------");

    //多个模式
    let x = 1;
    match x {   //可以使用 | 语法匹配多个模式，它代表 或（or）运算符模式
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!("-------------------------------");

    //通过 ..= 匹配值的范围
    let x = 5;
    match x {   //..= 语法允许你匹配一个闭区间范围内的值。
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    //使用 char 类型值范围的例子
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    println!("-------------------------------");

    //解构一个结构体的字段为单独的变量
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;   //解构结构体 Point 并绑定变量 a 和 b 到 p.x 和 p.y
    //let Point { x, y } = p; //如果变量名和字段名相同，可以简写为 let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    println!("p.x: {}, p.y: {}", a, b);
    match p {   //解构和匹配模式中的字面值
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }
    println!("-------------------------------");

    //解构包含不同类型值成员的枚举
    //let msg = Message::Quit;
    //let msg = Message::Move { x: 1, y: 2 };
    //let msg = Message::Write(String::from("hello"));
    let msg = Message::ChangeColor(0, 160, 255);
    match msg { //解构枚举 Message 的不同成员
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }
    println!("-------------------------------");

    //解构嵌套的结构体和枚举
    let msg = Messages::ChangeColor(Color::Hsv(0, 160, 255));
    match msg { //解构枚举 Messages 的不同成员
        Messages::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        Messages::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v);
        }
        _ => (),
    }
    println!("-------------------------------");

    //解构结构体和元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);
    println!("-------------------------------");

    //忽略整个值
    foo(3, 4);
    println!("-------------------------------");

    //使用嵌套的 _ 忽略部分值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {  //当不需要 Some 中的值时在模式内使用下划线来匹配 Some 成员
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
    println!("-------------------------------");

    //忽略元组中的某些值
    let numbers = (2, 4, 8, 16, 32);
    match numbers { //使用 _ 忽略元组中的某些值
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
    println!("-------------------------------");

    //通过在名字前以一个 _ 开头来忽略未使用的变量
    let _x = 5;
    let y = 10;
    println!("y = {}", y);
    println!("-------------------------------");

    //用..忽略剩余部分
    let origin = Points { x: 0, y: 0, z: 0 };
    match origin { //通过使用 .. 来忽略 Point 中除 x 以外的字段
        Points { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers { //通过使用 .. 忽略元组中的剩余部分
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    println!("-------------------------------");

    //匹配守卫提供的额外条件
    let num = Some(4);
    match num { //使用 match 守卫来匹配只有在条件为真时才匹配的值
        Some(x) if x % 2 == 0 => println!("Even number: {}", x),
        Some(x) => println!("Odd number: {}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x { //使用匹配守卫来测试与外部变量的相等性
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;
    match x { //结合多个模式与匹配守卫
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    println!("-------------------------------");

    //at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
    let msg = Messagess::Hello { id: 5 };
    match msg { //使用 @ 在模式中绑定值的同时测试它
        Messagess::Hello { id: id_variable @ 3..=7 } => {   //测试 Message::Hello 的 id 字段是否位于 3..=7 范围内，同时也希望能将其值绑定到 id_variable 变量中以便此分支相关联的代码可以使用它。
            println!("Found an id in range: {}", id_variable)
        }
        Messagess::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Messagess::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
    println!("-------------------------------");
}

//在函数签名中使用 _
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}