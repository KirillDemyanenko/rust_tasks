/*
    Напишите программу, которая считывает номер месяца (u8) в диапазоне от 1 до
    12 (1 январь, 2 февраль и т. д.) и выводит количество дней в этом месяце
    для невисокосного года.
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let month: u8 = buffer.trim().parse().expect("Parse errror!");
    if month == 2 {
        println! {"28"};
    } else {
        println!(
            "{}",
            if [4, 6, 9, 11].contains(&month) {
                30
            } else {
                31
            }
        )
    }
}
