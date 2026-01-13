use crate::lib;
use crate::utils::functions::строка_удалить_utf8_концы_строк;
use crate::utils::functions_add::system_pause;
use crate::utils::stringzilla::sz_найти;
use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Cursor, Read};
use std::sync::{
    Mutex,
    atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
};
use walkdir::WalkDir;

//чтение файла в UTF-8
pub fn read_utf8(путь_до_файла: &String) -> Vec<String> {
    let mut итог: Vec<String> = Vec::new(); //вектор строк - куда все помещается
    let содержимое: Box<dyn BufRead> = считать_файл(путь_до_файла); //чтение файла
    for (указатель, содержимое_в_байтах) in содержимое.split(b'\n').enumerate()
    {
        //перебор всех строк и переход на новые строки
        let указатель_строки: usize = указатель + 1;
        let строка_в_utf8: String =
            получить_строку_в_utf8(содержимое_в_байтах, указатель_строки); //сохранение строки как UTF-8
        итог.push(строка_в_utf8.to_string()) //добавление строки в вектор
    }
    let итог = удалить_shy_из_вектора(&итог);
    return итог;
}

pub fn read_utf8_без_переносов_строк_htm_не_архив(
    путь_до_файла: &String,
) -> Vec<String> {
    //println!("вхождение: read_utf8_без_переносов_строк_htm");
    let mut итог: Vec<String> = Vec::new(); //вектор строк - куда все помещается
    let содержимое: Box<dyn BufRead> = считать_файл(путь_до_файла); //чтение файла
    let mut строка_общая: String = String::new();
    let mut условие_p: bool = false;
    let mut условие_title: bool = false;
    for (указатель, содержимое_в_байтах) in содержимое.split(b'\n').enumerate()
    {
        //перебор всех строк и переход на новые строки
        let указатель_строки: usize = указатель + 1;
        let mut строка_в_utf8: String =
            получить_строку_в_utf8(содержимое_в_байтах, указатель_строки); //сохранение строки как UTF-8
        //итог.push(строка_в_utf8);

        //если нет закрытий
        //пустую строку сразу вкладываем
        //  if строка_в_utf8.is_empty() {
        //     итог.push(строка_в_utf8.to_string()) //добавление строки в вектор
        // };
        //если это статья в altium, habr - удалить рекламу
        /*if есть_ли_реклама_пропуск_строки(&строка_в_utf8) {
            итог.push(format!(r#"<!--{}"#,строка_в_utf8));
            continue;
        }*/
        if отсутствие_закрытого_p_тэга_html_в_строке(&строка_в_utf8)
        {
            // строка_общая=format!("{}{}",строка_общая,строка_в_utf8.trim_end().to_string());
            строка_общая = format!("{}{} ", строка_общая, строка_в_utf8);
            условие_p = true;
            continue;
        }
        if условие_p {
            // строка_общая=format!("{}{}",строка_общая,строка_в_utf8.trim_end().to_string());
            строка_общая = format!("{}{} ", строка_общая, строка_в_utf8);
            if !sz_найти(&строка_в_utf8, "</p>") {
                continue;
            } else {
                условие_p = false;
                итог.push(строка_общая.to_string()); //добавление строки в вектор
                строка_общая = String::new();
                continue;
            }
        }

        if отсутствие_закрытого_title_тэга_html_в_строке(
            &строка_в_utf8,
        ) {
            //строка_общая=format!("{}{}",строка_общая,строка_в_utf8.trim_end().to_string());
            строка_общая = format!("{}{} ", строка_общая, строка_в_utf8);
            условие_title = true;
            continue;
        }
        if условие_title {
            // строка_общая=format!("{}{}",строка_общая,строка_в_utf8.trim_end().to_string());
            строка_общая = format!("{}{} ", строка_общая, строка_в_utf8);
            if !sz_найти(&строка_в_utf8, "</title>") {
                continue;
            } else {
                условие_title = false;
                итог.push(строка_общая.to_string()); //добавление строки в вектор
                строка_общая = String::new();
                continue;
            }
        }

        if !условие_p && !условие_title {
            итог.push(строка_в_utf8.to_string()) //добавление строки в вектор
        }
    }
    let итог = удалить_shy_из_вектора(&итог);
    let итог = удалить_переносы_из_вектора(итог);
    //return удалить_переносы_строк_html(итог, 0);
    let итог = добавить_переносы_строк_html(итог, 1);

    let итог: Vec<String> =
        удалить_рекламу_после_разбиения_строк(итог);
    return удаление_script_мусора_после_разбиения_строк(итог);
}

