/*
    Напишите программу, которая считывает два целых числа start и end и выводит
    числа диапазона start..=end разделяя их символом |.
*/

fn main() {
    let mut buf = String::new();
    let mut tup = (-0, -0);
    std::io::stdin().read_line(&mut buf).expect("Input error!");
    tup.0 = buf.trim().parse().unwrap();
    buf.clear();
    std::io::stdin().read_line(&mut buf).expect("Input error!");
    tup.1 = buf.trim().parse().unwrap();
    if tup.1 < tup.0 {
        tup = (tup.1, tup.0)
    }
    for i in tup.0..=tup.1 {
        if i == tup.1 {
            println!("{i}")
        } else {
            print!("{i} | ")
        }
    }
}
