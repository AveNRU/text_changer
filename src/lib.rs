#![crate_type = "lib"]

use std::sync::atomic::AtomicUsize;
use foldhash::{HashMap, HashSet, HashSetExt, fast::RandomState};
use regex::Regex;

//пути
#[derive(Debug, Clone)]
pub struct Пути_Общие {
    pub книги: String,
    pub словари: String,
    pub переносы: String,
    pub вывод_книги: String,
    pub вывод_словари: String,
    pub вывод: String,
    pub вывод_книги_кучи: String,
    pub вывод_книги_пропуски: String,
    pub вывод_книги_проверка_после_замены_слов: String,
    pub вывод_книги_запись_и_чтение: String,
}

impl Default for Пути_Общие {
    fn default() -> Self {
        Self {
            книги: "./книги/".to_string(),
            словари: "./словари/".to_string(),
            переносы: "./перееносы/".to_string(),
            вывод: "./вывод/".to_string(),
            //вложенные
            вывод_словари: "./вывод/словари/".to_string(),
            вывод_книги: "./вывод/книги/".to_string(),
            вывод_книги_кучи: "./вывод/кучи/".to_string(),
            вывод_книги_пропуски: "./вывод/книги/пропуски/".to_string(),
            вывод_книги_проверка_после_замены_слов:
                "./вывод/книги/проверка_после_замены_слов/".to_string(),
            вывод_книги_запись_и_чтение: "./вывод/книги/запись_проверка/"
                .to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Пути_Вывода {
    //файлы
    pub вывод_сообщений: String,
    pub вывод_кучи_словаря: String,
    pub вывод_кучи_словаря_ключи: String,
    pub вывод_книг_проверки_замен: String,
    pub вывод_книги_запись_и_чтение: String,
}

impl Default for Пути_Вывода {
    fn default() -> Self {
        Self {
            //файлы пошли уже
            вывод_сообщений: "./вывод/сообщения.txt".to_string(),
            вывод_кучи_словаря: "./вывод/кучи/куча_".to_string(),
            вывод_кучи_словаря_ключи: "./вывод/кучи/куча_словарь_ключи_"
                .to_string(),
            вывод_книг_проверки_замен:
                "./вывод/книги/проверка_после_замены_слов/".to_string(),
            вывод_книги_запись_и_чтение: "./вывод/книги/запись_проверка/"
                .to_string(),
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct Сообщения_для_книги {
    pub имя_книги: String,
    pub сообщения: Vec<String>,
}
//содержимое
#[derive(Debug, Default, Clone)]
pub struct Сообщения {
    pub общие: Vec<String>,
    pub запись_и_чтение: Vec<Сообщения_для_книги>,
    pub проверка_после_замен: Vec<Сообщения_для_книги>,
    pub чтение_книг: Vec<String>,
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
    pub архив: HashMap<String, Vec<u8>>, //для zip
                                 //pub содержимое:Vec<String>,//сами строки
}
//содержимое - имя файла и его содержимое
#[derive(Debug, Default, Clone)]
pub struct Вложения {
    pub содержимое: Vec<String>, //содержимое
    pub содержимое_в_байтах: Vec<u8>,
    pub имя: String,
    pub имя_без_пути: String,
    //pub изображение: Vec<u8>, //если это картинки, нельзя в utf8 переводить
}
//словарь
#[derive(Debug, Default, Clone)]
pub struct Словарь {
    pub путь: String,                          //путь до книги
    pub имя: String,                           //имя книги
    pub разрешение: String,                    //формат
    pub простое: Vec<Ячейка_словаря>,          //одиночные слова
    pub составное: Vec<Ячейка_словаря>,        //сложные и составные
    pub составное_важное: Vec<Ячейка_словаря>, //сложные и составные (в 1 очередь)
    pub вездесущее: Vec<Ячейка_словаря>,       //сложные и составные
    pub неизменное: Vec<Ячейка_словаря>,       //
}

//словарь переносов
//словарь
#[derive(Debug, Clone)]
pub struct Ячейка_словаря {
    pub искомое_слово: String,
    pub re_образец: Regex,
    pub замена: String,
    // pub счёчтки:usize,
}
impl Default for Ячейка_словаря {
    fn default() -> Self {
        Self {
            искомое_слово: "".to_string(),
            re_образец: Regex::new(r"").unwrap(),
            замена: "".to_string(),
            //  счёчтки: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Словарь_Переносов {
   /* pub однобуквенные: [Ячейка_замены; 10],  //одиночные слова
    pub двубуквенные: [Ячейка_замены; 54],   //одиночные слова
    pub трехбуквенные: [Ячейка_замены; 48],  //одиночные слова
    pub многобуквенные: [Ячейка_замены; 24], //одиночные слова
    pub целиковые: [Ячейка_замены; 310],     //одиночные слова

    */
    pub однобуквенные: Vec<Ячейка_замены>,  //одиночные слова
    pub двубуквенные: Vec<Ячейка_замены>,     //одиночные слова
    pub трехбуквенные: Vec<Ячейка_замены>,    //одиночные слова
    pub многобуквенные: Vec<Ячейка_замены>,   //одиночные слова
    pub целиковые: Vec<Ячейка_замены>,       //одиночные слова
}

//словарь
#[derive(Debug, Clone)]
pub struct Ячейка_замены {
    pub искомое_слово: String,
    pub re_образец: Regex,
    pub замена: String,
    // pub счёчтки:usize,
}

#[derive(Debug)]
pub struct Счётчик_замен {
    pub однобуквенные: Vec<AtomicUsize>,  //одиночные слова
    pub двубуквенные: Vec<AtomicUsize>,   //одиночные слова
    pub трехбуквенные: Vec<AtomicUsize>,  //одиночные слова
    pub многобуквенные: Vec<AtomicUsize>, //одиночные слова
    pub целиковые: Vec<AtomicUsize>,     //одиночные слова
}
//случаи замены

//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Куча_Словарь {
    pub простое: foldhash::HashMap<String, HashSet<usize>>,
    pub составное: foldhash::HashMap<String, HashSet<usize>>,
    pub составное_важное: foldhash::HashMap<String, HashSet<usize>>,
    pub вездесущее: foldhash::HashMap<String, HashSet<usize>>,
    pub неизменное: foldhash::HashMap<String, HashSet<usize>>,
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Полный_Словарь {
    //одиночные
    pub простое: Vec<Ячейка_словаря>, //одиночные слова
    pub счётчик_простое: Vec<usize>,
    //сложные
    pub составное: Vec<Ячейка_словаря>, //сложные и составные
    pub счётчик_составное: Vec<usize>,
    //сложные в 1 очередь
    pub составное_важное: Vec<Ячейка_словаря>, //сложные и составные (в 1 очередь)
    pub счётчик_составное_важное: Vec<usize>,
    //вездесущие слова в 1 очередь
    pub вездесущее: Vec<Ячейка_словаря>, //сложные и составные
    pub счётчик_вездесущее: Vec<usize>,
    //неизменные
    pub неизменное: Vec<Ячейка_словаря>, //сложные и составные
    pub счётчик_неизменное: Vec<usize>,
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Быстрый_Словарь {
    //одиночные
    pub простое: Vec<String>, //одиночные слова
}

fn main() {}
