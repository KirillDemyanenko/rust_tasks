/*
    Напишите программу, которая считывает два целых числа start и end, а затем
    выводит результат произведения номера итерации на 2 для диапазона start..end.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let start: i16 = user_input.trim().parse().expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let end: i16 = user_input.trim().parse().expect("Parse error!");
    for i in start..end {
        println!("{i} * 2 = {}", i * 2)
    }
}
