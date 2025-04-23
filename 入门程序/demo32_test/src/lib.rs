//由 cargo new 自动生成的测试模块和函数
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //在 Rectangle 上实现 can_hold 方法，它获取另一个 Rectangle 实例作为参数，返回一个布尔值。
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//编写一个对其参数加二并返回结果的函数
pub fn add_two(a: usize) -> usize {
    a + 2
}

//根据人名进行问候的函数
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

//私有函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

#[cfg(test)]    //只在执行 cargo test 时才编译和运行测试代码，而在运行 cargo build 时不这么做
mod tests {
    use std::alloc::LayoutErr;

    use super::*;

    #[test] //这个属性表明这是一个测试函数
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);  //示例函数体通过使用 assert_eq! 宏来断言 2 加 2 等于 4
    }

   /*  //增加第二个因调用了 panic! 而失败的测试
    #[test]
    fn another() {
        panic!("Make this test fail");
    } */

    //一个 can_hold 的测试，检查一个较大的矩形确实能放得下一个较小的矩形
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn id_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);  //assert_ne! 宏断言两个值不相等
    }

    //自定义失败信息
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    //展示了一个检查 Guess::new 是否按照我们的期望出错的测试,这个测试会通过，因为 should_panic 属性中 expected 参数提供的值是 Guess::new 函数 panic 信息的子串。
    #[test]
    #[should_panic(expected = "less than or equal to 100")] //这个属性表明测试函数应该会导致 panic,一个会带有特定错误信息的 panic! 条件的测试
    fn greater_than_100() {
        Guess::new(200);
    }

    //将Result<T,E>用于测试
    #[test]
    fn result_test() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    //测试私有函数
    #[test]
    fn internal() {
        assert_eq!(internal_adder(2, 2), 4);
    }
}

/*
并行或连续的运行测试
cargo test -- --test-threads=1 //将测试线程设置为 1，告诉程序不要使用任何并行机制

显示函数输出
cargo test -- --show-output //显示测试函数的输出

通过指定名字来运行部分测试
cargo test larger_can_hold_smaller //只运行名字包含 larger_can_hold_smaller 的测试

过滤运行多个测试
cargo test add //运行了所有名字中带有 add 的测试，也过滤掉了所有名字中不带有 add 的测试

需要运行 ignored 的测试
cargo test -- --ignored //运行所有被忽略的测试

不管是否忽略都要运行全部测试
cargo test -- --ignored --include-ignored //运行所有测试，不管是否被忽略

#[test]
#[ignore]   //对于想要排除的测试，我们在 #[test] 之后增加了 #[ignore] 行
fn expensive_test() {
    // 需要运行一个小时的代码
}
*/