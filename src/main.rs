// // crossbeam::channel 提供了高效的通道,用于在线程间安全地传递任务.
// use crossbeam::channel::{ self };

// use futures::lock::Mutex;

// use std::sync::Arc;

// // futures::task 提供了构建 waker 的工具.
// // ArcWake trait 用于自定义如何从 Arc 指针创建 Waker.
// use futures::task::{ self, ArcWake };
// // std::future::Future 是 Rust 异步编程的核心 trait.
// use std::future::Future;
// // std::pin::Pin 用于确保数据在内存中不会被移动,这是 Future 正常工作所必需的.
// use std::pin::Pin;
// // std::task::Context 和 Poll 是 poll 方法的参数和返回值类型.
// use std::task::{ Context, Poll };
// use std::thread;
// use std::time::{ Duration, Instant };

// // Delay 结构体,一个简单的 Future 实现,用于模拟定时器.
// // 它包含一个 Instant 类型的字段,表示任务何时完成.
// struct Delay {
//     when: Instant,
// }

// // 为 Delay 实现 Future trait.
// impl Future for Delay {
//     // 任务完成后返回的类型是 &'static str.
//     type Output = &'static str;

//     // poll 方法是 Future trait 的核心,由异步运行时调用来检查任务状态.
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
//         // 如果当前时间已超过或等于预设的时间...
//         if Instant::now() >= self.when {
//             // ...任务完成,打印 "hello world",并返回 Poll::Ready 状态和结果.
//             println!("hello world");
//             Poll::Ready("done")
//         } else {
//             // ...否则,任务未完成,需要挂起.
//             // 1. 克隆当前的 waker.waker 知道如何唤醒这个任务.
//             let waker = cx.waker().clone();
//             // 2. 获取预设的时间.
//             let when = self.when;

//             // 3. 生成一个新线程来等待时间.
//             // 这样做是为了不阻塞主异步运行时,让主线程可以处理其他任务.
//             thread::spawn(move || {
//                 // 在新线程中,获取当前时间.
//                 let now = Instant::now();
//                 // 如果当前时间还未到预设时间...
//                 if now < when {
//                     // ...阻塞式地休眠,直到时间到达.
//                     thread::sleep(when - now);
//                 }
//                 // 休眠结束后,调用 waker 的 wake 方法.
//                 // 这会通知异步运行时:”我的任务已准备好,请重新调度我！"
//                 waker.wake();
//             });

//             // 4. 返回 Poll::Pending,告诉运行时这个任务需要等待,尚未完成.
//             Poll::Pending
//         }
//     }
// }

// // MiniTokio 结构体,这是一个简化的异步运行时(executor).
// struct MiniTokio {
//     // 任务队列的接收端.
//     // run 方法会从这里接收任务来执行.
//     scheduled: channel::Receiver<Arc<Task>>,
//     // 任务队列的发送端.
//     // 用于将新任务或被唤醒的任务发送到队列.
//     sender: channel::Sender<Arc<Task>>,
// }

// // Task 结构体,代表一个独立的异步任务.
// struct Task {
//     // future 字段封装了具体的异步逻辑(即 async 块).
//     // Mutex 用于确保多线程访问时的线程安全.
//     // Pin<Box<...>> 是一个 trait 对象,允许我们存储任何实现了 Future 的类型.
//     future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
//     // executor 字段是调度器的一个发送端.
//     // 这使得任务可以在内部访问调度器,从而实现自我重新调度.
//     executor: channel::Sender<Arc<Task>>,
// }

// // 为 Task 实现方法.
// impl Task {
//     // poll 方法用于驱动任务的 Future.
//     fn poll(self: Arc<Self>) {
//         // 1. 根据当前 Task 创建一个 waker.
//         // waker 知道如何通过 ArcWake trait 回到 Task 本身.
//         let waker = task::waker(self.clone());
//         // 2. 创建一个上下文对象,其中包含 waker.
//         let mut cx = Context::from_waker(&waker);

//         // 3. 尝试获取 Future 的锁,并开始轮询.
//         // try_lock() 尝试非阻塞地获取锁.
//         let mut future = self.future.try_lock().unwrap();
//         // 4. 调用 Future 的 poll 方法.
//         // `as_mut()` 将 Pin<Box<...>> 转换为可变引用.
//         let _ = future.as_mut().poll(&mut cx);
//     }

//     // schedule 方法将任务重新发送回调度队列.
//     fn schedule(self: &Arc<Self>) {
//         // 使用 executor 发送端,将自身(通过 clone 增加引用计数)发送回队列.
//         self.executor.send(self.clone());
//     }

//     // spawn 方法用于创建一个新的 Task,并将其发送到调度队列.
//     fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
//         where F: Future<Output = ()> + Send + 'static
//     {
//         // 为 Task 实现 ArcWake trait.
//         // 1. 创建一个新的 Task 实例,封装传入的 future 和 sender.
//         let task = Arc::new(Task {
//             // 使用 Mutex 和 Pin<Box> 封装 future.
//             future: Mutex::new(Box::pin(future)),
//             // 克隆 sender,让 Task 持有调度器的发送端.
//             executor: sender.clone(),
//         });

