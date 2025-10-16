use crate::utils::functions::строка_удалить_utf8_концы_строк;
use crate::utils::functions_add::system_pause;
use crate::utils::stringzilla::sz_найти;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::{Mutex, atomic::{AtomicU64, Ordering,AtomicUsize}};

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
    return итог;
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
    return содержимое.iter().map(|(имя, _)| имя.clone()).collect();
}

fn считать_содержимое_папки(
    путь_папки: &str,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    use crate::utils::functions::заменить_все_палки;
    let mut содержимое_папки = Vec::new();
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
                    // println!("добавленный путь: {}",путь2);
                    содержимое_папки.push((путь2, содержимое))
                }
                Err(ошибка) => {
                    if ошибка
                        .to_string()
                        .contains("stream did not contain valid UTF-8")
                    {
                        содержимое_папки.push((
                            путь.display().to_string(),
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
