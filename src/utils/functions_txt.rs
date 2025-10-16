use crate::utils::stringzilla::*;
use crate::{lib, utils::functions_add::system_pause};
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::str::FromStr;
use rayon::prelude::*;
//вывод сообщения на экран и вложение его в ряд строк
pub fn вывод_сообщения_на_экран_и_вложение_в_ряд(
    строка: String,
    mut ряд_сообщений: &mut Vec<String>,
) {
    println!("{}", строка);
    вложить_строку_в_ряд_с_проверкой(&mut ряд_сообщений, &строка)
}
pub fn вложить_строку_в_ряд_с_проверкой(
    ряд: &mut Vec<String>,
    строка: &String,
) {
    if ряд.par_iter().any(|i|i.as_str()==строка.as_str()) {
    //if !ряд.iter().any(|n| n.as_str() == строка.as_str()) {
        ряд.push(строка.clone());
    }
}
pub fn есть_ли_повторно_строка_в_ряде(
    ряд: &Vec<String>, сообщение: &str,
)  {
    //поиск уже добавленных слов
    (0..ряд.len())
        .into_par_iter()
        .for_each(|i| {
            ((i + 1)..ряд.len())
                .into_par_iter()
                .for_each(|j| {
                    if ряд[i] == ряд[j] {
                        println!(
                            "слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                             ряд[i],i, j
                        );
                    }
                });
        });
    }
pub fn вложена_ли_строка_в_ряд(
    ряд: &Vec<String>, строка: &String
) -> bool {
    if ряд.par_iter().any(|i|i.as_str()==строка.as_str()) {
        return true;
    }
    return false;
}
pub fn есть_ли_строка_в_куче(
    куча: &HashSet<String>, строка: &String
) -> bool {
    for содержимое in куча.iter() {
        if содержимое.as_str() == строка.as_str() {
            return true;
        }
    }
    return false;
}

pub fn вложить_строку_в_ряд_с_проверкой_и_пробелом(
    ряд: &mut Vec<String>,
    строка: &String,
) {
    if !ряд.iter().any(|n| n.as_str() == строка.as_str()) {
        ряд.push(строка.clone());
    }
    ряд.push("".to_string())
}
pub fn содержит_ли_ряд_строку(
    ряд: &Vec<String>, строка: &String
) -> bool {
    if ряд.iter().any(|n| n.as_str() == строка.as_str()) {
        return true;
    }
    return false;
}
pub fn ряд_в_строку(ряд: &Vec<String>, ошибка: &str) -> String {
    let mut итог: String = String::new();
    for i in 0..ряд.len() {
        итог = format!("{}|{}|", итог, ряд[i]);
    }
    return итог;
}

pub fn выводы_питания_в_строку(
    выводы_питания: &Vec<String>
) -> String {
    let mut строка: String = String::new();
    if выводы_питания.len() == 1 {
        return выводы_питания[0].clone().to_string();
    }
    for i in 0..выводы_питания.len() {
        //if i!=power_pins.len()-1 {
        строка = format!("{строка}|{}", выводы_питания[i].clone());
        //}
    }

    return строка;
}
//все ли цепи являются цепями земли
pub fn являются_ли_все_цепи_цепями_земли_или_нет(
    цепи_земли: &Vec<String>,
    цепи_на_проверку: &Vec<String>,
) -> bool {
    for i in 0..цепи_на_проверку.len() {
        //если в цепях земли содержится цепь, переданная на проверку, то возвращать истину - иначе ложь
        if !цепи_земли
            .iter()
            .any(|n| n.as_str() == цепи_на_проверку[i].as_str())
        {
            return false;
        }
    }
    return true;
}

//вложение одного вектора в основной, если в нём данная строка отсутствует
pub fn вложить_строки_ряд_в_ряд(vec_1: &mut Vec<String>, vec_2: &Vec<String>) {
    //перебор вспомогательного вектора
    for i in 0..vec_2.len() {
        //если в 1 векторе нет строки из второго вектора
        let _строка = vec_2[i].clone();
        if !vec_1.iter().any(|n| n.as_str() == _строка.as_str()) {
            // println!("не нашло: {}",&vec_2[i]);
            //вложение в 1 вектор строки из второго
            vec_1.push(vec_2[i].clone());
        }
    }
}
pub fn получить_напряжение_по_указателю_на_этаж(
    указатель: usize,
) -> f32 {
    return match указатель {
        0 => return 0.6,
        1 => return 0.675,
        2 => return 0.7,
        3 => return 0.75,
        4 => return 0.8,
        5 => return 0.9,
        6 => return 1.0,
        7 => return 1.024,
        8 => return 1.05,
        9 => return 1.1,
        10 => return 1.2,
        11 => return 1.238,
        12 => return 1.25,
        13 => return 1.35,
        14 => return 1.5,
        15 => return 1.8,
        16 => return 1.85,
        17 => return 1.9,
        18 => return 2.5,
        19 => return 3.3,
        20 => return 3.4,
        21 => return 5.0,
        22 => return 8.2,
        23 => return 12.0,
        24 => return 19.0,
        25 => return 24.0,
        26 => return 27.0,
        //случай когда напряжение не соответствует образцам
        27 => return 99.0,
        99 => return -199.0,
        _ => {
            println!("Не верный указатель этажа:{указатель}, нет такого напряжения в ядре",);
            system_pause();
            panic!();
        }
    };
}
