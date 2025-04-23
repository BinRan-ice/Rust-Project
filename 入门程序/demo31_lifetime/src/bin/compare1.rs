//泛型函数比较两个数值的大小，返回较大的那个数值
fn largest<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let num1 = 10;
    let num2 = 20;
    println!("The largest number is {}", largest(num1, num2));

    let char1 = 'a';
    let char2 = 'z';
    println!("The largest character is {}", largest(char1, char2));
}
