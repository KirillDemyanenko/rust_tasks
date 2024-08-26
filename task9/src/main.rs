/*
    Напишите программу, которая считывает целое число и выводит для него
    операцию побитового отрицания в виде сообщений. 
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let a: i8 = user_input
        .trim()
        .parse()
        .expect("Par/se error!");
    println!(" {a}: {a:032b} :a");
    println!("{}: {:032b} :!a", !a, !a);
}
