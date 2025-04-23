/*
衬衫公司赠送场景
*/
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy,PartialEq)]
enum ShirtColor {   // 衬衫颜色
    Red,
    Blue,
}

struct Inventory {   //公司库存
    shirts: Vec<ShirtColor>,
}

/*
unwrap_or_else 的作用

对于 Option：
如果是 Some(value)，unwrap_or_else 返回 value。
如果是 None，它会调用传入的闭包，并返回闭包的结果。

对于 Result：
如果是 Ok(value)，unwrap_or_else 返回 value。
如果是 Err(err)，它会调用传入的闭包，将 err 作为参数传入闭包，并返回闭包的结果。
*/
impl Inventory {    //库存实现
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //unwrap_or_else 接受所有三种类型的闭包，十分灵活
        user_preference.unwrap_or_else(|| self.most_stocked())  //闭包表达式 || self.most_stocked() 作为 unwrap_or_else 的参数（如果闭包有参数，它们会出现在两道竖杠之间）
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {  ////定义一个 Rectangle 结构体
    width: u32,
    height: u32,
}   

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_prefer1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_prefer1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_prefer1, giveaway1
    );

    let user_prefer2 = None;
    let giveaway2 = store.giveaway(user_prefer2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_prefer2, giveaway2
    );
    println!("-----------------------------------------");

    //为闭包的参数和返回值增加可选的类型注解
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    println!("The result is {}", expensive_closure(42));
    println!("-----------------------------------------");

    let example_closure = |x:u32| -> u32 {x};
    let n = example_closure(5);
    println!("The result is {}", n);
    println!("-----------------------------------------");

    //定义并调用一个捕获不可变引用的闭包
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    println!("-----------------------------------------");

    //定义并调用一个捕获可变引用的闭包
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(8);
    borrows_mutably();
    println!("After calling closure: {list:?}");
    println!("-----------------------------------------");

    //使用 move 来强制闭包为线程获取 list 的所有权
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    //生成了一个新的线程，并给这个线程传递一个闭包作为参数来运行，闭包体打印出列表。
    thread::spawn(move || println!("From closure: {list:?}"))   //move 关键字将 list 的所有权移动到闭包中，使得 list 的生命周期与闭包的生命周期一致。
        .join()
        .unwrap();
    println!("-----------------------------------------");

    //使用 sort_by_key 对长方形按宽度排序
    let mut list = [
        Rectangle { width: 10, height: 5 },
        Rectangle { width: 5, height: 10 },
        Rectangle { width: 8, height: 6 },
    ];
    list.sort_by_key(|r| r.width);  //sort_by_key 被定义为接收一个 FnMut 闭包的原因是它会多次调用这个闭包，而不是只调用一次。
    println!("{list:#?}");
    println!("-----------------------------------------");

    /* //尝试在 sort_by_key 上使用一个 FnOnce 闭包
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(value);    //这个闭包只能被调用一次；尝试第二次调用它将无法工作，因为这时 value 已经不在闭包的环境中，无法被再次插入 sort_operations 中！因而，这个闭包只实现了 FnOnce trait。
        r.width
    });
    println!("{list:#?}"); */

    //要修复上述问题，我们需要修改闭包体，使其不会将值移出环境。在环境中维护一个计数器，并在闭包体中递增其值，是计算闭包被调用次数的一个更简单直接的方法。
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}

/*
定义闭包：let expensive_closure = |num: u32| -> u32 { ... };

闭包的语法看起来很像函数的语法：
fn  add_one_v1   (x: u32) -> u32 { x + 1 }  //函数定义
let add_one_v2 = |x: u32| -> u32 { x + 1 }; //完整标注的闭包定义
let add_one_v3 = |x|             { x + 1 }; //省略了类型标注的闭包定义
let add_one_v4 = |x|               x + 1  ; //省略了大括号的闭包定义
*/

/*
将被捕获的值移出闭包和 Fn trait

1.FnOnce 适用于只能被调用一次的闭包。
所有闭包至少都实现了这个 trait，因为所有闭包都能被调用。
一个会将捕获的值从闭包体中移出的闭包只会实现 FnOnce trait，而不会实现其他 Fn 相关的 trait，因为它只能被调用一次。

2.FnMut 适用于不会将捕获的值移出闭包体，但可能会修改捕获值的闭包。这类闭包可以被调用多次。

3.Fn 适用于既不将捕获的值移出闭包体，也不修改捕获值的闭包，同时也包括不从环境中捕获任何值的闭包。
这类闭包可以被多次调用而不会改变其环境，这在会多次并发调用闭包的场景中十分重要。
*/