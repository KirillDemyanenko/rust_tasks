/*
    На экзамен вынесено какое-то количество вопросов. Руслан не выучил
    некоторые из них. Напишите программу, которая считывает два целых числа:
    количество вопросов и количество невыученных вопросов, а затем выводит
    вероятность (до 3 знаков) того, что Руслану попадется выученный вопрос,
    как показано в примере.

    Гарантируется, что второе число не превышает первое.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let number_of_questions: f64 = user_input.trim().parse().expect("Parse error!");
    user_input.clear();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let number_of_unlearned_questions: f64 = user_input.trim().parse().expect("Parse error!");
    println!(
        "Вероятность попадания выученного вопроса: {:.3}", 
        (number_of_questions - number_of_unlearned_questions) / number_of_questions
    )
}
