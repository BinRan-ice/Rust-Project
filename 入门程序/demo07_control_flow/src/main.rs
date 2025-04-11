fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    println!("-----------------------------------------");

    //多重条件
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!("-----------------------------------------");

    //在let语句中使用if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    //以下代码会报错。if 代码块中的表达式返回一个整数，而 else 代码块中的表达式返回一个字符串。Rust需要在编译时就确切的知道 number 变量的类型。
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    //使用loop重复执行代码
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");
    println!("-----------------------------------------");

    //循环标签：在嵌套循环中，break 语句只会停止最内层的循环。要想停止外层循环，可以给循环加上一个标签，并在 break 语句后面跟上这个标签。
    let mut counter = 0;
    'counting_up: loop {
        println!("counter: {counter}");
        let mut remaining = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9{
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End count = {counter}");
    println!("-----------------------------------------");

    //while循环
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("-----------------------------------------");

    //for循环
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
