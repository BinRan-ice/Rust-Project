//类型别名用来创建类型同义词
type Kilometers = i32;

//引入类型别名 Thunk 来减少重复
type Thunk = Box<dyn Fn() + Send + 'static>;

// 使用类型别名 Thunk 的函数
fn takes_long_type(f: Thunk) {
    f();  // 调用传入的闭包
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi from returned Thunk"))
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);  //因为 Kilometers 是 i32 的别名，它们是同一类型，可以将 i32 与 Kilometers 相加，也可以将 Kilometers 传递给获取 i32 参数的函数。

    // 示例使用 Thunk 类型别名
    let f: Thunk = Box::new(|| println!("hi from Thunk"));
    takes_long_type(f);  // 调用函数 takes_long_type
    let g = returns_long_type();
    g();  // 调用返回的闭包
    println!("-----------------------------------------");
}