use std::time::Duration;

/* //使用 `spawn_task` 启动两个计数任务
fn main() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {} from the first task!", i);
                trpl::sleep(Duration::from_millis(1)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {} from the second task!", i);
            trpl::sleep(Duration::from_millis(1)).await;
        }

        ////在一个 join 句柄上使用 `await` 使得任务运行直到结束
        handle.await.unwrap();
    });
} */

/* //使用 `trpl::join` 来 await 两个匿名 future
fn main() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {} from the first task!", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {} from the second task!", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
} */

/* //创建一个异步信道（async channel）并赋值其两端为 `tx` 和 `rx`
fn main() {
    trpl::run(async {
        let (tx,mut rx) = trpl::channel();
        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {}", received);
    });
} */

/* //通过异步信道发送和接收多个消息并在每个消息之间通过 `await` 休眠
fn main() {
    //一个可以工作的在 future 之间接收和发送消息的示例，其在结束后会正确地关闭信道
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        
        //将 `send` 和 `recv` 分隔到其各自的 `async` 代码块中并 await 这些代码块的 future
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("async"),
                String::from("world"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("received: {}", val);
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    })
} */

//通过多个异步代码块使用多个发送者
fn main() {
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

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;

    })
}