/*
    https://www.codewars.com/kata/5174a4c0f2769dd8b1000003/train/rust
*/

fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut res = arr.clone();
    res.sort();
    res
}

fn main() {
    println!("{:?}", sort_numbers(&vec![1, 2, 3, 10, 5]));
}
