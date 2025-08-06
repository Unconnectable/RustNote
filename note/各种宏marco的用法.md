`env!` 会读取环境的配置文件 这里是 `cargo.toml` 中的`name` 和`version`字段

```toml
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
```

```toml
[package]
name = "hecto"
version = "0.1.0"
edition = "2024"

[dependencies]
crossterm = "0.29.0"
```

### . 条件编译的 `other_expensive_check()`

这里有两个 `other_expensive_check()` 函数的定义,它们通过 `#[cfg(...)]` 属性进行条件编译.这意味着在编译时,Rust 编译器会根据特定的配置条件选择编译其中的一个版本,而不是两个都编译.

#### `#[cfg(debug_assertions)]`

```rust
#[cfg(debug_assertions)]
fn other_expensive_check() -> bool {
    println!("Thoroughly performing some other expensive check!");
    return true;
}
```

- `#[cfg(debug_assertions)]`:这个属性表示,只有当 Rust 编译器在**调试模式(Debug mode)**下编译时,才会包含这个函数定义.

#### `[cfg(not(debug_assertions))]`

```rust
#[cfg(not(debug_assertions))]
fn other_expensive_check() -> bool {
    println!("Only superficially performing some other expensive check!");
    return true;
}
```

- `#[cfg(not(debug_assertions))]`:这个属性表示,只有当 Rust 编译器在**非调试模式(即发布模式/Release mode)**下编译时,才会包含这个函数定义.

#### 调试模式检查 (`debug_assert!`)

```rust
    #[cfg(debug_assertions)]
    {
        println!("Debug Checks:");
    }

    debug_assert!(expensive_check());
    debug_assert!(expensive_check(), "Expensive check failed in Debug Build!");
    debug_assert_eq!(expensive_check(), true);
    debug_assert_ne!(expensive_check(), false);
```

- `#[cfg(debug_assertions)] { println!("Debug Checks:"); }`:这是一个**条件代码块**.只有在调试模式下,"Debug Checks:" 这行才会打印.
