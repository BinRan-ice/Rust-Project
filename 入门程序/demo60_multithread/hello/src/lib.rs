use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    //threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>, //修改 ThreadPool 存放 Worker 实例而不是直接存放线程
    //sender: mpsc::Sender<Job>, //定义一个 mpsc::Sender<Job> 来发送 Job 实例
    sender: Option<mpsc::Sender<Job>>, //将 sender 字段的类型更改为 Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>; //定义 Job 类型别名来存放闭包

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); //断言线程池的大小大于 0

        let (sender, receiver) = mpsc::channel(); //创建一个新的通道并将其存储在 sender 中

        let receiver = Arc::new(Mutex::new(receiver)); //将 receiver 包装在 Arc 和 Mutex 中，以便能够安全地在多个 Worker 之间共享所有权

        //let mut threads = Vec::with_capacity(size);  //创建一个新的 Vec 来存放线程
        let mut workers = Vec::with_capacity(size); //创建一个新的 Vec 来存放 Worker 实例

        for id in 0..size {
            //创建线程并将它们存放到 vector 中
            //threads.push(thread::spawn(|| {}));  //spawn 函数接受一个闭包并返回一个 JoinHandle
            //workers.push(Worker::new(id, receiver));  //创建 Worker 实例并将其存放到 vector 中
            workers.push(Worker::new(id, Arc::clone(&receiver))); //创建 Worker 实例并将其存放到 vector 中
        }

        ThreadPool { 
            workers, 
            sender: Some(sender), //将 sender 字段的类型更改为 Option<mpsc::Sender<Job>>
        }
    }

    pub fn execute<F>(&self, f: F)
    //为存放每一个闭包的 Box 创建一个 Job 类型别名，接着在信道中发出任务
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f); //将闭包放入 Box 中并发送给 Worker
        //self.sender.send(job).unwrap(); //send 方法返回一个 Result<T, E>，所以我们调用 unwrap 来处理任何错误
        self.sender.as_ref().unwrap().send(job).unwrap(); //send 方法返回一个 Result<T, E>，所以我们调用 unwrap 来处理任何错误
    }
}

//为 ThreadPool 实现 Drop Trait，以确保线程在 ThreadPool 被丢弃时正确关闭
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); //在 join worker 线程之前显式丢弃 sender

        //当线程池离开作用域时 join 每个线程
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            //worker.thread.join().unwrap();  //调用 join 方法等待线程结束
            if let Some(thread) = worker.thread.take() {    //将 thread 字段的类型更改为 Option<thread::JoinHandle<()>>
                thread.join().unwrap(); //调用 join 方法等待线程结束
            }
        }
    }
}

//Worker 结构体负责从 ThreadPool 中将代码传递给线程
struct Worker {
    id: usize,
    //thread: thread::JoinHandle<()>,
    thread: Option<thread::JoinHandle<()>>, //将 thread 字段的类型更改为 Option<thread::JoinHandle<()>>
}

//在 worker 线程中接收并执行任务
impl Worker {
    //Worker 结构体的 new 方法接受一个 id 和一个 mpsc::Receiver<Job>，并返回一个 Worker 实例
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        //Worker 结构体的 new 方法接受一个 id 和一个 mpsc::Receiver<Job>，并返回一个 Worker 实例
        let thread = thread::spawn(move || loop {
            //在新线程中调用 recv 方法来接收 Job，然后调用闭包
            //let job = receiver.lock().unwrap().recv().unwrap(); //调用 recv 方法来接收 Job，然后调用闭包
            //println!("Worker {} got a job; executing.", id);
            //job(); //调用闭包

            //在新线程中调用 recv 方法来接收 Job，然后调用闭包
            let message = receiver.lock().unwrap().recv(); //调用 recv 方法来接收 Job，然后调用闭包
            
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job(); //调用闭包
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread), //将 thread 字段的类型更改为 Option<thread::JoinHandle<()>>
        }
    }
}
