/*
    В редакторе кода представлен кортеж значений. Дополните код и выведите
    (до 2 знаков) сумму всех положительных значений.
*/

fn main() {
    let tup = (1, 3.14, -12.3, -50, 100, 250, -4, 7.6);
    let mut sum: f64 = if tup.0 as f64 > 0.0 { tup.0 as f64 } else { 0.0 };
    sum += if tup.1 as f64 > 0.0 { tup.1 as f64 } else { 0.0 };
    sum += if tup.2 as f64 > 0.0 { tup.2 as f64 } else { 0.0 };
    sum += if tup.3 as f64 > 0.0 { tup.3 as f64 } else { 0.0 };
    sum += if tup.4 as f64 > 0.0 { tup.4 as f64 } else { 0.0 };
    sum += if tup.5 as f64 > 0.0 { tup.5 as f64 } else { 0.0 };
    sum += if tup.6 as f64 > 0.0 { tup.6 as f64 } else { 0.0 };
    sum += if tup.7 as f64 > 0.0 { tup.7 as f64 } else { 0.0 };
    println!("{:.2}", sum);
}
