/*
    Напишите программу, которая считывает два целых числа a и b, и десять целых чисел в массив.
    Программа должна вывести получившийся массив с помощью заполнителя {:?}, где a и b будут
    переставлены местами.

    Гарантируется, что a и b содержатся в массиве и что повторяющихся чисел нет!
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
    let a: i16 = input::<i16>();
    let b: i16 = input::<i16>();
    let mut inx: (usize, usize) = (0, 0);
    let mut arr: [i16; 10] = [0; 10];
    for i in 0..10 {
        arr[i] = input::<i16>();
        if arr[i] == a {
            inx.0 = i
        } else if arr[i] == b {
            inx.1 = i
        }
    }
    arr.swap(inx.0, inx.1);
    println!("{:?}", arr);
}
