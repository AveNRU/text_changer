use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use foldhash::{HashMap, HashSet, HashSetExt, fast::RandomState};
use regex::Regex;
use std::fmt;
//use std::fs::File;
//use std::io::prelude::*;
use std::io::{Cursor, Seek, SeekFrom, Write};
//use crate::write;
use std::time::{
    //Duration,
    Instant,
};
extern crate rayon;
use rayon::prelude::*;

//use std::fs::read_to_string;
use crate::lib::{
    self,
    //Dictionary
};
use crate::utils::functions_add::system_pause;
use crate::utils::functions_txt::*;
use crate::utils::regex::re_получить_строку_с_описанием;
use lazy_static::lazy_static;
//use std::path::Path;
use clap::builder::Str;
use std::fs::{self, File, read_to_string};
use std::io::{
    //self,
    BufRead,
    BufReader, // Error,
    Read,      //Write
};
use std::sync::{Arc, Mutex};
use walkdir::{DirEntry, WalkDir};
//use xml::Encoding::Default;
use crate::utils::stringzilla::sz_найти;
use zip::{
    //write::FileOptions, CompressionMethod,
    ZipArchive,
    ZipWriter,
};
//use std::collections::HashMap;

lazy_static! {
    static ref re_расширение_файла: Regex = Regex::new(r"(?i)(?:\.)+([\d\w&&[^\.]]+)$").unwrap();//расширение файла
    static ref re_имя_файла: Regex = Regex::new(r"\.\\*/*books\\*/*(.+)\.(?:[\d\w&&[^\.]]+)").unwrap();//расширение файла
     //имя словаря вырезать




}

//Чтение файлов
//1 - книги, 2 - словари
pub fn считать_словари() -> Vec<String> {
    use crate::utils::read::получить_содержимое;
    //основной путь
    let пути_общие: lib::Пути_Общие = Default::default();
    //получение значение корневого доступа к скрипту (где он лежит, как решила ОС)
    //получение словарей
    let пути_до_словарей: Vec<String> = получить_содержимое(&пути_общие.словари);
    return пути_до_словарей;
}

