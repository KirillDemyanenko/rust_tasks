/*
    Напишите программу, которая считывает натуральное число n (u8) и столько же
    вводимых пользователем чисел, а затем выводит (до 1 знака) их сумму.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let n: u8 = user_input.trim().parse().expect("Parse error!");
    let mut sum: f64 = 0.0;
    for _ in 0..n {
        user_input.clear();
        std::io::stdin().read_line(&mut user_input).expect("Input error!");
        sum += user_input.trim().parse::<f64>().expect("Parse error!");
    }
    println!("Сумма считанных чисел равна: {sum:.1}");
}