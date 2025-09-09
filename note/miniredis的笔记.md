#

## 基础

### 下面使用`#[tokio::main]`的 marco 和手动创建 tokio 的异步运行时是一样的效果

```rust
async fn hello() -> String {
    String::from("this is async func")
}

#[tokio::main]
async fn main() {
    //let line = hello().await;
    let line2 = hello();
    println!("this is first line");
    println!("after line1 is await");
    println!("{}", line2.await);
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let line2 = hello();
        println!("this is first line");
        println!("after line1 is await");
        println!("{}", line2.await);
    })
}
```

### miniredis 的一个例子

```rust
use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 设置 key: "hello" 和 值: "world"

    client.set("hello", "world".into()).await?;

    // 获取"key=hello"的值
    let result = client.get("hello").await?;

    println!("从服务器端获取到结果={:?}", result);
    Ok(())
}
```

运行方法

1. `mini-redis-server`
2. `cargo run`

输出如下

```sh
从服务器端获取到结果=Some(b"world")
```

###

```rust
use core::panic;

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]

async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, ip) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

// 这是一个异步函数,负责处理单个客户端连接
// 当有新连接时,main 函数会调用它
async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // 键是 String 类型,值是 Vec<u8>(字节数组),用于存储任意二进制数据
    let mut db = HashMap::new();

    // 将底层的 TcpStream 封装进 Connection 类型,
    let mut connection = Connection::new(socket);

    // 这是一个 while 循环,只要客户端保持连接,并且持续发送数据帧,
    // 循环就会一直运行,读取并处理每一条命令
    while let Some(frame) = connection.read_frame().await.unwrap() {
        // 使用 match 表达式来匹配和分发命令.

        let response = match Command::from_frame(frame).unwrap() {
            // 如果命令是 Set,执行这个分支
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                // 返回一个简单的 "OK" 字符串帧,作为成功响应
                Frame::Simple("OK".to_string())
            }

            Get(cmd) => {
                // 在 db 中查找 key 对应的值
                // db.get() 返回一个 Option,如果找到是 Some(value),否则是 None
                if let Some(value) = db.get(cmd.key()) {
                    // .clone().into() 将 Vec<u8> 克隆并转换为 Bytes 类型,

                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            // 这是一个默认匹配分支,如果命令不是 Set 或 Get,就会触发
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // 将上一步构建好的响应帧写回给客户端
        // .await 会等待写操作完成
        connection.write_frame(&response).await.unwrap();
    }
}
```

```sh
               +--------------------+
               | 生产者 P1, P2...   |
               +--------------------+
                      |
                      | 发送请求
                      v
               +------------------------+
               | 消息通道 (请求缓冲区)  |
               +------------------------+
                      |
                      | 获取请求
                      v
+---------------+---------------+----------------+
|  消费者 C1    |   消费者 C2   |   消费者 C3... |
| (有连接 1)    |  (有连接 2)   |  (有连接 3...) |
+---------------+---------------+----------------+
           |       |       |
           |       |       |
           v       v       v
 +---------+ +---------+ +---------+
 | Client 1| | Client 2| | Client 3|
 +---------+ +---------+ +---------+
(与 Redis 服务器建立的连接)
```

### Rust 异步与同步消息通道对比

| 通道类型                                 | 生产者数量 | 消费者数量 | 消息传递方式         | 主要特点                                         | 适用场景                                 |
| ---------------------------------------- | ---------- | ---------- | -------------------- | ------------------------------------------------ | ---------------------------------------- |
| **`tokio::sync::mpsc`**                  | 多         | 单         | **点对点**(一对一)   | - 消息有**缓冲区**<br>- 接收者按发送顺序接收     | - 生产者-消费者模式<br>- 任务队列        |
| **`tokio::sync::oneshot`**               | 单         | 单         | **点对点**(一对一)   | - **无缓冲区**,只能发送和接收**一条**消息        | - 异步函数返回结果<br>- 请求-响应模式    |
| **`tokio::sync::broadcast`**             | 多         | 多         | **广播**(一对多)     | - 消息有缓冲区<br>- 每个消费者都能收到每条消息   | - 系统事件通知<br>- 实时数据分发         |
| **`tokio::sync::watch`**                 | 单         | 多         | **状态更新**(一对多) | - **只保存最新**的一条消息<br>- 旧消息会被覆盖   | - 实时配置更新<br>- 共享状态监听         |
| **`async-channel`**<br>(外部 crate)      | 多         | 多         | **点对点**(多对一)   | - 消息有缓冲区<br>- 消息只会被**一个**消费者接收 | - 多个工作者共享任务队列                 |
| **`std::sync::mpsc`**<br>(标准库)        | 多         | 单         | **点对点**(一对一)   | - **阻塞式**,等待消息时会阻塞当前线程            | - 非异步多线程编程<br>- 简单的线程间通信 |
| **`crossbeam::channel`**<br>(外部 crate) | 多         | 多         | **点对点**(多对一)   | - 性能极高,有**阻塞和非阻塞**模式                | - 追求极致性能的线程间通信               |

