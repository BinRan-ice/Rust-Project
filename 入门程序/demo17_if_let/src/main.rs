#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

//硬币枚举类型
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),   //Quarter 成员也存放了一个 UsState 值的 Coin 枚举
}

fn main() {
    //match 只关心当值为 Some 时执行代码，而忽略 None
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be: {}", max),
        _ => (),
    }
    println!("-------------------------------");

    //可以使用 if let 这种更短的方式编写
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be: {}", max);
    }
    println!("-------------------------------");

    //使用这样一个match表达式:计数所有不是 25 美分的硬币的同时也报告 25 美分硬币所属的州
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("-------------------------------");

    //使用 if let 重写这段代码
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
