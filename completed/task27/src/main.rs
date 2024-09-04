/*
    Напишите программу, которая считывает натуральное число n (u8) и столько же
    вводимых пользователем значений, а затем выводит их.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let n: u8 = user_input.trim().parse().expect("Parse error!");
    for _ in 0..n {
        user_input.clear();
        std::io::stdin().read_line(&mut user_input).expect("Input error!");      
        println!("Вы ввели: {}", user_input.trim());
    }
}