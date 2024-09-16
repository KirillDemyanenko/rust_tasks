/*
    Напишите программу, которая считывает трехзначное число, а затем формирует
    наименьшее число из его цифр и выводит результат.
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let num: i32 = buffer.trim().parse().expect("Parse errror!");
    let mut figures: [i32; 3] = [num / 100, num % 100 / 10, num % 10];
    figures.sort();
    if figures[0] == 0 {
        if figures[1] == 0 {
            figures.swap(0, 2);
        } else {
            figures.swap(0, 1);
        }
    }
    println!("{}{}{}", figures[0], figures[1], figures[2]);
}
