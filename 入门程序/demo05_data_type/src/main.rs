fn main() {
    // Rust是静态类型语言，也就是说在编译时就必须知道所有变量的类型,比如demo02的 “比较猜测的数字和秘密数字” 使用 parse 将 String 转换为数字时，必须增加类型注解
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {guess}");

    // 整数类型
    let a: u8 = 14; // 8位无符号整数
    let b: i8 = -14; // 8位有符号整数
    let c: u16 = 14; // 16位无符号整数
    let d: i16 = -14; // 16位有符号整数
    let e: u32 = 14; // 32位无符号整数
    let f: i32 = -14; // 32位有符号整数
    let g: u8 = 14; // 8位无符号整数,十进制
    let h: u8 = 0xff; // 8位无符号整数,十六进制
    let i: u8 = 0o77; // 8位无符号整数,八进制
    let j: u8 = 0b1111_1111; // 8位无符号整数,二进制
    println!("a: {a}, b: {b}, c: {c}, d: {d}, e: {e}, f: {f}, g: {g}, h: {h}, i: {i}, j: {j}");
    println!("-----------------------------------------");

    // 数值运算
    let sum = 5 + 10; // 加法
    let difference = 95.5 - 4.3; // 减法
    let product = 4 * 30; // 乘法
    let quotient = 56.7 / 32.2; // 除法
    let remainder = 43 % 5; // 取余
    println!("sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, remainder: {remainder}");
    println!("-----------------------------------------");

    // 浮点数类型
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {x}, y: {y}");
    println!("-----------------------------------------");

    // 布尔类型
    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");
    println!("-----------------------------------------");

    // 字符类型
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");
    println!("-----------------------------------------");

    // 复合类型:可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

    // 元组类型:元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，它们的长度不能增长或缩小。
    let tup: (i32, f64, u8) = (500, 6.4, 1); //赋值方式一
    let tup = (500, 6.4, 1); //赋值方式二
    println!("tup: {}, {}, {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup; //解构
    println!("The value of y is: {y}");
    let five_hundred = tup.0; //访问
    let six_point_four = tup.1; //访问
    let one = tup.2; //访问
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");
    println!("-----------------------------------------");

    // 数组类型:数组中的每个元素的类型必须相同。数组在 Rust 中是固定长度的：一旦声明，它们的长度不能增长或缩小。
    let a = [1, 2, 3, 4, 5]; //赋值方式一
    //这里，i32 是每个元素的类型。分号之后，数字 5 表明该数组包含五个元素。数组的类型是 [i32; 5]。
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //赋值方式二
    //变量名为 a 的数组将包含 5 个元素，这些元素的值最初都将被设置为 3。
    let a = [3; 5]; //赋值方式三
    println!("{}", a[0]);
}
