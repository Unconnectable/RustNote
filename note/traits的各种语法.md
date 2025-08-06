泛型+告知 trait

以下三种写法等价

```rust
fn compare_license_types(software1: impl Licensed, software2: impl Licensed)

fn compare_license_types<T: Licensed, V: Licensed>(software1: T, software2: V)

fn compare_license_types<T, V>(software1: T, software2: V) -> bool
where
    T: Licensed,
    V: Licensed,
{
    software1.licensing_info() == software2.licensing_info()
}
```

特征对象

当一个函数需要返回一个实现了某个特定 trait 的值,但这个值在运行时可能是多种不同具体类型中的一种时.

```rust
trait Shape {
    fn draw(&self);
}

struct Circle;
impl Shape for Circle {
    fn draw(&self) {
        println!("绘制圆形");
    }
}

struct Square;
impl Shape for Square {
    fn draw(&self) {
        println!("绘制方形");
    }
}

// 这个函数可以返回一个 Circle 或一个 Square,但都作为 dyn Shape 处理
fn get_random_shape(is_circle: bool) -> Box<dyn Shape> {
    if is_circle {
        Box::new(Circle)
    } else {
        Box::new(Square)
    }
}

fn main() {
    let shape1 = get_random_shape(true);
    shape1.draw(); // 在运行时才知道是调用 Circle 的 draw 还是 Square 的 draw

    let shape2 = get_random_shape(false);
    shape2.draw();
}
```
