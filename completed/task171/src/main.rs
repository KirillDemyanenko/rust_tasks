/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   вывести содержимое массива два раза. Первый раз в цикле, а второй с помощью println!().

   Изменять тип массива нельзя!
*/

fn main() {
    let array: &mut [&str; 5] = &mut ["Rust ", "is ", "an ", "amazing ", "language"];

    for i in *array {
        print!("{}", i)
    }
    println!();
    println!("{:?}", array);
}
