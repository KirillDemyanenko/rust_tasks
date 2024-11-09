/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать в массив 10 целых чисел и вывести с помощью заполнителя {:?} развернутую коллекцию.

   Разворот массива реализуйте в функции reverse() !
*/

use std::fmt::Debug;
use std::str::FromStr;

fn input<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    buffer.trim().parse::<T>().expect("Parse error")
}

fn main() {
    let arr: [i32; 10] = [0; 10].map(|_| input::<i32>());
    println!("{:?}", reverse(arr))
}

fn reverse(mut array: [i32; 10]) -> [i32; 10] {
    for i in 0..5 {
        array.swap(i, 9 - i);
    }
    array
}
