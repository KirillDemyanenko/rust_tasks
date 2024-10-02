/*
    Напишите программу, которая считывает натуральное число (u32) и выводит, являются ли все его
    цифры одинаковыми.

    Напишите программу, которая считывает натуральные числа (u32), пока не будет введено иное, а
    затем выводит недостающее число введенной последовательности в виде сообщения: Пропущено число {}.

    Гарантируется, что количество чисел в последовательности всегда на 1 меньше и стоп строка не равна пробельным символам!
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
    let mut nums = Vec::new();
    loop {
        match input::<String>().parse::<u32>() {
            Ok(n) => nums.push(n),
            Err(_e) => break,
        }
    }
    nums.sort();
    for num in 1..=nums[0] + nums.len() as u32 {
        if !nums.contains(&num) {
            println!("Пропущено число {}", num);
            break
        }
    }
}