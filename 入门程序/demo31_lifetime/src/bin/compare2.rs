//泛型函数＋生命周期+trait bounds比较两个数值的大小，返回较大的那个数值
fn longest<'a, T, U>(x: &'a T, y: &'a U) -> &'a str
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let x_str = x.as_ref();
    let y_str = y.as_ref();

    if x_str.len() > y_str.len() {
        x_str
    } else {
        y_str
    }
}

fn main() {
    let string1 = String::from("hello");
    let string2 = "world";
    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}
