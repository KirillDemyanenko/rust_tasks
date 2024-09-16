fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let city = String::from(buffer.trim());
    buffer.clear();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let num1: i32 = buffer.trim().parse().expect("Parse error!");
    buffer.clear();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    let num2: i32 = buffer.trim().parse().expect("Parse error!");
    if num1 % 2 == (if city == "Четный" { 0 } else { 1 }) {
        println!("{num1} в город {city} вход разрешен");
    } else {
        println!("{num1} в город {city} вход запрещен");
    }
    if num2 % 2 == (if city == "Четный" { 0 } else { 1 }) {
        println!("{num2} в город {city} вход разрешен");
    } else {
        println!("{num2} в город {city} вход запрещен");
    }
}
