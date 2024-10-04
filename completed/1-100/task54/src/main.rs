/*
    В редакторе кода представлен массив значений. Дополните код и считайте два
    индекса (usize), а затем выведите, являются ли считанные индексы индексами
    минимального или максимального элемента в массиве соответственно, как
    показано в примерах. Гарантируется, что вводимые индексы корректны!
*/

fn main() {
    let array = [3, 1, 0, -5, -1, 300, 4, 2];
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let idx_min: usize = buffer.trim().parse().expect("Parse errror!");
    buffer.clear();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let idx_max: usize = buffer.trim().parse().expect("Parse errror!");
    println!(
        "Считанный мин.индекс {}",
        if *array.iter().min().unwrap() == array[idx_min] {
            "корректный"
        } else {
            "некорректный"
        }
    );
    println!(
        "Считанный макс.индекс {}",
        if *array.iter().max().unwrap() == array[idx_max] {
            "корректный"
        } else {
            "некорректный"
        }
    );
}
