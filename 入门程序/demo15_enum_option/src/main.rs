#[derive(Debug)]
enum Option<T> {    //<T>:它是一个泛型类型参数,它允许我们定义一个Option枚举可以包含任何类型的数据。
    None,
    Some(T),
}

fn main() {
    let some_number = Option::Some(5);
    let some_char = Option::Some('a');
    let absent_number: Option<i32> = Option::None;
    println!("The some_number is: {:?}", some_number);
    println!("The some_char is: {:?}", some_char);
    println!("The absent_number is: {:?}", absent_number);
    println!("-----------------------------------------");
}
