## 基本知识和库函数语法

## 玩具教学版本

`client.rs` 和 `server.rs`

```rust
//server.rs
use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    // 打印客户端的地址
    println!("新连接来自:{}", stream.peer_addr()?);

    // 创建一个缓冲区,用于接收数据
    let mut buffer = [0; 512];

    loop {
        // 从连接中读取数据到缓冲区
        let bytes_read = stream.read(&mut buffer)?;

        // 如果读取到的字节数为0,表示连接已关闭,跳出循环
        if bytes_read == 0 {
            println!("连接已关闭");
            return Ok(());
        }

        // 将读取到的数据原样写回
        stream.write_all(&buffer[..bytes_read])?;
        println!("处理了 {} 字节数据.", bytes_read);
    }
}

fn main() -> io::Result<()> {
    // 绑定到本地地址 127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("服务器正在监听 127.0.0.1:8080...");

    // 接受传入的连接,并在循环中处理
    // iter() 方法会阻塞,直到有新的连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 在新线程中处理每个连接,防止阻塞主线程
                // 对于简单的示例,也可以直接调用 handle_client
                std::thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("处理客户端时出错:{}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("接受连接时出错:{}", e);
            }
        }
    }

    Ok(())
}
```

```rust
//client.rs
use std::io::{self, prelude::*};
use std::net::TcpStream;
fn main() -> io::Result<()> {
    //
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("succeed conneting server: 127.0.0.1:8080");

    let message = "message from client.rs";

    stream.write_all(message.as_bytes())?;
    let mut buf = [0; 512];
    let bytes_to_read = stream.read(&mut buf)?;

    if bytes_to_read > 0 {
        let response = String::from_utf8_lossy(&buf[..bytes_to_read]);
        println!("从server 收到response: {}", response);
    } else {
        println!("connect close");
    }
    Ok(())
}
```

### **主线程与工作线程模式**

这里的核心思想:

1. **`listener.incoming()` 阻塞主线程**:主线程的工作非常简单和单一:它只负责**监听新连接**.当它调用 `incoming()` 迭代器时,它会在这里暂停,等待客户端的到来.
2. **`std::thread::spawn()` 启动工作线程**:一旦 `incoming()` 成功接收到一个新连接,主线程就会立即将这个连接的 `TcpStream` 实例封装起来,然后交给 `std::thread::spawn` 来启动一个新的子线程.这个子线程才是真正的“工人”.
3. **子线程的独立性**:每个通过 `spawn` 创建的子线程都是**独立的**.它们彼此不关联,拥有自己的执行栈和数据.当子线程内的 `stream.read()` 或 `stream.write_all()` 阻塞时,它只会影响自身,而不会影响到主线程或任何其他处理其他连接的子线程.

---

### **并发处理能力的实现**

- 主线程可以快速地接受连接,每秒可以处理成千上万个连接请求,因为它的阻塞时间非常短.
- 同时,后台有多个子线程在并行地处理各自的客户端连接.即使某个客户端因为网络慢或数据量大而导致其专属子线程被阻塞了很长时间,其他子线程仍然可以继续为它们的客户端提供服务,并且主线程也可以继续接受新的连接,不会被任何一个慢速客户端拖慢.

### 以下是基本方法

#### `std::io::prelude::*`

这行代码是 Rust 的一个惯用做法.`prelude` 模块包含了一组常用的 trait,例如 `Read` 和 `Write`.通过使用 `use` 语句导入它,您就可以直接调用 `TcpStream` 实例上的 `read()` 和 `write_all()` 等方法,而无需显式地引用它们的完整路径(例如 `std::io::Read::read`).

---

#### `std::net::TcpListener::bind`

