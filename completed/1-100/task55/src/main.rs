/*
    В редакторе кода представлен массив значений. Дополните код и выведите
    отсортированный по возрастанию массив значений с помощью спецификатора :?.

    Сортировку массива необходимо сделать c помощью условных выражений.
*/

fn main() {
    let mut array = [5, -2, 8, -1, 3];
    for i in 1..(array.len() - 1) {
        for j in 0..(array.len() - i) {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
    println!("{array:?}")
}
