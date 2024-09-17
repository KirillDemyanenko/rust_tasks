/*
    Для проверки действительности номера кредитной карты может быть использован
    следующий алгоритм:

    1 Если сложить все цифры номера банковской карты;
    2 Добавить к этой сумме каждую вторую цифру, начиная со второй справа;
    3 Затем к получившейся сумме добавить количество цифр, превышающих четыре из
        банковского номера, также каждой второй, начиная со второй справа, то
        результат должен нацело делиться на 10.

    Если карта валидна, вывести: Карта с номером ???? ???? ???? ????
    действительна;
    Иначе: Карты с номером ???? ???? ???? ???? не существует.
*/

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Input error!");
    let card_number: Vec<u8> = buf
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let card_number_str: Vec<char> = buf.trim().chars().collect();
    let mut result: u8 = card_number.iter().sum();
    let mut result_string = String::from("");
    for (inx, value) in card_number.iter().enumerate() {
        if inx % 2 == 0 {
            result += value;
            if *value > 4 {
                result += 1;
            }
        }
    }
    for (inx, value) in card_number_str.iter().enumerate() {
        result_string.push(*value);
        if [3, 7, 11].contains(&inx) {
            result_string.push(' ');
        }
    }
    if result % 10 == 0 {
        println!("Карта с номером {} действительна", result_string);
    } else {
        println!("Карты с номером {} не существует", result_string);
    }
}
