/*
    Выведите n-е натуральное число, делящееся ровно на два из трёх чисел a,b,c.
    Если подходящего числа не существует или оно превышает 10_18, выведите − 1.
*/

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let numbers: Vec<u128> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut n: u128 = input.trim().parse().expect("Parse error");
    for i in 1..10_u128.pow(18) {
        let res1 = i % numbers[0] == 0;
        let res2 = i % numbers[1] == 0;
        let res3 = i % numbers[2] == 0;
        if ( res1 && res2 && !res3)
            || (res2 && res3 && !res1)
            || (res1 && res3 && !res2)
        {
            if n - 1 == 0_u128 {
                println!("{}", i);
                break;
            } else {
                n -= 1;
            }

        }
    }
}