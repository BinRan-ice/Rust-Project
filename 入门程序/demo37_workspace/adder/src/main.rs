use add_one;

//在 adder crate 中使用 add_one 库 crate
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}