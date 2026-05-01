use crate::utils::functions_txt::{
    есть_ли_повторно_строка_в_ряде, есть_ли_повторно_строка_в_ряде_regex,
};
use crate::utils::stringzilla::sz_найти;
use std::sync::LazyLock;
//use console::{Emoji, style};
//use foldhash::{HashMap, HashMapExt, rapidhash::fast::RapidHashSet};

use rayon::prelude::*;
use regex::Regex;
//use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

//use time::Month::January;
pub fn xml_получить_указатели_на_пропуски(
    содержимое: &Vec<String>,
) -> rapidhash::fast::RapidHashSet<usize> {
    const FB3_ИСКЛЮЧЕНИЯ_ПРОСТЫЕ: [&str; 5] = [
        r#"<fb3-body xmlns="#,
        r#"<rootfile"#,
        "<rootfiles>",
        "<container version",
        r#"<?xml version="1.0"?>"#,
    ];

    const FB3_ИСКЛЮЧЕНИЯ: [&str; 48] = [
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        //вложения
        "</section></body><binary",
        //содержание
        "<description>",
        "<title-info>",
        "</title-info>",
        "<author>",
        "</author>",
        "<annotation>",
        "</annotation>",
        "<coverpage>",
        "</coverpage>",
        "<history>",
        "</history>",
        "<publisher>",
        "</publisher>",
        "<document-info>",
        "</document-info>",
        "<publish-info>",
        "</publish-info>",
        "</description>",
        "<lang>",
        "<version>",
        "<book-name>",
        "<id>",
        "<src-url>",
        "<program-used>",
        "<date value=",
        "<city>",
        "<year>",
        "<isbn>",
        "<image",
        "<body>",
        "</body>",
        "<section>",
        "</section>",
        "<epigraph>",
        "</epigraph>",
        "<FictionBook>",
        "</FictionBook>",
        "<p>…</p>",
        "<date>",
        "<nickname>",
        "<empty-line/>",
        "<title>",
        "</title>",
        "<section id=",
        "</container>",
        "</rootfiles>",
    ];
    const FB3_ОБЯЗАЛОВО: [&str; 2] = ["<", ">"];
    const FB3_RE_ИСКЛЮЧЕНИЯ: LazyLock<[Regex; 48]> = LazyLock::new(|| {
        [
            Regex::new(r#"(?i)^<\?xml\s*version="\#1\.0"\s*encoding="UTF-8"\?>$"#).unwrap(),
            //вложения
            Regex::new(r"(?i)^</section></body><binary").unwrap(),
            //содержание
            Regex::new(r"(?i)^<description>$").unwrap(),
            Regex::new(r"(?i)^\s*<title-info>$").unwrap(),
            Regex::new(r"(?i)^\s*</title-info>$").unwrap(),
            Regex::new(r"(?i)^\s*<author>$").unwrap(),
            Regex::new(r"(?i)^\s*</author>$").unwrap(),
            Regex::new(r"(?i)^\s*<annotation>$").unwrap(),
            Regex::new(r"(?i)^\s*</annotation>$").unwrap(),
            Regex::new(r"(?i)^\s*<coverpage>$").unwrap(),
            Regex::new(r"(?i)^\s*</coverpage>$").unwrap(),
            Regex::new(r"(?i)^\s*<history>$").unwrap(),
            Regex::new(r"(?i)^\s*</history>$").unwrap(),
            Regex::new(r"(?i)^\s*<publisher>$").unwrap(),
            Regex::new(r"(?i)^\s*</publisher>$").unwrap(),
            Regex::new(r"(?i)^\s*<document-info>$").unwrap(),
            Regex::new(r"(?i)^\s*</document-info>$").unwrap(),
            Regex::new(r"(?i)^\s*<publish-info>$").unwrap(),
            Regex::new(r"(?i)^\s*</publish-info>$").unwrap(),
            Regex::new(r"(?i)^\s*</description>$").unwrap(),
            Regex::new(r"(?i)^\s*<lang>.+</lang>$").unwrap(),
            Regex::new(r"(?i)^\s*<version>\</version>$").unwrap(),
            Regex::new(r"(?i)^\s*<book-name>.*</book-name>$").unwrap(),
            Regex::new(r"(?i)^\s*<id>.*</id>$").unwrap(),
            Regex::new(r"(?i)^\s*<src-url>.*</src-url>$").unwrap(),
            Regex::new(r"(?i)^\s*<program-used>.*</program-used>$").unwrap(),
            Regex::new(r"(?i)^\s*<date value=.*</date>$").unwrap(),
            Regex::new(r"(?i)^\s*<city>.*</city>$").unwrap(),
            Regex::new(r"(?i)^\s*<year>.*</year>$").unwrap(),
            Regex::new(r"(?i)^\s*<isbn>.*</isbn>$").unwrap(),
            Regex::new(r#"(?i)^\s*<image.*\.jpg"/>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<body>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</body>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<section>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</section>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<epigraph>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</epigraph>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<FictionBook>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</FictionBook>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<p>…</p>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<date>.+</date>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<nickname>.+</nickname>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<empty-line/>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<title>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</title>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<section id=".+">$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</container>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</rootfiles>$"#).unwrap(),
        ]
    });
    //static RE_ПЕРВАЯ_СКОБКА: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<").unwrap());

    //static RE_ВТОРАЯ_СКОБКА: LazyLock<Regex> = LazyLock::new(|| Regex::new(r">").unwrap());

    let исключения_для_проверки: rapidhash::fast::RapidHashSet<usize> =
        rapidhash::fast::RapidHashSet::from_iter([0]);
    //проверка образцов
    //переменная для хранения счётчика проверки
    static СЧЁТЧИК_ПРОВЕРКИ: AtomicBool = AtomicBool::new(false);
    //сама проверка
    if !СЧЁТЧИК_ПРОВЕРКИ.swap(true, Ordering::SeqCst) {
        if !проверка_образцов_re_и_слов_для_кучи(
            &FB3_ИСКЛЮЧЕНИЯ.as_parallel_slice(),
            &*FB3_RE_ИСКЛЮЧЕНИЯ,
            &исключения_для_проверки,
            "fb3_исключения",
        ) {
            panic!()
        }
    };
    //получение значений

    //прогон
    let пропуски: rapidhash::fast::RapidHashSet<usize> = содержимое
        .par_iter()
        .enumerate()
        .filter_map(|(указатель, строка)| {
            if !есть_ли_кириллица(&строка)
                || fb3_проверка_содержимого_на_условия(&строка)
            {
                return Some(указатель);
            } else {
                None
            }
        })
        .collect::<rapidhash::fast::RapidHashSet<usize>>();
    return пропуски;
    //если истина-то переход к следующей строке
    //исключения для расширения
    //если возвращает истину - то переход на следующую строку
    fn fb3_обязательное_содержимое(стог_сена: &String) -> bool {
        //проверка что нет пустоты
        if стог_сена.is_empty() {
            return true;
        }
        //сначала что есть скобки
        return FB3_ОБЯЗАЛОВО
            .par_iter()
            .any(|образец| sz_найти(&стог_сена, &образец));
    }
    fn fb3_проверка_содержимого_на_условия(
        стог_сена: &String
    ) -> bool {
        //обязательно должны быть скобки
        if !fb3_обязательное_содержимое(стог_сена) {
            return true;
        }
        //поиск
        for образец in FB3_ИСКЛЮЧЕНИЯ_ПРОСТЫЕ.iter() {
            if sz_найти(&стог_сена, &образец) {
                //  println!("b3_исключения_простые: {стог_сена}");
                return true;
            }
        }
        if FB3_ИСКЛЮЧЕНИЯ_ПРОСТЫЕ
            .par_iter()
            .any(|образец| sz_найти(&стог_сена, &образец))
        {
            //println!("b3_исключения_простые: {стог_сена}");
            return true;
        }
        //проверка концов и окончаний строк
        //сначала что есть скобки
        return проверка_исключений_в_стоге_сена(
            &FB3_ИСКЛЮЧЕНИЯ.as_parallel_slice(),
            &*FB3_RE_ИСКЛЮЧЕНИЯ,
            &стог_сена,
        );
    }
}
pub fn fb2_получить_указатели_на_пропуски(
    содержимое: &Vec<String>,
) -> rapidhash::fast::RapidHashSet<usize> {
    const FB2_ИСКЛЮЧЕНИЯ: [&str; 46] = [
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        //вложения
        "</section></body><binary",
        //содержание
        "<description>",
        "<title-info>",
        "</title-info>",
        "<author>",
        "</author>",
        "<annotation>",
        "</annotation>",
        "<coverpage>",
        "</coverpage>",
        "<history>",
        "</history>",
        "<publisher>",
        "</publisher>",
        "<document-info>",
        "</document-info>",
        "<publish-info>",
        "</publish-info>",
        "</description>",
        "<lang>",
        "<version>",
        "<book-name>",
        "<id>",
        "<src-url>",
        "<program-used>",
        "<date value=",
        "<city>",
        "<year>",
        "<isbn>",
        "<image",
        "<body>",
        "</body>",
        "<section>",
        "</section>",
        "<epigraph>",
        "</epigraph>",
        "<FictionBook>",
        "</FictionBook>",
        "<p>…</p>",
        "<date>",
        "<nickname>",
        "<empty-line/>",
        "<title>",
        "</title>",
        "<section id=",
    ];
    const FB2_ОБЯЗАЛОВО: [&str; 2] = ["<", ">"];
    static FB2_RE_ИСКЛЮЧЕНИЯ: LazyLock<[Regex; 46]> = LazyLock::new(|| {
        [
            Regex::new(r#"(?i)^<\?xml\s*version="\#1\.0"\s*encoding="UTF-8"\?>$"#).unwrap(),
            //вложения
            Regex::new(r"(?i)^</section></body><binary").unwrap(),
            //содержание
            Regex::new(r"(?i)^<description>$").unwrap(),
            Regex::new(r"(?i)^\s*<title-info>$").unwrap(),
            Regex::new(r"(?i)^\s*</title-info>$").unwrap(),
            Regex::new(r"(?i)^\s*<author>$").unwrap(),
            Regex::new(r"(?i)^\s*</author>$").unwrap(),
            Regex::new(r"(?i)^\s*<annotation>$").unwrap(),
            Regex::new(r"(?i)^\s*</annotation>$").unwrap(),
            Regex::new(r"(?i)^\s*<coverpage>$").unwrap(),
            Regex::new(r"(?i)^\s*</coverpage>$").unwrap(),
            Regex::new(r"(?i)^\s*<history>$").unwrap(),
            Regex::new(r"(?i)^\s*</history>$").unwrap(),
            Regex::new(r"(?i)^\s*<publisher>$").unwrap(),
            Regex::new(r"(?i)^\s*</publisher>$").unwrap(),
            Regex::new(r"(?i)^\s*<document-info>$").unwrap(),
            Regex::new(r"(?i)^\s*</document-info>$").unwrap(),
            Regex::new(r"(?i)^\s*<publish-info>$").unwrap(),
            Regex::new(r"(?i)^\s*</publish-info>$").unwrap(),
            Regex::new(r"(?i)^\s*</description>$").unwrap(),
            Regex::new(r"(?i)^\s*<lang>.+</lang>$").unwrap(),
            Regex::new(r"(?i)^\s*<version>\</version>$").unwrap(),
            Regex::new(r"(?i)^\s*<book-name>.*</book-name>$").unwrap(),
            Regex::new(r"(?i)^\s*<id>.*</id>$").unwrap(),
            Regex::new(r"(?i)^\s*<src-url>.*</src-url>$").unwrap(),
            Regex::new(r"(?i)^\s*<program-used>.*</program-used>$").unwrap(),
            Regex::new(r"(?i)^\s*<date value=.*</date>$").unwrap(),
            Regex::new(r"(?i)^\s*<city>.*</city>$").unwrap(),
            Regex::new(r"(?i)^\s*<year>.*</year>$").unwrap(),
            Regex::new(r"(?i)^\s*<isbn>.*</isbn>$").unwrap(),
            Regex::new(r#"(?i)^\s*<image.*\.jpg"/>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<body>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</body>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<section>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</section>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<epigraph>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</epigraph>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<FictionBook>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</FictionBook>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<p>…</p>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<date>.+</date>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<nickname>.+</nickname>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<empty-line/>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<title>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</title>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<section id=".+">$"#).unwrap(),
        ]
    });
    static RE_ПЕРВАЯ_СКОБКА: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<").unwrap());
    static RE_ВТОРАЯ_СКОБКА: LazyLock<Regex> = LazyLock::new(|| Regex::new(r">").unwrap());

    let исключения_для_проверки: rapidhash::fast::RapidHashSet<usize> =
        rapidhash::fast::RapidHashSet::from_iter([0]);
    //переменная для хранения счётчика проверки
    static СЧЁТЧИК_ПРОВЕРКИ: AtomicBool = AtomicBool::new(false);
    //
    if !СЧЁТЧИК_ПРОВЕРКИ.swap(true, Ordering::SeqCst) {
        if !проверка_образцов_re_и_слов_для_кучи(
            &FB2_ИСКЛЮЧЕНИЯ.as_parallel_slice(),
            &*FB2_RE_ИСКЛЮЧЕНИЯ,
            &исключения_для_проверки,
            "fb2_исключения",
        ) {
            panic!()
        }
    };
    //получение значений

    let пропуски: rapidhash::fast::RapidHashSet<usize> =
    //прогон
    содержимое
        .par_iter()
        .enumerate()
        .filter_map(|(указатель, строка)| {
            if !есть_ли_кириллица(&строка)|| fb2_проверка_содержимого_на_условия(&строка) {
                return Some(указатель)
            }
            else {None}
        }).collect::<rapidhash::fast::RapidHashSet<usize>>();
    return пропуски;
    //исключения для расширения
    //если возвращает истину - то переход на следующую строку
    fn fb2_обязательное_содержимое(стог_сена: &String) -> bool {
        //проверка что нет пустоты
        if стог_сена.is_empty() {
            return true;
        }
        //сначала что есть скобки
        return FB2_ОБЯЗАЛОВО
            .par_iter()
            .any(|образец| sz_найти(&стог_сена, &образец));
    }

    fn fb2_проверка_содержимого_на_условия(
        стог_сена: &String
    ) -> bool {
        //обязательно должны быть скобки
        if !fb2_обязательное_содержимое(стог_сена) {
            return true;
        }
        //поиск
        if sz_найти(стог_сена, &"</binary>".to_string()) {
            let количество_открытий = RE_ПЕРВАЯ_СКОБКА.find_iter(&стог_сена).count(); // считывает кол {  }
            let количество_закрытий = RE_ВТОРАЯ_СКОБКА.find_iter(&стог_сена).count(); // считывает кол {  }
            if количество_открытий == 1 && количество_закрытий == 1
            {
                return true;
            }
        }
        //проверка концов и окончаний строк
        //сначала что есть скобки
        return проверка_исключений_в_стоге_сена(
            &*FB2_ИСКЛЮЧЕНИЯ.as_parallel_slice(),
            &*FB2_RE_ИСКЛЮЧЕНИЯ,
            &стог_сена,
        );
    }
    //если истина-то переход к следующей строке
}

fn html_получить_указатели_на_пропуски(
    содержимое: &[String],
    _условие_архива: bool,
) -> rapidhash::fast::RapidHashSet<usize> {
    let пропуски: rapidhash::fast::RapidHashSet<usize> = содержимое
        .par_iter()
        .enumerate()
        .filter_map(|(указатель, строка)| {
            if !есть_ли_кириллица(&строка)
                || html_проверка_содержимого_на_условия(&строка)
            {
                return Some(указатель);
            } else {
                None
            }
        })
        .collect::<rapidhash::fast::RapidHashSet<usize>>();
    return пропуски;

    fn html_проверка_содержимого_на_условия(
        стог_сена: &String,
    ) -> bool {
        const HTML_ИСКЛЮЧЕНИЯ: [&str; 1] = [r#"<!DOCTYPE html PUBLIC"#];
        const HTML_ИСКЛЮЧЕНИЯ_С_ПРОВЕРКОЙ: [&str; 2] =
            ["<blockquote><div>", "</div></body></html>"];
        static RE_HTML_ИСКЛЮЧЕНИЯ_С_ПРОВЕРКОЙ: LazyLock<[Regex; 2]> = LazyLock::new(|| {
            [
                Regex::new(r#"(?i)^\s*<blockquote><div>$"#).unwrap(),
                Regex::new(r#"(?i)^\s*</div></body></html>$"#).unwrap(),
            ]
        });
        const СТРОКИ_ИСКЛЮЧНИЯ: [&str; 12] = [
            r####"<link rel="icon" href="###"####,
            r##"<link rel="preload" href=""##,
            r####"<link rel="stylesheet" href=""####,
            r##"<img src=""##,
            r##"<img class="logo" src=""##,
            r##"<img class="mm-header-search-close" src=""##,
            r##"<script src=""##,
            r##"src="."##,
            r#"<script async="" src="#,
            r#"<link href="./"#,
            r#"href="./"#,
            r#"<a class="lightbox" href=""#,
        ];

        static СЧЁТЧИК_ПРОВЕРКИ: AtomicBool = AtomicBool::new(false);
        //сама проверка
        if !СЧЁТЧИК_ПРОВЕРКИ.swap(true, Ordering::SeqCst) {
            есть_ли_повторно_строка_в_ряде(
                &СТРОКИ_ИСКЛЮЧНИЯ.as_parallel_slice(),
                "исключения html",
                Text_Changer::Раздел_Словаря::Не_является_разделом,
            );
        }
        //поиск
        if HTML_ИСКЛЮЧЕНИЯ
            .par_iter()
            .any(|образец| sz_найти(&стог_сена, образец))
        {
            return true;
        }
        if СТРОКИ_ИСКЛЮЧНИЯ
            .par_iter()
            .enumerate()
            .any(|(_указатель, образец)| sz_найти(&стог_сена, образец))
        {
            //println!("Найдено исключение: {}",стог_сена);
            return true;
        }
        return проверка_исключений_в_стоге_сена(
            &HTML_ИСКЛЮЧЕНИЯ_С_ПРОВЕРКОЙ.as_parallel_slice(),
            &*RE_HTML_ИСКЛЮЧЕНИЯ_С_ПРОВЕРКОЙ,
            &стог_сена,
        );
        //проверка концов и окончаний строк
        //сначала что есть скобки
    }
}
pub fn получить_пропуски_для_содержимого(
    содержимое: &Vec<String>,
    имя_файла: &String,
    расширение_книги: &String,
) -> rapidhash::fast::RapidHashSet<usize> {
    // println!("имя файла: {имя_файла}, расширение_книги: {расширение_книги}");
    //fb2
    if расширение_книги.as_str() == "fb2".to_string() {
        return fb2_получить_указатели_на_пропуски(&содержимое);
    } else if расширение_книги.as_str() == "md".to_string() {
        return md_получить_указатели_на_пропуки(&содержимое);
    } else if расширение_книги.as_str() == "fb3".to_string()
        || расширение_книги.as_str() == "epub".to_string()
    {
        if !sz_найти(имя_файла, ".rels") {
            if sz_найти(имя_файла, ".xml") {
                if расширение_книги.as_str() == "fb3".to_string()
                    && !sz_найти(имя_файла, "body.xml")
                {
                    return xml_получить_указатели_на_пропуски(
                        &содержимое,
                    );
                }
            }
            if sz_найти(имя_файла, ".html") {
                //архив если
                return html_получить_указатели_на_пропуски(
                    &содержимое,
                    true,
                );
            }
        }
    }
    //если это отдельный файл .htm или .html
    else if sz_найти(расширение_книги, "htm") {
        //что не архив
        return html_получить_указатели_на_пропуски(&содержимое, false);
    }
    return rapidhash::fast::RapidHashSet::with_hasher(rapidhash::fast::RandomState::default());
}

pub fn md_получить_указатели_на_пропуки(
    содержимое: &Vec<String>,
) -> rapidhash::fast::RapidHashSet<usize> {
    const MD_ИСКЛЮЧЕНИЯ: [&str; 25] = [
        r#"---"#,
        r#"***"#,
        r#"___"#,
        r#"```"#,
        //содержимое
        r#"<table>"#,
        r#"</table>"#,
        r#"<tr>"#,
        r#"</tr>"#,
        r#"<td>"#,
        r#"</td>"#,
        r#"[comment]:"#,
        r#"[!"#,
        r#":::"#,
        r#"<!"#,
        r#""resource""#,
        r#"{"#,
        r#"}"#,
        r#"],"#,
        r#""**/"#,
        r#"//"#,
        r#"("#,
        r#")"#,
        r#"<tr><td>"#,
        r#"/<tr><td>"#,
        r#"ms.date"#,
    ];
    static MD_RE_ИСКЛЮЧЕНИЯ: LazyLock<[Regex; 25]> = LazyLock::new(|| {
        [
            Regex::new(r#"(?i)^\s*(-+)$"#).unwrap(),
            Regex::new(r#"(?i)^\s*(\*+)$"#).unwrap(),
            Regex::new(r#"(?i)^\s*(_+)$"#).unwrap(),
            Regex::new(r#"(?i)^\s*(`+).+$"#).unwrap(),
            //содержимое
            Regex::new(r#"(?i)^\s*<table>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</table>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<tr>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</tr>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*<td>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*</td>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*[comment]:"#).unwrap(),
            Regex::new(r#"(?i)^\s*\[\!"#).unwrap(),
            Regex::new(r#"(?i)^\s*(:+)"#).unwrap(),
            Regex::new(r#"(?i)^\s*<!"#).unwrap(),
            Regex::new(r#"(?i)^\s*"resource""#).unwrap(),
            Regex::new(r#"(?i)^\s*\{"#).unwrap(),
            Regex::new(r#"(?i)^\s*\}"#).unwrap(),
            Regex::new(r#"(?i)^\s*\]\,"#).unwrap(),
            Regex::new(r#"(?i)^\s*"(\*+)"#).unwrap(),
            Regex::new(r#"(?i)^\s*(/+)"#).unwrap(),
            Regex::new(r#"(?i)^\s*\("#).unwrap(),
            Regex::new(r#"(?i)^\s*\)"#).unwrap(),
            Regex::new(r#"(?i)^\s*<tr><td>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*/<tr><td>$"#).unwrap(),
            Regex::new(r#"(?i)^\s*ms\.date:"#).unwrap(),
            //вложения
        ]
    });
    //static MD_ПРИМЕЧАНИЕ: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?i)^#"#).unwrap());

    let исключения_для_проверки: rapidhash::fast::RapidHashSet<usize> =
        rapidhash::fast::RapidHashSet::from_iter([0, 1, 2, 3, 11, 12, 17, 18, 19, 24]);
    //переменная для хранения счётчика проверки
    static СЧЁТЧИК_ПРОВЕРКИ: AtomicBool = AtomicBool::new(false);
    //
    if !СЧЁТЧИК_ПРОВЕРКИ.swap(true, Ordering::SeqCst) {
        if !проверка_образцов_re_и_слов_для_кучи(
            &*MD_ИСКЛЮЧЕНИЯ.as_parallel_slice(),
            &*MD_RE_ИСКЛЮЧЕНИЯ,
            &исключения_для_проверки,
            "md_исключения",
        ) {
            panic!()
        }
    }

    //получение значений

    let пропуски: rapidhash::fast::RapidHashSet<usize> = содержимое
        .into_par_iter()
        .enumerate()
        .filter_map(|(указатель, _строка_внутри)| {
            if !есть_ли_кириллица(&содержимое[указатель])
                || md_проверка_содержимого_на_условия(
                    &содержимое[указатель],
                )
            {
                Some(указатель)
            } else {
                None
            }
        })
        .collect::<rapidhash::fast::RapidHashSet<usize>>();
    //прогон
    //исключения для расширения
    return пропуски;
    //если возвращает истину - то переход на следующую строку
    fn md_обязательное_содержимое(стог_сена: &String) -> bool {
        if стог_сена.is_empty() {
            return true;
        }
        //сначала что есть скобки
        return false;
    }

    fn md_проверка_содержимого_на_условия(
        стог_сена: &String
    ) -> bool {
        //обязательно должны быть скобки
        if !md_обязательное_содержимое(стог_сена) {
            return true;
        }
        //поиск
        //проверка концов и окончаний строк
        //сначала что есть скобки
        return проверка_исключений_в_стоге_сена(
            &MD_ИСКЛЮЧЕНИЯ.as_parallel_slice(),
            &*MD_RE_ИСКЛЮЧЕНИЯ,
            &стог_сена,
        );
    }
    //если истина-то переход к следующей строке
}
pub fn проверка_исключений_в_стоге_сена(
    исключения: &[&str],
    re_исключения: &[Regex],
    стог_сена: &String,
) -> bool {
    // Проверяем, что массивы одинаковой длины
    if исключения.len() != re_исключения.len() {
        return false;
    }
    (0..исключения.len()).into_par_iter().any(|указатель| {
        sz_найти(стог_сена, &исключения[указатель]) && re_исключения[указатель].is_match(стог_сена)
    })
}
fn проверка_образцов_re_и_слов_для_кучи(
    исключения: &[&str],
    исключения_re: &[Regex],
    исключения_проверки: &rapidhash::fast::RapidHashSet<usize>,
    сообщение: &str,
) -> bool {
    if исключения.len() != исключения_re.len() {
        panic!("не равно количество исключений md")
    }

    for указатель in 0..исключения.len() {
        //если бессмысленно сравнивать образцы
        if исключения_проверки.contains(&указатель) {
            continue;
        }
        //
        (0..исключения.len())
            .into_par_iter()
            .filter(|&указатель| !исключения_проверки.contains(&указатель))
            .for_each(|указатель| {
                //сам поиск образца
                if !sz_найти(
                    &исключения_re[указатель].to_string(),
                    &исключения[указатель],
                ) {
                    panic!(
                        "md: Re образец:{} не соответствует обычному образцу: {}, порядковый указатель: {}",
                        исключения_re[указатель], исключения[указатель], указатель
                    )
                }
            });
    }
    //есть ли повторно исключения - строки обычные в ряде
    есть_ли_повторно_строка_в_ряде(
        &исключения,
        сообщение,
        Text_Changer::Раздел_Словаря::Не_является_разделом,
    );
    //перебор RE
    есть_ли_повторно_строка_в_ряде_regex(&исключения_re, сообщение);

    return true;
}

pub fn есть_ли_кириллица(стог_сена: &String) -> bool {
    use crate::utils::functions_txt::есть_ли_повторно_знак_в_ряде_строк;
    const МАЛЫЕ_БУКВЫ: [char; 33] = [
        'а', 'б', 'в', 'г', 'д', 'ж', 'з', 'е', 'ё', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р',
        'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ];
    const БОЛЬШИЕ_БУКВЫ: [char; 33] = [
        'А', 'Б', 'В', 'Г', 'Д', 'Ж', 'З', 'Е', 'Ё', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р',
        'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
    ];
    static СЧЁТЧИК_ПРОВЕРКИ: AtomicBool = AtomicBool::new(false);
    //сама проверка
    if !СЧЁТЧИК_ПРОВЕРКИ.swap(true, Ordering::SeqCst) {
        есть_ли_повторно_знак_в_ряде_строк(
            &МАЛЫЕ_БУКВЫ.as_ref(),
            "малые буквы",
        );
        есть_ли_повторно_знак_в_ряде_строк(
            &БОЛЬШИЕ_БУКВЫ.as_ref(),
            "большие буквы",
        );
    }
    //малые буквы
    if МАЛЫЕ_БУКВЫ
        .par_iter()
        .enumerate()
        .any(|(_указатель, строка_внутри)| sz_найти(&стог_сена, &строка_внутри.to_string()))
    {
        return true;
    }
    //
    return БОЛЬШИЕ_БУКВЫ
        .par_iter()
        .enumerate()
        .any(|(_указатель, строка_внутри)| sz_найти(&стог_сена, &строка_внутри.to_string()));
}
