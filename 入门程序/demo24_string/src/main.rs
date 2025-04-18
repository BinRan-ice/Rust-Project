fn main() {
    //新建一个空的 String
    let mut s = String::new();

    //使用 to_string 方法从字符串字面值创建 String
    let data = "initial contents";
    let s = data.to_string();
    //等价于
    let s = "initial contents".to_string();
    println!("s: {}", s);
    println!("-----------------------------------------");

    //使用 String::from 函数来从字符串字面值创建 String
    let s = String::from("initial contents");
    println!("s: {}", s);
    println!("-----------------------------------------");

    //使用 push_str 和 push 附加字符串
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {}", s);
    println!("-----------------------------------------");

    //将字符串 slice 的内容附加到 String 后使用它
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("-----------------------------------------");

    //使用 push 将一个字符加入 String 值中
    let mut s = String::from("lo");
    s.push('l');
    println!("s: {}", s);
    println!("-----------------------------------------");

    //使用 + 运算符将两个 String 值合并到一个新的 String 值中
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    /*
    + 运算符使用了 add 函数:fn add(self, s: &str) -> String {
    add 函数的 s 参数：只能将 &str 和 String 相加，不能将两个 String 值相加
    &s2 的类型是 &String, 而不是 add 第二个参数所指定的 &str。那么为什么示例 8-18 还能编译呢？
    之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str.
     */
    let s3 = s1 + &s2;  //注意 s1 被移动了，不能继续使用
    println!("s3: {}", s3);
    println!("-----------------------------------------");

    //使用 format! 宏级联多个字符串
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);
    println!("-----------------------------------------");

    //尝试对字符串使用索引语法
    let s1 = String::from("hello");
    //let h = s1[0]; //这里会导致编译错误，因为 Rust 的字符串不支持索引

    //字符串 slice
    let hello = "Здравствуйте";
    let s = &hello[0..4];   //这里会返回 "Зд"
    println!("s: {}", s);
    println!("-----------------------------------------");

    //遍历字符串的字符
    for c in "Зд".chars() {
        println!("{}", c);
    }
    println!("-----------------------------------------");

    // bytes 方法返回每一个原始字节
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
