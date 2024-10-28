/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать два целых числа a (i32) и b (u16), а затем вывести следующее наибольшее целое число к a,
   которое кратно числу b.

   Нахождение количества слов реализуйте в функции lsg_multi() !
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
    println!("{}", lsg_multi(input::<i32>(), input::<u16>()));
}

fn lsg_multi(a: i32, b: u16) -> i32 {
    if a % b as i32 == 0 {
        a + b as i32
    } else {
        a / b as i32 * b as i32 + b as i32
    }
}
