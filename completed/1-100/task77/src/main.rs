/*
    Напишите программу, которая считывает количество часов (u8) отработанных за неделю, а затем
    выводит (до 2 знаков) заработную плату до вычетов и после, а также сумму налогов, учитывая:

     - базовый почасовой тариф: 1500 руб/час.
     - переработку более 40 часов в неделю: базовый почасовой тариф * 1.5.
     - налоговая ставка: 13%.
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
    let x: (i32, i32) = (input::<i32>(), input::<i32>());
    let y: (i32, i32) = (input::<i32>(), input::<i32>());
    let z: (i32, i32) = (input::<i32>(), input::<i32>());
    if (x.0 * y.1 + y.0* z.1 + z.0 * x.1) - (y.0 * x.1 + z.0 * y.1 + x.0 * z.1) == 0 {
        println!("Точки коллинеарны");
    } else {
        println!("Точки не коллинеарны");
    }
 }
