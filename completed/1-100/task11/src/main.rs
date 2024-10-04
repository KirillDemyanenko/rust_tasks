/*
    Вероятность того, что новый электрический чайник прослужит больше года,
    равна некоторому значению (от 0 до 1). Вероятность того, что он прослужит
    больше двух лет, также равна какому-то значению (от 0 до 1).

    Напишите программу, которая считывает два вещественных числа: вероятность
    службы больше года и вероятность службы больше двух лет, а затем выводит
    вероятность (до 2 знаков) того, что он прослужит меньше двух лет, но больше
    года
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let probability_1: f64 = user_input.trim().parse().expect("Parse error!");
    user_input.clear();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let probability_2: f64 = user_input.trim().parse().expect("Parse error!");
    println!(
        "Вероятность того, что чайник прослужит меньше двух лет, но больше года равна: {:.2}", 
        probability_1 - probability_2
    )
}
