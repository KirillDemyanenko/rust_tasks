/*
    Напишите программу, которая считывает индекс (usize) и десять целых
    значений, меняет местами по считанному индексу предыдущий и следующие
    элементы, а затем выводит получившийся массив с помощью спецификатора :?.

    Гарантируется, что 0 < индекс < 9.
*/

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let n: usize = buffer.trim().parse().expect("Parse error!");
    let mut arr = vec![0_i32; 10];
    for i in 0..10 {
        buffer.clear();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Input error!");
        if i == n + 1 {
            arr[n - 1] = buffer.trim().parse::<i32>().expect("Parse error!");
        } else if i == n - 1 {
            arr[n + 1] = buffer.trim().parse::<i32>().expect("Parse error!");
        } else {
            arr[i] = buffer.trim().parse::<i32>().expect("Parse error!");
        }
    }
    println!("{:?}", arr);
}
