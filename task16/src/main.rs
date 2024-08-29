/*
    Напишите программу, которая считывает целое число n и выводит с новой
    строки сообщение: Rustacean 🦀 n раз.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let n: u8 = user_input.trim().parse().expect("Parse error!");
    for _ in 0..n {
        println!("Rustacean 🦀")
    }
}
