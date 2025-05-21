use std::thread;
use std::time::Duration;

fn main() {
    //使用 spawn 创建了一个新的线程，并立即执行传入的闭包
    let handle = thread::spawn(|| {  //thread::spawn 的返回值类型是 JoinHandle。JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束。
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //handle.join().unwrap();   //移动到该位置，主线程会等待直到新建线程执行完毕之后才开始执行 for 循环，所以输出将不会交替出现

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //使用 join 等待所有线程结束,输出将会交替出现，如果不执行下面的代码，主线程结束，子线程也会结束
    handle.join().unwrap(); //从 thread::spawn 保存一个 JoinHandle 以确保该线程能够运行至结束

    let v = vec![1, 2, 3];
    let handles = thread::spawn(move || {  //move 闭包获取了 v 的所有权，所以当新建线程运行时，它获取了 v 的所有权
        println!("Here's a vector: {:?}", v);
    });
    handles.join().unwrap();
}
