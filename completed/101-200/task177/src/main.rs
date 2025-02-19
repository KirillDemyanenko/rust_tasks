/*
    https://www.codewars.com/kata/55908aad6620c066bc00002a/train/rust
 */

fn xo(string: &'static str) -> bool {
    let s = string.to_lowercase();
    s.matches("x").count() == s.matches("o").count()
}

fn main() {
    println!("{}", xo("zzoo"));
}