### tokio 的消息通道

```rust

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("fron tx1 ").await;
    });

    tokio::spawn(async move {
        tx2.send("from tx2").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
```

```rust
use bytes::Bytes;
use mini_redis::client;
use tokio::sync::oneshot;
#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: String::from("tx1 hello"),
        };
        tx.send(cmd).await.unwrap();
    });
    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: String::from("tx1 hello"),
            val: "bar".into(),
        };
        tx2.send(cmd).await.unwrap();
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                // Get { key } => {
                //     client.get(&key).await;
                // }
                // Set { key, val } => {
                //     client.set(&key, val).await;
                // }
                Get { key } => {
                    if let Ok(value) = client.get(&key).await {
                        println!("从 Redis 得到: {:?}", value);
                    }
                }
                Set { key, val } => {
                    if let Err(e) = client.set(&key, val).await {
                        eprintln!("执行 SET 命令失败: {}", e);
                    }
                }
            }
        }
    });
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
```

#### 实现同时使用 mpsc 和 oneshot 的生产者 tasks 和消费者 manager

```rust
use bytes::Bytes;
use mini_redis::client;
use tokio::sync::oneshot;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        response: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        response: Responder<()>,
    },
}
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    //task1 t2是生产者 生产任务 manager是消费者 处理任务 一般来说manager只用于处理简单的任务 于是需要把处理后的数据upload到任务
    let task1 = tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: String::from("tx1 hello"),
            response: oneshot_tx,
        };
        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        let response = oneshot_rx.await;
        println!("Got: {:?}", response);
    });
    let task2 = tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: String::from("tx1 hello"),
            val: "bar".into(),
            response: oneshot_tx,
        };
        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }
        //let response = oneshot_rx.await;
        let response = oneshot_rx.await;
        println!("Got (Set): {:?}", response);
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, response } => {
                    let result = client.get(&key).await; // 访问 Redis 数据库
                    let _ = response.send(result); // 将结果发回
                }
                Set { key, val, response } => {
                    let result = client.set(&key, val).await;
                    let _ = response.send(result);
                }
            }
        }
    });
    manager.await.unwrap();
    task1.await.unwrap();
    task2.await.unwrap();
}
```

输出

```sh
Got: Ok(Ok(Some(b"bar")))
Got (Set): Ok(Ok(()))
```

#### 尝试把`one-shot`的 rx 加入`enum`中同时读取 rx 的数据 最后失败

