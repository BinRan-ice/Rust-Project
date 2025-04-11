/*
所有权规则
1. Rust 中的每一个值都有一个被称为其所有者的变量。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。
*/
fn main() {
    {   // s 在这里无效, 它还没有被声明
        let s = "hello"; // s 是有效的
    
        //使用 s
    }   // 这里 s 不再有效

    // String 类型
    {
        //这两个冒号 :: 是运算符，允许将特定的from函数置于String类型的命名空间下，而不需要使用类似string_from这样的名字
        let str = String::from("hello");    //基于字符串字面值来创建 String
        let mut s = String::from("hello");  //String 类型可变
        s.push_str(", world!"); // push_str() 在字符串后追加字面值

        println!("{}", s); // 将打印 `hello, world!`
        println!("-----------------------------------------");
    }

    {   // s 在这里无效, 它还没有被声明
        let s = String::from("hello");// s 是有效的
        //使用 s
    }   // 这里 s 不再有效

    //变量与数据交互的方式：移动
    {
        let s1 = String::from("hello");
        let s2 = s1; // s1 的值被移动到 s2 中
        //println!("{}, world!", s1); // 这行代码会导致编译错误。为了确保内存安全，在let s2 = s1;之后，Rust认为s1不再有效，因此Rust不需要在s1离开作用域后清理任何东西。
    }

    //这段代码似乎与我们刚刚学到的内容相矛盾：没有调用 clone，不过x依然有效且没有被移动到y中?
    {
        let x = 5;
        let y = x; // x 的值被复制到 y。i32 类型是已知大小的类型，所以它的值被完全存储在栈上，所以拷贝速度非常快。
        /*
        这意味着没有理由在创建变量 y 后使 x 无效。
        换句话说，这里没有深浅拷贝的区别，所以这里调用 clone 并不会与通常的浅拷贝有什么不同
         */
    }

    //变量与数据交互的方式：克隆
    {
        //深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做clone的通用函数
        let s1 = String::from("hello");
        let s2 = s1.clone(); // 克隆
        println!("s1 = {}, s2 = {}", s1, s2);
        println!("-----------------------------------------");
    }

    //所有权与函数
    {
        let s = String::from("hello");  // s 进入作用域

        takes_ownership(s);             // s 的值移动到函数里 ...
                                        // ... 所以到这里不再有效

        let x = 5;                      // x 进入作用域

        makes_copy(x);                  // x 应该移动函数里，
                                        // 但 i32 是 Copy 的，所以在后面可继续使用 x
    }   // 这里，x 先离开作用域，然后是 s。但因为 s 的值已被移走，所以不会有特殊操作

    //返回值与作用域
    {
        let s1 = gives_ownership();         // gives_ownership 将返回值移给 s1

        let s2 = String::from("hello");     // s2 进入作用域

        let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                            // takes_and_gives_back 中，它也将返回值移给 s3
    }   // 这里，s3 先离开作用域，然后是 s2。但因为 s2 的值已被移走，所以不会有特殊操作。s1 离开作用域并被丢弃

    //转移返回值的所有权
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
        println!("-----------------------------------------");
    }
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
    println!("-----------------------------------------");
} // 这里，some_string 移出作用域并调用 `drop` 方法。内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
    println!("-----------------------------------------");
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域
    some_string                              // some_string 被返回并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // a_string 被返回并移出给调用的函数
} // 这里，a_string 移出作用域并调用 `drop` 方法。内存被释放

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}