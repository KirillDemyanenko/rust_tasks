/*
    Напишите программу, которая считывает показатель степени (u8) числа 9 и
    выводит его последнюю цифру без возведени
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let n: u8 = buffer.trim().parse().expect("Parse error!");
    if n % 2 == 1 {
        println!("Последняя цифра 9 в степени {n} равна 9");
    } else {
        println!("Последняя цифра 9 в степени {n} равна 1");
    }
}
