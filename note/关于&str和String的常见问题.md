# Rust 中 `String` 与 `&str` 的相互转换

在 Rust 中，字符串主要有两种核心类型：

- **`String`**: 这是一个**拥有所有权**的、可增长的、堆分配的字符串类型。你可以修改它的内容。
- **`&str`**: 这是一个**字符串切片**（string slice），它是对 UTF-8 编码的字符串数据（可能位于堆上、栈上或只读数据段中）的**不可变引用**。`&str` 本身不拥有数据，只是“借用”它。

理解这两种类型及其转换方式是 Rust 所有权和借用概念的核心应用。

---

## 1. `String` 转换为 `&str` (借用)

将 `String` 转换为 `&str` 是一种**借用**操作。这意味着你只是获取了一个指向 `String` 内部数据的引用，而不会复制数据，因此这是一个**零开销**的操作。

### 方法 1.1：隐式解引用强制转换 (Deref Coercion)

这是 Rust 中最常用和最惯用的方法。当一个函数或方法期望 `&str` 类型作为参数，而你提供了一个 `&String` 类型的引用时，Rust 编译器会自动为你执行转换。

```rust
fn print_str_slice(s: &str) {
    println!("通过函数参数传递 (&str): {}", s);
}

fn main() {
    let my_string = String::from("Hello from String!");

    // &String 会自动转换为 &str
    print_str_slice(&my_string);
}
```

### 方法 1.2：使用 `.as_str()` 方法

`String` 类型提供了一个显式的 `.as_str()` 方法，可以获取其对应的 `&str` 切片。这使得代码意图更加明确。

```rust
fn main() {
    let my_string = String::from("Explicit conversion with .as_str()");
    let str_slice: &str = my_string.as_str();
    println!("使用 .as_str(): {}", str_slice);
}
```

### 方法 1.3：通过切片语法 `&s[..]`

你可以使用 Rust 的切片语法来获取整个 `String` 的切片。这与 `.as_str()` 方法的效果相同。

```rust
fn main() {
    let my_string = String::from("Slice syntax works too!");
    let str_slice: &str = &my_string[..];
    println!("使用切片语法: {}", str_slice);
}
```

### 方法 1.4：显式 Deref (不常用)

`String` 实现了 `Deref<Target=str>` trait，这意味着它可以在需要 `str`（或其引用 `&str`）的地方被解引用。虽然你可以显式地调用 `.deref()` 方法，但在大多数情况下，隐式解引用或 `.as_str()` 更常见。

```rust
use std::ops::Deref;

fn main() {
    let my_string = String::from("Explicit Deref");
    let str_slice: &str = my_string.deref();
    println!("使用 .deref(): {}", str_slice);
}
```

---

## 2. `&str` 转换为 `String` (拥有所有权)

将 `&str` 转换为 `String` 意味着你希望创建一个**拥有自己数据拷贝**的字符串。由于 `&str` 只是一个引用，这个过程通常涉及**内存分配和数据复制**。

### 方法 2.1：使用 `.to_string()` 方法

这是将 `&str` 转换为 `String` 最常用且最简洁的方法。它会创建 `&str` 内容的一个新副本。

```rust
fn main() {
    let str_slice: &str = "Convert me to String!";
    let my_string: String = str_slice.to_string();
    println!("使用 .to_string(): {}", my_string);
}
```

### 方法 2.2：使用 `String::from()` 关联函数

`String::from()` 是一个通用方法，适用于任何实现了 `Into<String>` trait 的类型，包括 `&str`。它也会创建数据副本。

```rust
fn main() {
    let str_slice: &str = "Creating String from &str using String::from()";
    let my_string: String = String::from(str_slice);
    println!("使用 String::from(): {}", my_string);
}
```

### 方法 2.3：使用 `.to_owned()` 方法

`.to_owned()` 是一个泛型方法，定义在 `ToOwned` trait 上。对于 `&str` 来说，它会返回一个拥有所有权的 `String`。与 `to_string()` 类似，它也会复制数据。

```rust
fn main() {
    let str_slice: &str = "To owned String";
    let my_string: String = str_slice.to_owned();
    println!("使用 .to_owned(): {}", my_string);
}
```

### 方法 2.4：通过格式化宏 `format!`

`format!` 宏可以用于从 `&str` 或其他类型构建 `String`。如果你需要同时进行一些格式化操作，这会非常方便。

```rust
fn main() {
    let str_slice: &str = "Rust";
    let formatted_string: String = format!("Hello, {} world!", str_slice);
    println!("使用 format! 宏: {}", formatted_string);
}
```

### 方法 2.5：使用 `.into()` 方法 (需要类型标注)

由于 `&str` 实现了 `Into<String>` trait，你可以使用 `.into()` 方法进行转换。但因为 `Into` trait 比较通用，通常需要**显式的类型标注**来帮助编译器推断你想要的目标类型。

```rust
fn main() {
    let str_slice: &str = "Into String!";
    let my_string: String = str_slice.into(); // 必须有 `my_string: String`
    println!("使用 .into(): {}", my_string);
}
```

### 方法 2.6：使用 `String::new()` 和 `push_str()` (不推荐用于简单转换)

这种方法更适合于**逐步构建**一个字符串，而不是简单地转换一个已有的 `&str`。它效率较低，因为涉及多次字符串修改。

```rust
fn main() {
    let str_slice: &str = "Building up";
    let mut my_string = String::new();
    my_string.push_str(str_slice);
    my_string.push_str(" a String.");
    println!("使用 String::new() 和 push_str(): {}", my_string);
}
```

获取`String` 类型的切片 ,比如

```rust
let mut s = String::from("hello, world");
//想要获取"hello"
let slice1: &str = &s[0..5];
```

如何对`Stirng`类型进行加法?

```rust
fn main() {
    let s1 = "hello";
    let s2 = "world";
    let combined_str: String = format!("{}, {}!", s1, s2);
    let S1: &str = &combined_str;
    println!("combined_str1: {S1}");
    println!("");

    let s3 = "hello";
    let s4 = ", world!";
    let combined_str2: String = s3.to_owned() + s4;
    let S2: &str = &combined_str2;
    println!("combined_str2: {S2}");
    println!("");

    let s5 = "hello";
    let s6 = ", world!";
    let mut combined_str3: String = String::new();
    combined_str3.push_str(s5); // 添加第1个 &str
    combined_str3.push_str(s6); // 添加第2个 &str
    let S3: &str = &combined_str3;
    println!("combined_str3: {S3}");
    println!("");
}
```





String的三个元素:

罗指针 当前的使用的长度和容量

```rust
use std::mem;

fn main() {
    let story = String::from("Rust By Practice");

    // 阻止 String 的数据被自动 drop
    let mut story = mem::ManuallyDrop::new(story);

    //获得裸指针 这个行为是 Unsafe 的
    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    // 我们可以基于 ptr 指针、长度和容量来重新构建 String.
    // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!")
}
```

