/* Make it work */
use std::collections::HashMap;
fn main() {
    let names = [("sunface", 18), ("sunfei", 18)];
    let folks: HashMap<_, _> = names.into_iter().collect();

    println!("{:?}", folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    // 4种答案

    let v2: Vec<i32> = v1.iter().cloned().collect();
    //let v2: Vec<i32> = v1.iter().map(|&x| x).collect();
    //let v2: Vec<i32> = v1.iter().map(|x| *x).collect();

    /*
    let mut v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter_mut().map(|&mut x| x + 100).collect();
    let v2: Vec<i32> = v1.iter_mut().map(|x| *x + 100).collect();
    */

    assert_eq!(v2, vec![1, 2, 3]);
}
