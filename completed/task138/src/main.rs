/*
    В редакторе кода представлена неполная программа, которую необходимо доделать.
    Программа должна считать первичные баллы (u8) единого государственного экзамена по информатике
    и выводить тестовые баллы.

    Редактировать можно только функции get_primary_sc() и print_test_sc() !
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
    print_test_sc(get_primary_sc());
}

fn get_primary_sc() -> u8 {
    input::<u8>()
}

fn print_test_sc(scores: u8) {
    let scores_map =[0, 7, 14, 20, 27, 34, 40,
        43, 46, 48, 51, 54, 56, 59, 62, 64,
        67, 70, 72, 75, 78, 80, 83, 85, 88,
        90, 93, 95, 98, 100];
    println!("{}", scores_map[scores as usize]);
}
