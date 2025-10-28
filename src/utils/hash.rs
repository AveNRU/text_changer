use crate::utils::functions_txt::{
    есть_ли_повторно_строка_в_ряде, есть_ли_повторно_строка_в_ряде_regex,
};
use crate::utils::stringzilla::sz_найти;
use foldhash::{HashMap, HashMapExt, HashSet};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::sync::Mutex;
//use time::Month::January;
pub fn xml_получить_указатели_на_пропуски(
    содержимое: &Vec<String>,
) -> HashSet<usize> {
    lazy_static! {

             static ref fb3_исключения_простые: [String;5] = [
            r#"<fb3-body xmlns="#.to_string(),
            r#"<rootfile"#.to_string(),
            "<rootfiles>".to_string(),
            "<container version".to_string(),
            r#"<?xml version="1.0"?>"#.to_string(),
        ];

        static ref fb3_исключения: [String;48] = [
            r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string(),
            //вложения
            "</section></body><binary".to_string(),
            //содержание
            "<description>".to_string(),
            "<title-info>".to_string(),
            "</title-info>".to_string(),
            "<author>".to_string(),
            "</author>".to_string(),
            "<annotation>".to_string(),
            "</annotation>".to_string(),
            "<coverpage>".to_string(),
            "</coverpage>".to_string(),
            "<history>".to_string(),
            "</history>".to_string(),
            "<publisher>".to_string(),
            "</publisher>".to_string(),
            "<document-info>".to_string(),
            "</document-info>".to_string(),
            "<publish-info>".to_string(),
            "</publish-info>".to_string(),
            "</description>".to_string(),
            "<lang>".to_string(),
            "<version>".to_string(),
             "<book-name>".to_string(),
              "<id>".to_string(),
              "<src-url>".to_string(),
             "<program-used>".to_string(),
                "<date value=".to_string(),
                "<city>".to_string(),
             "<year>".to_string(),
             "<isbn>".to_string(),
             "<image".to_string(),
             "<body>".to_string(),
             "</body>".to_string(),
            "<section>".to_string(),
             "</section>".to_string(),
                "<epigraph>".to_string(),
             "</epigraph>".to_string(),
              "<FictionBook>".to_string(),
             "</FictionBook>".to_string(),
            "<p>…</p>".to_string(),
            "<date>".to_string(),
                 "<nickname>".to_string(),
             "<empty-line/>".to_string(),
             "<title>".to_string(),
                "</title>".to_string(),
            "<section id=".to_string(),
            "</container>".to_string(),
                    "</rootfiles>".to_string(),
        ];
        static ref fb3_обязалово: [String;2] = ["<".to_string(), ">".to_string(),];
        static ref fb3_re_исключения: [Regex;48] = [
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
        ];
          static ref  re_первая_скобка:Regex= Regex::new(r"<").unwrap();
         static ref  re_вторая_скобка:Regex= Regex::new(r">").unwrap();
    }
    let исключения_для_проверки: HashSet<usize> = HashSet::from_iter([0]);

    if !проверка_образцов_для_кучи(
        &*fb3_исключения,
        &*fb3_re_исключения,
        &исключения_для_проверки,
    ) {
        panic!()
    }
    //получение значений

    //прогон
    let пропуски: HashSet<usize> = содержимое
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
        .collect::<HashSet<usize>>();
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
        return fb3_обязалово
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
        for образец in fb3_исключения_простые.iter() {
            if sz_найти(&стог_сена, &образец) {
                //  println!("b3_исключения_простые: {стог_сена}");
                return true;
            }
        }
        if fb3_исключения_простые
            .par_iter()
            .any(|образец| sz_найти(&стог_сена, &образец))
        {
            //println!("b3_исключения_простые: {стог_сена}");
            return true;
        }
        //проверка концов и окончаний строк
        //сначала что есть скобки
        return проверка_исключений_в_стоге_сена(
            &*fb3_исключения,
            &*fb3_re_исключения,
            &стог_сена,
        );
    }
}
pub fn fb2_получить_указатели_на_пропуки(
    содержимое: &Vec<String>,
) -> HashSet<usize> {
    lazy_static! {

        static ref fb2_исключения: [String;46] = [
            r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string(),
            //вложения
            "</section></body><binary".to_string(),
            //содержание
            "<description>".to_string(),
            "<title-info>".to_string(),
            "</title-info>".to_string(),
            "<author>".to_string(),
            "</author>".to_string(),
            "<annotation>".to_string(),
            "</annotation>".to_string(),
            "<coverpage>".to_string(),
            "</coverpage>".to_string(),
            "<history>".to_string(),
            "</history>".to_string(),
            "<publisher>".to_string(),
            "</publisher>".to_string(),
            "<document-info>".to_string(),
            "</document-info>".to_string(),
            "<publish-info>".to_string(),
            "</publish-info>".to_string(),
            "</description>".to_string(),
            "<lang>".to_string(),
            "<version>".to_string(),
             "<book-name>".to_string(),
              "<id>".to_string(),
              "<src-url>".to_string(),
             "<program-used>".to_string(),
                "<date value=".to_string(),
                "<city>".to_string(),
             "<year>".to_string(),
             "<isbn>".to_string(),
             "<image".to_string(),
             "<body>".to_string(),
             "</body>".to_string(),
            "<section>".to_string(),
             "</section>".to_string(),
                "<epigraph>".to_string(),
             "</epigraph>".to_string(),
              "<FictionBook>".to_string(),
             "</FictionBook>".to_string(),
            "<p>…</p>".to_string(),
            "<date>".to_string(),
                 "<nickname>".to_string(),
             "<empty-line/>".to_string(),
             "<title>".to_string(),
                "</title>".to_string(),
            "<section id=".to_string(),
        ];
        static ref fb2_обязалово: [String;2] = ["<".to_string(), ">".to_string(),];
        static ref fb2_re_исключения: [Regex;46] = [
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
        ];
          static ref  re_первая_скобка:Regex= Regex::new(r"<").unwrap();
         static ref  re_вторая_скобка:Regex= Regex::new(r">").unwrap();
    }
    let исключения_для_проверки: HashSet<usize> = HashSet::from_iter([0]);
    if !проверка_образцов_для_кучи(
        &*fb2_исключения,
        &*fb2_re_исключения,
        &исключения_для_проверки,
    ) {
        panic!()
    }
    //получение значений

    let пропуски: HashSet<usize> =
    //прогон
    содержимое
        .par_iter()
        .enumerate()
        .filter_map(|(указатель, строка)| {
            if !есть_ли_кириллица(&строка)|| fb2_проверка_содержимого_на_условия(&строка) {
                return Some(указатель)
            }
            else {None}
        }).collect::<HashSet<usize>>();
    return пропуски;
    //исключения для расширения
    //если возвращает истину - то переход на следующую строку
    fn fb2_обязательное_содержимое(стог_сена: &String) -> bool {
        //проверка что нет пустоты
        if стог_сена.is_empty() {
            return true;
        }
        //сначала что есть скобки
        return fb2_обязалово
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
            let количество_открытий = re_первая_скобка.find_iter(&стог_сена).count(); // считывает кол {  } 
            let количество_закрытий = re_вторая_скобка.find_iter(&стог_сена).count(); // считывает кол {  } 
            if количество_открытий == 1 && количество_закрытий == 1
            {
                return true;
            }
        }
        //проверка концов и окончаний строк
        //сначала что есть скобки
        return проверка_исключений_в_стоге_сена(
            &*fb2_исключения,
            &*fb2_re_исключения,
            &стог_сена,
        );
    }
    //если истина-то переход к следующей строке
}