```rust
use bytes::Bytes;
use mini_redis::client;
use tokio::sync::oneshot;

use std::sync::{Arc, Mutex};
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
type SharedReceiver<T> = Arc<Mutex<oneshot::Receiver<mini_redis::Result<T>>>>;
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        response: Responder<Option<Bytes>>,
        receiver: SharedReceiver<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        response: Responder<()>,
        receiver: SharedReceiver<()>,
    },
}
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    //task1 t2是生产者 生产任务 manager是消费者 处理任务 一般来说manager只用于处理简单的任务 于是需要把处理后的数据upload到任务
    let task1 = tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();
        let shared_rx = Arc::new(Mutex::new(oneshot_rx));
        let cmd = Command::Get {
            key: String::from("tx1 hello"),
            response: oneshot_tx,
            receiver: Arc::clone(&shared_rx),
        };
        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        //let response = shared_rx.lock().unwrap().take().unwrap().await;
        //let response = shared_rx.lock().unwrap().take().unwrap().await;
        let response = shared_rx.lock().unwrap().try_recv();
        println!("Got: {:?}", response);
    });
    let task2 = tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();
        let shared_rx = Arc::new(Mutex::new(oneshot_rx));
        let cmd = Command::Set {
            key: String::from("tx1 hello"),
            val: "bar".into(),
            response: oneshot_tx,
            receiver: Arc::clone(&shared_rx),
            //receiver: oneshot_rx,
        };
        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }
        //let response = oneshot_rx.await;
        let response = shared_rx.lock().unwrap().try_recv();
        println!("Got (Set): {:?}", response);
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get {
                    key,
                    response,
                    receiver,
                } => {
                    let result = client.get(&key).await; // 访问 Redis 数据库
                    let _ = response.send(result); // 将结果发回
                }
                Set {
                    key,
                    val,
                    response,
                    receiver,
                } => {
                    let result = client.set(&key, val).await;
                    let _ = response.send(result);
                } // Get { key } => {
                  //     if let Ok(value) = client.get(&key).await {
                  //         println!("从 Redis 得到: {:?}", value);
                  //     }
                  // }
                  // Set { key, val } => {
                  //     if let Err(e) = client.set(&key, val).await {
                  //         eprintln!("执行 SET 命令失败: {}", e);
                  //     }
                  // }
            }
        }
    });
    manager.await.unwrap();
    task1.await.unwrap();
    task2.await.unwrap();
}
```

输出

```sh
Got: Err(Empty)
Got (Set): Err(Empty)
```

在 **Tokio** 异步编程中,如何通过限制消息通道(或队列)来确保系统稳定性和可靠性.这是一种非常重要的设计原则,尤其是在处理高并发和高负载场景时.

1. **限制消息队列的重要性:**
   - **防止内存耗尽:** 如果消息生成速度远快于消费速度,消息会无限制地在队列中堆积,最终耗尽系统内存.
   - **维持系统性能:** 即使内存不耗尽,过大的队列也会增加消息处理延迟,导致系统整体性能下降.
2. **Tokio 的“惰性”特性:**
   - 这是一个关键点.在 **Tokio** 中,异步操作(`async fn` 或 `async {}` 块)默认是**惰性**的.这意味着,除非你显式地使用 **`.await`** 或通过 **`tokio::spawn`** 将其放入运行时中,否则它不会被执行.
   - 这与许多其他语言中的“线程启动”或“任务创建”模型不同,后者可能在调用函数时立即将任务放入队列,从而可能导致快速的队列堆积.
3. **Tokio 中显式引入并发:**
   - 由于 **Tokio** 的惰性特性,你必须有意识地使用特定的工具来创建并发任务和队列:
     - **`tokio::spawn`**: 用于将一个 `async` 任务放入后台执行.
     - **`select!`**: 用于同时等待多个异步操作中的一个完成.
     - **`join!`**: 用于同时等待多个异步操作全部完成.
     - **`mpsc::channel`**: 消息传递通道,用于在不同的任务之间传递数据.
4. **控制并发度的必要性:**
   - 使用上述工具引入并发时,必须谨慎地进行控制.
   - **例子 1 (TCP 连接):** 无限制地接受新的 TCP 连接会导致系统资源(如文件描述符)迅速耗尽.因此,你需要限制同时打开的 **socket** 数量.这通常通过信号量(`Semaphore`)或其他计数机制来实现.
   - **例子 2 (`mpsc::channel`):** 当使用 **`mpsc` (多生产者,单消费者) 通道**时,**必须**设置一个缓冲区容量 (`buffer`).这个容量限制了队列中可以存放的最大消息数量.一旦队列满了,发送方 (`Sender`) 在发送新消息时会阻塞,直到队列中有空间可用.

### **总结**

**Tokio** 在设计上的一个核心思想:**显式控制和限制**.它迫使开发者在引入并发时必须考虑其带来的潜在风险,并通过设定限制值(如通道缓冲区大小、最大并发连接数)来确保系统的安全、可靠运行.

### `async`的读写

手动使用缓冲区

```rust
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);

    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;
        Ok::<_, io::Error>(())
    });
    let mut buffer = vec![0; 128];
    loop {
        let n = rd.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        println!("GOT {:?}", &buffer[..n]);
    }

    Ok(())
}
```

自动读写 复制

