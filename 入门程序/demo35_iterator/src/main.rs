fn main() {
    //在一个 for 循环中使用迭代器
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();  //调用 iter 方法创建一个迭代器
    for val in v1_iter {
        println!("Got: {val}");
    }

    //调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();  //map 方法是一个迭代器适配器，它获取一个闭包并生成一个新的迭代器，该迭代器使用闭包中的逻辑来转换元素
    let result = assert_eq!(v2, vec![2, 3, 4]);
    println!("{result:#?}");
}
