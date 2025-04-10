fn main() {
    // 常量的声明
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    //隐藏:定义一个与之前变量同名的新变量，新变量会隐藏之前的变量
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    //mut和隐藏的区别,隐藏的变量不会报错
    let spaces = "   ";
    let spaces = spaces.len();

    //以下代码会报错，不能改变变量的类型
    let mut space = "   ";
    space = space.len();
}
