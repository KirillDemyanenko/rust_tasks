/*
    В сосуд, содержащий определенный объем литров некоторого процентного водного
    раствора какого-то вещества, добавили еще дополнительные литры воды.

    Напишите программу, которая считывает три вещественных числа: исходный объем
    в литрах, процент водного раствора и добавленные литры воды. Программа
    должна вывести концентрацию получившегося раствора (в % до 3 знаков).
*/

fn main() {
    let mut buffer = String::new();
    let mut input: [f64; 3] = [0.0; 3];
    for i in 0..=2 {
        buffer.clear();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Input error!");
        input[i] = buffer.trim().parse().expect("Parse errror!");
    }
    println!(
        "Концентрация получившегося раствора: {:.3}%",
        
    );
}
