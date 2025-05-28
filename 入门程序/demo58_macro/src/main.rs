use demo58_macro::HelloMacro;

macro_rules! my_vec {
    ( $( $x:expr ),* ) => {  // 匹配不定数量的表达式
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);  // 每个表达式都被 push 进向量
            )*
            temp_vec
        }
    };
}

#[derive(HelloMacro)]
struct Pancakes;

/* //手动实现 HelloMacro
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
} */

fn main() {
    //声明宏
    let v: Vec<i32> = vec![1, 2, 3];

    let v = my_vec!(1, 2, 3, 4, 5);  // 传入多个元素
    println!("{:?}", v); // 输出: [1, 2, 3, 4, 5]
    println!("-----------------------------------------");

    Pancakes::hello_macro();    // 输出: Hello, Macro! My name is Pancakes!
    println!("-----------------------------------------");
}
