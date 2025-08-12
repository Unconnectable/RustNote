use std::cell::Cell;

fn main() {
    let mut x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    retain_0(&mut x);
    retain_1(&mut x);
    retain_2(&mut x);
    println!("{:?}", x);
    assert_eq!(x, vec![2, 4, 6, 8, 10]);
}
fn is_even(i: i32) -> bool {
    i % 2 == 0
}
fn retain_0(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if is_even(nums[j]) {
            nums[i] = nums[j];
            i += 1;
        }
    }
    nums.truncate(i);
}
// fn retain_even_0(nums: &mut Vec<i32>) {
//     let mut i = 0;
//     for num in nums.iter_mut().filter(|num| is_even(**num)) {
//         nums[i] = *num;
//         i += 1;
//     }
//     nums.truncate(i);
// }

fn retain_1(nums: &mut Vec<i32>) {
    //调用库函数 `.retain()` 用于原地过滤向量
    nums.retain(|&num| is_even(num));
}

fn retain_2(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
    let mut count = 0;
    //对新得到的 &[Cell<i32>]类型的切片 slice进行处理
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[count].set(num.get());
        count += 1;
    }
    nums.truncate(count);
}
