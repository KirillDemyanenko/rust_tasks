/*
    Напишите программу, которая считывает целое неотрицательное число n (u16) и
    выводит c новой строки диапазон от 0 до n (невключительно)
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let n: u16 = user_input.trim().parse().expect("Parse error!");
    for i in 0..n {
        println!("{i}")
    }
}
