/*
    Напишите программу, которая считывает два натуральных (u32) числа end и block, а затем выводит
    диапазон 1..=end блочно в соответствии с block.
*/
use std::fmt::Debug;
use std::str::FromStr;

fn input<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    buffer.trim().parse::<T>().expect("Parse error")
}

fn main() {
    let end: u32 = input::<u32>();
    let block: u32 = input::<u32>();
    for i in (1..=end).step_by(block as usize) {
        for j in i..i + block {
            if j <= end {
                if j < i + block - 1 {
                    print!("{} ", j);
                } else {
                    print!("{}", j);
                }
            }
        }
        println!();
    }
}
