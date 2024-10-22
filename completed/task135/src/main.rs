/*
    В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
    считывать два натуральных (u8) числа M и N, а затем выводить сетку M×N. Каждая ячейка сетки должна иметь ширину 3×2 символа.

    Редактировать можно только функцию print_grid()!
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

fn print_pmmm(){
    print!("+--");
}
fn print_p(){
    println!("+");
}
fn print_vss(){
    print!("|  ");
}
fn print_v(){
    println!("|");
}
fn main() {
    print_grid();
}

fn print_grid() {
    let m = input::<u8>();
    let n = input::<u8>();
    for _ in 0..m{
        for _ in 0..n{
            print_pmmm();
        }
        print_p();
        for _ in 0..n{
            print_vss();
        }
        print_v();
    }
    for _ in 0..n{
        print_pmmm();
    }
    print_p();
}