## `.map()` 和`.and_then()`

`map` 的闭包要求返回一个普通的非 `Result` 值.`map` 会自动将这个返回值封装到 `Ok` 变体中.

- `map` 闭包返回 `x + 2`,`map` 会将它自动变成 `Ok(x + 2)`.
- `and_then` 闭包**必须**手动返回一个 `Ok` 或 `Err`.
