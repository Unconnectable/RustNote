转换静态为 vec

```rust
fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array
    let v = [10, 20, 30, 40].to_vec();
    (a, v)
}
```

### 使用 collect 进行简单的转换

注意 `iter()` 和 `into_iter()` 的区别：`iter()` 产生对元素的引用，如果你需要拥有所有权，通常需要 `cloned()` 或 `copied()`；而 `into_iter()` 产生元素本身的所有权。

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 示例 1: 筛选偶数并收集到新的 Vec
    let even_numbers: Vec<i32> = numbers
        .iter() // 获取迭代器，产生对元素的引用 (&i32)
        .filter(|&x| x % 2 == 0) // 筛选偶数
        .cloned() // 因为 filter 产生的是 &i32，cloned() 将其变为 i32
        .collect(); // 收集到 Vec<i32>

    println!("偶数: {:?}", even_numbers); // 输出: 偶数: [2, 4]

    println!("原始数字: {:?}", numbers);

    // 示例 2: 使用iter筛选然后map收集到新的 Vec
    let squared_numbers: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("偶数的平方: {:?}", squared_numbers); // 输出: 偶数的平方: [4, 16]

    // 示例 3: 映射并收集到新的 Vec
    let doubled_numbers: Vec<i32> = numbers
        .into_iter() // 获取所有权迭代器，产生元素本身 (i32)
        .map(|x| x * 2) // 每个元素乘以 2
        .collect(); // 收集到 Vec<i32>

    //println!("原始数字: {:?}", numbers); // 原始数字已经被消耗，不能再使用
    println!("翻倍的数字: {:?}", doubled_numbers); // 输出: 翻倍的数字: [2, 4, 6, 8, 10]
}
```