```rust
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
let linstenr = TcpListener::bind("").await?;

    loop {
        let (mut socket, _) = linstenr.accept().await?;
        tokio::spawn(async move {
            let (mut read, mut write) = socket.split();
            if io::copy(&mut read, &mut write).await.is_err() {
                println!("failed to copy");
            }
        });
    }

}
```

在 Rust 的异步编程中,堆上分配(如 Vec)比栈上分配(如 [T; N])更好.

这是因为 async 函数在编译后会变成一个状态机,它所有的局部变量(包括缓冲区)都必须被保存在一个任务(Task)结构体中.

如果使用栈上数组,即使你在不同的 .await 点使用了它们,任务结构体的大小也必须足够大,以容纳所有这些数组,导致任务变得臃肿和笨重,占用大量内存.
比如这样

```rust
struct Task {
    task: enum {
        AwaitingRead {
            socket: TcpStream,
            buf: [BufferType],
        },
        AwaitingWriteAll {
            socket: TcpStream,
            buf: [BufferType],
        }

    }
}
```

而如果使用堆上 Vec,任务结构体只需要保存一个指向堆内存的指针,其本身大小很小,这使得任务结构体轻巧高效,更利于性能和内存管理.

### `parse_frame`

```rust
use bytes::{Buf, BytesMut};
use mini_redis::frame::Error::Incomplete;
use mini_redis::{Frame, Result};
use std::io::Cursor;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf: Cursor<&[u8]> = Cursor::new(&self.buffer[..]);
        // 检查是否读取了足够解析出一个帧的数据
        match Frame::check(&mut buf) {
            Ok(_) => {
                // 获取组成该帧的字节数
                let len = buf.position() as usize;

                // 在解析开始之前,重置内部的游标位置
                buf.set_position(0);

                // 解析帧
                let frame = Frame::parse(&mut buf)?;

                // 解析完成,将缓冲区该帧的数据移除
                self.buffer.advance(len);

                // 返回解析出的帧
                Ok(Some(frame))
            }

            // 缓冲区的数据不足以解析出一个完整的帧
            Err(Incomplete) => Ok(None),
            // 遇到一个错误
            Err(e) => Err(e.into()),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()?{
              return Ok(Some(frame))
            }

            //缓冲区中的数据不完整,不足以解析出一个帧 才会到这里
            //0 代表到了数据的末尾
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                //如果没有数据 说明所有都处理完了
                //如果还有数据 但是此时已经断开了链接 发送了部分 说明出现了某些问题
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }
    // pub async fn write_all(&mut self,frame) ->Result<()>{
    //   //
    // }
}
fn main() {
    //
}
```

`parse_frame` 函数的核心作用是:**从缓冲区中解析并提取一个完整的 Redis 帧**.

它通过一个**光标(`Cursor`)**在缓冲区上进行操作,分两步完成任务:

1. **检查完整性**:它首先使用 `Frame::check` 快速判断缓冲区中的数据是否足够构成一个完整的帧.如果不够,则返回 `None`.
2. **解析与移除**:如果数据完整,它会使用 `Frame::parse` 将其解析成一个 `Frame` 对象,并用 `buffer.advance()` 高效地移除这部分已处理的数据,为下次解析做准备.

简单来说,`parse_frame` 确保了你**每次**都能从原始字节流中,得到**一个完整且有效的消息**.

一个完整的例子

```rust
use bytes::{Buf, BytesMut};
use mini_redis::Frame;
use mini_redis::Result;
use mini_redis::frame::Error::Incomplete;
use std::io::{self, Cursor};
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    async fn read_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf: Cursor<&[u8]> = Cursor::new(&self.buffer[..]);
        // 检查是否读取了足够解析出一个帧的数据
        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);

                Ok(Some(frame))
            }

            // 缓冲区的数据不足以解析出一个完整的帧
            Err(Incomplete) => Ok(None),
            // 遇到一个错误
            Err(e) => Err(e.into()),
        }
    }

    async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                //self.stream.write_all(b"$-1\r\n").await?;
                self.stream.write_all(b"$-1\r\n").await?;
            }

            Frame::Bulk(val) => {
                let len = val.len();
                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;

                self.stream.write_all(&val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => {
                unimplemented!();
            }
        }
        self.stream.flush().await?;
        Ok(())
    }

    async fn write_decimal(&mut self, val: u64) -> io::Result<()> {
        use std::io::Write;
        let mut buf = [0u8; 20];
        let mut buf_slice = &mut buf[..];
        write!(&mut buf_slice, "{}", val)?;
        self.stream.write_all(&buf_slice).await?;
        Ok(())
    }
}
// 示例 main 函数以展示如何使用
#[tokio::main]
async fn main() -> Result<()> {
    // 这里需要连接到一个 mini-redis 服务器来运行
    let stream = TcpStream::connect("127.0.0.1:6379").await?;
    let mut connection = Connection::new(stream);

    // 示例:发送一个 PING 命令
    let ping_frame = Frame::Array(vec![Frame::Bulk("PING".into())]);
    connection.write_frame(&ping_frame).await?;

    // 示例:读取响应
    if let Some(frame) = connection.read_frame().await? {
        println!("Received frame: {:?}", frame);
    }

    Ok(())
}
```

