#[derive(Debug)] //加上外部属性，打印出调试信息
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );
    println!("-----------------------------------------");

    let rect1 = (40, 60);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
    println!("-----------------------------------------");

    let rect2 = Rectangle {
        width: 50,
        height: 70,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );
    println!("-----------------------------------------");

    //在 {} 中加入 :? 指示符告诉 println! 我们想要使用叫做 Debug 的输出格式
    println!("rect2 is {:?}", rect2);
    println!("-----------------------------------------");

    //当我们有一个更大的结构体时，能有更易读一点的输出就好了，为此可以使用 {:#?} 替换 println! 字符串中的 {:?}
    println!("rect2 is {:#?}", rect2);
    println!("-----------------------------------------");

    //另一种使用 Debug 格式打印数值的方法是使用 dbg! 宏。dbg! 宏接收一个表达式的所有权
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3);
}

fn area1(width1: u32, height1: u32) -> u32 {
    width1 * height1
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
