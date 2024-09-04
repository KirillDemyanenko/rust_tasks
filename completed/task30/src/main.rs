/*
    Из некоторой массы свежих фруктов при сушке получили какую-то массу сушеных.
    Напишите программу, которая считывает два вещественных числа: массу
    исходного сырья в кг и получившуюся массу сухофруктов. Программа должна
    выводить в процентах (до 3 знаков) какую часть от исходной массы фруктов
    составляет масса сухофруктов и сколько процентов массы теряется при сушке.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let weight_of_feedstock: f64 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let mass_of_dried_fruits: f64 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    println!(
        "Доля сухофруктов относительно свежих фруктов составляет: {:.3}%",
        mass_of_dried_fruits / weight_of_feedstock * 100.0
    );
    println!(
        "Процент массы, потерянный при сушке: {:.3}%",
        100.0 - mass_of_dried_fruits / weight_of_feedstock * 100.0
    );
}
