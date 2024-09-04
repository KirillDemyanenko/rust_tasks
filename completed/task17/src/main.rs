/*
    Напишите программу, которая считывает строку, целое число n и выводит
    считанную строку n раз.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let text: String = user_input.trim().to_string();
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let n: u8 = user_input.trim().parse().expect("Parse error!");
    for _ in 0..n {
        println!("{text}")
    }
}
