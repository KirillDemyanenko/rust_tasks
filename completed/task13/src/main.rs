/*
    Вы вложили некоторое количество денег в акции какой-то компании и получили
    некий процент дохода. Напишите программу, которая считывает два вещественных
    числа: сумму вложения и процент дохода, после чего выводит (до 3 знаков)
    получившийся доход.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let investment_amount: f64 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let percentage_of_income: f64 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    println!(
        "{:.3}", 
        investment_amount * (percentage_of_income / 100.0)
    )
}
