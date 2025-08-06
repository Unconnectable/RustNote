### 初始化`vector`的方法:

```rust
fn main() {
    let v0 = vec![2, 3, 4];
    println!("{:#?}", v0);
    println!();

    let mut v: Vec<i128> = Vec::with_capacity(24);
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    println!("{:#?}", v);

    // 通过get得到的是Some包裹的类型 而且如果不借用 加上&的话 会消耗onwership
    let x: &Option<&i128> = &v.get(2);
    let y = &v[2];
    println!("{:#?}", x);
    assert_eq!(x.unwrap(), y);
}
```

### 转换静态数组为 vec

```rust
fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array
    let v = [10, 20, 30, 40].to_vec();
    (a, v)
}
```

### 使用 collect 进行简单的转换

注意 `iter()` 和 `into_iter()` 的区别:`iter()` 产生对元素的引用,如果你需要拥有所有权,通常需要 `cloned()` 或 `copied()`;而 `into_iter()` 产生

元素本身的所有权.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 示例 1: 筛选偶数并收集到新的 Vec
    let even_numbers: Vec<i32> = numbers
        .iter() // 获取迭代器,产生对元素的引用 (&i32)
        .filter(|&x| x % 2 == 0) // 筛选偶数
        .cloned() // 因为 filter 产生的是 &i32,cloned() 将其变为 i32
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
        .into_iter() // 获取所有权迭代器,产生元素本身 (i32)
        .map(|x| x * 2) // 每个元素乘以 2
        .collect(); // 收集到 Vec<i32>

    //println!("原始数字: {:?}", numbers); // 原始数字已经被消耗,不能再使用
    println!("翻倍的数字: {:?}", doubled_numbers); // 输出: 翻倍的数字: [2, 4, 6, 8, 10]
}
```

使用`into_iter` 和 `for _ in vec`的对比遍历一个 Vec 然后返回修改的值

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

### 总结和关系

| 方法                 | 稳定性 | 比较逻辑     | 要求数据类型           | 性能特点 | 适用场景                                       |
| -------------------- | ------ | ------------ | ---------------------- | -------- | ---------------------------------------------- |
| `sort()`             | 稳定   | 默认 (`Ord`) | `T: Ord`               | 良好     | 最常见的稳定排序,元素可直接比较               |
| `sort_by()`          | 稳定   | 自定义闭包   | `T: PartialOrd` (通常) | 良好     | 需要稳定排序,但有自定义比较逻辑               |
| `sort_unstable()`    | 不稳定 | 默认 (`Ord`) | `T: Ord`               | **快**   | 不需要稳定排序,元素可直接比较                 |
| `sort_unstable_by()` | 不稳定 | 自定义闭包   | `T: PartialOrd` (通常) | **最快** | 不需要稳定排序,有自定义比较逻辑或处理浮点数等 |

`PartialEq` → `Eq`
`PartialEq` → `PartialOrd` → `Ord`

---

比较 `PartialEq`、`Eq`、`PartialOrd` 和 `Ord` 这四个 Rust Trait 的表格:

| Trait 名称       | 作用/定义                                                                                        | 运算符               | 返回类型           | 特性/限制                                                                                                          | 依赖关系                | 浮点数 (`f32`/`f64`) 是否实现? |
| ---------------- | ------------------------------------------------------------------------------------------------ | -------------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------ | ----------------------- | ------------------------------- |
| **`PartialEq`**  | **偏等性**:定义了偏序相等关系,允许 `==` 和 `!=` 运算符.某些值可能无法与自身相等(如 `NaN`). | `==`, `!=`           | `bool`             | 要求 `a == b` 当且仅当 `a != b` 为假.不要求自反性(`a == a` 总是真).                                            | 无                      | **是**                          |
| **`Eq`**         | **全等性**:在 `PartialEq` 基础上,定义了全序相等关系.是一个**标记 Trait**,没有额外方法.      | `==`, `!=`           | `bool`             | 强调**自反性** (`a == a` 总是 `true`).用于 `HashMap` 和 `HashSet` 的键.                                          | 依赖 `PartialEq`        | **否** (`NaN != NaN`)           |
| **`PartialOrd`** | **偏序性**:定义了偏序大小关系,允许 `<, >, <=, >=` 运算符.某些值可能无法相互比较.             | `<`, `>`, `<=`, `>=` | `Option<Ordering>` | `partial_cmp` 返回 `Option`,因为无法比较时返回 `None`.                                                           | 依赖 `PartialEq`        | **是**                          |
| **`Ord`**        | **全序性**:在 `PartialOrd` 和 `Eq` 基础上,定义了全序大小关系.                                 | `<`, `>`, `<=`, `>=` | `Ordering`         | 强调所有值都可明确比较大小,且满足自反、反对称、传递性.用于 `BTreeMap` 和 `BTreeSet` 的键,以及 `sort()` 等方法. | 依赖 `PartialOrd`, `Eq` |                                 |

当 Rust 自动为结构体派生 `Ord` 和 `PartialOrd` Trait 时,它的默认行为是:

1. **按结构体字段的声明顺序进行比较.**
2. **每个字段都按其自身的默认升序(从小到大)进行比较.**
3. **只有当所有前面的字段都相等时,才会比较下一个字段.**

```rust
#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
    ];

    people.sort_unstable();

    println!("{:#?}", people);
}
```