fn html_получить_указатели_на_пропуски(
    содержимое: &[String],
) -> HashSet<usize> {
    let пропуски: HashSet<usize> = содержимое
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
        .collect::<HashSet<usize>>();
    return пропуски;

    fn html_проверка_содержимого_на_условия(
        стог_сена: &String,
    ) -> bool {
        lazy_static! {
            static ref html_исключения: [String; 1] = [r#"<!DOCTYPE html PUBLIC"#.to_string()];
            static ref html_исключения_с_проверкой: [String; 2] = [
                "<blockquote><div>".to_string(),
                "</div></body></html>".to_string(),
            ];
            static ref re_html_исключения_с_проверкой: [Regex; 2] = [
                Regex::new(r#"(?i)^\s*<blockquote><div>$"#).unwrap(),
                Regex::new(r#"(?i)^\s*</div></body></html>$"#).unwrap(),
            ];
        }
        //поиск
        if html_исключения
            .par_iter()
            .any(|образец| sz_найти(&стог_сена, образец))
        {
            return true;
        }
        return проверка_исключений_в_стоге_сена(
            &*html_исключения_с_проверкой,
            &*re_html_исключения_с_проверкой,
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
) -> HashSet<usize> {
    //fb2
    if расширение_книги.as_str() == "fb2".to_string() {
        return fb2_получить_указатели_на_пропуки(&содержимое);
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
                return html_получить_указатели_на_пропуски(&содержимое);
            }
        }
    }
    return HashSet::with_hasher(foldhash::fast::RandomState::default());
}

pub fn md_получить_указатели_на_пропуки(
    содержимое: &Vec<String>,
) -> HashSet<usize> {
    lazy_static! {
        static ref md_исключения: [String;25] = [
            r#"---"#.to_string(),
              r#"***"#.to_string(),
               r#"___"#.to_string(),
              r#"```"#.to_string(),
            //содержимое
             r#"<table>"#.to_string(),
               r#"</table>"#.to_string(),
            r#"<tr>"#.to_string(),
                    r#"</tr>"#.to_string(),
            r#"<td>"#.to_string(),
             r#"</td>"#.to_string(),
            r#"[comment]:"#.to_string(),
              r#"[!"#.to_string(),
            r#":::"#.to_string(),
              r#"<!"#.to_string(),
                   r#""resource""#.to_string(),
                r#"{"#.to_string(),
               r#"}"#.to_string(),
              r#"],"#.to_string(),
                    r#""**/"#.to_string(),
                r#"//"#.to_string(),
              r#"("#.to_string(),
              r#")"#.to_string(),
                r#"<tr><td>"#.to_string(),
              r#"/<tr><td>"#.to_string(),
            r#"ms.date"#.to_string(),

        ];
        static ref md_re_исключения: [Regex;25] = [
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
        ];
        static ref md_примечание:Regex= Regex::new(r#"(?i)^#"#).unwrap();
    }
    let исключения_для_проверки: HashSet<usize> =
        HashSet::from_iter([0, 1, 2, 3, 11, 12, 17, 18, 19, 24]);
    if !проверка_образцов_для_кучи(
        &*md_исключения,
        &*md_re_исключения,
        &исключения_для_проверки,
    ) {
        panic!()
    }
    //получение значений

    let пропуски: HashSet<usize> = содержимое
        .into_par_iter()
        .enumerate()
        .filter_map(|(указатель, строка_внутри)| {
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
        .collect::<HashSet<usize>>();
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
            &*md_исключения,
            &*md_re_исключения,
            &стог_сена,
        );
    }
    //если истина-то переход к следующей строке
}
pub fn проверка_исключений_в_стоге_сена(
    исключения: &[String],
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
fn проверка_образцов_для_кучи(
    исключения: &[String],
    исключения_re: &[Regex],
    исключения_проверки: &HashSet<usize>,
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
        есть_ли_повторно_строка_в_ряде(&исключения, "исключения_", false);
    }
    //перебор RE
    есть_ли_повторно_строка_в_ряде_regex(&исключения_re, "исключения_re");

    return true;
}

pub fn есть_ли_кириллица(стог_сена: &String) -> bool {
    lazy_static! {
        static ref малые_буквы: [char; 35] = [
            'а', 'б', 'в', 'г', 'д', 'ж', 'з', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н',
            'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю',
            'я',
        ];
        static ref большие_буквы: [char; 35] = [
            'А', 'Б', 'В', 'Г', 'Д', 'Ж', 'З', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н',
            'О', 'П', 'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю',
            'Я',
        ];
    }
    //малые буквы
    if малые_буквы
        .par_iter()
        .enumerate()
        .any(|(указатель, строка_внутри)| sz_найти(&стог_сена, &строка_внутри.to_string()))
    {
        return true;
    }
    //
    return большие_буквы
        .par_iter()
        .enumerate()
        .any(|(указатель, строка_внутри)| sz_найти(&стог_сена, &строка_внутри.to_string()));
}
