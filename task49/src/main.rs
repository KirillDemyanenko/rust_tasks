/*
    Напишите программу, которая считывает три вещественных числа и выводит
    (до 1 знака) их в порядке возрастания. Гарантируется, что одинаковых чисел
    нет!
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
    if input[0] < input[1] && input[0] < input[2] {
        if input[1] < input[2] {
            println!("{:.1}, {:.1}, {:.1}", input[0], input[1], input[2]);
        } else {
            println!("{:.1}, {:.1}, {:.1}", input[0], input[2], input[1]);
        }
    } else if input[1] < input[0] && input[1] < input[2] {
        if input[0] < input[2] {
            println!("{:.1}, {:.1}, {:.1}", input[1], input[0], input[2]);
        } else {
            println!("{:.1}, {:.1}, {:.1}", input[1], input[2], input[0]);
        }
    } else {
        if input[0] < input[1] {
            println!("{:.1}, {:.1}, {:.1}", input[2], input[0], input[1]);
        } else {
            println!("{:.1}, {:.1}, {:.1}", input[2], input[1], input[0]);
        }
    }
}