//         // 2. 将创建的 Task 发送到调度队列.
//         // 这里的 `_ =` 是为了忽略 send 方法的返回值(通常是 Result).
//         let _ = sender.send(task);
//     }
// }

// // 当 waker 被调用 也就是let waker = task::waker(self.clone()) ,这个 trait 的方法会被执行.
// // 用于对异步运行 实现Waker机制 同时实现通过Arc的管理
// impl ArcWake for Task {
//     // wake_by_ref 方法在 waker 被唤醒时被调用.
//     fn wake_by_ref(arc_self: &Arc<Self>) {
//         // 它所做的就是调用 Task 自身的 schedule 方法,将自己重新加入调度队列.
//         arc_self.schedule();
//     }
// }

// // 为 MiniTokio 实现方法.
// impl MiniTokio {
//     // 构造函数,用于创建 MiniTokio 实例.
//     fn new() -> MiniTokio {
//         // 创建一个无界通道(unbounded channel).
//         let (sender, scheduled) = channel::unbounded();
//         MiniTokio { scheduled, sender }
//     }

//     fn spawn<F>(&mut self, future: F) where F: Future<Output = ()> + Send + 'static {
//         // 内部调用 Task 的 spawn 方法.
//         Task::spawn(future, &self.sender);
//     }

//     // run 方法启动异步运行时的事件循环.
//     fn run(&mut self) {
//         // 从Minitokio的
//         while let Ok(task) = self.scheduled.recv() {
//             // 一旦接收到任务,就调用其 poll 方法来驱动它.
//             task.poll();
//         }
//     }
// }

// // main 函数,程序的入口点.
// fn main() {
//     // 1. 创建 MiniTokio 异步运行时实例.
//     let mut mini_tokio = MiniTokio::new();

//     // 2. 使用 mini_tokio 的 spawn 方法启动一个异步任务.
//     // async 块本身就是一个 Future.
//     mini_tokio.spawn(async {
//         // 计算一个未来 10 毫秒的时间点.
//         let when = Instant::now() + Duration::from_millis(10);
//         // 创建一个 Delay Future 实例.
//         let future = Delay { when };

//         // 使用 .await 语法等待 future 完成.
//         // .await 会反复调用 Delay 的 poll 方法,直到它返回 Poll::Ready.
//         let out = future.await;

//         // 任务完成后,验证结果是否正确.
//         assert_eq!(out, "done");
//     });
//     mini_tokio.run();
// }

// async fn f1() -> String {
//     "1".to_string()
// }

// async fn f2() -> i32 {
//     //"2".to_string()
//     2
// }
// #[tokio::main]
// async fn main() {
//     //
//     let out =
//         tokio::select! {
//         result1 = f2() =>{
//             result1
//         }
//         result2 = f2() =>{
//             result2
//         }
//     };
// }

// use tokio::net::TcpListener;
// use tokio::sync::oneshot;
// use std::io;

// async fn process(socket: tokio::net::TcpStream) {
//     // 假设已定义
// }

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // [设置 `rx` oneshot 消息通道] ← 这里没写全
//     let (tx, rx) = oneshot::channel::<i32>(); // 补全假设

//     let listener = TcpListener::bind("localhost:3465").await?;

//     tokio::select! {
//         res = async {
//             loop {
//                 let (socket, _) = listener.accept().await?;
//                 tokio::spawn(async move { process(socket) });
//             }

//             Ok::<_, io::Error>(()) } => {
//             res?;
//         }
//         _ = rx => {
//             println!("terminating accept loop");
//         }
//     }

//     Ok(())
// }

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::{ clone, io };
use std::net::SocketAddr;

async fn race(data: &mut [u8], addr1: SocketAddr, addr2: SocketAddr) -> io::Result<()> {
    tokio::select! {
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr1).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr2).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        else => {}
    }

    Ok(())
}
// #[tokio::main]
// async fn main() {
//     //race(data, addr1, addr2)
// }
// use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() {
//     let (tx1, mut rx1) = mpsc::channel(128);
//     let (tx2, mut rx2) = mpsc::channel(128);
//     let (tx3, mut rx3) = mpsc::channel(128);

//     for _ in 0..5 {
//         let msg =
//             tokio::select! {
//             Some(msg) = rx1.recv() => msg,
//             Some(msg) = rx2.recv() => msg,
//             Some(msg) = rx3.recv() => msg,
//             else => { break }
//         };

//         println!("Got {}", msg);
//     }

//     println!("All channels have been closed.");
// }

use tokio_stream::StreamExt;
use mini_redis::client;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 发布一些数据
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    client.publish("Sharon", "LOVE".into()).await?;
    client.publish("Kelly", "LOVE".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(
        vec!["numbers".to_string(), "Sharon".to_string(), "Kelly".to_string()]
    ).await?;

    let messages = subscriber
        .into_stream()
        .filter(|msg| {
            match msg {
                //Ok(msg) if msg.content.len() == 1 => true,
                Ok(msg) if msg.content.len() == 4 => true,
                _ => false,
            }
        })
        .map(|msg| msg.unwrap().content)
        .take(3);

    tokio::pin!(messages);
    while let Some(v) = messages.next().await {
        println!("GOT: {:?}", v);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });

    subscribe().await?;

    println!("DONE");

    Ok(())
}
