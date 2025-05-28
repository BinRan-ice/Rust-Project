fn add_one(x: i32) -> i32 {
    x + 1
}

//do_twice 函数接受一个函数 f 和一个 i32 参数 arg，并调用两次 f，传递 arg 作为参数
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

//返回闭包
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);    //do_twice 函数调用 add_one 函数两次，传递 5 作为参数
    println!("The answer is: {}", answer);
    println!("-----------------------------------------");

    //将函数作为 map 的参数来代替闭包
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();    //使用闭包
    println!("{:?}", list_of_strings);
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();    //使用函数指针
    println!("{:?}", list_of_strings);
    println!("-----------------------------------------");

    //使用返回闭包的函数
    let f = returns_closure();
    let answer = f(5);
    println!("The answer is: {}", answer);
    println!("-----------------------------------------");
}