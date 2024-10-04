/*
    Напишите программу, которая считывает два целых числа start и end, а затем
    выводит сумму сгенерированной последовательности диапазоном start..end.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let start: i16 = user_input.trim().parse().expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let end: i16 = user_input.trim().parse().expect("Parse error!");
    let mut sum: i16 = 0;
    for i in start..end {
        sum += i;
    }
    println!("{sum}")
}
