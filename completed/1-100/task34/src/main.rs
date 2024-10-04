/*
    Напишите программу, которая считывает два целых числа start и end и выводит
    с новой строки числа диапазона start..=end с конца, используя метод rev().
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let start: i32 = user_input.trim().parse().expect("Parse error!");
    user_input.clear();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let end: i32 = user_input.trim().parse().expect("Parse error!");
    for i in (start..=end).rev() {
        println!("{i}")
    }
}
