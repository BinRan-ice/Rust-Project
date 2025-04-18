use std::collections::HashMap;

fn main() {
    //新建一个哈希 map 并插入一些键值对
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);
    println!("-----------------------------------------");

    //访问哈希 map 中的值
    let team_name = String::from("Blue");
    //通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，接着调用 unwrap_or 在 scores 中没有该键所对应的项时将其设置为零.
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {}", score);
    println!("-----------------------------------------");

    //遍历哈希 map 中的每一个键值对
    for (key, value) in &scores {
        println!("{key}: {value}"); //会以任意顺序打印出每一个键值对
    }
    println!("-----------------------------------------");

    //一旦键值对被插入后就为哈希 map 所拥有
    let filed_name = String::from("Favorite color");
    let filed_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(filed_name, filed_value);
    //这里 filed_name 和 filed_value 不再有效，尝试使用它们会导致编译错误

    //覆盖一个值
    scores.insert(String::from("Blue"), 25);
    println!("scores: {:?}", scores);
    println!("-----------------------------------------");

    //只在键没有对应值时插入键值对
    //哈希 map 有一个特有的 API，叫做 entry，它获取我们想要检查的键作为参数。entry 函数的返回值是一个枚举，Entry，它代表了可能存在也可能不存在的值。
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);//or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores: {:?}", scores);
    println!("-----------------------------------------");

    //根据旧值更新一个值.通过哈希 map 储存单词和计数来统计出现次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for  word in text.split_whitespace()  {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map: {:?}", map);
    println!("-----------------------------------------");
}