pub fn определить_кодировку(
    ряд_строк: &Vec<String>
) -> lib::Кодировка {
    // Если вы используете Rayon для параллельной обработки (into_par_iter),
    // нужно убедиться, что он подключен в Cargo.toml и импортирован
    use rayon::prelude::*; // Добавьте это вверху файла или здесь
    //let количество_строк: usize = usize::try_from(ряд_строк.len() as f32 *0.1).unwrap().into();
    //let mut кодировка:lib::Кодировка=lib::Кодировка::utf8;
    for указатель in 0..30 {
        if let Some(строка) = ряд_строк.get(указатель) {
            if sz_найти(&строка, r#"content="text/html; charset=windows-1251""#) {
                return lib::Кодировка::windows_1251;
            }
        }
    }
    /*
    // Находим первую подходящую кодировку
    if let Some(кодировка) = ряд_строк
        .into_par_iter() // Параллельная итерация
        .find_map_any(|строка| { // find_map_any возвращает первый найденный результат
            if sz_найти(&строка,r#"content="text/html; charset=windows-1251""#) {
                Some(lib::Кодировка::windows_1251)
            } else if sz_найти(&строка,r#"content="text/html; charset=utf-8""#) {
                Some(lib::Кодировка::utf8)
            }/* else if sz_найти(&строка,r#"charset=windows-1251"#) {
                Some(lib::Кодировка::windows_1251)
            } else if sz_найти(&строка,r#"charset=utf-8"#) {
                Some(lib::Кодировка::utf8)
            } else {
                None
            }*/
        })
    {
        return кодировка;
    }
    */
    // Если ничего не найдено, возвращаем кодировку по умолчанию
    return lib::Кодировка::utf8;
}

