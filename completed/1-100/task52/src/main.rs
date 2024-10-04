/*
    Напишите программу, которая считывает номер месяца (u8) в диапазоне от 1 до
    12 (1 январь, 2 февраль и т. д.) и выводит название соответствующего времени
    года (зима, весна, лето или осень).
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let month: i32 = buffer.trim().parse().expect("Parse errror!");
    if [1, 2, 12].contains(&month) {
        println!("Зима");
    } else if [3, 4, 5].contains(&month) {
        println!("Весна");
    } else if [6, 7, 8].contains(&month) {
        println!("Лето");
    } else {
        println!("Осень")
    }
}
