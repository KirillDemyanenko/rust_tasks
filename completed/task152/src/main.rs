/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать целочисленные коэффициенты квадратного уравнения (a, b и c) и вывести количество корней.
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
    find_root(input::<i32>(), input::<i32>(), input::<i32>());
}

fn find_root(a: i32, b: i32, c: i32) {
    let d = b.pow(2) - 4 * a * c;
    match d {
        0 => println!("Уравнение имеет 1 корень"),
        _ => {
            if d < 0 {
                println!("Уравнение не имеет действительных корней")
            } else {
                println!("Уравнение имеет 2 различных корня")
            }
        }
    }
}
