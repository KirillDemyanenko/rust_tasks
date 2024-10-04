/*
    В мороз сразу после запуска холодного двигателя нужно отводить как можно меньше тепла, чтобы
    мотор быстрее разогрелся до оптимальной температуры. Затем резко увеличить теплоотдачу, чтобы
    двигатель не перегрелся. Для этого в системе охлаждения установлен специальный клапан -
    термостат. Он отвечает за то, как именно циркулирует в системе охлаждающая жидкость.

    Пока температура ниже 80-90 °С, термостат полностью закрыт и направляет поток жидкости не
    через радиатор, а через байпас по малому кругу охлаждения. Теплоотдача системы без радиатора
    мала, и жидкость возвращается в двигатель без охлаждения. По мере роста температуры термостат
    открывается и направляет все большее количество жидкости через большой круг охлаждения через
    радиатор.

    Термостат может быть механическим, а может управляться электроникой. Механический термостат
    работает автономно, открывается при нагреве с помощью термочувствительного элемента. Электронный
    термостат закрывается и открывается по команде блока управления двигателем.

    Напишите программу, которая считывает:

    Строку останова (String);
    "Бесконечно" вводимую температуру двигателя в °С (i32).
    Программа должна выводить Термостат закрыт, если температура ниже нормы и Термостат открыт,
    если температура выше нормы.
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
    let stop = input::<String>();
    let mut command = input::<String>();
    while stop != command {
        let temp = command.parse::<i32>().unwrap_or(0);
        if temp > 90 {println!("Термостат открыт")} else { println!("Термостат закрыт") }
        command = input::<String>();
    }
}