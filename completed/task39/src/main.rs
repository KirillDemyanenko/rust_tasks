/*
    Напишите программу, которая считывает четыре целых числа (x, a, b, и c) и
    выводит, является ли x делителем чисел a, b и c одновременно.

    Если является вывести: {x} является делителем чисел {a}, {b}, {c}.
    Иначе вывести: {x} не является делителем всех чисел.
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let x: i32 = buffer.trim().parse().expect("Parse error!");
    buffer.clear();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let a: i32 = buffer.trim().parse().expect("Parse error!");
    buffer.clear();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let b: i32 = buffer.trim().parse().expect("Parse error!");
    buffer.clear();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let c: i32 = buffer.trim().parse().expect("Parse error!");
    if a % x == 0 && b % x == 0 && c % x == 0 {
        println!("{x} является делителем чисел {a}, {b}, {c}");
    } else {
        println!("{x} не является делителем всех чисел");
    }
}
