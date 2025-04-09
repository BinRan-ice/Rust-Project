fn main() {
    //变量默认是不可变的，加上mut关键字可以使变量可变
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
