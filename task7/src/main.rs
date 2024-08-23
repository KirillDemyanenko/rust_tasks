/*
    Напишите программу, которая считывает два целых числа a (i8) и b (i8) и
    выводит для них операцию побитового И в виде сообщений.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let a: i8 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!"); 
    let b: i8 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    println!("{a:08b} :a\n{b:08b} :b\n{:08b} :a & b", a & b)
}