pub fn считать_книги(
    сообщения_приход: &mut lib::Сообщения
) -> Vec<lib::Книги> {
    use crate::utils::functions::*;
    use crate::utils::functions_add::прочитать_содержимое_построчно;
    use crate::utils::read::*;
    use crate::utils::regex::*;
    use crate::utils::zip::{zip_архив_в_память, Архив_в_озу};
    use std::default::Default;
    //основной путь
    let пути_общие: lib::Пути_Общие = Default::default();
    let mut содержимое_папки: Arc<Mutex<lib::Содержимое_папок>> =
        Arc::new(Mutex::new(lib::Содержимое_папок::default()));
    //получение значение корневого доступа к скрипту (где он лежит, как решила ОС)
    //let полный_путь: String = полный_путь_до_файла().unwrap();
    //let стопки_книг: Mutex<Vec<lib::Книги>> = Mutex::new(Vec::new());
    //получение книг
    let пути_до_книг: Vec<String> = получить_содержимое(&пути_общие.книги);
    //получение словарей
    //чтение содержимого файлов в разделе книг
    let сообщения: Arc<Mutex<lib::Сообщения>> = Arc::new(Mutex::new(lib::Сообщения::default()));
    //получение книг
    let стопки_книг: Vec<lib::Книги>=
        пути_до_книг.par_iter().enumerate().filter_map(|(i,путь)|{
        //открытие файла с библиотекой
        //расширение файла
        let расширение: String = re_получить_строку_с_описанием(
            &путь,
            &re_расширение_файла,
            "Не удалось выдрать расширение файла",
        );
        //имя файла
        let название_книги: String = определить_имя_книги(путь);
        //если .rtf либо .fb2 (1 файл содержит)
        if fb2_rtf_mhtml(&путь) {
          
            let содержимое: Vec<String> = {
                if расширение.contains("mhtlml") {
                    read_utf8(&путь) //чтение файла в UTF-8
                } else {
                    //прочитать_содержимое_построчно(&путь)
                    read_utf8(&путь) //чтение файла в UTF-8
                }
            };
            //вложение
            let стопка: Vec<lib::Вложения> = vec![lib::Вложения {
                содержимое: содержимое,
                содержимое_в_байтах: Vec::new(),
                имя: название_книги.clone(),
                имя_без_пути: re_получить_имя_файла_без_пути(
                    &название_книги,
                ),
            }];
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.lock().unwrap().файлы,
                &путь,
            );
            //вложение не архива в стопку
            Some(lib::Книги {
                вложения: стопка,
                //содержимое:содержимое,
                путь: путь.clone(),             //путь полный
                название_книги: название_книги, //имя книги
                расширение,
                ..Default::default()
            })
            //стопки_книг.push(книга);
        }
        //если архивный файл
        else if fb3_epub(&путь) {
            let mut сообщения_свои :lib::Сообщения=lib::Сообщения::default();
            // сообщения.lock().unwrap().чтение_книг.extend(сообщения_свои.чтение_книг);
            //println!("это архив");
            let mut книга_в_озу: Архив_в_озу =
                foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
            match zip_архив_в_память(&путь, &mut книга_в_озу) {
                Ok(успех) => успех,
                Err(ошибка) => {
                    if sz_найти(&ошибка.to_string(), "Пустой файл") {
                        вывод_сообщения_на_экран_и_вложение_в_ряд(
                            format!("Запись2:  книга {}. Пустое содержимого. Перезапись", путь),
                            &mut сообщения_свои.чтение_книг,
                        );
                        //return None;
                    } else {
                        panic!("Ошибка при распаковке файла в архив: {путь}")
                    }
                }
            }
            //zip_архив_в_память(&путь, &mut книга_в_озу).unwrap();
            let приложения_книги: Vec<lib::Вложения> =
            //перебор всех файлов архива
            // let mut указатель_изображения:usize=0;
             книга_в_озу.into_par_iter().filter_map(|(имя, содержимое_архива)|  {
           // for (имя, содержимое_архива) in книга_в_озу.into_iter() {
                //картинки не загонять в utf8
                if изображение_расширение_с_точкой(&имя) || sz_найти(&имя, ".ttf")
                {
                    Some(lib::Вложения {
                        содержимое: Vec::new(), //пустота - так это рисунок, нельзя читать
                        имя_без_пути:
                            re_получить_имя_файла_без_пути(&имя),
                        имя,
                        содержимое_в_байтах: содержимое_архива.clone(),
                    })
                }
                //если это не изображение и  не шриафты
                else {
                    Some(lib::Вложения {
                        содержимое: read_utf8_из_ряда_u8(&содержимое_архива, &имя),
                        имя_без_пути:
                            re_получить_имя_файла_без_пути(&имя),
                        имя,
                        содержимое_в_байтах: содержимое_архива.clone(),
                    })
                }
            }).collect();
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.lock().unwrap().файлы,
                &путь,
            );
            сообщения.lock().unwrap().чтение_книг.extend(сообщения_свои.чтение_книг);
            let архив: foldhash::HashMap<String, Vec<u8>> =
                foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
            //вложение содержимого всего архива в стопку
            Some(lib::Книги {
                вложения: приложения_книги,
                архив,
                путь: путь.clone(),             //путь полный
                название_книги: название_книги, //имя книги
                расширение,
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
            let архив: foldhash::HashMap<String, Vec<u8>> =
                foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
            Some(lib::Книги {
                вложения: Vec::new(),
                архив,
                путь: путь.clone(),             //путь полный
                название_книги: название_книги, //имя книги
                расширение,
            })
        } else if md_fs_yml(&путь) {
            let содержимое: Vec<String> = read_utf8(&путь); //чтение файла в UTF-8
            let стопка: Vec<lib::Вложения> = vec![lib::Вложения {
                содержимое: содержимое,
                имя: название_книги.clone(),
                имя_без_пути: re_получить_имя_файла_без_пути(
                    &название_книги,
                ),
                содержимое_в_байтах: Vec::new(),
            }];
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.lock().unwrap().файлы,
                &путь,
            );
            //вложение не архива в стопку
            Some(lib::Книги {
                вложения: стопка,
                //содержимое:содержимое,
                путь: путь.clone(),             //путь полный
                название_книги: название_книги, //имя книги
                расширение,
                ..Default::default()
            })
            //стопки_книг.push(книга);
        }
        // else if является_ли_md() {}
        else {
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.lock().unwrap().не_вложено,
                &format!("{}", путь),
            );
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.lock().unwrap().ошибки,
                &format!("{} разрешение файла не определено", путь),
            );
            return None
        }
    }).collect();
    let mut сообщения = Arc::try_unwrap(сообщения).unwrap().into_inner().unwrap();
    let mut содержимое_папки = Arc::try_unwrap(содержимое_папки).unwrap().into_inner().unwrap();
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
    return стопки_книг;
}
