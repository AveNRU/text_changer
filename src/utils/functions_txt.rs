//use crate::utils::functions_add::system_pause;
use crate::utils::functions::вложить_строку_в_ряд_с_проверкой;
use crate::utils::stringzilla::*;
use Text_Changer::Раздел_Словаря;
use convert_case::{Case, Casing};
//use foldhash::{HashMap, HashMapExt, rapidhash::fast::RapidHashSet, rapidhash::fast::RapidHashSetExt};
use rayon::prelude::*;
use regex::Regex;
//use std::sync::LazyLock;
//use std::str::FromStr;
use std::sync::{
    //Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
//вывод сообщения на экран и вложение его в ряд строк
pub fn вывод_сообщения_на_экран_и_вложение_в_ряд(
    строка: String,
    mut ряд_сообщений: &mut Vec<String>,
    добавить_разрыв: bool,
) {
    println!("{}", строка);
    //еслт нужен доп перенос
    if добавить_разрыв {
        println!();
    }
    вложить_строку_в_ряд_с_проверкой(&mut ряд_сообщений, &строка)
}
/*pub fn вложить_строку_в_ряд_с_проверкой(
    ряд: &mut Vec<String>,
    строка: &String,
) {
    if !ряд.par_iter().any(|i| i.as_str() == строка.as_str()) {
        ряд.push(строка.clone());
    }
}*/
pub fn есть_ли_повторно_строка_в_ряде(
    ряд: &[&str],
    сообщение: &str,
    вид_раздела: Text_Changer::Раздел_Словаря,
    //условие_вложенности: bool,
) {
    let условие_понижения_букв: bool = match вид_раздела {
        Раздел_Словаря::Простые => true,
        Раздел_Словаря::Составные => true,
        Раздел_Словаря::Составные_важные => true,
        Раздел_Словаря::Огласовки => true,
        _ => false,
    };
    //куча для всех повторов - куда все слова вносятся
    let куча_повторов:rapidhash::fast::RapidHashSet<String> =
    //поиск уже добавленных слов
    (0..ряд.len()).into_par_iter().flat_map(|i| {
        //если само себя нашло - то далее
        ((0..ряд.len())).into_par_iter().filter(move |j|*j!=i).filter_map(move|j| {//
            //сравнение
            if ряд[i] == ряд[j] {
                let слово_для_вывода:String=if условие_понижения_букв{ряд[i].to_case(Case::Lower)} else {ряд[i].to_string()};
                Some(
                format!(
                    "Повторы: слово в словаре: |{}| {сообщение}",
                    слово_для_вывода,// i, j
                ))
            } else {None}
            //вложенность
        })
     //   куча_1.into_iter().collect::<rapidhash::fast::RapidHashSet<String>>()
    }).collect();
    //вывод всех повторов слов
    if куча_повторов.len() > 0 {
        println!("\r\n🌐Обнаружены повторы слов в |{сообщение}|");
        for образец in куча_повторов.iter() {
            println!("{}", образец);
        }
    } else {
        // println!("🔥Повторы слов 'в |{сообщение}| не обнаружены")
    }
}

pub fn есть_ли_повторно_строка_в_срезе_строк(
    ряд: &[&str],
    сообщение: &str,
    //условие_вложенности: bool,
    вид_раздела: Раздел_Словаря,
) {
    let условие_понижения_букв: bool = match вид_раздела {
        Раздел_Словаря::Простые => true,
        Раздел_Словаря::Составные => true,
        Раздел_Словаря::Составные_важные => true,
        Раздел_Словаря::Огласовки => true,
        _ => false,
    };
    //куча для всех повторов - куда все слова вносятся
    let куча_повторов:rapidhash::fast::RapidHashSet<String> =
        //поиск уже добавленных слов
        (0..ряд.len()).into_par_iter().flat_map(|i| {
            //если само себя нашло - то далее
            ((0..ряд.len())).into_par_iter().filter(move |j|*j!=i).filter_map(move|j| {//
                //сравнение
                if ряд[i] == ряд[j] {
                    let слово_для_вывода:String=if условие_понижения_букв{ряд[i].to_case(Case::Lower)} else {ряд[i].to_string()};
                    Some(
                        format!(
                            "Повторы: образец в срезе строк: |{}| {сообщение}",
                            слово_для_вывода,// i, j
                        ))
                } else {None}
                //вложенность
            })
            //   куча_1.into_iter().collect::<rapidhash::fast::RapidHashSet<String>>()
        }).collect();
    //вывод всех повторов слов
    if куча_повторов.len() > 0 {
        println!("\r\n🌍Обнаружены повторы слов в срезе строк: {сообщение}");
        for образец in куча_повторов.iter() {
            println!("{}", образец);
        }
    } else {
        //println!("🔥Повторы слов 'в |{сообщение}| не обнаружены")
    }
}

pub fn есть_ли_повторно_знак_в_ряде_строк(
    ряд: &[char],
    сообщение: &str,
    //условие_вложенности: bool,
) {
    //куча для всех повторов - куда все слова вносятся
    let куча_повторов:rapidhash::fast::RapidHashSet<String> =
        //поиск уже добавленных слов
        (0..ряд.len()).into_par_iter().flat_map(|i| {
            //если само себя нашло - то далее
            ((0..ряд.len())).into_par_iter().filter(move |j|*j!=i).filter_map(move|j| {//
                //сравнение
                if ряд[i] == ряд[j] {
                    Some(
                        format!(
                            "Повторы: знак в ряде: |{}| {сообщение}",
                            ряд[i],// i, j
                        ))
                } else {None}
                //вложенность
            })
            //   куча_1.into_iter().collect::<rapidhash::fast::RapidHashSet<String>>()
        }).collect();
    //вывод всех повторов слов
    if куча_повторов.len() > 0 {
        println!("\r\n🌐🌍Обнаружены повторы знака в |{сообщение}|");
        for образец in куча_повторов.iter() {
            println!("{}", образец);
        }
    } else {
        //  println!("🔥Повторы знаков 'в |{сообщение}| не обнаружены")
    }
}

pub fn есть_ли_повторно_слова_в_ряде_с_regex(
    ряд: &[&str],
    ряд_regex: &Vec<Regex>,
    сообщение: &str,
    условие_вложенности: bool,
    раздел_словаря: Раздел_Словаря,
) {
    let условие_понижения_букв: bool = match раздел_словаря {
        Раздел_Словаря::Простые => true,
        Раздел_Словаря::Составные => true,
        Раздел_Словаря::Составные_важные => true,
        Раздел_Словаря::Огласовки => true,
        _ => false,
    };
    //слова без првоерки regex
    есть_ли_повторно_строка_в_ряде(&ряд, &сообщение, раздел_словаря);
    //при условии вложенности - проверка с regex
    if условие_вложенности {
        //повторы при вложенности
        let куча_повторов: rapidhash::fast::RapidHashSet<String> =
            //поиск уже добавленных слов
            (0..ряд.len()).into_par_iter().flat_map(|i| {
                //если само себя нашло - то далее
                ((0..ряд.len())).into_par_iter().filter(move |j| *j != i).filter_map(move |j| { //
                    if условие_вложенности {
                        //если есть тире - не учитывать, разные случаи так как бывают
                        if !sz_найти_в_str(ряд[j], "-") && sz_найти_в_str(&ряд[j], &ряд[i]) {
                            //let строка = format!(r#"\<({})\>"#, ряд[i]);
                            let образец_re = &ряд_regex[i];
                            //let образец_re: LazyLock<Regex> = LazyLock::new(|| Regex::new(&строка).unwrap();
                            if образец_re.is_match(&ряд[j]) {
                                let слово_для_вывода:String=if условие_понижения_букв{ряд[i].to_case(Case::Lower)} else {ряд[i].to_string()};
                                Some(
                                    format!(
                                        "Пересечения - Regex: слово в словаре: |{}| {сообщение}",
                                        слово_для_вывода, // i, j
                                    ))
                            } else { None }
                        } else { None }
                    } else { None }
                })
            }).collect::<rapidhash::fast::RapidHashSet<String>>();
        //вывод всех повторов слов
        if куча_повторов.len() > 0 {
            println!("\r\n🌐Обнаружены повторы - пересечения (вложенность) слов 'в |{сообщение}|");
            for образец in куча_повторов.iter() {
                println!("{}", образец);
            }
        } else {
            //  println!("🔥Повторы слов (вложенность) пересечения 'в |{сообщение}| не обнаружены")
        }
    }
}
/*
pub fn есть_ли_повторно_строка_в_ряде_с_удалением(
    ряд: &Vec<Ячейка_словаря>,
    сообщение: &str,
    условие_вложенности: bool,
) -> Vec<Ячейка_словаря> {
    //поиск уже добавленных слов
    //поиск уже добавленных слов
    let mut куча: Arc<Mutex<rapidhash::fast::RapidHashSet<String>>> = Arc::new(Mutex::new(rapidhash::fast::RapidHashSet::default()));
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
    //раздел_словаря: Раздел_Словаря,
) -> bool {
    /*let условие_понижения_букв:bool= match вид_раздела {
        Раздел_Словаря::простые=>true,
        Раздел_Словаря::составные=>true,
        Раздел_Словаря::составные_важные=>true,
        Раздел_Словаря::огласовки=>true,
        _=>false,
    };*/
    let ряд = ряд.as_ref();
    //поиск уже добавленных слов
    (0..ряд.len()).into_par_iter().any(|i| {
        (0..ряд.len()).into_par_iter().filter(|j| *j != i).any(|j| {
            if ряд[i].as_str() == ряд[j].as_str() {
                //let слово_для_вывода:String=if условие_понижения_букв{ряд[i].to_case(Case::Lower)} else {ряд[i].to_string()};
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
    куча: &rapidhash::fast::RapidHashSet<String>,
    строка: &String,
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
    if ряд.par_iter().any(|n| n.as_str() == строка.as_str()) {
        return true;
    }
    return false;
}
pub fn ряд_в_строку(ряд: &Vec<String>, _ошибка: &str) -> String {
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
    let множество: rapidhash::fast::RapidHashSet<&String> = ряд_1.iter().collect();
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
    _путь: &String,
) -> bool {
    //если количество строк не равно
    /*if ряд_1.len() != ряд_2.len() {
        return false;
    }*/
    //
    let куча_1: rapidhash::fast::RapidHashSet<&str> =
        ряд_1.iter().map(|строка| строка.as_str()).collect();
    let куча_2: rapidhash::fast::RapidHashSet<&str> =
        ряд_2.iter().map(|строка| строка.as_str()).collect();
    //
    //сравнение двух куч - равны они или нет
    if куча_1 == куча_2 {
        return true;
    } else {
        false
    }
    /*if куча_1.len() != куча_2.len() {
        return false;
    } else {
        if
        return true;
    }*/
}
pub fn сравнение_двух_рядов_построчно_срез_строк(
    ряд_1: &[&str],
    ряд_2: &[String],
    _путь: &String,
) -> bool {
    //если количество строк не равно
    /*if ряд_1.len() != ряд_2.len() {
        return false;
    }*/
    //
    let куча_1: rapidhash::fast::RapidHashSet<&str> = ряд_1.iter().map(|строка| *строка).collect();
    let куча_2: rapidhash::fast::RapidHashSet<&str> =
        ряд_2.iter().map(|строка| строка.as_str()).collect();
    //
    //
    if куча_1 == куча_2 {
        return true;
    } else {
        false
    }
}

pub fn сравнение_двух_рядов_побайтово(
    ряд_1: &[u8],
    ряд_2: &[u8],
    _путь: &String,
) -> bool {
    //если количество строк не равно
    if ряд_1.len() != ряд_2.len() {
        return false;
    }
    //
    let куча_1: rapidhash::fast::RapidHashSet<&u8> = ряд_1.iter().map(|строка| строка).collect();
    let куча_2: rapidhash::fast::RapidHashSet<&u8> = ряд_2.iter().map(|строка| строка).collect();
    //
    if куча_1 == куча_2 {
        return true;
    } else {
        false
    }
    /* let счётчик_совпадений = AtomicUsize::new(0);
    //перебор вспомогательного вектора
    ряд_1
        .par_iter()
        .enumerate()
        .for_each(|(указатель, _строка_искомая)| {
            if ряд_1[указатель] == ряд_2[указатель] {
                счётчик_совпадений.fetch_add(1, Ordering::Relaxed);
            }
        });
    if счётчик_совпадений.load(Ordering::Relaxed) == ряд_1.len() {
        return true;
    } else {
        return false;
    }*/
}
