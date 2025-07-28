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





使用`into_iter` 和 `for _ in vec`的对比遍历一个Vec然后返回修改的值

```rust

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // input
        //     .into_iter()
        //     .map(|(s, command)| {
        //         match command {
        //             Command::Uppercase => s.to_uppercase(),
        //             Command::Trim => s.trim().to_string(),
        //             Command::Append(n) => format!("{}{}", s, "bar".repeat(n)), //format!("{}{}", s, "bar".repeat(n)),
        //         }
        //     })
        //     .collect()
        let mut output = Vec::new();
        for (s, command) in input {
            let s_ = match command {
                Command::Uppercase => s.to_uppercase(),
                Command::Trim => s.trim().to_string(),
                Command::Append(n) => format!("{}{}", s, "bar".repeat(n)),
            };
            output.push(s_);
        }
        output
    }
}

fn main() {
    //
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5))
        ];
        let output = transformer(input);

        assert_eq!(output, ["HELLO", "all roads lead to rome!", "foobar", "barbarbarbarbarbar"]);
    }
}
```

