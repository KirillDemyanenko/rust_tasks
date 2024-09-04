/*
    Напишите программу, которая считывает целое неотрицательное число n (u32) и
    выводит для него факториал в виде сообщения:

    Факториал числа {n} равен: {}
    Факториал натурального числа n определяется, как произведение всех
    натурального чисел от 1 до n включительно. Для n = 0 принимается в качестве
    соглашения, что 0! = 1
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let n: u32 = user_input.trim().parse().expect("Parse error!");
    let mut fact: u32 = 1;
    for i in 1..=n {
        fact *= i;
    }
    println!("Факториал числа {n} равен: {fact}")
}
