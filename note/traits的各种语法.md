泛型+告知trait

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

