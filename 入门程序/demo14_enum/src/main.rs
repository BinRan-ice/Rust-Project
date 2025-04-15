#[derive(Debug)] //为了能够使用println!打印出枚举的实例，我们需要在结构体定义之前加上 #[derive(Debug)] 注解。
enum IpAddrKind {
    V4, //成员
    V6,
}

#[derive(Debug)] //为了能够使用println!打印出结构体的实例，我们需要在结构体定义之前加上 #[derive(Debug)] 注解。
struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

//简洁方式实现上述功能
#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

//每个成员可以处理不同类型和数量的数据
#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

//内嵌了多种多样的类型
#[derive(Debug)]
enum Message {
    Quit, //不带任何数据
    Move { x: i32, y: i32 }, //匿名结构体
    Write(String), //单个String
    ChangeColor(i32, i32, i32), //三个i32
}

//枚举上定义方法
impl Message {
    fn call(&self) {
        //方法体
        println!("The Message is: {:?}", self);
    }
}

fn main() {
    let home = IpAddr1 {
        kind: IpAddrKind::V4,   //注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开。
        address: String::from("127.0.0.1"),
    };
    println!("The IpAddr is: {:?}", home);

    let loopback = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("The IpAddr is: {:?}", loopback);
    println!("-----------------------------------------");

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("The IpAddrs is: {:?}", home);
    println!("The IpAddrs is: {:?}", loopback);
    println!("-----------------------------------------");

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
    println!("The IpAddrs is: {:?}", home);
    println!("The IpAddrs is: {:?}", loopback);
    println!("-----------------------------------------");

    let m = Message::Write(String::from("hello"));
    m.call();
}