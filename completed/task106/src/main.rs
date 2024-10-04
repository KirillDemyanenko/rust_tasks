/*
    Напишите программу, которая считывает три значения:

        Правильный пароль (String);
        Количество попыток (u8);
        "Бесконечно" вводимое пользователем значение (String);

    Программа должна выводить на каждую попытку ввода, предоставлен ли пользователю доступ Доступ
    предоставлен или нет Неверный пароль. Если количество попыток превысит установленное значение,
    вывести Слишком много попыток, пожалуйста повторите позже.
*/
use std::fmt::Debug;
use std::str::FromStr;

fn input<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    buffer.trim().parse::<T>().expect("Parse error")
}

fn main() {
    let correct_password = input::<String>();
    let mut attempts = input::<u32>();
    let mut password = input::<String>();
    while attempts > 0 {
        if password == correct_password {
            println!("Доступ предоставлен");
            break;
        } else {
            println!("Неверный пароль");
            attempts -= 1;
        }
        if attempts == 0 {
            println!("Слишком много попыток, пожалуйста повторите позже");
            break;
        }
        password = input::<String>();
    }
}
