/*
    https://stepik.org/lesson/1213901/step/12?unit=1227167
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
    for [group, right] in [["User", &*input::<String>()], ["Group", &*input::<String>()], ["Other", &*input::<String>()]] {
        println!("{group}{}", if right == "0" {" (no access)."} else if right == "7" { " (full access):"} else { ":" });
        match right { 
            "1" => println!("    - execute only"),
            "2" => println!("    - write access only"),
            "3" => println!("    - write\n    - execute"),
            "4" => println!("    - read only"),
            "5" => println!("    - read\n    - execute"),
            "6" => println!("    - read\n    - write"),
            "7" => println!("    - read\n    - write\n    - execute"),
            &_ => print!("{}", "")
        }
    }
}
