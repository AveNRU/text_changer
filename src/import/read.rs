use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use foldhash::{HashMap, HashSet, HashSetExt, fast::RandomState, quality::FixedState};
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
use std::fs::{self, read_to_string};
use std::io::{
    //self,
    BufRead,
    BufReader, // Error,
    Read,      //Write
};
use walkdir::{DirEntry, WalkDir};
//use xml::Encoding::Default;
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
    use crate::utils::functions::{
        заменить_все_палки, полный_путь_до_файла, строка_удалить_utf8_концы_строк,
    };
    use crate::utils::functions_add::прочитать_содержимое_построчно;
    use crate::utils::read::{
        read_utf8, получить_содержимое, попытка_открыть_файл
    };
    //основной путь
    let пути_общие: lib::Пути_Общие = Default::default();
    //получение значение корневого доступа к скрипту (где он лежит, как решила ОС)
    //let полный_путь: String = полный_путь_до_файла().unwrap();
    //получение словарей
    let пути_до_словарей: Vec<String> = получить_содержимое(&пути_общие.словари);
    //println!("ряд_путей_к_словарям: {:?}", &пути_до_словарей);
    //for i in 0..пути_до_словарей.len() {
    //    println!("словарь! {i}: {}", пути_до_словарей[i])
    // }
    return пути_до_словарей;
}
pub fn поиск_путей(ряд: &Vec<String>) {
    use crate::output::functions::проверка_наличия_папок_в_случае_их_отсутствия_создать_папки;
    let mut папки: Vec<String> = Vec::new();
    for строка in ряд.iter() {}
}
pub fn считать_книги(
    сообщения: &mut lib::Сообщения
) -> Vec<lib::Книги> {
    use crate::utils::functions::{
        полный_путь_до_файла, строка_удалить_utf8_концы_строк
    };
    use crate::utils::functions_add::прочитать_содержимое_построчно;
    use crate::utils::read::{
        read_utf8, получить_содержимое, попытка_открыть_файл
    };
    use crate::utils::regex::*;
    use crate::utils::zip::{VirtualFs, zip_архив_в_память};
    use std::default::Default;
    //основной путь
    let пути_общие: lib::Пути_Общие = Default::default();
    let mut содержимое_папки: lib::Содержимое_папок = Default::default();
    //получение значение корневого доступа к скрипту (где он лежит, как решила ОС)
    //let полный_путь: String = полный_путь_до_файла().unwrap();
    let mut стопки_книг: Vec<lib::Книги> = Vec::new();
    //получение книг
    let пути_до_книг: Vec<String> = получить_содержимое(&пути_общие.книги);
    //получение словарей
    //чтение содержимого файлов в разделе книг

    for i in 0..пути_до_книг.len() {
        //открытие файла с библиотекой
        //попытка_открыть_файл(&пути_до_книг[i]);
        //расширение файла
        let расширение: String = re_получить_строку_с_описанием(
            &пути_до_книг[i],
            &re_расширение_файла,
            "Не удалось выдрать расширение файла",
        );
        //имя файла
        let название_книги: String = определить_имя_книги(&пути_до_книг[i]);
        //получение расширения файла
        //если .rtf либо .fb2 (1 файл содержит)
        if fb2_rtf_mhtml(&пути_до_книг[i]) {
            let содержимое: Vec<String> = {
                if расширение.contains("mhtlml") {
                    read_utf8(&пути_до_книг[i]) //чтение файла в UTF-8
                } else {
                    //прочитать_содержимое_построчно(&пути_до_книг[i])
                    read_utf8(&пути_до_книг[i]) //чтение файла в UTF-8
                }
            };
            //вложение
            let стопка: Vec<lib::Вложения> = vec![lib::Вложения {
                содержимое: содержимое,
                имя: название_книги.clone(),
                изображение: Vec::new(),
            }];

            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.файлы,
                &пути_до_книг[i],
            );
            //вложение не архива в стопку
            let книга = lib::Книги {
                вложения: стопка,
                //содержимое:содержимое,
                путь: пути_до_книг[i].clone(),  //путь полный
                название_книги: название_книги, //имя книги
                расширение,
                ..Default::default()
            };
            стопки_книг.push(книга);
        }
        //если архивный файл
        else if fb3_epub(&пути_до_книг[i]) {
            //println!("это архив");
            let архив: foldhash::HashMap<String, Vec<u8>> =
                foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
            let mut книга_в_озу: VirtualFs = архив;
            zip_архив_в_память(&пути_до_книг[i], &mut книга_в_озу).unwrap();
            let mut приложения_книги: Vec<lib::Вложения> = Vec::new();
            for (имя, содержимое_архива) in книга_в_озу.into_iter() {
                let mut содержимое_строки: String = String::new();
                //картинки не загонять в utf8
                if изображение_расширение(&имя) {
                    приложения_книги.push(lib::Вложения {
                        содержимое: Vec::new(), //пустота - так это рисунок, нельзя читать
                        имя,
                        изображение: содержимое_архива.clone(), //изображение
                    });
                }
                //если это не изображение
                else {
                    содержимое_строки = строка_удалить_utf8_концы_строк(
                        &содержимое_архива,
                        2,
                    );
                    приложения_книги.push(lib::Вложения {
                        содержимое: vec![содержимое_строки],
                        имя,
                        изображение: Vec::new(), //пустота - так как нет рисунков
                    });
                }
            }
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.файлы,
                &пути_до_книг[i],
            );
            let архив: foldhash::HashMap<String, Vec<u8>> =
                foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
            //вложение содержимого всего архива в стопку
            стопки_книг.push(lib::Книги {
                вложения: приложения_книги,
                архив,
                путь: пути_до_книг[i].clone(),  //путь полный
                название_книги: название_книги, //имя книги
                расширение,
            });
        } else if doc_docx(&пути_до_книг[i]) {
            /*use docx_rust::document::Paragraph;
            use docx_rust::Docx;
            use docx_rust::DocxFile;

            let docx = DocxFile::from_file(&ряд_книги[i]).unwrap();
            let mut docx = docx.parse().unwrap();

            let строка = Paragraph::default().push_text("Lorem Ipsum");
            docx.document.push(строка);

            docx.write_file(format!("./проверка/{}.docx",название_книги)).unwrap();*/
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.файлы,
                &название_книги,
            );
            let архив: foldhash::HashMap<String, Vec<u8>> =
                foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
            стопки_книг.push(lib::Книги {
                вложения: Vec::new(),
                архив,
                путь: пути_до_книг[i].clone(),  //путь полный
                название_книги: название_книги, //имя книги
                расширение,
            });
        } else if md_fs_yml(&пути_до_книг[i]) {
            let содержимое: Vec<String> = read_utf8(&пути_до_книг[i]); //чтение файла в UTF-8
            let стопка: Vec<lib::Вложения> = vec![lib::Вложения {
                содержимое: содержимое,
                имя: название_книги.clone(),
                изображение: Vec::new(),
            }];
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.файлы,
                &пути_до_книг[i],
            );
            //вложение не архива в стопку
            let книга = lib::Книги {
                вложения: стопка,
                //содержимое:содержимое,
                путь: пути_до_книг[i].clone(),  //путь полный
                название_книги: название_книги, //имя книги
                расширение,
                ..Default::default()
            };
            стопки_книг.push(книга);
        }
        // else if является_ли_md() {}
        else {
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.не_вложено,
                &format!("{}", пути_до_книг[i]),
            );
            вложить_строку_в_ряд_с_проверкой(
                &mut содержимое_папки.ошибки,
                &format!("{} разрешение файла не определено", пути_до_книг[i]),
            );
        }
    }
    //let mut ряд_матерных_слов: Vec<String> = Vec::new();
    //перебор содержимого книги на предмет наличия трех точек подряд
    /*
    for i in 0.._book_struct.len() {
        for i2 in 0.._book_struct[i].file.len() {
            if _book_struct[i].file[i2].имя.contains("document.xml") {
                 println!("имя: {}", _book_struct[i].file[i2].имя);
                if _book_struct[i].file[i2].содержимое.contains("…s") {

                }
                 println!("содержимое: {:?}", _book_struct[i].file[i2].содержимое);
            }
        }
    }*/
    crate::output::write::вывод_содержимого_папок_по_умолчанию(
        &содержимое_папки,
        "книги",
        &mut сообщения.общие,
    )
    .unwrap();
    crate::output::dir::заменить_путь_выходным_книгам(
        &содержимое_папки.файлы,
        "книги",
    );
    return стопки_книг;
}
