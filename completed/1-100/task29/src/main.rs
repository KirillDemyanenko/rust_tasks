/*
    В магазин привезли партию лампочек. Среди них оказалось какое-то количество
    разбитых, что составило определенный процент от общего числа. Напишите
    программу, которая считывает два числа: количество разбитых лампочек (u32)
    и процент от общего числа (f64), а затем выводит количество лампочек,
    которые привезли в магазин
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let broken_light_bulbs: u32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let percentage_of_the_total: f64 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    println!("Количество привезенных лампочек: {:.0}",
        broken_light_bulbs as f64 * 100.0 / percentage_of_the_total
    );
}