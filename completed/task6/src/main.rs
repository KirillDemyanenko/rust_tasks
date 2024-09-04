/*
    Напишите программу, которая считывает четыре целых (u32) числа:
    скорость передачи голосового сообщения (бит/с) записанное в стерео
    (2 канала) формате;
    глубину кодирования (бит) на отсчет;
    частоту дискретизации  (отсчеты в секунду);
    время записи  (в секундах).
    Выведите время (в секундах) передачи голосового сообщения, если голосовое
    сообщение записано в стерео формате
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let transmission_speed: u32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!"); 
    let coding_depth: u32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let sampling_rate: u32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let recording_time: u32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    println!(
        "Время передачи голосового сообщения: {} секунд",
        2 * coding_depth * sampling_rate * recording_time / transmission_speed
    )
}
