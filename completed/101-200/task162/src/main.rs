/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать натуральное число (u32) и вывести все его делители, не считая 1 и самого числа:

   Считывание реализуйте в функции get_input() с возвратом считанного значения!
*/

fn main() {
    let n: u32 = get_input().trim().parse().unwrap();
    let mut res: Vec<u32> = Vec::new();
    for i in 2..n {
        if n % i == 0 {
            res.push(i);
        }
    }
    println!(
        "{}",
        res.into_iter()
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn get_input() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
