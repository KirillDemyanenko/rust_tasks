/*
    После успешного внедрения "Кассовый Компаньон 1.0" спрос на ваши аппараты
    возрос, и теперь проверки одних монет  недостаточно. Вам необходимо
    улучшить программу к "Кассовый Компаньон 1.0" чтобы помимо монет она
    определяла и купюры.

    Напишите программу для монето- и купюро приемника механического торгового
    автомата, которая считывает два целых числа и определяет, является ли
    первая монетой, а вторая купюрой, принимаемой вашим автоматом:

    Если первое значение соответствует одной из монет (1, 2, 5 или 10)
    программа должна вывести сообщение: Принята монета номинала {}. В
    противном случае программа должна вывести Монеты такого номинала не
    принимаются.

    Если второе значение соответствует одной из купюр (5, 10, 50, 100, 200,
    500, 1000, 2000 или 5000) программа должна вывести сообщение: Принята
    купюра номинала {}. В противном случае программа должна вывести Купюры
    такого номинала не принимаются.
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let coin: i32 = buffer.trim().parse().expect("Parse errror!");
    buffer.clear();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let banknote: i32 = buffer.trim().parse().expect("Parse errror!");
    if [1, 2, 5, 10].contains(&coin) {
        println!("Принята монета номинала {coin}");
    } else {
        println!("Монеты такого номинала не принимаются");
    }
    if [5, 10, 50, 100, 200, 500, 1000, 2000, 5000].contains(&banknote) {
        println!("Принята купюра номинала {banknote}");
    } else {
        println!("Купюры такого номинала не принимаются");
    }
}
