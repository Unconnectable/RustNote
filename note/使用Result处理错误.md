### `Result` 类型概述

在 Rust 中，`Result<T, E>` 是一个枚举（enum），用于表示可能成功也可能失败的操作。它有两个主要变体：

- `Ok(T)`：表示操作成功，并返回一个值 `T`。
- `Err(E)`：表示操作失败，并返回一个错误 `E`。

注意match时候的返回`Result`返回类型

**以下的例子中 `value = 0 || v <0` 会返回`CreationError`的类型，是`Err`，而不是第一个参数`Ok`的`Self`类型**

```rust
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            v if v > 0 => Ok(Self(v as u64)),
            0 => Err(CreationError::Zero),
            _ => Err(CreationError::Negative),
        }
    }
}

fn main() {
    //
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10))
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative)
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
```





枚举 结构体 混合和Result的处理

看`rustlings`的  [rustlings/exercises/13_error_handling/errors6.rs ](https://github.com/rust-lang/rustlings/blob/main/exercises/13_error_handling/errors6.rs) 的案例

```rust
use std::num::ParseIntError;

// 用来储存错误类型的枚举
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// A custom error type that we will be using in `PositiveNonzeroInteger::parse`.
// 自定义组合错误类型,包括 std的无法解析为数字的错误类型 ParseIntError 比如"abc" 和上述的自定义错误枚举
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

/// 对自定义的组合错误类型实现方法
/// 如果是 CreationError 也就是 < 或 =0 的 解析为 当前枚举的 Creation类型
/// 无法解析为数字的 为 ParseInt类型
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // 以下是原来的方法 如果直接unwarp 可能 panic
        // let x: i64 = s.parse().unwrap();
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;

        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
```

首先这里用到了`map_err`

### 1. 理解 `Result` 的 `map_err` 方法

首先，让我们回顾 `Result<T, E>` 的 `map_err` 方法的定义：

```rust
impl<T, E> Result<T, E> {
    fn map_err<F, O>(self, op: O) -> Result<T, F>
    where
        O: FnOnce(E) -> F,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(op(e)),
        }
    }
}
```

从定义中我们可以看出：

- `map_err` 是一个泛型方法，它接受一个闭包 `op` 作为参数。
- 这个闭包 `op` 的作用是：如果 `Result` 是 `Err(e)`，那么 `op` 会接收到这个错误值 `e`（类型是 `E`），然后执行闭包内部的逻辑，并返回一个新的错误值（类型是 `F`）。
- 如果 `Result` 是 `Ok(t)`，`map_err` 不会做任何操作，直接返回 `Ok(t)`。
- 最终，`map_err` 返回一个新的 `Result`，它的 `Ok` 类型保持不变（`T`），但 `Err` 类型变成了新的类型 `F`。

也就是接受一个函数参数返回错误处理类型

恰好通过`parse()`后的返回类型是 `Result<i64, std::num::ParseIntError>`



### 1. 理解 `Result` 的 `map_err` 方法

回顾 `Result<T, E>` 的 `map_err` 方法的定义：

```rust
impl<T, E> Result<T, E> {
    fn map_err<F, O>(self, op: O) -> Result<T, F>
    where
        O: FnOnce(E) -> F,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(op(e)),
        }
    }
}
```

从定义中我们可以看出：

- `map_err` 是一个泛型方法，它接受一个闭包 `op` 作为参数。
- 这个闭包 `op` 的作用是：如果 `Result` 是 `Err(e)`，那么 `op` 会接收到这个错误值 `e`（类型是 `E`），然后执行闭包内部的逻辑，并返回一个新的错误值（类型是 `F`）。
- 如果 `Result` 是 `Ok(t)`，`map_err` 不会做任何操作，直接返回 `Ok(t)`。
- 最终，`map_err` 返回一个新的 `Result`，它的 `Ok` 类型保持不变（`T`），但 `Err` 类型变成了新的类型 `F`。

------



### 2. `s.parse().map_err(ParsePosNonzeroError::from_parse_int)` 的步骤分解

步骤2

```rust
let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
```

#### **步骤 A: `s.parse()` 的执行**

当 `s.parse()` 被调用时，它会尝试将字符串 `s` 解析为 `i64`。这个操作的结果是一个 `Result<i64, std::num::ParseIntError>`。

此时，我们有两种可能的结果：

- **情况 1: 解析成功** `Ok(value: i64)`，例如 `Ok(123)`。这里的 `value` 是解析出来的整数。
- **情况 2: 解析失败** `Err(error: std::num::ParseIntError)`，例如 `Err(ParseIntError { ... })`。这里的 `error` 是一个具体的解析错误。

------



#### **步骤 B: `.map_err(ParsePosNonzeroError::from_parse_int)` 的应用**



现在，`map_err` 方法会作用在 **步骤 A** 得到的结果上。

回忆 `map_err` 的行为：

- **如果结果是 `Ok(value)` (情况 1)**： `map_err` 不会执行任何闭包，它会直接返回 `Ok(value)`。此时，`Result` 的类型仍然是 `Result<i64, std::num::ParseIntError>`，但这个 `Err` 类型实际上不会被用到，因为它成功了。

- **如果结果是 `Err(error)` (情况 2)**： 这是关键！`map_err` 会接收到这个 `error`（类型是 `std::num::ParseIntError`），然后它会调用传递给它的闭包 `op`。在这里，我们的 `op` 是 **`ParsePosNonzeroError::from_parse_int`**。

  那么，`ParsePosNonzeroError::from_parse_int(error)` 会被调用。让我们看看这个函数：

  ```rust
  impl ParsePosNonzeroError {
      fn from_parse_int(err: ParseIntError) -> Self {
          Self::ParseInt(err) // 这行代码将 `ParseIntError` 包装成 `ParsePosNonzeroError::ParseInt`
      }
  }
  ```

  所以，当 `from_parse_int` 被调用时，它接收到 `ParseIntError`，并将其封装在 `ParsePosNonzeroError::ParseInt(error)` 中返回。

  因此，`map_err` 最终会返回 `Err(ParsePosNonzeroError::ParseInt(original_parse_int_error))`。

------



#### **步骤 C: `"?"` 运算符的介入**

最后，`?` 运算符会作用在 **步骤 B** 得到的结果上。

- **如果结果是 `Ok(i64)` (成功路径)**： `?` 运算符会解包 `Ok` 中的 `i64` 值，并将其赋值给 `x`。此时，`x` 的类型就是 `i64`。
- **如果结果是 `Err(ParsePosNonzeroError::ParseInt(...))` (失败路径)**： `?` 运算符会立即从当前的 `parse` 函数中返回这个 `Err(ParsePosNonzeroError::ParseInt(...))`。这意味着整个 `parse` 函数的执行会在这里停止，并将错误传递给它的调用者。





`Self::new(x).map_err(ParsePosNonzeroError::from_creation)`

如果上一步`let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;` 解析字符串为数字成功的话，得到的x是i64类型的话，会进行下一步的对x的范围进行判断

```rust
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }
}
```

这里根据x的值返回不同类型的 `Ok(u64)` 或者 `Err(CreationError)` 

继续使用`map_err`  ,这个时候传入的 闭包函数是

```rust
fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
}
```

使用 `x` 尝试创建 `PositiveNonzeroInteger`。

- 如果 `x` 小于或等于零，将 `CreationError` 转换为 `ParsePosNonzeroError::Creation` 并返回。
- 如果 `x` 大于零，成功创建 `PositiveNonzeroInteger` 实例，并将其包装在 `Ok` 中返回。
