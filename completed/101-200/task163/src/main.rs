fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let user_input = buf.trim();
    println!("{:p}", &user_input);
}
