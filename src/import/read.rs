//use encoding_rs::WINDOWS_1251;
//use encoding_rs_io::DecodeReaderBytesBuilder;
//use foldhash::{rapidhash::fast::RapidHashMap, HashSet, HashSetExt, fast::RandomState};
use regex::Regex;
//use std::fmt;
//use std::fs::File;
//use std::io::prelude::*;
//use std::io::{Cursor, Seek, SeekFrom, Write};
//use crate::write;
use crate::utils::functions::вложить_строку_в_ряд_с_проверкой;
use crate::utils::functions::вывод_сообщения_на_экран_и_вложение_в_ряд;
use crate::utils::read::*;
use crate::utils::zip::zip_архив_в_память;
use crate::utils::zip::Архив_в_озу;
use console::style;
use std::sync::LazyLock;
use std::time::{
    //Duration,
    Instant,
};
extern crate rayon;
use rayon::prelude::*;

use Text_Changer::{
    self,
    //Dictionary
};
//use crate::utils::functions_add::system_pause;
//use crate::utils::functions_txt::*;
use crate::utils::regex::re_получить_строку_с_описанием;

//use std::path::Path;
//use clap::builder::Str;
use std::fs::File;
use std::sync::{Arc, Mutex};
//use walkdir::{DirEntry, WalkDir};
//use xml::Encoding::Default;
use crate::ALLOCATOR;
use crate::utils::stringzilla::sz_найти;
//use Text_Changer::Кодировка;
//use cap::Cap;
//use std::alloc;

//use std::collections::rapidhash::fast::RapidHashMap;

// 1. Публичная регулярка
pub static RE_РАСШИРЕНИЕ_ФАЙЛА_C_ТОЧКОЙ: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:\.)+([\d\w\s&&[^\.]]+)$").unwrap());
/*
// 2. Без точки
static RE_РАСШИРЕНИЕ_ФАЙЛА_БЕЗ_ТОЧКИ: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:\/.)+([\d\w]+)$").unwrap());

// 3. Третий случай
static RE_РАСШИРЕНИЕ_ФАЙЛА_3_Й_СЛУЧАЙ: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:\\)+([\d\w_-]+)$").unwrap());
*/

// 4. Имя файла
//static RE_ИМЯ_ФАЙЛА: LazyLock<Regex> =
//  LazyLock::new(|| Regex::new(r"\.\*/*книги\\*/*(.+)\.(?:[\d\w&&[^\.]]+)").unwrap());*/
//имя словаря вырезать

use crate::utils::read::получить_содержимое;
//Чтение файлов
//1 - книги, 2 - словари
pub fn считать_словари() -> Vec<String> {
    //основной путь
    let пути_общие: Text_Changer::Пути_Общие = Default::default();
    //получение значение корневого доступа к скрипту (где он лежит, как решила ОС)
    //получение словарей
    let пути_до_словарей: Vec<String> = получить_содержимое(&пути_общие.словари);
    return пути_до_словарей;
}

