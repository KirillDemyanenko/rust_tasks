/*
    Напишите программу, где будет определена 5 уровневая ссылка на значение 5 и выведите адреса, по
    которым необходимо проследовать, чтобы получить итоговое значение. Вывод оформить в виде
    сообщений:
 */
fn main() {
    let ref_num: &&&&&&u8 = &&&&&&5;
    println!("{:p} -> {:p}", *****ref_num, ****ref_num);
    println!("{:p} -> {:p}", ****ref_num, ***ref_num);
    println!("{:p} -> {:p}", ***ref_num, **ref_num);
    println!("{:p} -> {:p}", **ref_num, *ref_num);
    println!("{:p} -> {}", *ref_num, ref_num);
}