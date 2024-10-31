/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать сторону квадрата (f64) и вывести (до 3 знаков) его периметр P=4⋅a.

   Вычисления периметра реализуйте в соответствующей функции calc_perim() !    
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
    println!("{:.3}", calc_perim(input::<f64>()));
}

fn calc_perim(side: f64) -> f64 {
    side * 4.0
}
