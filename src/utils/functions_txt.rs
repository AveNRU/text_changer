use crate::lib::Ячейка_словаря;
use crate::utils::stringzilla::*;
use crate::{lib, utils::functions_add::system_pause};
use foldhash::{HashMap, HashMapExt, HashSet};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::str::FromStr;
use std::sync::{
    Arc, Mutex,
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
    ряд: &[String],
    сообщение: &str,
    условие_вложенности: bool,
) {
    //поиск уже добавленных слов
    (0..ряд.len()).into_par_iter().for_each(|i| {
        ((0..ряд.len())).into_par_iter().filter(|j|*j!=i).for_each(|j| {
            //если само себя нашло - то далее
          //  if j==i {return}
            //сравнение
            if ряд[i].as_str() == ряд[j].as_str() {
                println!(
                    "Повторы: слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                    ряд[i], i, j
                );
            }
            if условие_вложенности {
                //если есть тире - не учитывать, разные случаи так как бывают
                if !sz_найти(&ряд[j], "-") && sz_найти(&ряд[j], &ряд[i]) {
                    let строка = format!(r#"\<({})\>"#, ряд[i]);
                    let образец_re: Regex = Regex::new(&строка).unwrap();
                    if образец_re.is_match(&ряд[j]) {
                        println!(
                            "Пересечения - Regex: слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                            ряд[i], i, j
                        );
                    } else {
                        // if sz_найти(&ряд[j], &ряд[i]){
                       /*println!(
                            "Пересечения - без Regex: слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                            ряд[i], i, j
                        );*/

                    }
                }
            }
        });
    })
}
/*
pub fn есть_ли_повторно_строка_в_ряде_с_удалением(
    ряд: &Vec<Ячейка_словаря>,
    сообщение: &str,
    условие_вложенности: bool,
) -> Vec<Ячейка_словаря> {
    //поиск уже добавленных слов
    //поиск уже добавленных слов
    let mut куча: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::default()));
    let ряд_два:Vec<Ячейка_словаря>=
    (0..ряд.len()).into_par_iter().flat_map(|i| {

       //вложение слова в кучу
        куча.lock().unwrap().insert(ряд[i].искомое_слово.clone());
    //перебор внутри стопки на предмет наличия совпадений
        ((i + 1)..ряд.len()).into_par_iter().for_each(move|j| {
            if ряд[i].искомое_слово.as_str() == ряд[j].искомое_слово.as_str() {
                println!(
                    "Повторы: слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                    ряд[i].искомое_слово, i, j
                );
            }
            if условие_вложенности {
                //если есть тире - не учитывать, разные случаи так как бывают
                if !sz_найти(&ряд[j].искомое_слово, "-") && sz_найти(&ряд[j].искомое_слово, &ряд[i].искомое_слово) {
                    println!(
                        "Пересечения: слово в словаре: |{}| {сообщение}. Номер строки 1){}, 2){}",
                        ряд[i].искомое_слово, i, j
                    );
                }
            }

        });
        if !куча.lock().unwrap().contains(&ряд[i].искомое_слово) {
            Some(ряд[i].clone())
        } else {
            None
        }

    }).collect();
    return ряд_два;
}
*/
pub fn есть_ли_повторно_строка_в_ряде_regex(
    ряд: impl AsRef<[Regex]>,
    сообщение: &str,
) -> bool {
    let ряд = ряд.as_ref();
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
    ряд: &[String], строка: &String
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
    let множество: HashSet<&String> = ряд_1.iter().collect();
    let уникальные: Vec<String> = ряд_2
        .iter()
        .filter(|с| !множество.contains(с))
        .cloned()
        .collect();
    ряд_1.extend(уникальные);
}

pub fn сравнение_двух_рядов_построчно(
    ряд_1: &[String],
    ряд_2: &[String],
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

pub fn сравнение_двух_рядов_побайтово(
    ряд_1: &[u8],
    ряд_2: &[u8],
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
            if ряд_1[указатель] == ряд_2[указатель] {
                счётчик_совпадений.fetch_add(1, Ordering::Relaxed);
            }
        });
    if счётчик_совпадений.load(Ordering::Relaxed) == ряд_1.len() {
        return true;
    } else {
        return false;
    }
}
