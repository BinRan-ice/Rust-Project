//使用互斥器，实现同一时刻只允许一个线程访问数据
use std::sync::{Mutex, Arc};  //Mutex<T> 是一个智能指针，类似于 Box<T>，允许在运行时获取锁
use std::thread;

fn main() {
    let m = Mutex::new(5);  //Mutex<T> 是一个智能指针，类似于 Box<T>，允许在运行时获取锁
    {
        let mut num = m.lock().unwrap();  //调用 lock 方法来获取锁，这个调用会阻塞当前线程，直到我们拥有锁为止
        *num = 6;
    }  //当 Mutex<T> 离开作用域时，锁会自动释放
    println!("m = {:?}", m);
    println!("-----------------------------------------");

    //使用 Arc<T> 包装一个 Mutex<T> 能够实现在多线程之间共享所有权
    let counter = Arc::new(Mutex::new(0));  //原子引用计数 Arc<T>
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);  //增加 Arc<T> 的引用计数
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  //调用 lock 方法来获取锁
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}