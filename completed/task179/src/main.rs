/*
   https://www.codewars.com/kata/5648b12ce68d9daa6b000099/train/rust
*/

fn number(bus_stops: &[(i32, i32)]) -> i32 {
    let mut people_still_on_the_bus = 0;
    for bus_stop in bus_stops {
        people_still_on_the_bus += bus_stop.0 - bus_stop.1;
    }
    people_still_on_the_bus
}

fn main() {
    println!("{}", number(&[(10, 0), (3, 5), (5, 8)]));
}
