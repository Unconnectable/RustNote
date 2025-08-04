use restaurant::eat_at_restaurant;
fn main() {
    assert_eq!(eat_at_restaurant(), "yummy yummy!".to_string());

    assert_eq!(restaurant::eat_at_restaurant(), "yummy yummy!");

    assert_eq!(restaurant::hosting::seat_at_table(), "sit down please");
}
