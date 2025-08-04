// 给 Info 枚举添加一个生命周期参数 `'a` 和一个泛型参数 `T`
enum Info<'a, T> {
    Name(&'a str),
    Age(usize),
    Sex(&'a str),
    Score(T), // Score 变体现在使用泛型类型 T
}

// argu_func 函数也需要生命周期参数 `'a` 和泛型参数 `T`
fn argu_func<'a, T, F>(f: F, age: usize, name: &'a str, sex: &'a str, score: T) -> Info<'a, T>
where
    // 闭包的签名中，'a 和 T 确保类型一致
    F: Fn(usize, &'a str, &'a str, T) -> Info<'a, T>,
{
    f(age, name, sex, score)
}

fn main() {
    // 闭包接受年龄、姓名、性别和成绩，并返回一个 Info::Score 变体
    let get_score_info = |age: usize, name: &str, sex: &str, score: f64| {
        println!(
            "处理信息：{}，年龄：{}，性别：{}，成绩：{}",
            name, age, sex, score
        );
        Info::Score(score)
    };

    // 调用 argu_func 函数，传入 get_score_info 闭包和浮点数成绩
    let info1 = argu_func(get_score_info, 20, "Alice", "female", 95.5);

    match info1 {
        Info::Score(score) => println!("成功获取浮点数成绩: {}", score),
        _ => println!("不是成绩信息"),
    }

    // ---

    // 另一个闭包，处理整数成绩
    let get_int_score_info = |age: usize, name: &str, sex: &str, score: i32| {
        println!(
            "处理信息：{}，年龄：{}，性别：{}，成绩：{}",
            name, age, sex, score
        );
        Info::Score(score)
    };

    // 调用 argu_func 函数，传入 get_int_score_info 闭包和整数成绩
    let info2 = argu_func(get_int_score_info, 22, "Bob", "male", 88);

    match info2 {
        Info::Score(score) => println!("成功获取整数成绩: {}", score),
        _ => println!("不是成绩信息"),
    }
}


fn argu_func<'a, T, F>(f: F, name: &'a str, age: usize, sex: &'a str, score: T) -> Info<'a, T>
where
    // 闭包的参数顺序现在与 argu_func 的参数顺序一致：name, age, sex, score
    F: Fn(&'a str, usize, &'a str, T) -> Info<'a, T>,
{
    // 调用闭包时，也按照 name, age, sex, score 的顺序传入参数
    f(name, age, sex, score)
}