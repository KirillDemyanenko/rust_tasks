/*
   Напишите программу, которая считывает емкость аккумулятора (в Ампер-часах, Ah) f64, напряжение
   аккумулятора (в Вольтах, В) f64 и энергопотребление устройства (в Ваттах, Вт) f64. Программа
   должна и вывести оставшееся время работы аккумулятора.

    Если остались минуты, вывести в виде: Аккумулятора хватит еще на {} мин;
    Иначе, вывести в виде: Аккумулятора хватит еще на {} ч {} мин.
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
    let c = input::<f64>();
    let u = input::<f64>();
    let p = input::<f64>();
    let t = c * u / p;
    if t > 0.99 {
        println!("Аккумулятора хватит еще на {:.0} ч {:.0} мин", t - t % 1.0, t % 1.0 * 60.0);
    } else {
        println!("Аккумулятора хватит еще на {:.0} мин", t * 60.0)
    }
}