- **用途**:这是创建 TCP 服务器的第一步.它将一个 `TcpListener` 实例绑定到一个特定的 IP 地址和端口上,使其开始监听传入的连接.
- **参数**:`addr: impl ToSocketAddr`.这里的 `impl` 关键字表示它接受任何实现了 `ToSocketAddr` trait 的类型.在您的代码中,传入的是一个字符串字面量 `"127.0.0.1:8080"`,它就是一种可以转换为套接字地址的类型.
- **返回值**:`io::Result<TcpListener>`.这是一个 `Result` 枚举,表示操作可能成功也可能失败.
  - `Ok(listener)`:如果绑定成功,它返回一个 `TcpListener` 实例,代表了正在监听的服务器.
  - `Err(e)`:如果绑定失败(例如,端口已被其他程序占用),它返回一个 `io::Error` 实例,包含了具体的错误信息.

---

#### `std::net::TcpListener::incoming`

- **用途**:这个方法返回一个迭代器,它用于接受来自客户端的连接.当服务器接收到新的连接请求时,迭代器就会生成一个 `TcpStream`.
- **参数**:无.
- **返回值**:`Incoming`.这是一个迭代器类型.当你对它进行循环(如 `for stream in listener.incoming()`)时,它会**阻塞**当前的线程,直到有新的连接到来.每当接受到一个连接,它就会产生一个 `io::Result<TcpStream>`.
- **注意**:由于它是阻塞的,所以为了处理多个连接,您的代码在 `for` 循环内部使用 `std::thread::spawn` 来为每个连接创建一个新的线程.

---

#### `std::net::TcpStream::connect`

- **用途**:这是 TCP 客户端用来连接服务器的方法.它会尝试建立一个到指定地址和端口的连接.
- **参数**:`addr: impl ToSocketAddr`.和 `bind` 类似,它接受一个可转换为套接字地址的类型,例如 `"127.0.0.1:8080"`,这代表了服务器的地址.
- **返回值**:`io::Result<TcpStream>`.
  - `Ok(stream)`:如果连接成功,它返回一个 `TcpStream` 实例,代表了客户端和服务器之间的双向通信通道.
  - `Err(e)`:如果连接失败(例如,服务器地址不正确或服务器未运行),它返回一个 `io::Error`.

---

#### `std::io::Read::read`

- **用途**:这是从一个可读的源(如 `TcpStream`)中读取数据的方法.
- **参数**:`buf: &mut [u8]`.它需要一个可变的字节切片作为缓冲区.读取到的数据会被填充到这个切片中.
- **返回值**:`io::Result<usize>`.
  - `Ok(bytes_read)`:如果读取成功,返回实际读取到的字节数.
  - `Ok(0)`:如果返回 0,通常表示数据流已结束或连接的另一端已关闭.
  - `Err(e)`:如果读取过程中出现错误,返回一个 `io::Error`.

---

#### `std::io::Write::write_all`

- **用途**:这是将一个字节切片完整地写入一个可写的目的地(如 `TcpStream`)的方法.
- **参数**:`buf: &[u8]`.它需要一个不可变的字节切片作为要写入的数据.
- **返回值**:`io::Result<()>`.
  - `Ok(())`:如果所有字节都被成功写入,它返回一个空元组 `()`.
  - `Err(e)`:如果写入过程中出现任何错误,它会返回一个 `io::Error`.这个方法会确保所有数据都被写入,如果无法完成,就会返回错误.

---

#### `std::thread::spawn`

- **用途**:这是创建并运行一个新线程的方法.这对于执行耗时或阻塞操作非常有用,可以避免阻塞主线程.
- **参数**:`f: F`,其中 `F` 是一个闭包.这个闭包就是新线程要执行的代码.您的代码中传入的是一个**闭包** `|| { ... }`,这个闭包会调用 `handle_client` 函数.
- **返回值**:`JoinHandle<T>`.这是一个句柄,您可以用来等待新线程完成执行,或者获取其返回值.不过在您的代码中,这个返回值被忽略了,因为服务器不需要等待客户端线程结束.

---

#### `String::from_utf8_lossy`

