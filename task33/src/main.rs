/*
    Напишите программу, которая считывает три целых числа start (i32),
    end (i32) и step (usize) и выводит все значения диапазона start..=end с
    шагом step, для которых делителем является step.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let start: i32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let end: i32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let step: usize = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    for i in (start..=end).step_by(step) {
        if i % step as i32 == 0 {
            println!("{i}")
        }
    }
}
