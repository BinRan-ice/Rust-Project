fn main() {
    /*
    编程小习题：编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。
    如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
     */
    let mut s = String::from("hello world");
    let word = first_word1(&s);  // word 的值为 5
    s.clear(); // 这清空了字符串，使其等于 ""
    // word 现在是无效的引用！
    //但是，编译器并不会报错，因为编译器无法检测到这个问题。这种错误很容易出现，而且很难调试。

    //slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它没有所有权。
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("The first word is: {}", hello);
    println!("The second word is: {}", world);
    println!("-----------------------------------------");

    //如下两个语句是相同的
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    println!("The first word is: {}", slice);
    println!("-----------------------------------------");

    //如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字
    let s = String::from("world");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    println!("The first word is: {}", slice);
    println!("-----------------------------------------");

    //也可以同时舍弃这两个值来获取整个字符串的 slice。所以如下亦是相同的
    let s = String::from("hello,world");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];
    println!("The first word is: {}", slice);
    println!("-----------------------------------------");

    let s = String::from("hello world");
    let words = first_word2(&s);  // word 的值为 5
    println!("The first word is: {}", words);
    println!("-----------------------------------------");

    //字符串字面值就是 slice
    let s = "hello world";//这里s的类型是 &str：它是一个指向二进制程序特定位置的slice。这也就是为什么字符串字面值是不可变的；&str是一个不可变引用。

    //字符串 String 作为参数
    let my_string = String::from("hello world");
    // first_word3适用于String(的slice),部分或全部
    let word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    // first_word3也适用与string的slice，这等价与整个string的slice
    let word = first_word3(&my_string);

    //字符串 slice 作为参数
    let my_string_literal = "hello world";
    // first_word3适用于字符串字面值，部分或全部
    let word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);
    //因为字符串字面值已经是字符串slice了，这也是适用的，无需slice语法
    let word = first_word3(my_string_literal);
}

fn first_word1(s:&String)->usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}

//“字符串 slice” 的类型声明写作 &str
fn first_word2(s:&String)->&str{
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

//如果有一个字符串 slice，可以直接传递它。如果有一个 String，则可以传递整个 String 的 slice 或对 String 的引用。
fn first_word3(s:&str)->&str{
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