- **用途**:这个方法用于将一个字节切片(`&[u8]`)转换为一个 `Cow<str>`(一个智能指针,可以看作是借用或拥有的字符串).它会尝试将字节按 UTF-8 编码解码.
- **参数**:`v: &[u8]`.需要被解码的字节切片.
- **返回值**:`Cow<str>`.它的特点是,如果字节是有效的 UTF-8 序列,它会**借用**原始的切片;如果包含无效的 UTF-8 序列,它会**替换**这些无效序列并返回一个新拥有的 `String`.`lossy` 这个词就是指它在遇到无效编码时不会报错,而是用特殊字符(如 ``)来代替.

## 单线程

### 1 简单的 request 和返回一个 html 页面的 response

```rust
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    //
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let http_requests: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let len = contents.len();
    // response是String类型 网络中只能传输二进制数据 也就是字节(byte)
    // 通过 as.bytes()把文本转为字节 类型是 &[u8]的切片 刚好和 .to_string()相反 是字节转为文本
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, len, contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {:#?}", http_requests);
}
fn main() {
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("{GREEN}connection established!{RESET}");
    }
}
```

关于 request 和 response 的内容大概:

```sh
# request
Method Request-URI HTTP-Version
headers CRLF

message-body

# response

HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF

message-body
```

**`BufReader` 是一个缓冲读取器**(Buffered Reader).可以当成一个中间层.

当你的程序从 `TcpStream` 读取数据时,每一次读取操作都可能是一个昂贵的系统调用.如果数据量很小,比如每次只读取一个字节,那么频繁的系统调用会大大降低程序性能.

`BufReader` 的作用就是解决这个问题.它会:

1. **一次性从底层读取器**(这里是 `TcpStream`)**读取一大块数据**,并将其存储在内存中的缓冲区里.
2. 当你的程序需要读取数据时,它首先从这个缓冲区中获取,而不需要再次进行系统调用.
3. 只有当缓冲区中的数据用完时,`BufReader` 才会再次从 `TcpStream` 读取新的一块数据.

所以,用 `TcpStream` 初始化 `BufReader` 的目的是:**提高 I/O 效率**.这对于处理网络流数据非常重要,因为它能减少与操作系统内核的交互次数,从而让程序运行得更快.

一些处理方法:

- **`.lines()`**:这个方法来自 `BufReader`.它遍历字节流,根据换行符 `\n` 或 `\r\n` 来自动将字节分隔成一行行的字符串,从而让我们能以行的粒度来处理数据.
- **`.map(|result| result.unwrap())`**:`.lines()` 方法返回的是一个 `Result` 类型的迭代器,因为读取每一行都可能失败(比如连接中断).`map` 用来处理这个 `Result`,在这里我们简单地用 `unwrap()` 取出里面的字符串.
- **`.take_while(|line| !line.is_empty())`**:这是解析 HTTP 请求头的关键.HTTP 协议规定,请求头部以一个**空行**(`\r\n\r\n`)作为结束标志.`take_while` 会不断地从迭代器中取出行,直到遇到第一个满足条件的行(在这里就是遇到一个空行),然后就停止.这确保了我们只读取 HTTP 请求的头部信息,而忽略了后面的请求体.

### 验证请求,比如当访问不存在的页面的时候如何处理

```rust
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        //
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{}Content-Length: {}\r\n\r\n{}",
            status_line, length, contents
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        //
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{}Content-Length: {}\r\n\r\n{}",
            status_line, length, contents
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
    println!("Request: {:#?}", request_line);
}
fn main() {
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("{GREEN}connection established!{RESET}");
    }
}
```

#### 二者的区别

**第一个程序** 读取了整个 HTTP 请求头,并将其存储在一个向量中,所以你看到了完整的请求信息.  
**第二个程序** 只读取了 HTTP 请求的第一行(请求行),并将其存储在一个字符串中,所以你只看到了请求行.

##### 用到的 html 文件

`hello.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Hello!</title>
  </head>

  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```

`404.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>你好!</title>
  </head>

  <body>
    <h1>很抱歉!</h1>
    <p>由于运维删库跑路,我们的数据全部丢失,总监也已经准备跑路,88</p>
  </body>
</html>
```

## 多线程

## tokio **异步**

```

```
