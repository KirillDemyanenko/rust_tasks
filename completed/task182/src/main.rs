fn series_sum(n: u32) -> String {
    let mut sum = 0.0;
    for i in 1..=n {
        sum += 1.0 / (1.0 + (i as f64 - 1.0) * 3.0)
    }
    format!("{:.2}", sum)
}

fn main() {
    println!("{}", series_sum(1));
    println!("{}", series_sum(2));
    println!("{}", series_sum(3));
    println!("{}", series_sum(4));
    println!("{}", series_sum(7));
    println!("{}", series_sum(39));
    println!("{}", series_sum(0));
}
