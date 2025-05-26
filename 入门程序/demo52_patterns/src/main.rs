/*
所有可能会用到模式的位置
比如 match 表达式、if let 表达式、while let 表达式、for 循环、let 语句、函数参数、闭包参数、结构体字段、枚举成员、
宏、模块、trait、类型别名、类型参数、甚至是常量、静态变量、全局变量等等。
*/

fn main() {
    //一个模式常用的位置是 match 表达式的分支
    let x: Option<i32> = Some(5);
    match x {
        //match 表达式必须是 穷尽（exhaustive）的，意为 match 表达式所有可能的值都必须被考虑到
        None => println!("None"),
        Some(i) => println!("Some: {}", i + 1),
    }
    println!("-------------------------------");

    //模式也可以用在 if let 表达式中
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {   //结合 if let、else if、else if let 以及 else
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    println!("-------------------------------");

    //模式也可以用在 while let 表达式中
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });
    while let Ok(val) = rx.recv() { //使用 while let 循环只要 stack.pop() 返回 Some 就打印出其值
        println!("Got: {}", val);
    }
    println!("-------------------------------");

    //模式也可以用在 for 循环中
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {    //在 for 循环中使用模式来解构元组
        println!("{} is at index {}", value, index);
    }
    println!("-------------------------------");

    //模式也可以用在 let 语句中
    let (x, y, z) = (1, 2, 3);  //在 let 语句中使用模式解构元组
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("-------------------------------");

    //模式也可以用在函数参数中
    let point = (3, 5);
    print_coordinates(&point);
    println!("-------------------------------");
}

fn print_coordinates(&(x, y): &(i32, i32)) {   //在函数参数中使用模式解构元组
    println!("Current location: ({}, {})", x, y);
}