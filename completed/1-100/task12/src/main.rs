/*
    Из сахарной свеклы получают сахар, масса которого составляет 18% массы
    свеклы. Напишите программу, которая считывает одно вещественное число:
    массу исходного сырья в тоннах и выводит в килограммах (до 3 знаков)
    сколько сахара получится при переработке
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let mass_of_raw_materials: f64 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    println!(
        "Из {:.3} тонн(ы) получится {:.3} кг сахара", 
        mass_of_raw_materials, mass_of_raw_materials * 0.18 * 1000.0
    )
}
