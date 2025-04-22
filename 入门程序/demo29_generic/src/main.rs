//结构体中使用泛型
struct Point1<T> {
    //定义了一个 Point<T> 结构体，它有一个泛型类型 T。
    x: T,
    y: T,
}

//使用两个泛型的 Point，这样 x 和 y 可能是不同类型
struct Point2<T, U> {
    x: T,
    y: U,
}

//枚举定义中的泛型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

//方法定义中的泛型:在 Point<T> 结构体上实现方法 x，它返回 T 类型的字段 x 的引用
impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//构建一个只用于拥有泛型参数 T 的结构体的具体类型的 impl 块
impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt() //计算点实例与坐标 (0.0, 0.0) 之间的距离
    }
}

//方法使用了与结构体定义中不同类型的泛型
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    println!("-----------------------------------------");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    println!("-----------------------------------------");

    let integer = Point1 { x: 5, y: 10 }; //在 Point 结构体定义中，我们告诉 Rust 该结构体的 x 和 y 字段都有一个泛型类型 T。
    let float = Point1 { x: 1.0, y: 4.0 };
    println!("integer.x = {}, integer.y = {}", integer.x, integer.y);
    println!("float.x = {}, float.y = {}", float.x, float.y);
    println!("-----------------------------------------");

    //以下代码会报错
    // let wont_work = Point { x: 5, y: 4.0 }; //这里 x 是一个 i32 类型，而 y 是一个 f64 类型。这会导致一个错误，因为 Point 结构体定义要求 x 和 y 是相同类型。

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!(
        "both_integer.x = {}, both_integer.y = {}",
        both_integer.x, both_integer.y
    );
    println!(
        "both_float.x = {}, both_float.y = {}",
        both_float.x, both_float.y
    );
    println!(
        "integer_and_float.x = {}, integer_and_float.y = {}",
        integer_and_float.x, integer_and_float.y
    );
    println!("-----------------------------------------");

    let p = Point1 { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("-----------------------------------------");

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("-----------------------------------------");
}

//在函数定义中使用泛型（以下代码会报错：largest 的函数体不能适用于 T 的所有可能的类型）
fn largest<T>(list: &[T]) -> &T {
    //函数 largest 有泛型类型 T。它有个参数 list，其类型是元素为 T 的 slice。largest 函数会返回一个与 T 相同类型的引用。
    let mut largest = &list[0]; //largest 变量用来存储 list 中最大的元素的引用。我们初始化 largest 为 list 中的第一个元素的引用。
    for item in list {
        //遍历 list 中的每个元素
        if item > largest {
            //如果 item 大于 largest
            largest = item; //将 largest 设置为 item
        }
    }
    largest //返回 largest
}