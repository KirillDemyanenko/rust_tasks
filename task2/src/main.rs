/*
    Напишите программу, которая считывает целое число (u64),
    представляющее собой количество секунд, прошедших с начала суток,
    и выводит это время в формате: X сек = H час M минут S секунд.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let seconds: u64 = user_input.trim().parse().expect("Parse.error!");
    println!(
        "{seconds} сек = {} час {} минут {} секунд",
        seconds / 60 / 60 % 60,
        seconds / 60 % 60,
        seconds % 60,
    )
}
