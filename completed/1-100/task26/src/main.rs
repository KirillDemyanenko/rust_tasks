/*
    Напишите программу, которая считывает 10 строковых значений и выводит их.
*/

fn main() {
    let mut user_input = String::new();
    for _ in 0..10 {
        std::io::stdin().read_line(&mut user_input).expect("Input error!");      
        println!("Вы ввели: {}", user_input.trim());
        user_input.clear();
    }
}