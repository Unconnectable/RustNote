#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// 修复 main 函数中的错误
fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // "hello你好"的类型是 &str size of &str = 16
    check_size([(); 31].map(|_| "hello你好".to_string())); // String 24 字节
    check_size(['中'; 191]); // 4字节
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
