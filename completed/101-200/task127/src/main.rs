/*
    https://stepik.org/lesson/1300620/step/4?unit=1315402
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
    let n: usize = input::<usize>();
    let mut p: f64 = 0.0;
    for i in 1..=n {
        p += (-1_f64).powi((i + 1) as i32) * (1.0 / i as f64)
    }
    println!("{:.1$}", p, n);
}
