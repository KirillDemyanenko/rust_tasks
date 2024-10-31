/*
    Строительство любого дома включает в себя укладку стяжки пола, которая рассчитывается умножением
    площади на цену одного квадратного метра.

    В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
    считывать длину (f64) и ширину (f64) пола, и стоимость (f64) одного квадратного метра, а затем
    вывести (до 2 знаков) стоимость стяжки.

    Для считывания сторон используйте функцию get_len_width(). Расчёт стоимости произвести в
    calc_cost(), а вывод в print_cost().
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

fn get_len_width() {
    calc_cost(input::<f64>(), input::<f64>(), input::<f64>());
}

fn main() {
    get_len_width();
}

fn calc_cost(length: f64, width: f64, price: f64) {
    print_cost(length * width * price);
}

fn print_cost(cost: f64) {
    println!("{:.2}", cost);
}
