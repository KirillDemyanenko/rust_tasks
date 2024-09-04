/*
    Напишите программу, которая считывает два целых числа a и b, а затем выводит
    в порядке возрастания все целые числа, расположенные между a и b (включая
    их самих)
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let start: i16 = user_input.trim().parse().expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let end: i16 = user_input.trim().parse().expect("Parse error!");
    if start > end {
        for i in end..=start {
            println!("{i}");
        }
    } else {
        for i in start..end {
            println!("{i}");
        }
    }
}
