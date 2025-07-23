use std::ops::Deref; // 用于显式使用 Deref trait

fn main() {
    // --- 1. String 转换为 &str (借用：零开销，不复制数据) ---

    let my_string = String::from("Hello, Rustaceans!");

    // 方法 1.1: 隐式解引用强制转换 (Deref Coercion)
    // 这是最常用、最惯用的方法。当函数或方法期望 &str 类型，而你提供了 &String 时，
    // 编译器会自动进行转换。
    println!("\n--- String -> &str (Deref Coercion) ---");
    fn print_str_slice(s: &str) {
        println!("通过函数参数传递 (&str): {}", s);
    }
    print_str_slice(&my_string); // &String 会自动转换为 &str

    // 方法 1.2: 使用 .as_str() 方法
    // 显式地获取 String 对应的 &str 切片。
    println!("\n--- String -> &str (.as_str()) ---");
    let str_slice_from_as_str: &str = my_string.as_str();
    println!("使用 .as_str(): {}", str_slice_from_as_str);

    // 方法 1.3: 使用切片语法 &s[..]
    // 获取整个 String 的切片，效果与 .as_str() 相同。
    println!("\n--- String -> &str (&s[..]) ---");
    let str_slice_from_slice_syntax: &str = &my_string[..];
    println!("使用切片语法: {}", str_slice_from_slice_syntax);

    // 方法 1.4: 显式 Deref (不常用，因为有 Deref Coercion 和 .as_str())
    // String 实现了 Deref<Target=str> trait，所以可以手动解引用。
    println!("\n--- String -> &str (显式 Deref) ---");
    let str_slice_from_deref: &str = my_string.deref();
    println!("使用 .deref(): {}", str_slice_from_deref);

    // --- 2. &str 转换为 String (拥有所有权：涉及内存分配和数据复制) ---

    let str_literal: &str = "Life is short, Rust is long.";
    let str_slice_variable: &str = str_literal;

    // 方法 2.1: 使用 .to_string() 方法
    // 这是将 &str 转换为 String 最常用且最简洁的方法。
    println!("\n--- &str -> String (.to_string()) ---");
    let string_from_to_string: String = str_slice_variable.to_string();
    println!("使用 .to_string(): {}", string_from_to_string);

    // 方法 2.2: 使用 String::from() 关联函数
    // 适用于任何实现了 Into<String> trait 的类型（包括 &str）。
    println!("\n--- &str -> String (String::from()) ---");
    let string_from_from: String = String::from(str_slice_variable);
    println!("使用 String::from(): {}", string_from_from);

    // 方法 2.3: 使用 .to_owned() 方法
    // 这是泛型方法，适用于任何实现了 ToOwned trait 的借用类型。
    // 对于 &str 来说，它会返回一个 String。
    println!("\n--- &str -> String (.to_owned()) ---");
    let string_from_to_owned: String = str_slice_variable.to_owned();
    println!("使用 .to_owned(): {}", string_from_to_owned);

    // 方法 2.4: 通过格式化宏 format!
    // 虽然不是专门用于类型转换，但可以用来从 &str 构建 String，
    // 如果你需要同时进行一些格式化。
    println!("\n--- &str -> String (format! 宏) ---");
    let formatted_string: String = format!("这是格式化的字符串：{}", str_slice_variable);
    println!("使用 format! 宏: {}", formatted_string);

    // 方法 2.5: 使用 .into() 方法 (需要类型标注)
    // 因为 &str 实现了 Into<String> trait，所以可以使用 .into()。
    // 但由于 Into trait 比较通用，通常需要类型标注来帮助编译器推断。
    println!("\n--- &str -> String (.into()) ---");
    let string_from_into: String = str_slice_variable.into(); // 需要显式类型标注
    println!("使用 .into(): {}", string_from_into);

    // 方法 2.6: 使用 String::new() 和 push_str() (不推荐用于简单转换)
    // 这种方法更适合构建字符串。
    println!("\n--- &str -> String (String::new() + push_str()) ---");
    let mut string_built_up = String::new();
    string_built_up.push_str(str_slice_variable);
    println!("使用 String::new() 和 push_str(): {}", string_built_up);

    // --- 3. 注意事项 ---

    // 悬垂引用错误示例 (尝试在 String 被销毁后使用其 &str 引用)
    // fn get_str_slice_from_string() -> &'static str { // 'static 生命周期通常指程序整个运行时间
    //     let s = String::from("Will be dropped");
    //     // &s.as_str() // 编译错误！'s' does not live long enough
    //     // 这里无法返回 &str，因为 String 's' 在函数结束时会被销毁。
    //     // 必须返回拥有所有权 String，或从全局/静态数据中借用。
    // }

    // String::from("") 和 "".to_string() 都是从空字符串切片创建空 String。
    let empty_string_from_from = String::from("");
    let empty_string_from_to_string = "".to_string();
    println!("\n空字符串转换：{} {}", empty_string_from_from, empty_string_from_to_string);
}
