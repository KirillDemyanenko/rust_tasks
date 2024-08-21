/*
    Напишите программу, которая считывает расстояние (f64) в километрах
    и конвертирует его в мили, ярды, футы и дюймы. Достаточно вывести до
    3 знаков.
*/

const KM_TO_MILES: f64 = 0.621371;
const KM_TO_YARDS: f64 = 1093.61;
const KM_TO_FEET: f64 = 3280.84;
const KM_TO_INCHES: f64 = 39370.1;

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let distance: f64 = user_input.trim().parse().expect("Parse error!");
    println!("{distance} км = {:.3} миль", distance * KM_TO_MILES);
    println!("{distance} км = {:.3} ярд", distance * KM_TO_YARDS);
    println!("{distance} км = {:.3} фут", distance * KM_TO_FEET);
    println!("{distance} км = {:.3} дюйм", distance * KM_TO_INCHES);
}
