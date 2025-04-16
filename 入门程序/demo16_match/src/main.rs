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
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("The value of the coin is: {}", value);
    println!("-------------------------------");

    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("The value of the coin is: {}", value);
    println!("-------------------------------");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("The value of six is: {:?}", six);
    println!("The value of none is: {:?}", none);
    println!("-------------------------------");

    //通配模式和 _ 占位符(如果你掷出 3 或 7 以外的值，你的回合将无事发生。)
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

//一个以枚举成员作为模式的 match 表达式
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {   //如果 Coin::Quarter 成员匹配了，我们想要在 match 表达式的分支中使用 state 的值
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//一个在 Option<i32> 上使用 match 表达式的函数
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
