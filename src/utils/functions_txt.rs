use crate::utils::stringzilla::*;
use crate::{lib, utils::functions_add::system_pause};
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::{
    Mutex,
    atomic::{AtomicU64, AtomicUsize, Ordering},
};
use regex::Regex;   
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
    if !ряд.par_iter().any(|i| i.as_str() == строка.as_str()) {
        ряд.push(строка.clone());
    }
}
pub fn есть_ли_повторно_строка_в_ряде(
    ряд: &Vec<String>, сообщение: &str
)->bool {
    //поиск уже добавленных слов
    (0..ряд.len()).into_par_iter().any(|i| {
        ((i + 1)..ряд.len()).into_par_iter().any(|j| {
            if ряд[i].as_str() == ряд[j].as_str()  {
                println!(
                    "слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                    ряд[i], i, j
                );
                true
            } else {false}
            
        })
    })
}

pub fn есть_ли_повторно_строка_в_ряде_regex(
    ряд: &Vec<Regex>, сообщение: &str
) ->bool{
    //поиск уже добавленных слов
    (0..ряд.len()).into_par_iter().any(|i| {
        ((i + 1)..ряд.len()).into_par_iter().any(|j| {
            if ряд[i].as_str()  == ряд[j].as_str()  {
                println!(
                    "слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                    ряд[i], i, j
                );
                true 
            } else {false}
        })
    })
}
pub fn вложена_ли_строка_в_ряд(
    ряд: &Vec<String>, строка: &String
) -> bool {
    if ряд.par_iter().any(|i| i.as_str() == строка.as_str()) {
        return true;
    }
    return false;
}
pub fn есть_ли_строка_в_куче(
    куча: &HashSet<String>, строка: &String
) -> bool {
    if куча
        .par_iter()
        .any(|строка_в_куче| строка_в_куче.as_str() == строка.as_str())
    {
        return true;
    };
    return false;
}

pub fn вложить_строку_в_ряд_с_проверкой_и_пробелом(
    ряд: &mut Vec<String>,
    строка: &String,
) {
    if !вложена_ли_строка_в_ряд(&ряд, &строка) {
        ряд.push("".to_string())
    }
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
pub fn вложить_строки_ряд_в_ряд(
    ряд_1: &mut Vec<String>, ряд_2: &Vec<String>
) {
    let ряд_1_mutex = Mutex::new(ряд_1);
    //перебор вспомогательного вектора
    ряд_2.par_iter().for_each(|строка_искомая| {
        let mut guard = ряд_1_mutex.lock().unwrap();
        if !guard.iter().any(|j| j == строка_искомая) {
            guard.push(строка_искомая.clone());
        }
    });
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
