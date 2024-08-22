/*
Напишите программу, которая считывает три (u16) числа: количество этажей в доме,
количество квартир на каждом этаже и номер квартиры. Затем программа определяет,
в каком подъезде и на каком этаже этого дома находится квартира с заданным
номером. Предполагается, что в доме нет строительных особенностей, таких как
пропуски этажей или квартир, а также что нумерация квартир начинается с 1.

Гарантируется, что номер квартиры не превышает общее количество квартир в доме.
*/

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let number_of_floors: u16 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!"); 
    let apartments_on_floor: u16 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).expect("Input error!");
    let apartment_number: u16 = user_input
        .trim()
        .parse()
        .expect("Parse error!");
    println!(
        "Квартира с номером {} находится в подъезде {} на {} этаже",
        apartment_number,
        (apartment_number - 1) / (number_of_floors * apartments_on_floor) + 1,
        ((apartment_number - 1) / apartments_on_floor) % number_of_floors + 1
    )
}