### 深入 async

如何给自己类型实现 future 类型 用于实现异步任务调用 await

看一个例子

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
struct Delay {
    when: Instant,
}
impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("hello world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}
```

执行流程:

1. `main` 函数启动,创建一个 `Delay` 实例.
2. `future.await` 首次调用 `Delay` 的 `poll` 方法.
3. 如果当前时间还没到 `when`(大概率是这样),`poll` 会返回 `Poll::Pending`,并调用 `waker`.`tokio` 运行时会接收到 `Poll::Pending`,并将此任务挂起.
4. 由于 `poll` 方法中调用了 `wake_by_ref()`,`tokio` 会立即(在极短的时间内)再次调用 `Delay` 的 `poll` 方法.
5. 这个过程会持续重复,形成一个非常快的循环,直到当前时间超过了 `when`.
6. 一旦时间满足条件,`poll` 返回 `Poll::Ready("done")`,`await` 表达式结束等待,并将结果 `"done"` 赋值给 `out`.
7. `assert_eq!(out, "done")` 成功断言,程序正常退出.

解释这里的参数
`self:Pin<&mut Self>`:需要使用 Pin 把内容顶在内存中保证不改变位置
`cx: &mut Context<'_>`: Context 有对 waker 的引用 需要保证 waker 不能比她活得更久 出现悬垂引用

手动实现类似 aysnc 的异步状态机

```rust
use futures::task;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}
impl Future for Delay {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("hello world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
struct MiniTokio {
    tasks: VecDeque<Task>,
}

//允许将任何实现了 Future Trait 的具体类型(如 async { ... } 块或 Delay)放入同一个队列中
type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    /// 生成一个 Future并放入 mini-tokio 实例的任务队列中
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
        //Future<Output = ()>: 必须是一个返回 () 的 Future.
    {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        //取出任务
        while let Some(mut task) = self.tasks.pop_front() {
            //轮询任务 如果没有完成 把它从后面放回队列
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}

fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    mini_tokio.run();
}
```

如何通过 wake 执行 poll 呢

```rust
impl Future for Delay {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("hello world");
            Poll::Ready("done")
        } else {
            // cx.waker().wake_by_ref();
            // Poll::Pending

            let waker = cx.waker().clone();
            let when = self.when;
            thread::spawn(move || {
                let now = Instant::now();

                //没有到时间 让线程睡眠 过一会再唤醒
                if now < when {
                    thread::sleep(when - now);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}
```

### 第一个版本:忙循环(Busy Loop)

这个版本在 `poll` 方法中直接使用了 `cx.waker().wake_by_ref()`.

```rust
// 如果时间未到
else {
    cx.waker().wake_by_ref(); // 立即唤醒
    Poll::Pending
}
```

**运行流程(假设延迟 10 秒)**:

1. **0 秒**:`poll` 方法被调用,发现时间未到.
2. **0 秒**:它立即调用 `wake_by_ref`,请求执行器再次轮询.然后它返回 `Poll::Pending`.
3. **0.0001 秒**:执行器收到唤醒通知,再次调用 `poll`.
4. **0.0001 秒**:`poll` 方法再次发现时间未到,再次调用 `wake_by_ref`.
5. **持续 10 秒**:这个过程以每秒成千上万次的频率重复进行.执行器线程在这 10 秒内几乎 100% 都在忙于重复调用 `poll`,无法处理任何其他任务.

**数据总结**:

- **CPU 占用**:极高,因为执行器线程在 10 秒内几乎 100% 都在忙于重复调用 `poll`.
- **效率**:极低,所有其他任务都被阻塞.

---

### 第二个版本:线程阻塞(Thread Blocking)

这个版本将 `waker` 的调用转移到了一个新的线程中.

```rust
// 如果时间未到
else {
    let waker = cx.waker().clone();
    let when = self.when;

    thread::spawn(move || {
        thread::sleep(when - Instant::now()); // 阻塞式休眠
        waker.wake(); // 休眠结束后唤醒
    });

    Poll::Pending // 立即返回
}
```

**运行流程(假设延迟 10 秒)**:

1. **0 秒**:`poll` 方法被调用,发现时间未到.
2. **0 秒**:`poll` 方法立即派生一个新线程,并返回 `Poll::Pending`.
3. **0.0001 秒**:主执行器线程立即返回,可以去处理队列中的其他任务.
4. **10 秒**:新线程在后台休眠 10 秒后,被操作系统唤醒.
5. **10 秒**:新线程调用 `waker.wake()`,这会向主执行器发送一个信号.
6. **10.0001 秒**:主执行器收到信号,将该任务再次放入其队列.在下一次循环中,执行器会再次轮询该任务.

**数据总结**:

- **CPU 占用**:主执行器线程的 CPU 占用在 10 秒内极低,因为大部分时间它都在等待或处理其他任务.只有在任务最终完成时,它才会被再次唤醒.
- **效率**:高,主线程没有被阻塞,可以同时处理其他任务.但要注意,额外创建的 OS 线程会有一些内存和上下文切换的开销.

### 一个简化的 Rust 异步运行时(executor),我们称之为 MiniTokio.它包含了异步编程中的核心组件:任务(Task)、调度器(Scheduler) 和 唤醒器(Waker).

```rust
use crossbeam::channel::{self};
use futures::lock::Mutex;
use std::sync::Arc;

use futures::task::{self, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};
struct Delay {
    when: Instant,
}
impl Future for Delay {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("hello world");
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}
struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}
struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}
impl Task {
    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();
        let _ = future.as_mut().poll(&mut cx);
    }
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }

    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}
impl MiniTokio {
    fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio { scheduled, sender }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&mut self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    mini_tokio.run();
}
```

### **异步任务的流程总结**

#### **1. 任务启动**

1. **MiniTokio** 创建一个**通道**，得到一个**发送端**（`sender`）和一个**接收端**（`scheduled`）。
   - `let (sender, scheduled) = channel::unbounded();`
2. 它把这个**发送端**交给一个**新的任务（Task）**。
   - `mini_tokio.spawn(async { ... });`
3. **Task** 用这个**发送端**，把自己**投递**到通道里。
   - `Task::spawn(...)` 调用 `sender.send(task);`

#### **2. 任务执行**

1. **MiniTokio** 的主循环从通道的**接收端**（`scheduled`）取出**任务**。
   - `while let Ok(task) = self.scheduled.recv() { ... }`
2. 它调用**任务**的 `poll` 方法，开始执行。
   - `task.poll();`
3. **任务**执行时，发现还没完成，就告诉 **MiniTokio**：“我需要等待。”
   - `Delay::poll(...)` 返回 `Poll::Pending`

#### **3. 任务挂起与唤醒**

1. **任务**返回“等待”状态，同时把自己的**唤醒凭证（waker）** 交给**外部线程**去保管。
   - `let waker = cx.waker().clone();`
2. **外部线程**等待时间到了，就使用这个**唤醒凭证**。
   - `thread::spawn(...)` 内部调用 `waker.wake();`
3. **唤醒凭证**触发，告诉**任务**：“醒来吧，可以继续了。”
   - `waker.wake()` 触发 `Task` 的 `wake_by_ref()` 方法。
4. **任务**被唤醒后，使用一开始得到的**发送端**（`executor` 成员），再次把自己投递回通道。
   - `arc_self.schedule()` 调用 `self.executor.send(self.clone());`

#### **4. 再次执行**

1. **MiniTokio** 循环再次从通道的**接收端**（`scheduled`）取出**任务**。
   - `while let Ok(task) = self.scheduled.recv() { ... }`
2. 这次，任务可能已经完成，于是返回“完成”状态，流程结束。
   - `Delay::poll(...)` 返回 `Poll::Ready("done")`
