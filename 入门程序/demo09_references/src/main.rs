fn main() {
    /*
    针对demo08_ownership/src/main.rs中的代码，我们可以看到：
    1. 当变量离开作用域时，其值将被丢弃。
    2. 当变量的值被移动到另一个变量时，原始变量将无效。
    3. 当变量的值被克隆到另一个变量时，原始变量仍然有效。
    
    Rust 对此提供了一个不用获取所有权就可以使用值的功能，叫做 引用（references）。引用允许你使用值但不获取其所有权。
    引用规则：
    1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    2. 引用必须总是有效的。
     */
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
        println!("-----------------------------------------");
    }

    /*
    我们将创建一个引用的行为称为借用（borrowing）。正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。我们并不拥有它。
    如果我们尝试修改借用的变量呢？
    */
    {
        let s = String::from("hello");
        change(&s);
    }

    //可变引用
    {
        let mut s = String::from("hello");
        changes(&mut s);
    }

    //一个特定作用域中的特定数据有且只有一个可变引用
    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        //let r2 = &mut s; //这里会导致编译错误，因为在特定作用域中，只能有一个可变引用
        println!("{}", r1);
        println!("-----------------------------------------");
    }

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 在这里离开作用域，所以我们可以创建一个新的引用
    let r2 = &mut s;

    //不能在拥有不可变引用的同时拥有可变引用
    {
        let mut s = String::from("hello");
        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        //let r3 = &mut s; // 这里会导致编译错误，因为在特定作用域中，只能有一个可变引用
        println!("{}, {}", r1, r2);
        println!("-----------------------------------------");
    }

    //因为最后一次使用不可变引用（println!)，发生在声明可变引用之前，所以如下代码是可以编译
    {
        let mut s = String::from("hello");
        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用
        let r3 = &mut s; // 没问题
        println!("{}", r3);
        println!("-----------------------------------------");
    }

    //悬垂引用：在 Rust 中编译器确保引用永远不会变成悬垂状态
    {
        let reference_to_nothing = dangle();
    }
}

//下面是如何定义并使用一个（新的）calculate_length 函数，它以一个对象的引用作为参数而不是获取值的所有权
fn calculate_length(s: &String) -> usize {
    // s 是一个指向 String 的引用
    s.len()
} // 这里，s 离开作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生

fn change(some_string: &String) {
    //some_string.push_str(", world"); //这里会导致编译错误，因为 some_string 是不可变的，而 push_str() 会尝试修改其内容
    println!("{}", some_string);
    println!("-----------------------------------------");
}

fn changes(some_thing: &mut String) {
    some_thing.push_str(", world");
}

/*
//以下代码会报错
fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello");  // s 是一个新字符串
    &s  // 返回字符串 s 的引用c
} // 这里 s 离开作用域并被丢弃。其内存被释放。危险！
*/

// 这里的解决方法是直接返回 String
fn dangle() -> String {
    let s = String::from("hello");
    s
} // 所有权被移动出去，所以没有值被释放.
