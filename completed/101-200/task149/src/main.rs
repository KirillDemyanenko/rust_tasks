/*
   На мониторах цвета формируются в формате Red-Green-Blue (RGB), где значения R, G и B варьируются
   в целочисленном диапазоне от 0 до 255. При печати изображений используется формат
   Cyan-Magenta-Yellow-Black (CMYK), где значения C, M, Y и K варьируются в диапазоне от 0.0 до 1.0.

    В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
    считать три неотрицательных (u8) целых числа R, G и B, а затем вывести (до 2 знаков) их в CMYK
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
    let (c, m, y, k) = rgb_to_cmyk(input::<i32>(), input::<i32>(), input::<i32>());
    print_cmyk(c, m, y, k);
}

fn rgb_to_cmyk(r: i32, g: i32, b: i32) -> (f64, f64, f64, f64) {
    let (mut c, mut m, mut y, mut k) = (0.0, 0.0, 0.0, 1.0);
    if r == g && g == b && b == 0 {
        return (c, m, y, k)
    }
    let tmp = vec![r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0]
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap());
    let white: f64;
    match tmp {
        Some(v) => white = v,
        _ => white = 0.0,
    }
    c = (white - r as f64 / 255.0) / white;
    m = (white - g as f64 / 255.0) / white;
    y = (white - b as f64 / 255.0) / white;
    k = 1.0 - white;
    (c, m, y, k)
}

fn print_cmyk(c: f64, m: f64, y: f64, k: f64) {
    println!("C: {:.2}", c);
    println!("M: {:.2}", m);
    println!("Y: {:.2}", y);
    println!("K: {:.2}", k);
}
