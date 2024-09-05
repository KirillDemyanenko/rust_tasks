/*
    Напишите программу, которая считывает трехзначное число и выводит сумму
    чисел, образованные его цифрами
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let mut user_number: i32 = buffer.trim().parse().expect("Parse error!");
    if user_number < 0 {
        user_number *= -1;
        println!(
            "{}",
            (user_number % 100) / 10 + user_number % 10 - user_number / 100
        );
    } else {
        println!(
            "{}",
            user_number / 100 + (user_number % 100) / 10 + user_number % 10
        );
    }
}