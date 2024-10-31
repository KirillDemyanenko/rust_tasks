/*
    На стадионе три категории сидячих мест А, B и C. В редакторе кода представлена неполная
    программа, которую необходимо доделать. Программа должна по порядку считать шесть значений:

        Стоимость одного билета класса A (u32);
        Стоимость одного билета класса B (u32);
        Стоимость одного билета класса C (u32);
        Количество проданных билетов класса А (u16);
        Количество проданных билетов класса B (u16);
        Количество проданных билетов класса C (u16);
        И вывести сумму дохода, полученного от продажи билетов.

    Для считывания значений используйте функцию input(), расчёт дохода в calc_profit(), а вывод в
    print_profit().
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
    // A = one_tkt_cst[0], B = one_tkt_cst[1], C = one_tkt_cst[2]
    let one_tkt_cst = [input::<u32>(), input::<u32>(), input::<u32>()];

    // A = num_tks_sld[0], B = num_tks_sld[1], C = num_tks_sld[2]
    let num_tks_sld = [input::<u16>(), input::<u16>(), input::<u16>()];

    calc_profit(one_tkt_cst, num_tks_sld);
}

fn main() {
    input1();
}

fn calc_profit(one_tkt_cst: [u32; 3], num_tks_sld: [u16; 3]) {
    print_profit(
        one_tkt_cst[0] * num_tks_sld[0] as u32
            + one_tkt_cst[1] * num_tks_sld[1] as u32
            + one_tkt_cst[2] * num_tks_sld[2] as u32,
    )
}

fn print_profit(profit: u32) {
    println!("{}", profit);
}
