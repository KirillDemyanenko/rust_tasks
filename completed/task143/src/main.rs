/*
    Напишите программу, которая считывет строку и выведит её ёмкость и длину в виде сообщения:

    capacity = {}
    length = {}
 */

fn main() {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    println!("capacity = {}\nlength = {}", str.capacity(), str.len());
}