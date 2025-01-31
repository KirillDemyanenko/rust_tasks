/*
   https://www.codewars.com/kata/52e88b39ffb6ac53a400022e/train/rust
*/

fn int32_to_ip(int: u32) -> String {
    let bin_str = format!("{:0>width$b}", int, width = 32);
    format!(
        "{}.{}.{}.{}",
        i32::from_str_radix(&bin_str[0..8], 2).unwrap(),
        i32::from_str_radix(&bin_str[8..16], 2).unwrap(),
        i32::from_str_radix(&bin_str[16..24], 2).unwrap(),
        i32::from_str_radix(&bin_str[24..32], 2).unwrap()
    )
}

fn main() {
    println!("{}", int32_to_ip(15));
}