pub fn считать_книги(
    сообщения_приход: &mut Text_Changer::Сообщения,
) -> (Vec<Text_Changer::Книги>, usize) {
    // use crate::utils::functions::*;
    //use crate::utils::functions_add::прочитать_содержимое_построчно;
    //use crate::utils::read::*;
    use crate::utils::regex::{
        doc_docx, fb2_rtf_mht_mhtml, fb3_epub, htm_html_xhtml, md_fs_yml,
        re_получить_имя_файла_без_пути, изображение_расширение_с_точкой,
        мусорное_содержимое_архивов, определить_имя_книги,
    };
    //use crate::utils::zip::{zip_архив_в_память, Архив_в_озу};
    use std::default::Default;
    //время, занятое на этот шаг
    let точка_отсчёта_по_времени: Instant = Instant::now();
    // Set the limit to 30000MiB.
    ALLOCATOR.set_limit(30000 * 1024 * 1024).unwrap();
    // ...
    // println!("Выделено памяти: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    //основной путь
    let пути_общие: Text_Changer::Пути_Общие = Default::default();
    let содержимое_папки: Arc<Mutex<Text_Changer::Содержимое_папок>> =
        Arc::new(Mutex::new(Text_Changer::Содержимое_папок::default()));
    //
    let куча_нераспознанных_расширений: Arc<Mutex<rapidhash::fast::RapidHashSet<String>>> =
        Arc::new(Mutex::new(rapidhash::fast::RapidHashSet::default()));
    //вывод этапа
    println!("{}", style(format!("\t[1/4]: Считывание книг")).yellow(),);
    //получение значение корневого доступа к скрипту (где он лежит, как решила ОС)
    //let полный_путь: String = полный_путь_до_файла().unwrap();
    //let стопки_книг: Mutex<Vec<Text_Changer::Книги>> = Mutex::new(Vec::new());
    //получение книг
    let пути_до_книг: Vec<String> = получить_содержимое(&пути_общие.книги);
    //получение словарей
    //чтение содержимого файлов в разделе книг
    let сообщения: Arc<Mutex<Text_Changer::Сообщения>> =
        Arc::new(Mutex::new(Text_Changer::Сообщения::default()));
    //получение книг
    let стопки_книг: Vec<Text_Changer::Книги> = пути_до_книг
        .par_iter()
        .enumerate()
        .filter_map(|(_i, путь)| {
            if sz_найти(&путь, r#"\"#) | sz_найти(&путь, r#"\\"#) {
                println!("путь с палкой: {путь}");
            };
            //открытие файла с библиотекой
            //расширение файла
            let mut расширение: String =
                match re_получить_строку_с_описанием(
                    &путь,
                    &RE_РАСШИРЕНИЕ_ФАЙЛА_C_ТОЧКОЙ,
                    "Не удалось выдрать расширение файла с точкой",
                ) {
                    Ok(путь) => путь,
                    Err(_) => "".to_string(),
                    /*match re_получить_строку_с_описанием(
                                        &путь,
                                        &re_расширение_файла_без_точки,
                                        "Не удалось выдрать расширение файла без точки",
                                    ) {

                                        Ok(исход_2) => исход_2,
                                        Err(_) =>
                                            match re_получить_строку_с_описанием(
                                                &путь,
                                                &re_расширение_файла_3_й_случай,
                                                "Не удалось выдрать расширение файла 3-й случай",
                                            ) {
                                                Ok(исход_3)=>исход_3,
                                                Err(_) =>panic!("Не удалось извлечь расширение файла без точки и с точкой: {}", путь),
                                    }
                                    }
                    */
                };
            let расширение_подробно: Text_Changer::Основной_Вид_Расширения = определить_расширение_подробно(
                &расширение.to_lowercase(),
                &mut куча_нераспознанных_расширений.lock().unwrap());
            if расширение.len() > 6 && !sz_найти(&расширение, "Без названия")
            {
                println!("Путь, где расширение больше 6 букв: {путь}");
                println!("Расширение больше 6 букв: {расширение}");
                расширение = "".to_string()
            };
            //имя файла
            let название_книги: String = определить_имя_книги(путь);
            //если .rtf либо .fb2 (1 файл содержит)
            if fb2_rtf_mht_mhtml(&путь) {
                // println!("вхождение: fb2_rtf_mht_mhtml");
                let содержимое: Vec<String> = {
                    if sz_найти(&расширение, "mhtlml") {
                        считать_в_utf8(&путь) //чтение файла в UTF-8
                    } else {
                        //прочитать_содержимое_построчно(&путь)
                        считать_в_utf8(&путь) //чтение файла в UTF-8
                    }
                };

                //вложение
                let стопка: Vec<Text_Changer::Вложения> = vec![Text_Changer::Вложения {
                    содержимое: содержимое,
                    содержимое_в_байтах: Vec::new(),
                    имя: название_книги.clone(),
                    имя_без_пути: re_получить_имя_файла_без_пути(
                        &название_книги,
                    ),
                    кодировка: Text_Changer::Кодировка::Utf8,
                }];
                вложить_строку_в_ряд_с_проверкой(
                    &mut содержимое_папки.lock().unwrap().файлы,
                    &путь,
                );
                //вложение не архива в стопку
                Some(Text_Changer::Книги {
                    вложения: стопка,
                    //содержимое:содержимое,
                    путь: путь.clone(),             //путь полный
                    название_книги: название_книги, //имя книги
                    расширение,
                    книга_ли: true,
                    архив: Default::default(),
                     расширение_подробно,
                })
                //стопки_книг.push(книга);
            }
            //если архивный файл
            else if fb3_epub(&путь) {
                let mut сообщения_свои: Text_Changer::Сообщения = Text_Changer::Сообщения::default();
                let книга_в_озу: Архив_в_озу = match zip_архив_в_память(
                    &путь,
                    rapidhash::fast::RapidHashMap::with_hasher(rapidhash::fast::RandomState::default()),
                ) {
                    Ok(успех) => успех,
                    Err(ошибка) => {
                        if sz_найти(&ошибка.to_string(), "Пустой файл") {
                            вывод_сообщения_на_экран_и_вложение_в_ряд(
                            format!("☢️ Запись2:  книга {}. Пустое содержимое книги", путь),
                            &mut сообщения_свои.чтение_книг,
                        );
                            rapidhash::fast::RapidHashMap::with_hasher(rapidhash::fast::RandomState::default())
                            //return None;
                        } else {
                            panic!("Ошибка при распаковке файла в архив: {путь}")
                        }
                    }
                };
                let приложения_книги: Vec<Text_Changer::Вложения> =
            //перебор всех файлов архива
             книга_в_озу.into_par_iter().filter_map(|(имя, содержимое_архива)|  {
                //картинки не загонять в utf8
                if изображение_расширение_с_точкой(&имя) || мусорное_содержимое_архивов(&имя)
                {
                    Some(Text_Changer::Вложения {
                        содержимое: Vec::new(), //пустота - так это рисунок, нельзя читать
                        имя_без_пути:
                            re_получить_имя_файла_без_пути(&имя),
                        имя,
                        содержимое_в_байтах: содержимое_архива.clone(),
                        кодировка:Text_Changer::Кодировка::Не_определено,
                    })
                }
                //если это не изображение и  не шриафты
                else {

                    let содержимое_файла: Vec<String> =read_utf8_из_ряда_u8(&содержимое_архива, &имя);
                    //если это htm страница
                    if htm_html_xhtml(&имя) {
                        let mut сообщения_свои: Text_Changer::Сообщения = Text_Changer::Сообщения::default();
                        let новое_содержимое_файла: Vec<String> =htm_utf8_без_переносов_строк(&содержимое_файла,&имя);
                        let кодировка: Text_Changer::Кодировка=определить_кодировку(&новое_содержимое_файла,&имя,&путь,&mut сообщения_свои.кодировка);
                       //
                        сообщения
                            .lock()
                            .unwrap()
                            .кодировка
                            .extend(сообщения_свои.кодировка);
                       // println!("содержимое xhtml: {}",re_получить_имя_файла_без_пути(&имя));
                        Some(Text_Changer::Вложения {
                            содержимое: новое_содержимое_файла,
                            имя_без_пути:
                            re_получить_имя_файла_без_пути(&имя),
                            имя,
                            содержимое_в_байтах: содержимое_архива.clone(),
                            кодировка,
                        })
                    }
                    //если это не htm содержимое
                    else {
                        let mut сообщения_свои: Text_Changer::Сообщения = Text_Changer::Сообщения::default();
                        let кодировка: Text_Changer::Кодировка=определить_кодировку(&содержимое_файла,&имя,&путь,&mut сообщения_свои.кодировка);
                        //
                        сообщения
                            .lock()
                            .unwrap()
                            .кодировка
                            .extend(сообщения_свои.кодировка);
                        //
                        Some(Text_Changer::Вложения {
                            содержимое: содержимое_файла,
                            имя_без_пути:
                            re_получить_имя_файла_без_пути(&имя),
                            имя,
                            содержимое_в_байтах: содержимое_архива.clone(),
                            кодировка:кодировка,
                        })
                    }
                }
            }).collect();
                вложить_строку_в_ряд_с_проверкой(
                    &mut содержимое_папки.lock().unwrap().файлы,
                    &путь,
                );
                сообщения
                    .lock()
                    .unwrap()
                    .чтение_книг
                    .extend(сообщения_свои.чтение_книг);
                сообщения
                    .lock()
                    .unwrap()
                    .кодировка
                    .extend(сообщения_свои.кодировка);
                let архив: rapidhash::fast::RapidHashMap<String, Vec<u8>> = rapidhash::fast::RapidHashMap::with_hasher(rapidhash::fast::RandomState::default());
                //вложение содержимого всего архива в стопку
                Some(Text_Changer::Книги {
                    вложения: приложения_книги,
                    архив,
                    путь: путь.clone(),             //путь полный
                    название_книги: название_книги, //имя книги
                    расширение,
                    книга_ли: true,
                     расширение_подробно,
                })
            } else if doc_docx(&путь) {
                /*use docx_rust::document::Paragraph;
                use docx_rust::Docx;
                use docx_rust::DocxFile;

                let docx = DocxFile::from_file(&ряд_книги[i]).unwrap();
                let mut docx = docx.parse().unwrap();

                let строка = Paragraph::default().push_text("Lorem Ipsum");
                docx.document.push(строка);

                docx.write_file(format!("./проверка/{}.docx",название_книги)).unwrap();*/
                вложить_строку_в_ряд_с_проверкой(
                    &mut содержимое_папки.lock().unwrap().файлы,
                    &название_книги,
                );
                let архив: rapidhash::fast::RapidHashMap<String, Vec<u8>> = rapidhash::fast::RapidHashMap::with_hasher(rapidhash::fast::RandomState::default());
                Some(Text_Changer::Книги {
                    вложения: Vec::new(),
                    архив,
                    путь: путь.clone(),             //путь полный
                    название_книги: название_книги, //имя книги
                    расширение,
                    книга_ли: true,
                     расширение_подробно,
                })
            } else if md_fs_yml(&путь) {
                let mut сообщения_свои: Text_Changer::Сообщения = Text_Changer::Сообщения::default();
                //   println!("вхождение: md_fs_yml");
                let содержимое: Vec<String> = считать_в_utf8(&путь); //чтение файла в UTF-8
                let кодировка: Text_Changer::Кодировка =
                    определить_кодировку(&содержимое,&название_книги,&путь,&mut сообщения_свои.кодировка);
                let стопка: Vec<Text_Changer::Вложения> = vec![Text_Changer::Вложения {
                    содержимое: содержимое,
                    имя: название_книги.clone(),
                    имя_без_пути: re_получить_имя_файла_без_пути(
                        &название_книги,
                    ),
                    содержимое_в_байтах: Vec::new(),
                    кодировка,
                }];
                вложить_строку_в_ряд_с_проверкой(
                    &mut содержимое_папки.lock().unwrap().файлы,
                    &путь,
                );
                //
                сообщения
                    .lock()
                    .unwrap()
                    .кодировка
                    .extend(сообщения_свои.кодировка);
                //вложение не архива в стопку
                Some(Text_Changer::Книги {
                    вложения: стопка,
                    //содержимое:содержимое,
                    путь: путь.clone(),             //путь полный
                    название_книги: название_книги, //имя книги
                    расширение,
                    книга_ли: true,
                    архив: Default::default(),
                     расширение_подробно,
                })
                //стопки_книг.push(книга);
            } else if htm_html_xhtml(&путь) {
                let mut сообщения_свои: Text_Changer::Сообщения = Text_Changer::Сообщения::default();
                //      println!("вхождение: htm_html_xhtml");
                let содержимое: Vec<String> =
                    считать_в_utf8_без_переносов_строк_htm_не_архив(&путь, &название_книги); //чтение файла в UTF-8
                let кодировка: Text_Changer::Кодировка =
                    определить_кодировку(&содержимое,&название_книги,&путь,&mut сообщения_свои.кодировка);
                let стопка: Vec<Text_Changer::Вложения> = vec![Text_Changer::Вложения {
                    содержимое: содержимое,
                    имя: название_книги.clone(),
                    имя_без_пути: re_получить_имя_файла_без_пути(
                        &название_книги,
                    ),
                    содержимое_в_байтах: Vec::new(),
                    кодировка,
                }];
                вложить_строку_в_ряд_с_проверкой(
                    &mut содержимое_папки.lock().unwrap().файлы,
                    &путь,
                );
                //
                сообщения
                    .lock()
                    .unwrap()
                    .кодировка
                    .extend(сообщения_свои.кодировка);
                //вложение не архива в стопку
                Some(Text_Changer::Книги {
                    вложения: стопка,
                    //содержимое:содержимое,
                    путь: путь.clone(),             //путь полный
                    название_книги: название_книги, //имя книги
                    расширение,
                    книга_ли: true,
                    архив: Default::default(),
                     расширение_подробно,
                    //  ..Default::default()
                })
            }
            // если вложения - не книга
            else {
                //     println!("вхождение: если вложения - не книга");
                let имя_файла_без_пути =
                    re_получить_имя_файла_без_пути(&название_книги);
                use std::io::{ Read};
                //если изображение
                if изображение_расширение_с_точкой(&путь)
                    || !sz_найти(&имя_файла_без_пути, ".")
                {
                    let mut file = File::open(&путь).unwrap();
                    let mut содержимое: Vec<u8> = Vec::new();
                    file.read_to_end(&mut содержимое).unwrap();
                    let стопка: Vec<Text_Changer::Вложения> = vec![Text_Changer::Вложения {
                        содержимое: Vec::new(), //пустота - так это рисунок, нельзя читать
                        имя: название_книги.clone(),
                        имя_без_пути: имя_файла_без_пути,
                        содержимое_в_байтах: содержимое,
                        кодировка: Text_Changer::Кодировка::Не_определено,
                    }];
                    Some(Text_Changer::Книги {
                        вложения: стопка,
                        //содержимое:содержимое,
                        путь: путь.clone(),             //путь полный
                        название_книги: название_книги, //имя книги
                        расширение,
                        книга_ли: false,
                        архив: Default::default(),
                        расширение_подробно,
                        //..Default::default()
                    })
                } else {
                    let mut сообщения_свои: Text_Changer::Сообщения = Text_Changer::Сообщения::default();
                    //         println!("вхождение: если не изображение");
                    //если не изображение
                    let содержимое: Vec<String> = считать_в_utf8(&путь); //чтение файла в UTF-8
                    let кодировка: Text_Changer::Кодировка =
                        определить_кодировку(&содержимое,&название_книги,&путь,&mut сообщения_свои.кодировка);
                    let стопка: Vec<Text_Changer::Вложения> = vec![Text_Changer::Вложения {
                        содержимое: содержимое,
                        имя: название_книги.clone(),
                        имя_без_пути: имя_файла_без_пути,
                        содержимое_в_байтах: Vec::new(),
                        кодировка,
                    }];
                    //
                    сообщения
                        .lock()
                        .unwrap()
                        .кодировка
                        .extend(сообщения_свои.кодировка);
                    //
                    Some(Text_Changer::Книги {
                        вложения: стопка,
                        //содержимое:содержимое,
                        путь: путь.clone(),             //путь полный
                        название_книги: название_книги, //имя книги
                        расширение,
                        книга_ли: false,
                        архив: Default::default(),
                        расширение_подробно,
                        //..Default::default()
                    })
                }
                /*вложить_строку_в_ряд_с_проверкой(
                    &mut содержимое_папки.lock().unwrap().не_вложено,
                    &format!("{}", путь),
                );
                вложить_строку_в_ряд_с_проверкой(
                    &mut содержимое_папки.lock().unwrap().ошибки,
                    &format!("{} разрешение файла не определено", путь),
                );*/
                //return None;
            }
        })
        .collect();
    let mut сообщения = Arc::try_unwrap(сообщения).unwrap().into_inner().unwrap();
    let mut содержимое_папки = Arc::try_unwrap(содержимое_папки)
        .unwrap()
        .into_inner()
        .unwrap();
    //перебор содержимого книги на предмет наличия трех точек подряд
    crate::output::write::вывод_содержимого_папок_по_умолчанию(
        &mut содержимое_папки,
        "книги",
        &mut сообщения.общие,
    )
    .unwrap();
    crate::output::dir::заменить_путь_выходным_книгам(
        &mut содержимое_папки.файлы,
        "книги",
    );

    сообщения
        .проверка_после_замен
        .extend(vec![Default::default(); стопки_книг.len()]);
    //println!("количество в стопке: {}",&сообщения.проверка_после_замен.len());
    *сообщения_приход = сообщения;
    //for i in 0..стопки_книг.len() {
    //    println!("#:{i}, путь: {}, расширение:{}, название книги: {}",стопки_книг[i].путь,стопки_книг[i].расширение,стопки_книг[i].название_книги)
    // }
    //println!("Выделено памяти: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    //вывод этапа
    let количество_мегабайт: usize = (ALLOCATOR.allocated() / 1024) / 1024;
    println!(
        "{}{}",
        style(format!(
            "\tВыделено памяти на этапе [1/4]: Считывание книг:  "
        ))
        .bold(),
        style(format!("Мегабайт: {}", количество_мегабайт))
            .blue()
            .bold(),
    );
    //вывод отчёта по времени, занятого на этот шаг
    println!(
        "{}",
        style(format!(
            "⌚  Время занятое на чтение книг: {:.2?}",
            точка_отсчёта_по_времени.elapsed()
        ))
        .true_color(154, 136, 252)
        .blink()
    );
    //
    let куча_нераспознанных_расширений: rapidhash::fast::RapidHashSet<String> =
        Arc::try_unwrap(куча_нераспознанных_расширений)
            .unwrap()
            .into_inner()
            .unwrap();
    //
    for расширение in куча_нераспознанных_расширений {
        println!("Не распознанное расширение: {}", расширение);
    }

    return (стопки_книг, количество_мегабайт);
}
pub fn определить_расширение_подробно(
    расширение: &str,
    куча: &mut rapidhash::fast::RapidHashSet<String>,
) -> Text_Changer::Основной_Вид_Расширения {
    match расширение {
        "txt" => Text_Changer::Основной_Вид_Расширения::Простая_письменность(Text_Changer::Вид_простой_письменности::Txt),
        "docx" => Text_Changer::Основной_Вид_Расширения::Word(Text_Changer::Вид_Word::Docx),
        "xlsx" => Text_Changer::Основной_Вид_Расширения::Excel(Text_Changer::Вид_Excel::Xlsx),
        "html" => Text_Changer::Основной_Вид_Расширения::Разметка_Паутины(Text_Changer::Вид_Разметки_Паутины::Html),
        "htm" => Text_Changer::Основной_Вид_Расширения::Разметка_Паутины(Text_Changer::Вид_Разметки_Паутины::Htm),
        "png" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Png),
        "jpg" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Jpg),
        "jpeg" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Jpeg),
        "svg" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Svg),
        "gif" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Gif),
        "bmp" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Bmp),
        "tif" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Tif),
        "avif" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Avif),
        "webp" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Webp),
        "wmf" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Wmf),
        "wpg" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Wpg),
        "eps" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Eps),
        "epub" => Text_Changer::Основной_Вид_Расширения::Книга(Text_Changer::Вид_Книги::Архивная(Text_Changer::Вид_Архивной_Книги::Epub)),
        "fb3" => Text_Changer::Основной_Вид_Расширения::Книга(Text_Changer::Вид_Книги::Архивная(Text_Changer::Вид_Архивной_Книги::Fb3)),
        "php" => Text_Changer::Основной_Вид_Расширения::Разметка_Паутины(Text_Changer::Вид_Разметки_Паутины::Php),
        "html" => Text_Changer::Основной_Вид_Расширения::Разметка_Паутины(Text_Changer::Вид_Разметки_Паутины::Html),
        "tif" => Text_Changer::Основной_Вид_Расширения::Шрифты(Text_Changer::Вид_Шрифтов::Tif),
        "css" => Text_Changer::Основной_Вид_Расширения::Мусорные_Разметка(Text_Changer::Вид_Мусорные_Разметки_Паутины::Css),
        "fb2" => Text_Changer::Основной_Вид_Расширения::Книга(Text_Changer::Вид_Книги::Одиночная(Text_Changer::Вид_одичноной_книги::Fb2)),
        "xml" => Text_Changer::Основной_Вид_Расширения::XML,
        "gz" => Text_Changer::Основной_Вид_Расширения::Архив(Text_Changer::Вид_Архива::Gz),
        "gzip" => Text_Changer::Основной_Вид_Расширения::Архив(Text_Changer::Вид_Архива::Gzip),
        "emf" => Text_Changer::Основной_Вид_Расширения::Изображение(Text_Changer::Вид_Изображения::Emf),
        "js" => Text_Changer::Основной_Вид_Расширения::JS(Text_Changer::Вид_JS::Js),
        "mjs" => Text_Changer::Основной_Вид_Расширения::JS(Text_Changer::Вид_JS::Mjs),
        "cjs" => Text_Changer::Основной_Вид_Расширения::JS(Text_Changer::Вид_JS::Cjs),
        "thmx" => Text_Changer::Основной_Вид_Расширения::Мусорные_Разметка(Text_Changer::Вид_Мусорные_Разметки_Паутины::Thmx),
        "tcl" => Text_Changer::Основной_Вид_Расширения::Приказы(Text_Changer::Вид_приказов::Tcl),
        "fcg" => Text_Changer::Основной_Вид_Расширения::Приказы(Text_Changer::Вид_приказов::Fcg),
        "cgi" => Text_Changer::Основной_Вид_Расширения::Приказы(Text_Changer::Вид_приказов::Cgi),
        "без названия" => Text_Changer::Основной_Вид_Расширения::Без_Названия,
        "cnt" => Text_Changer::Основной_Вид_Расширения::Справка(Text_Changer::Вид_Справи::Cnt),
        "hlp" => Text_Changer::Основной_Вид_Расширения::Справка(Text_Changer::Вид_Справи::Hlp),
        "chm" => Text_Changer::Основной_Вид_Расширения::Справка(Text_Changer::Вид_Справи::Chm),


        _ => {
            //println!("Расширение '{}' не поддерживается", расширение);
            куча.insert(расширение.to_string());
            Text_Changer::Основной_Вид_Расширения::Не_определено
        },
    }
}
