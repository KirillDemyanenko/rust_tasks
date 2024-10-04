/*
    Напишите программу, которая считывает три (u32) числа: количество страниц
    набранной на компьютере статьи, количество строк на каждой странице и
    количество символов в строке. В одном из представлений Unicode каждый
    символ кодируется двумя байтами. Выведите информационный объем статьи в
    Кбайтах в этом варианте представления Unicode, как показано в примере.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let number_of_pages: u32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!"); 
    let numbers_on_lines: u32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let number_of_characters: u32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    println!(
        "Информационный объем статьи в Кбайтах: {}",
        number_of_pages * number_of_characters * numbers_on_lines * 2 / 1024
    )
}
