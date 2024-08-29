/*
    В редакторе кода представлен массив из 5 значений. Дополните код и считайте
    целое положительное число x, а затем выведите массив с помощью :?, применив
    к его элементам составные операторы:

    К первому элементу: += x;
    Ко второму элементу: -= x;
    К третьему элементу: *= x;
    К четвертому элементу: /= x;
    К пятому элементу: %= x.
*/

fn main() {
    let mut array = [1, 2, 3, 4, 5];
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Input error!");
    let user_number: i32 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    array[0] += user_number;
    array[1] -= user_number;
    array[2] *= user_number;
    array[3] /= user_number;
    array[4] %= user_number;
    println!("{:?}", array)
}
