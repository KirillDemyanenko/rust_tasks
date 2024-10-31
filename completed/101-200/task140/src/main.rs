/*
    Транспортный налог в РФ - это региональный налог. Им облагаются не только автомобили, но и
    другие виды транспорта, например яхты, катера, самолеты, вертолеты и др.

    В редакторе кода представлена неполная программа, которую необходимо доделать. 
    Программа должна считывать налоговую ставку в рублях (f64), мощность транспорта в л.с (u16)
    и период владения в месяцах (u8), а затем выводить (до 2 знаков) транспортный налог по следующей
    формуле:

        транспортный налог = ставка налога × мощность × (период владения / 12)

    Для считывания значений используйте функцию input(), расчёт транспортного налога произвести в
    calc_tax(), а вывод в print_tax().
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

fn input1() {
    calc_tax(input::<f64>(), input::<u16>(), input::<u8>());
}

fn main() {
    input1();
}

fn calc_tax(tax_rate: f64, veh_pow: u16, own_per: u8) {
    print_tax(tax_rate * veh_pow as f64 * (own_per as f64 / 12.0));
}
fn print_tax(tax: f64) {
    println!("{tax:.2}")
}
