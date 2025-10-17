#![crate_type = "lib"]
use foldhash::HashSet;
use foldhash::fast::RandomState;
use regex::Regex;
use std::collections::HashMap;

//пути
#[derive(Debug, Clone)]
pub struct Пути_Общие {
    pub книги: String,
    pub словари: String,
    pub вывод_книги: String,
    pub вывод_словари: String,
    pub вывод: String,
    pub вывод_пропуски: String,
    //файлы
    pub вывод_сообщений: String,
    pub вывод_кучи_словаря: String,
    pub вывод_кучи_словаря_ключи: String,
    pub вывод_для_куч: String,
}

impl Default for Пути_Общие {
    fn default() -> Self {
        Self {
            книги: "./книги/".to_string(),
            словари: "./словари/".to_string(),
            вывод_словари: "./вывод/словари/".to_string(),
            вывод_книги: "./вывод/книги/".to_string(),
            вывод: "./вывод/".to_string(),
            вывод_для_куч: "./вывод/кучи/".to_string(),
            вывод_пропуски: "./вывод/книги/пропуски/".to_string(),
            //файлы пошли уже
            вывод_сообщений: "./вывод/сообщения.txt".to_string(),
            вывод_кучи_словаря: "./вывод/кучи/куча_".to_string(),
            вывод_кучи_словаря_ключи: "./вывод/кучи/куча_словарь_ключи_"
                .to_string(),
        }
    }
}
//содержимое
#[derive(Debug, Default, Clone)]
pub struct Сообщения {
    pub общие: Vec<String>,
    pub запись_и_чтение: Vec<String>,
}
//содержимое
#[derive(Debug, Default, Clone)]
pub struct Содержимое_папок {
    pub файлы: Vec<String>,
    pub ошибки: Vec<String>,
    pub не_вложено: Vec<String>,
}

//Стопка с путём до книги и содержимым виде вектора строк
#[derive(Debug, Default, Clone)]
pub struct Книги {
    pub путь: String,            //путь до книги
    pub название_книги: String,  //имя книги
    pub вложения: Vec<Вложения>, //содержимое
    pub расширение: String,      //формат
    pub архив: HashMap<String, Vec<u8>, RandomState>, //для zip
                                 //pub содержимое:Vec<String>,//сами строки
}
//содержимое - имя файла и его содержимое
#[derive(Debug, Default, Clone)]
pub struct Вложения {
    pub содержимое: Vec<String>, //содержимое
    pub имя: String,
    pub изображение: Vec<u8>, //если это картинки, нельзя в utf8 переводить
}
//словарь
#[derive(Debug, Default, Clone)]
pub struct Словарь {
    pub путь: String,                         //путь до книги
    pub имя: String,                          //имя книги
    pub разрешение: String,                   //формат
    pub одиночное: Vec<String>,               //одиночные слова
    pub re_одиночное: Vec<Regex>,             //одиночные слова Regex
    pub замена_одичное: Vec<String>,          //замена одиночные слова
    pub составное: Vec<String>,               //сложные и составные
    pub re_составное: Vec<Regex>,             //сложные и составные Regex
    pub замена_составное: Vec<String>,        //сложные и составные
    pub составное_важное: Vec<String>,        //сложные и составные (в 1 очередь)
    pub re_составное_важное: Vec<Regex>,      //сложные и составные Regex (в 1 очередь)
    pub замена_составное_важное: Vec<String>, //сложные и составные (в 1 очередь)
    pub вездесушее: Vec<String>,              //сложные и составные
    pub re_вездесушее: Vec<Regex>,            //сложные и составные Regex
    pub замена_вездесушее: Vec<String>,       //сложные и составные
}

//случаи замены

//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Куча_Словарь {
    pub простое: foldhash::HashMap<String, HashSet<usize>>,
    pub составное: foldhash::HashMap<String, HashSet<usize>>,
    pub составное_важное: foldhash::HashMap<String, HashSet<usize>>,
    pub вездесущее: foldhash::HashMap<String, HashSet<usize>>,
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct ПолныйСловарь {
    //одиночные
    pub простое: Vec<String>,         //одиночные слова
    pub re_простое: Vec<Regex>,       //одиночные слова Regex
    pub замена_простому: Vec<String>, //замена одиночные слова
    // pub замена_простому_верхнее: Vec<String>, //замена одиночные слова
    pub счётчик_простое: Vec<usize>, //количество замен одиночных слов
    //сложные
    pub составное: Vec<String>,        //сложные и составные
    pub re_составное: Vec<Regex>,      //сложные и составные Regex
    pub замена_составное: Vec<String>, //сложные и составные
    // pub замена_составное_верхнее: Vec<String>, //сложные и составные
    pub счётчик_составное: Vec<usize>, //количество замен сложных и составных слов
    //сложные в 1 очередь
    pub составное_важное: Vec<String>, //сложные и составные (в 1 очередь)
    pub re_составное_важное: Vec<Regex>, //сложные и составные Regex (в 1 очередь)
    pub замена_составное_важное: Vec<String>, //сложные и составные (в 1 очередь)
    // pub замена_составное_важное_верхнее: Vec<String>, //сложные и составные (в 1 очередь)
    pub счётчик_составное_важное: Vec<usize>, //количество замен сложных и составных слов (в 1 очередь)
    //вездесущие слова в 1 очередь
    pub вездесущее: Vec<String>,        //сложные и составные
    pub re_вездесущее: Vec<Regex>,      //сложные и составные Regex
    pub замена_вездесущее: Vec<String>, //сложные и составные
    //pub замена_вездесущее_верхнее: Vec<String>, //сложные и составные
    pub счётчик_вездесущее: Vec<usize>, //количество замен сложных и составных слов (в 1 очередь)
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Быстрый_Словарь {
    //одиночные
    pub простое: Vec<String>, //одиночные слова
}

fn main() {}
