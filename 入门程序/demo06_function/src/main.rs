fn main() {
    println!("Hello, world!");
    println!("-----------------------------------------");
    another_function1();
    println!("-----------------------------------------");
    another_function2(5);
    println!("-----------------------------------------");
    another_function3(5, 'c');
    println!("-----------------------------------------");
    another_function4();
    println!("-----------------------------------------");
    let x = another_function5();
    println!("The value of x is: {x}");
    println!("-----------------------------------------");
    let y = another_function6(5);
    println!("The value of y is: {y}");
    println!("-----------------------------------------");
}

//注意，源码中another_function可以定义在main函数之后；也可以定义在之前。Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行。
fn another_function1() {
    println!("Another function.");
}

//函数参数
fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

//多个参数
fn another_function3(value: i32, unit_label: char) {
    println!("The value is: {value}, The unit label is: {unit_label}");
}

//表达式:表达式是计算并产生一个值的代码。表达式不以分号结尾。如果在表达式的结尾加上分号，它就变成了一个语句，而语句不返回值。
fn another_function4() {
    let y = {
        let x = 3; //这是一个语句
        x + 1 //注意，这里没有分号，这是因为这是一个表达式。如果在 x + 1 后面加上分号，这就变成了一个语句，而语句不返回值。
    };
    println!("The value of y is: {y}");
}

//具有返回值的函数：函数体中的最后一个表达式被用作返回值。或者说，函数体中最后一个表达式的值被隐式返回。
fn another_function5() -> i32 {
    5
}

//运行代码会打印出 The value of x is: 6。但如果在包含 x + 1 的行尾加上一个分号，把它从表达式变成语句，我们将看到一个错误。
fn another_function6(x: i32) -> i32 {
    x + 1
}