/* Fill in the blank and fix the error*/
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num)
    }
}
fn factory_(x: i32) -> impl Fn(i32) -> i32 {
    let num = 5;

    //即使表达式一样也被认定为不同的类型 需要提前写好返回的闭包
    /*
    if x > 1 {
        move |x| x + num
    } else {
        move |x| x + num
    }
    */
    let closure = move |x| x + num;
    closure
}
fn main() {
    //
}
