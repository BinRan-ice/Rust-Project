//定义一个枚举，以便能在 vector 中存放不同类型的数据
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //新建一个空的 vector 来储存 i32 类型的值
    let v: Vec<i32> = Vec::new();

    //vec! 宏:会根据我们提供的值来创建一个新的 vector
    let v = vec![1, 2, 3];

    //更新 vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v: {:?}", v);

    //读取 vector 中的元素
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];    //使用 & 和 [] 会得到一个索引位置元素的引用
    println!("The third element is {}", third);
    println!("-----------------------------------------");
    
    let third: Option<&i32> = v.get(2); //当使用索引作为参数调用 get 方法时，会得到一个可以用于 match 的 Option<&T>
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    println!("-----------------------------------------");

    let mut v = vec![100, 32, 57];
    let first = &v[0];
    //v.push(6); //这里会导致编译错误，因为在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存.
    println!("The first element is: {}", first);
    println!("-----------------------------------------");

    //通过 for 循环遍历 vector 的元素并打印
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    println!("-----------------------------------------");

    //遍历 vector 中元素的可变引用
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;   //为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
    }
    println!("v: {:?}", v);
    println!("-----------------------------------------");

    //使用枚举来存放多种类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {:?}", row);
    println!("-----------------------------------------");

    //移除 vector 中的元素
    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0];
    v.remove(0);    //移除第一个元素
    println!("The first element is: {}", first);
    println!("v: {:?}", v);
}