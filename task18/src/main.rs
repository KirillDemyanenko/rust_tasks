/*
    Напишите программу, которая считывает строку, два целых числа start и end,
    а затем выводит считанную строку с диапазоном start..end.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let text: String = user_input.trim().to_string();
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let start: u8 = user_input.trim().parse().expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let end: u8 = user_input.trim().parse().expect("Parse error!");
    for _ in start..end {
        println!("{text}")
    }
}
