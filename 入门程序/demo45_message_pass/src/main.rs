use std::sync::mpsc;  //mpsc 是多个生产者，单个消费者（multiple producer, single consumer）通道的缩写
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();  //mpsc::channel 函数创建一个新的通道，返回值是元组，第一个元素是发送端，第二个元素是接收端

    //通过克隆发送者来创建多个生产者
    let tx1 = tx.clone();   //克隆发送者
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //发送多个值并观察接收者的等待
    thread::spawn(move || {
        /* let val = String::from("hi");
        tx.send(val).unwrap();  //send 方法返回一个 Result<T, E> 类型，当接收端已经被丢弃时，会返回错误 */
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /* //另一个方法try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用
    let received = rx.recv().unwrap();  //recv 方法会阻塞主线程直到从通道中接收一个值
    println!("Got: {}", received); */

    for received in rx {  //for 循环不断尝试接收消息，直到发送端关闭通道，这时 for 循环也会退出
        println!("Got: {}", received);
    }
}
