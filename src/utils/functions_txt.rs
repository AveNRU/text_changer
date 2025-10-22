use crate::utils::stringzilla::*;
use crate::{lib, utils::functions_add::system_pause};
use foldhash::{HashMap, HashMapExt, HashSet};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::str::FromStr;
use std::sync::{
    Mutex,
    atomic::{AtomicU64, AtomicUsize, Ordering},
};

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
    ряд: &Vec<String>,
    сообщение: &str,
    условие_вложенности:bool,
)  {
    //поиск уже добавленных слов
    (0..ряд.len()).into_par_iter().for_each(|i| {
        ((i + 1)..ряд.len()).into_par_iter().for_each(|j| {
            if ряд[i].as_str() == ряд[j].as_str() {
                println!(
                    "Повторы: слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                    ряд[i], i, j
                );}
            if условие_вложенности {
                if sz_найти(&ряд[j] ,&ряд[i]) {
                    println!(
                        "Пересечения: слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                        ряд[i], i, j
                    );}
                }
            }
               
        );
    })
}

pub fn есть_ли_повторно_строка_в_ряде_regex(
    ряд: &Vec<Regex>,
    сообщение: &str,
) -> bool {
    //поиск уже добавленных слов
    (0..ряд.len()).into_par_iter().any(|i| {
        ((i + 1)..ряд.len()).into_par_iter().any(|j| {
            if ряд[i].as_str() == ряд[j].as_str() {
                println!(
                    "слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                    ряд[i], i, j
                );
                true
            } else {
                false
            }
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

pub fn сравнение_двух_рядов_построчно(
    ряд_1: &Vec<String>,
    ряд_2: &Vec<String>,
    путь: &String,
) -> bool {
    //если количество строк не равно
    if ряд_1.len() != ряд_2.len() {
        return false;
    }
    let mut счётчик_совпадений = AtomicUsize::new(0);
    //перебор вспомогательного вектора
    ряд_1
        .par_iter()
        .enumerate()
        .for_each(|(указатель, строка_искомая)| {
            if ряд_1[указатель].as_str() == ряд_2[указатель].as_str() {
                счётчик_совпадений.fetch_add(1, Ordering::Relaxed);
            }
        });
    if счётчик_совпадений.load(Ordering::Relaxed) == ряд_1.len() {
        return true;
    } else {
        return false;
    }
}
