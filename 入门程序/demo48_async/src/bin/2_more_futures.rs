use std::thread;
use std::time::Duration;
use std::future::Future;
use std::pin::Pin;

/* fn main() {
    //一个可以工作的在 future 之间接收和发送消息的示例，其在结束后会正确地关闭信道
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        //通过使用一个显式类型声明来修复余下的类型不匹配错误
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        trpl::join_all(futures).await; //trpl::join_all 函数接受任何实现了 Iterator trait 的类型
    })
} */

/* /*
三个不同类型的 futures
我们可以使用 trpl::join! 来 await 它们，因为它允许你传递多个 future 类型并产生一个这些类型的元组。
我们不能使用 trpl::join_all，因为它要求传递的 future 都拥有相同的类型。请记住，那个错误正是我们开启 Pin 探索之旅的原因！
*/
fn main() {
    trpl::run(async {
        let a = async { 1u32 };
        let b = async { "hello" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
    });
} */

/* //使用 `race` 来获取哪个 future 最先结束的结果
fn main() {
    trpl::run(async {
        let slow = async {
            println!("slow future started");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("slow future finished");
        };

        let fast = async {
            println!("fast future started");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("fast future finished");
        };

        let winner = trpl::race(slow, fast).await;
        println!("winner: {:?}", winner);
    })
} */

/* //使用 `thread::sleep` 来模拟缓慢的操作
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}ms");
}

//两个 future 的工作会相互交替运行
fn main() {
    trpl::run(async {
        let one_ms = Duration::from_millis(1);

        //使用 `sleep` 让操作切换以继续进行
        let a = async {
            println!("'a' strated.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' strated.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 35);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    })
} */

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}ms");
}

fn main() {
    trpl::run(async {
        let one_ms = Duration::from_millis(1);

        //使用 `yield_now` 让操作切换以继续进行
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 35);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    })
}