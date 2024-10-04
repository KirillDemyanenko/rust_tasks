/*
    Напишите программу, которая считывает три целых числа start (i32),
    end (i32) и step (usize) и выводит (до 1 знака) для диапазона start..=end с
    шагом step произведение его чисел.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let start: i32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let end: i32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let step: usize = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    let mut total: f64 = 1.0;
    for i in (start..=end).step_by(step) {
        total *= i as f64;
    }
    println!("{:.1}", total)
}
