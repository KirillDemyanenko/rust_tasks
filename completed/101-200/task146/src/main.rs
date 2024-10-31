/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать часы от 0 до 23 и вывести в формате {hour} am / pm / midnight / noon, как показано в
   примерах.

   Вывод реализуйте в функции twelve_h_clck() !
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
    twelve_h_clck(input::<u8>());
}

fn twelve_h_clck(mut hours: u8) {
    match hours {
        0 => println!("12 midnight"),
        12 => println!("12 noon"),
        _ => println!(
            "{} {}",
            if hours < 12 { hours } else { hours - 12 },
            if hours < 12 { "am" } else { "pm" }
        ),
    }
}
