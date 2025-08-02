use restaurant::eat_at_restaurant;

fn main() {
    assert_eq!(eat_at_restaurant(), "yummy yummy!".to_string());

    //或者不引入lib.rs 直接调用
    assert_eq!(restaurant::eat_at_restaurant(), "yummy yummy!");
}
