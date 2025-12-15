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
    atomic::{AtomicU64, AtomicUsize, Ordering},
};
use walkdir::WalkDir;
use crate::lib;

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
    путь_до_файла: &String
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
        if есть_ли_реклама_пропуск_строки(&строка_в_utf8) {
            continue;
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
    let итог = удалить_shy_из_вектора(&итог);
    let итог = удалить_переносы_из_вектора(итог);
    //return удалить_переносы_строк_html(итог, 0);
    //let итог = добавить_переносы_строк_html(итог, 1);

    return удалить_рекламу_после_разбиения_строк(итог);

    return итог;
}

pub fn определить_кодировку(ряд_строк: &Vec<String>) -> lib::Кодировка {
    // Если вы используете Rayon для параллельной обработки (into_par_iter),
    // нужно убедиться, что он подключен в Cargo.toml и импортирован
    use rayon::prelude::*; // Добавьте это вверху файла или здесь

    // Находим первую подходящую кодировку
    if let Some(кодировка) = ряд_строк
        .into_par_iter() // Параллельная итерация
        .find_map_any(|строка| { // find_map_any возвращает первый найденный результат
            if sz_найти(&строка,r#"content="text/html; charset=windows-1251""#) {
                Some(lib::Кодировка::windows_1251)
            } else if sz_найти(&строка,r#"content="text/html; charset=utf-8""#) {
                Some(lib::Кодировка::utf8)
            } else if sz_найти(&строка,r#"charset=windows-1251"#) {
                Some(lib::Кодировка::windows_1251)
            } else if sz_найти(&строка,r#"charset=utf-8"#) {
                Some(lib::Кодировка::utf8)
            } else {
                None
            }
        })
    {
        return кодировка;
    }

    // Если ничего не найдено, возвращаем кодировку по умолчанию
    lib::Кодировка::utf8
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
        if есть_ли_реклама_пропуск_строки(&строка_в_utf8) {
            continue;
        }
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
    let итог = добавить_переносы_строк_html(итог, 1);
    return удалить_рекламу_после_разбиения_строк(итог);
    // return итог;
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
        }
    }
    // println!("возврат рекламы");
    return итог;
}
fn есть_ли_реклама_после_разбиения_строк(
    строка: &String
) -> bool {
    lazy_static! {
        static ref ряд: [String; 37] = [
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
        ];

    }
    for i in 0..ряд.len() {
        if sz_найти(&строка, &ряд[i]) {
            // println!("нашло объяву");
            return true;
        }
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
    lazy_static!{
                static ref образцы:[String;53]= [
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
            r#"</h3>"#.to_string(),
             r#"</h2>"#.to_string(),
             r#"</h1>"#.to_string(),
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
  /*  есть_ли_повторно_строка_в_ряде(
        &образцы.as_ref(),
        "Образцы разбиения строк",
        true
    );*/


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
        новый_ряд_строк=разбить_строки_через_образец(новый_ряд_строк,&образцы[i]);
    }
    return новый_ряд_строк
    //входные_строки
}

pub fn разбить_строки_через_образец(исходный_ряд_строк:Vec<String>,образец:&String) ->Vec<String>{
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
