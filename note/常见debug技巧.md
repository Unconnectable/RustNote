## 输出调试

输出当前 var 的类型和值

```rust
fn print_type<T>(_: &T) {
    println!("类型是 {}", std::any::type_name::<T>());
}
fn main() {
    let y = 0..10;
    print_type(&y);
}
```
