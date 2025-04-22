/*
什么时候使用panic?
示例、代码原型和测试都非常适合 panic
*/
use std::net::IpAddr;

//当我们比编译器知道更多的情况
fn main() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Expected a valid IP address");
}

//创建一个 Guess 类型，它只在值位于 1 和 100 之间时才继续
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}