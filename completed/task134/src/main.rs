/*
    В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
    считывать в функцию func4() строку и выводить ее. Редактировать можно только функции func4() и
    start().
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

fn func2() {
    func3();
}

fn func1() {
    func2();
}

fn start() {
    func1();
}

fn main() {
    start();
}

fn func3() {
    func4();
}
fn func4() {
    println!("{}", input::<String>());
}