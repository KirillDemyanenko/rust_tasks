/*
    Напишите программу, которая считывает возраст (u8) и два целых (u8) числа
    являющиеся показателями медицинского прибора для измерения артериального
    давления (АД):

    - SYS - показывает систолическое (верхнее) давление.
    - DIA - показывает диастолическое (нижнее) давление.
    Программа должна вывести следующее:

    Если показатель(и) в норме, то напечатать:
    Систолическое АД в норме
    или
    Диастолическое АД в норме
    или
    Систолическое и Диастолическое АД в норме.
    В противном случае вывести на сколько завышен или занижен каждый показатель:
    Систолическое АД {} ниже нормы на {} или Систолическое АД {} выше нормы на {}
    Диастолическое АД {} ниже нормы на {} или Диастолическое АД {} выше нормы на {}
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
    return buffer.trim().parse::<T>().expect("Parse error");
}

fn main() {
    let age: u8 = input::<u8>();
    let sys: u8 = input::<u8>();
    let dia: u8 = input::<u8>();
    if age < 40 {
        if (sys < 140 && sys > 89) && (dia > 59 && dia < 90) {
            println!("Систолическое и Диастолическое АД в норме");
        } else {
            if sys > 139 || sys < 89 {
                if sys > 139 {
                    println!("Систолическое АД {sys} выше нормы на {}", sys - 139)
                } else {
                    println!("Систолическое АД {sys} ниже нормы на {}", 90 - sys)
                }
            } else {
                println!("Систолическое АД в норме")
            }
            if dia < 60 || dia > 89 {
                if dia > 89 {
                    println!("Диастолическое АД {dia} выше нормы на {}", dia - 89)
                } else {
                    println!("Диастолическое АД {dia} ниже нормы на {}", 60 - dia)
                }
            } else {
                println!("Диастолическое АД в норме")
            }
        }
    } else if age < 60 {
        if (sys < 150 && sys > 90) && (dia > 60 && dia < 92) {
            println!("Систолическое и Диастолическое АД в норме");
        } else {
            if sys > 149 || sys < 91 {
                if sys > 140 {
                    println!("Систолическое АД {sys} выше нормы на {}", sys - 149)
                } else {
                    println!("Систолическое АД {sys} ниже нормы на {}", 91 - sys)
                }
            } else {
                println!("Систолическое АД в норме")
            }
            if dia < 61 || dia > 91 {
                if dia > 90 {
                    println!("Диастолическое АД {dia} выше нормы на {}", dia - 91)
                } else {
                    println!("Диастолическое АД {dia} ниже нормы на {}", 61 - dia)
                }
            } else {
                println!("Диастолическое АД в норме")
            }
        }
    } else {
        if (sys < 160 && sys > 90) && (dia > 60 && dia < 92) {
            println!("Систолическое и Диастолическое АД в норме");
        } else {
            if sys > 159 || sys < 91 {
                if sys > 159 {
                    println!("Систолическое АД {sys} выше нормы на {}", sys - 159)
                } else {
                    println!("Систолическое АД {sys} ниже нормы на {}", 91 - sys)
                }
            } else {
                println!("Систолическое АД в норме")
            }
            if dia < 61 || dia > 91 {
                if dia > 91 {
                    println!("Диастолическое АД {dia} выше нормы на {}", dia - 91)
                } else {
                    println!("Диастолическое АД {dia} ниже нормы на {}", 61 - dia)
                }
            } else {
                println!("Диастолическое АД в норме")
            }
        }
    }
}
