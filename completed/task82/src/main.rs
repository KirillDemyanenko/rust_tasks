/*
    Наш луноход может перемещаться в четырех направлениях (Север, Юг, Запад и Восток) и принимать
    команды:

        0 - продолжить движение.
        1 - повернуть налево.
        2 - повернуть направо.
    Напишите программу, которая считывает исходное направление (строка) и команду (u8), а затем
    выводит направление лунохода Направление лунохода после выполнения команды: Север, Юг, Запад
    или Восток после выполнения считанной команды, как показано в примере.

Гарантируется, что исходное направление и команды корректны!
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
    const DIRECTION: [&str; 4] = ["Север", "Запад", "Юг", "Восток"];
    let direction = input::<String>();
    let command = input::<u8>();
    if command == 0 {
        println!("Направление лунохода после выполнения команды: {direction}")
    } else { 
        for i in 0..4 {
            if DIRECTION[i as usize] == direction {
                if command == 1 {
                    println!("Направление лунохода после выполнения команды: {}", if i == 3 { DIRECTION[0] } else {DIRECTION[ (i + 1) as usize]})
                } else {
                    println!("Направление лунохода после выполнения команды: {}", if i == 0 { DIRECTION[3] } else {DIRECTION[ (i - 1) as usize]})
                }
            }
        }
    }
}