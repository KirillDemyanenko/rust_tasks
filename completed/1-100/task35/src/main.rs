/*
    Напишите программу, которая считывает натуральное значение n (u8) и столько
    же вводимых пользователем целых чисел. Программа должна вывести среди них
    максимальный и минимальный элемент в виде сообщений: Максимальное число: {}
    и Минимальное число: {}
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let n: usize = buffer.trim().parse().expect("Parse error!");
    let mut arr = vec![0_i32; n];
    for i in 0..n {
        buffer.clear();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Input error!");
        arr[i] = buffer.trim().parse::<i32>().expect("Parse error!");
    }
    println!("Максимальное число: {}", *arr.iter().max().unwrap());
    println!("Минимальное число: {}", *arr.iter().min().unwrap());
}