pub fn htm_utf8_без_переносов_строк(
    содержимое: &Vec<String>
) -> Vec<String> {
    let mut итог: Vec<String> = Vec::new(); //вектор строк - куда все помещается
    let mut строка_общая: String = String::new();
    let mut условие_p: bool = false;
    let mut условие_title: bool = false;
    let mut калибри: bool = false;
    for (указатель, строка_в_utf8) in содержимое.iter().enumerate() {
        //перебор всех строк и переход на новые строки
        let указатель_строки: usize = указатель + 1;
        //если нет закрытий
        //пустую строку сразу вкладываем
        if строка_в_utf8.is_empty() {
            итог.push(строка_в_utf8.to_string()) //добавление строки в вектор
        };
        //если это статья в altium, habr - удалить рекламу
        /*if есть_ли_реклама_пропуск_строки(&строка_в_utf8) {
            итог.push(format!(r#"<!--{}"#,строка_в_utf8));
            continue;
        }*/
        //
        if !калибри {
            if sz_найти(&строка_в_utf8, r#"class="calibre7"#) {
                калибри = true;
            }
        }

        if отсутствие_закрытого_p_тэга_html_в_строке(&строка_в_utf8)
        {
            // строка_общая=format!("{}{}",строка_общая,строка_в_utf8.trim_end().to_string());
            строка_общая = format!("{}{} ", строка_общая, строка_в_utf8);
            условие_p = true;
            continue;
        }
        if условие_p {
            // строка_общая=format!("{}{}",строка_общая,строка_в_utf8.trim_end().to_string());
            строка_общая = format!("{}{} ", строка_общая, строка_в_utf8);
            if !sz_найти(&строка_в_utf8, "</p>") {
                continue;
            } else {
                условие_p = false;
                итог.push(строка_общая.to_string()); //добавление строки в вектор
                строка_общая = String::new();
                continue;
            }
        }

        if отсутствие_закрытого_title_тэга_html_в_строке(
            &строка_в_utf8,
        ) {
            //строка_общая=format!("{}{}",строка_общая,строка_в_utf8.trim_end().to_string());
            строка_общая = format!("{}{} ", строка_общая, строка_в_utf8);
            условие_title = true;
            continue;
        }
        if условие_title {
            // строка_общая=format!("{}{}",строка_общая,строка_в_utf8.trim_end().to_string());
            строка_общая = format!("{}{} ", строка_общая, строка_в_utf8);
            if !sz_найти(&строка_в_utf8, "</title>") {
                continue;
            } else {
                условие_title = false;
                итог.push(строка_общая.to_string()); //добавление строки в вектор
                строка_общая = String::new();
                continue;
            }
        }

        if !условие_p && !условие_title {
            итог.push(строка_в_utf8.to_string()) //добавление строки в вектор
        }
    }
    // let итог = удалить_shy_из_вектора(&итог);
    //удалить лишние пробелы
    let итог = удалить_лишние_пробелы(&итог);
    if калибри {
        return удалить_переносы_калибри(&итог);
    }
    //let итог=удалить_переносы_из_вектора(итог);
    let итог: Vec<String> = добавить_переносы_строк_html(итог, 1);
    let итог: Vec<String> =
        удалить_рекламу_после_разбиения_строк(итог);
    return удаление_script_мусора_после_разбиения_строк(итог);
}
fn удалить_рекламу_после_разбиения_строк(
    ряд: Vec<String>,
) -> Vec<String> {
    //println!("количество строк: {}",ряд.len());
    let mut итог: Vec<String> = Vec::new();
    for (i, строка) in ряд.into_iter().enumerate() {
        if !есть_ли_реклама_после_разбиения_строк(&строка) {
            /* println!("не нашло объяву {}",i+1);
            println!("сама строка: {}",строка.to_string());
            println!();*/

            итог.push(строка.to_string())
        } else {
            //  let строка_без_продвижения:String=format!(r#"<!--{}"#,строка);
            let строка = строка.replace("<!--", "");
            let строка = строка.replace("[-->", "");
            let строка = строка.replace("-->", "");
            итог.push(format!(r#"<!--{}-->"#, строка))
        }
    }
    // println!("возврат рекламы");
    //удалить пустые строки
    let mut итог2: Vec<String> = Vec::new();
    for i in 0..итог.len() {
        //if итог[i].is_empty() {continue} else {итог2.push(итог[i].clone())}
        if если_пустая_строка_с_отделителями(&итог[i]) {
            continue;
        } else {
            итог2.push(итог[i].clone())
        }
    }
    //удалить скрипты

    return итог2;
}
pub fn удалить_разделы_html_по_ключевым_словам(
    ряд: Vec<String>,
    начало_строки: &String,
    конец_строки: &String,
) -> Vec<String> {
    let mut ряд_общий: Vec<String> = Vec::new();
    //условие - есть ли конец script
    let mut условие_script: bool = false;
    //сам ряд куда вкладываются строки
    let mut итог: Vec<String> = Vec::new();
    //
    //let mut условие_начала:bool=false;
    for i in 0..ряд.len() {
        //если начался script
        if условие_script {
            /*   if sz_найти(&ряд[i], &начало_строки) {
                println!("Условие: i:{}|{}|",i+1,&ряд[i]);
            }*/
            if sz_найти(&ряд[i], конец_строки) {
                //добавляем окончание
                ряд_общий.push(
                    убрать_примечания_из_строки_c_окончанием(
                        &ряд[i],
                    ),
                );
                //условие возвращается в ложь, что нет script
                условие_script = false;
                //вложение в итог
                итог.extend(ряд_общий);
                //уничтожение содержимого для нового использования
                ряд_общий = Vec::new();
                continue;
            }
            //если нет конца
            else {
                ряд_общий.push(убрать_примечания_из_строки_без_преобразований(&ряд[i]));
                continue;
            }
        } else
        //ищет начало script
        if sz_найти(&ряд[i], начало_строки) {
            //если есть закрытие script в строке
            if sz_найти(&ряд[i], конец_строки) {
                итог.push(убрать_примечания_из_строки_с_преобразованием(&ряд[i]));
                условие_script = false;
                continue;
                //итог.push(format!("{}"));
            }
            //если нет закрытия script в строке
            else {
                // println!("НЕТ КОНЦА: i:{}|{}|",i+1,&ряд[i]);
                условие_script = true;
                let строка =
                    убрать_примечания_из_строки_c_началом(&ряд[i]);
                //   println!("строка с началом без конца: {}",строка);
                ряд_общий.push(строка);
                continue;
                // итог.push(ряд[i].clone());
            }
        }
        //если обычная строка
        if !условие_script {
            итог.push(ряд[i].clone());
        }
    }
    return итог;
}

pub fn удалить_разделы_html_по_ключевым_словам_с_повторами(
    ряд: Vec<String>,
    начало_строки: &String,
    конец_строки: &String,
) -> Vec<String> {
    let mut ряд_общий: Vec<String> = Vec::new();
    //условие - есть ли конец script
    let mut условие_script: bool = false;
    //сам ряд куда вкладываются строки
    let mut итог: Vec<String> = Vec::new();
    //
    let mut счётчик_открытий: usize = 0;
    let mut счётчик_закрытий: usize = 0;
    //let mut условие_начала:bool=false;
    for i in 0..ряд.len() {
        //если начался script
        if условие_script {
            if sz_найти(&ряд[i], &начало_строки) {
                // println!("Условие: i:{}|{}|",i+1,&ряд[i]);
                счётчик_открытий += 1;
            }
            if sz_найти(&ряд[i], конец_строки) {
                счётчик_закрытий += 1;
                //добавляем окончание
                ряд_общий.push(
                    убрать_примечания_из_строки_c_окончанием(
                        &ряд[i],
                    ),
                );
                //если это закрытие родное
                if счётчик_открытий == счётчик_закрытий {
                    //условие возвращается в ложь, что нет script
                    условие_script = false;
                    //вложение в итог
                    итог.extend(ряд_общий);
                    //уничтожение содержимого для нового использования
                    ряд_общий = Vec::new();
                    //
                    счётчик_открытий = 0;
                    счётчик_закрытий = 0;
                    continue;
                } else {
                    continue;
                }

            }
            //если нет конца
            else {
                ряд_общий.push(убрать_примечания_из_строки_без_преобразований(&ряд[i]));
                continue;
            }
        } else
        //ищет начало script
        if sz_найти(&ряд[i], начало_строки) {
            //если есть закрытие script в строке
            if sz_найти(&ряд[i], конец_строки) {
                итог.push(убрать_примечания_из_строки_с_преобразованием(&ряд[i]));
                условие_script = false;
                счётчик_открытий += 1;
                continue;
                //итог.push(format!("{}"));
            }
            //если нет закрытия script в строке
            else {
                // println!("НЕТ КОНЦА: i:{}|{}|",i+1,&ряд[i]);
                условие_script = true;

                let строка =
                    убрать_примечания_из_строки_c_началом(&ряд[i]);
                //   println!("строка с началом без конца: {}",строка);
                ряд_общий.push(строка);
                continue;
                // итог.push(ряд[i].clone());
            }
        }
        //если обычная строка
        if !условие_script {
            итог.push(ряд[i].clone());
        }
    }
    return итог;
}
fn удаление_script_мусора_после_разбиения_строк(
    ряд: Vec<String>,
) -> Vec<String> {
    const ЧИСЛО_ПРОСТОЕ: usize = 8;
    const ЧИСЛО_СЛОЖНОЕ: usize = 4;
    lazy_static! {
    //static ref начало_строки:String="<script".to_string();
      //   static ref конец_строки:String="</script>".to_string();
        static ref начала_строк:[String;ЧИСЛО_ПРОСТОЕ]=[
            "<script".to_string(),
            "<noscript".to_string(),

        r#"<a href="https://t.me/"#.to_string(),
            r#"<footer class="PFModalDual"#.to_string(),
            format!(r#"<footer class="footer">"#),
            format!(r#"<header class="header fixed"#),
            format!(r#"<aside id="column-"#),
              format!(r##"<section class="section_letter">"##).to_string(),//2
        ];
            static ref концы_строк:[String;ЧИСЛО_ПРОСТОЕ]=[
            "</script>".to_string(),
            "</noscript>".to_string(),

            "</footer>".to_string(),
            "</a>".to_string(),
              "</footer>".to_string(),
            "</header>".to_string(),
            " </aside>".to_string(),
                "</section>".to_string(),//2
        ];
        //сложные
         static ref начала_строк_сложные:[String;ЧИСЛО_СЛОЖНОЕ]=[
            format!(r#"<div class="PFModalHeader"#),
            format!(r#"<div class=" PFButtonsContainer"#),
            format!(r#"<div class="PFModalBody""#),
            r#"<div class=" PFQrText ">"#.to_string(),

        ];
            static ref концы_строк_сложные:[String;ЧИСЛО_СЛОЖНОЕ]=[
            "</div>".to_string(),
             "</div>".to_string(),
            "</div>".to_string(),
            "</div>".to_string(),


        ];
    }
    let mut итог: Vec<String> = ряд;
    //удаление с вложенностями
    for i in 0..начала_строк_сложные.len() {
        итог=удалить_разделы_html_по_ключевым_словам_с_повторами(итог,&начала_строк_сложные[i],&концы_строк_сложные[i]);
    }
    //простые
    for i in 0..начала_строк.len() {
        итог = удалить_разделы_html_по_ключевым_словам(
            итог,
            &начала_строк[i],
            &концы_строк[i],
        );
    }

    return итог;
}

pub fn убрать_примечания_из_строки_с_преобразованием(
    строка: &String,
) -> String {
    let строка = строка.replace("<!--", "");
    let строка = строка.replace("[-->", "");
    let строка = строка.replace("-->", "");
    return format!(r#"<!--{}-->"#, строка);
}

pub fn убрать_примечания_из_строки_без_преобразований(
    строка: &String,
) -> String {
    let строка = строка.replace("<!--", "");
    let строка = строка.replace("[-->", "");
    let строка = строка.replace("-->", "");
    return строка;
}

pub fn убрать_примечания_из_строки_c_окончанием(
    строка: &String,
) -> String {
    let строка = строка.replace("<!--", "");
    let строка = строка.replace("[-->", "");
    let строка = строка.replace("-->", "");
    return format!(r#"{}-->"#, строка);
}

pub fn убрать_примечания_из_строки_c_началом(
    строка: &String,
) -> String {
    let строка = строка.replace("<!--", "");
    let строка = строка.replace("[-->", "");
    let строка = строка.replace("-->", "");
    return format!(r#"<!--{}"#, строка);
}

//
pub fn если_пустая_строка_с_отделителями(
    стог_сена: &String
) -> bool {
    lazy_static! {
        static ref образец: Regex = Regex::new(r#"^\s*$"#).unwrap();
    }
    if образец.is_match(стог_сена) {
        return true;
    } else {
        return false;
    }
}
fn есть_ли_реклама_после_разбиения_строк(
    строка: &String
) -> bool {
    lazy_static! {
        static ref ряд: [String; 53] = [
            //radio prog
            r#"<a class="wcommunity_avatar""#.to_string(),
            r#"<a class="wcommunity_subscribers"#.to_string(),
            //https://www.avclub.pro/
            "PFQrScanImage".to_string(),
            "Chat widget".to_string(),
            format!(r#"<a target="_blank" href="https://widget"#),
            "сделано в </span".to_string(),
            //
            "tawk-chat-message-container".to_string(),
            "tm-article-presente".to_string(),
            "quest__button".to_string(),
            "quest__text".to_string(),
            "v_u_ablock".to_string(),
            "tm-entity-image".to_string(),
            "tm-article-presenter".to_string(),
            "tm-user-info__user".to_string(),
            "snippet__author".to_string(),
            "tm-article-sticky-panel".to_string(),
            "tm-publication".to_string(),
           "swiper-button-next".to_string(),
            //"tm-article".to_string(),
            "tm-scroll-top".to_string(),
                 r#"element-wrapper above-header"#.to_string(),
            r#"tm-layout__wrapper"#.to_string(),
             r#"tm-user-card"#.to_string(),
            r#"tm-counter"#.to_string(),
            r#"content-text"#.to_string(),
            r#"sponsor-block"#.to_string(),
            r#"tm-stories"#.to_string(),
            r#"project-block"#.to_string(),
              r#"tm-project"#.to_string(),
             r#"tm-promo"#.to_string(),
            r#"tm-event"#.to_string(),
            //r#"data-v-"#.to_string(),
             r#"banner-info visible"#.to_string(),
              r#"content-action"#.to_string(),
             r#"content-container"#.to_string(),
            r#"sponsor-mark"#.to_string(),
            r#"sponsorship_hub"#.to_string(),
             r#"tm-header"#.to_string(),
           r#"tm-digest"#.to_string(),
             r#"tm-copyright"#.to_string(),
            r#"tm-footer"#.to_string(),
            r#"tm-description-list tm"#.to_string(),
             r#"tm-block"#.to_string(),
            r#"tm-company"#.to_string(),

            //старое
            r#"tm-description-list__body"#.to_string(),
            r#"tm-widget-banner-content__image-wrapper"#.to_string(),
            r#"> Реклама <"#.to_string(),
            r#"company-card-top-image"#.to_string(),
            //Skyeng
            r#"class="promotion-banner -link"#.to_string(),
            r#"another-page-banner"#.to_string(),
            r#"ssm-articles-content-author"#.to_string(),
            r#"/banner.html"#.to_string(),
           // r#">Learn More<"#.to_string(),
           // r#"sticky_top _blue-theme _renasas"#.to_string(),
              r#"tm-input-text"#.to_string(),
            r#"tm-button"#.to_string(),
            //пошли сами объявления
            r#"class="subtitle">Присылаем лучшие статьи раз"#.to_string(),
        ];
        static ref двойные_слова_1:[String;1]=[
            "Promotions".to_string(),
        ];
        /*static ref двойные_слова_2:[String;1]=[

        ];*/
    }
    for i in 0..ряд.len() {
        if sz_найти(&строка, &ряд[i]) {
            // println!("нашло объяву");
            return true;
        }
    }
    if есть_ли_реклама_пропуск_строки(&строка) {
        return true;
    }

    return false;
}
fn есть_ли_реклама_пропуск_строки(строка: &String) -> bool {
    lazy_static! {
        static ref ряд: [String; 3] = [
            r#"One Vision, Three Solutions - Introducing Altium Discover, Altium Develop and Altium Agile"#.to_string(),
            r#">Learn More<"#.to_string(),
                            //skyeng
            r#"sticky_top _blue-theme _renasas"#.to_string(),
           /*  r#"class="promotion-banner -link"#.to_string(),
            r#"another-page-banner"#.to_string(),
            r#"ssm-articles-content-author"#.to_string(),*/
        ];

    }
    ряд.par_iter().any(|образец| sz_найти(&строка, образец))
}

//  r#">РЕКЛАМА<"#.to_string(),];
pub fn добавить_переносы_строк_html(
    входные_строки: Vec<String>,
    указатель: usize,
) -> Vec<String> {
    use crate::utils::functions_txt::есть_ли_повторно_строка_в_ряде;
    lazy_static! {
                static ref образцы:[String;56]= [
            //r#"<span data"#.to_string(),
           // r#"<div class"#.to_string(),
           // r#"<article class"#.to_string(),
            "</defs>".to_string(),
             r#"content="text/html; charset=UTF-8">"#.to_string(),
            "</template>".to_string(),
            "</head>".to_string(),
            r#"<!--]-->"#.to_string(),
                    r#"<!---->"#.to_string(),
               r#"</code>"#.to_string(),
            r#"</summary>"#.to_string(),
            r#"</s>"#.to_string(),
             r#"</strong>"#.to_string(),
             r#"</dd>"#.to_string(),
             r#"</dl>"#.to_string(),
             r#"</dt>"#.to_string(),
            r#"</article>"#.to_string(),
              r#"</p>"#.to_string(),
               r#"</section>"#.to_string(),
             r#"</time>"#.to_string(),
            r#"</form>"#.to_string(),
           // r#"<!--]-->"#.to_string(),
            //r#"</h3>"#.to_string(),
            // r#"</h2>"#.to_string(),
            // r#"</h1>"#.to_string(),
             r#"</em>"#.to_string(),
            //r#"<!--[-->"#.to_string(),
           // r#"<!---->"#.to_string(),
            r#"</title>"#.to_string(),
            r#"</div>"#.to_string(),
            r#"</figcaption>"#.to_string(),
            r#"</script>"#.to_string(),
            r#"</picture>"#.to_string(),
            r#"</br>"#.to_string(),
            r#"</style>"#.to_string(),
             r#"</img>"#.to_string(),
             r#"</path>"#.to_string(),
             r#"</use>"#.to_string(),
             r#"</mask>"#.to_string(),
             r#"</stop>"#.to_string(),
             r#"</clippath>"#.to_string(),
             r#"</lineargradient>"#.to_string(),
             r#"</radialgradient>"#.to_string(),
             r#"</symbol>"#.to_string(),
             r#"</rect>"#.to_string(),
            r#"</span>"#.to_string(),
            r#"</svg>"#.to_string(),
            r#"</iframe>"#.to_string(),
             r#"</a>"#.to_string(),
             r#"</femergenode>"#.to_string(),
            r#"</femorphology>"#.to_string(),
            r#"</femerge>"#.to_string(),
              r#"</filter>"#.to_string(),
             r#"</circle>"#.to_string(),
                  r#"</ellipse>"#.to_string(),
               r#"</g>"#.to_string(),
            r#"</button>"#.to_string(),
            r#"</header>"#.to_string(),
            r#"</li>"#.to_string(),
               r#"</ul>"#.to_string(),
             r#"</i>"#.to_string(),
            r#"</blockquote>"#.to_string(),
             r#"</video>"#.to_string(),
                 r#"</textarea>"#.to_string(),
              r#"</body>"#.to_string(),

        ];
    }
    //переменная для хранения счётчика проверки
    static СЧЁТЧИК_ПРОВЕРКИ: AtomicBool = AtomicBool::new(false);
    //сама проверка
    if !СЧЁТЧИК_ПРОВЕРКИ.swap(true, Ordering::SeqCst) {
        есть_ли_повторно_строка_в_ряде(
            &образцы.as_ref(),
            "Образцы разбиения строк html",
            //true
        );
    }

    /*let mut новый_ряд_строк: Vec<String> = Vec::new();
    for i in 0..входные_строки.len() {
        if sz_найти(&входные_строки[i], r#"</div>"#) {
            //   println!("нашло div");
            let строки: Vec<String> = входные_строки[i]
                .split_inclusive("</div>")
                .map(|s| s.to_string())
                .collect();
            новый_ряд_строк.extend(строки);
            continue;
        } else {
            новый_ряд_строк.push(входные_строки[i].clone())
        }
    }
    return новый_ряд_строк;*/
    let mut новый_ряд_строк: Vec<String> = входные_строки;
    //прогон через все образцы
    for i in 0..образцы.len() {
        новый_ряд_строк =
            разбить_строки_через_образец(новый_ряд_строк, &образцы[i]);
    }
    return новый_ряд_строк;
    //входные_строки
}

pub fn разбить_строки_через_образец(
    исходный_ряд_строк: Vec<String>,
    образец: &String,
) -> Vec<String> {
    let mut новый_ряд_строк: Vec<String> = Vec::new();
    for i in 0..исходный_ряд_строк.len() {
        if sz_найти(&исходный_ряд_строк[i], &образец) {
            //   println!("нашло div");
            let строки: Vec<String> = исходный_ряд_строк[i]
                .split_inclusive(образец.as_str())
                .map(|s| s.to_string())
                .collect();
            новый_ряд_строк.extend(строки);
            continue;
        } else {
            новый_ряд_строк.push(исходный_ряд_строк[i].clone())
        }
    }
    return новый_ряд_строк;
}

pub fn удалить_переносы_калибри(строки: &[String]) -> Vec<String> {
    строки
        .iter()
        .map(|строка| {
            строка
                .replace(r#"</span><span class="calibre10">"#, "")
                .replace(r#"</span><span class="calibre13">"#, "")
                .replace(r#"<span class="calibre13">"#, "")
        })
        .collect()
}

pub fn удалить_лишние_пробелы(строки: &[String]) -> Vec<String> {
    строки
        .iter()
        .map(|строка| строка.replace(r#"  "#, " "))
        .collect()
}
fn нет_ссылки_на_папку(строка: &String) -> bool {
    lazy_static! {
        static ref ряд: [String; 2] = [r#"src="./"#.to_string(), r#"href="./"#.to_string(),];
    }
    ряд.par_iter().any(|образец| sz_найти(&строка, образец))
}
fn удалить_shy_из_вектора(строки: &[String]) -> Vec<String> {
    let строки: Vec<String> = строки
        .iter()
        .map(|строка| {
            if sz_найти(&строка, "\u{00A0}") && !нет_ссылки_на_папку(&строка)
            {
                строка.replace("\u{00A0}", " ") // Unicode символ
            } else {
                строка.to_string()
            }
        })
        .collect();
    строки
        .iter()
        .map(|строка| {
            строка
                .replace("&shy;", "")
                .replace("­", "")
                .replace("&nbsp;", " ")
                .replace("\u{00AD}", "")
                .replace("&#8209;", "-")
                .replace("\u{2011}", "-")
                .replace("&#160;", "") // числовая форма
                //.replace("\u{00A0}", " ") // Unicode символ
                .replace("&#xA0;", " ") // шестнадцатеричная форма
                // <
                .replace("&lt;", "<")
                .replace("&gt;", ">") // больше
                .replace("&amp;", "&") // амперсанд
                .replace("&quot;", "\"") // двойная кавычка
                //.replace("&#39;", "'") // одинарная кавычка
                .replace("&#x27;", "'") // одинарная кавычка (hex)
                //>
                //двойная кавычка
                .replace("&#34;", "\"") // числовая форма
                .replace("&#x22;", "\"") // шестнадцатеричная форма
        })
        .collect()
}
fn удалить_переносы_из_вектора(
    mut строки: Vec<String>
) -> Vec<String> {
    lazy_static! {
        static ref ряд: [Regex; 3] = [
            Regex::new(r"(?i)-</span>(.[^%]+)(.[^>]+)>").unwrap(),
            Regex::new(r"(?i)-</h1>(.[^%]+)(.[^>]+)>").unwrap(),
            Regex::new(r"(?i)-</p><h1(.[^%]+)(.[^>]+)>").unwrap(),
        ];
    }
    // println!("Зашло в удаление переносов");
    for i in 0..ряд.len() {
        строки.par_iter_mut().for_each(|mut строка| {
            let замененная_строка = ряд[i].replace_all(&строка, "");
            let замененная_строка = замененная_строка.to_string();
            if замененная_строка.as_str() != строка.as_str() {
                // Увеличиваем атомарный счетчик
                //  println!("Произведена замена строки");
                *строка = замененная_строка
            }
        });
    }
    строки
}
fn отсутствие_закрытого_p_тэга_html_в_строке(
    стог_сена: &String,
) -> bool {
    lazy_static! {
        static ref открытый_p: String = "<p>".to_string();
        static ref закрытый_p: String = "</p>".to_string();
    }
    let mut условие = false;
    let условие_начала = sz_найти(&стог_сена, &открытый_p);
    let условие_конца = sz_найти(&стог_сена, &закрытый_p);
    if (условие_начала && !условие_конца) {
        условие = true
    }
    return условие;
}

fn отсутствие_закрытого_title_тэга_html_в_строке(
    стог_сена: &String,
) -> bool {
    lazy_static! {
        static ref открытый_p: String = "<title>".to_string();
        static ref закрытый_p: String = "</title>".to_string();
    }
    let mut условие = false;
    let условие_начала = sz_найти(&стог_сена, &открытый_p);
    let условие_конца = sz_найти(&стог_сена, &закрытый_p);
    if (условие_начала && !условие_конца) {
        условие = true
    }
    return условие;
}

// Чтение данных из Vec<u8> как UTF-8 текста с разделением на строки
pub fn read_utf8_из_ряда_u8(
    ряд_байтов: &Vec<u8>, имя_файла: &String
) -> Vec<String> {
    let mut итог: Vec<String> = Vec::new();

    // Создаем BufRead из Vec<u8>
    let курсор = Cursor::new(ряд_байтов.clone());

    // Читаем построчно
    for (указатель, строка_результат) in курсор.clone().lines().enumerate()
    {
        let указатель_строки: usize = указатель + 1;

        match строка_результат {
            Ok(строка) => {
                итог.push(строка);
            }
            Err(ошибка) => {
                /*let mut данные = DecodeReaderBytesBuilder::new()
                    .encoding(Some(WINDOWS_1251))
                    .build(ряд_байтов.as_slice());
                let mut буффер = String::new();
                let _number_of_bytes = match данные.read_to_string(&mut буффер) {
                    Ok(указатель) => указатель,
                    Err(why) => {
                        eprintln!("Сбой при чтении данных из файла в ОЗУ!");
                        eprintln!("Строка № {указатель_строки}");
                        eprintln!("Используемая кодировка: WINDOWS_1251.");
                        eprintln!("Попробуйте другой вид кодировки!");
                        println!("Ошибка при преобразовании данных в UTF-8 по причине: {why}");
                        system_pause();
                        panic!("Ошибка при преобразовании данных в UTF-8 по причине: {why}")
                    }
                };
                буффер

                 */

                // Обработка ошибок UTF-8
                eprintln!(
                    "Файл: {имя_файла} | Ошибка в строке {}: {}",
                    указатель_строки, ошибка
                );

                // Альтернатива: использовать lossy конверсию
                let потерянная_строка =
                    String::from_utf8_lossy(&ряд_байтов[курсор.position() as usize..]).to_string();
                итог.push(потерянная_строка);
                break;
            }
        }
    }
    //println!("UTF8: {имя_файла} количество строк: {}",итог.len());
    итог
}
//чтение файла
fn считать_файл(путь: &str) -> Box<dyn BufRead> {
    let содержимое = match fs::File::open(путь) {
        //попытка открытия файла
        Ok(успех) => успех,
        Err(почему) => {
            //если ошибка
            println!("Ошибка при открытии файла: \"{путь}\" по причине: \n{почему:?}");
            system_pause();
            panic!("Ошибка при открытии файла: \"{путь}\" по причине: \n{почему:?}")
        }
    };
    Box::new(BufReader::new(содержимое))
}

pub fn попытка_открыть_файл(путь: &String) {
    //открытие файла с библиотекой
    let итог = match File::open(путь) {
        Ok(положительно) => положительно,
        Err(ошибка) => panic!(
            "Не получилось открыть файл: {}, по причине: {}",
            путь, ошибка
        ),
    };
}
//получение строки в виде UTF-8
fn получить_строку_в_utf8(
    ряд_байтов_итоговый: Result<Vec<u8>, std::io::Error>, //вектор байт
    указатель_строки: usize,                              //номер строки
) -> String {
    let ряд_байтов: Vec<u8> = match ряд_байтов_итоговый {
        //попытка сопоставить вектор байт
        Ok(значения) => значения,
        Err(why) => {
            println!(
                "Ошибка при чтении строки: |{}| по причине: {why}",
                указатель_строки
            );
            system_pause();
            panic!(
                "Ошибка при чтении строки: |{}| по причине: {why}",
                указатель_строки
            )
        }
    };
    строка_удалить_utf8_концы_строк(&ряд_байтов, указатель_строки)
}

pub fn получить_содержимое(путь: &str) -> Vec<String> {
    let содержимое: Vec<(String, String)> = считать_содержимое_папки(путь).unwrap();
    //println!("содержимое получить: {:?}",содержимое);
    return содержимое.par_iter().map(|(имя, _)| имя.clone()).collect();
}

fn считать_содержимое_папки(
    путь_папки: &str,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    use crate::utils::functions::заменить_все_палки;
    let mut содержимое_папки: Vec<(String, String)> = Vec::new();
    let путь_новый = путь_папки.to_string();
    for вхождение in WalkDir::new(путь_папки)
        .min_depth(0)
        .max_depth(6)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let полученное = вхождение.path().to_string_lossy().to_string();
        if вхождение.file_type().is_file() && !sz_найти(&полученное, "~$") {
            let путь = вхождение.path();
            match fs::read_to_string(путь) {
                Ok(содержимое) => {
                    // let путь_исходный=путь.display().to_string();
                    let путь2 = заменить_все_палки(путь.display().to_string());
                    if sz_найти(&путь2, r#"\"#) | sz_найти(&путь2, r#"\\"#) {
                        println!("путь с палкой в начале: {путь2}");
                    };
                    содержимое_папки.push((путь2, содержимое))
                }
                Err(ошибка) => {
                    if ошибка
                        .to_string()
                        .contains("stream did not contain valid UTF-8")
                    {
                        let путь2 = заменить_все_палки(путь.display().to_string());
                        if sz_найти(&путь2, r#"\"#) | sz_найти(&путь2, r#"\\"#) {
                            println!("путь с палкой в начале (ошибка): {путь2}");
                        };
                        содержимое_папки.push((
                            путь2,
                            format!("нет содержания: {}", ошибка.to_string()).to_string(),
                        ));
                    } else {
                        println!("ошибка: {:?}", ошибка)
                    }
                }
            }
        }
    }
    Ok(содержимое_папки)
}
