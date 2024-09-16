/*
    Перевести число из десятичной системы счисления в троичную
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let mut num: i32 = buffer.trim().parse().expect("Parse errror!");
    let mut result = String::new();
    while num > 0 {
        result = [(num % 3).to_string(), result].concat();
        num /= 3;
    }
    println!("{result}");
}
