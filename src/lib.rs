#![crate_type = "lib"]

use foldhash::{HashMap, HashSet, HashSetExt, fast::RandomState};
use regex::Regex;
use std::sync::atomic::AtomicUsize;

pub enum Расширение {
    fb2,
    fb3,
    epub,
    html,
    htm,
    xhtml,
    mhtml,
    js,
    css,
    jpeg,

}
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
    pub вывод_книг_с_разделением: String,
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
            вывод_книг_с_разделением: "./вывод/книги_с_разделениями/"
                .to_string(),
            вывод_книги_кучи: "./вывод/кучи/".to_string(),
            вывод_книги_пропуски: "./вывод/пропуски/".to_string(),
            вывод_книги_проверка_после_замены_слов:
                "./вывод/проверка_после_замены_слов/".to_string(),
            вывод_книги_запись_и_чтение: "./вывод/запись_проверка/"
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
    pub вывод_кодировка:String,
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
                "./вывод/проверка_после_замены_слов/".to_string(),
            вывод_книги_запись_и_чтение: "./вывод/запись_проверка/"
                .to_string(),
            вывод_кодировка:"./вывод/остальное/кодировка.txt".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Сообщения_для_книги {
    pub имя_книги: String,
    pub путь_откудаво: String,
    pub расширение: String,
    pub сообщения: Vec<String>,
}
//содержимое
#[derive(Debug, Default, Clone)]
pub struct Сообщения {
    pub общие: Vec<String>,
    pub запись_и_чтение: Vec<Сообщения_для_книги>,
    pub проверка_после_замен: Vec<Сообщения_для_книги>,
    pub чтение_книг: Vec<String>,
    pub кодировка:Vec<String>,
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
    pub путь: String,                    //путь до книги
    pub название_книги: String,          //имя книги
    pub вложения: Vec<Вложения>,         //содержимое
    pub расширение: String,              //формат
    pub архив: HashMap<String, Vec<u8>>, //для zip
    pub книга_ли: bool,
    //pub содержимое:Vec<String>,//сами строки
}

//содержимое - имя файла и его содержимое
#[derive(Debug, Clone)]
pub struct Вложения {
    pub содержимое: Vec<String>, //содержимое
    pub содержимое_в_байтах: Vec<u8>,
    pub имя: String,
    pub имя_без_пути: String,
    pub кодировка: Кодировка,
    //pub изображение: Vec<u8>, //если это картинки, нельзя в utf8 переводить
}

impl Default for Вложения {
    fn default() -> Self {
        Self {
            содержимое: Vec::new(),
            содержимое_в_байтах: Vec::new(),
            имя: "".to_string(),
            имя_без_пути: "".to_string(),
            кодировка: Кодировка::не_определён,
            //  счёчтки: 0,
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Имена_страниц {
    простые,
    составные,
    составные_важные,
    огласовки,
    вездесущие,
    неизменные,
    неизменные_длинные,
    неизменные_короткие,
}
use std::fmt;
//
impl fmt::Display for Имена_страниц {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Имена_страниц::простые => write!(f, "Простые"),
            Имена_страниц::составные => write!(f, "Составные"),
            Имена_страниц::составные_важные => write!(f, "Составные важные"),
            Имена_страниц::огласовки => write!(f, "Огласовки"),
            Имена_страниц::вездесущие => write!(f, "Вездесущие"),
            Имена_страниц::неизменные => write!(f, "Неизменные"),
            Имена_страниц::неизменные_длинные => write!(f, "Неизменные длинные"),
            Имена_страниц::неизменные_короткие => write!(f, "Неизменные короткие"),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Кодировка {
    windows_1251,
    utf8,
    windows_1252,
    не_определён,
}

/*impl Clone for Кодировка {
    fn clone() -> Self { { SomeStruct
            кодировка: Кодировка::не_определён,
            //  счёчтки: 0,
    }}
}*/
//словарь
#[derive(Debug, Default, Clone)]
pub struct Словарь {
    pub путь: String,                             //путь до книги
    pub имя: String,                              //имя книги
    pub разрешение: String,                       //формат
    pub простое: Vec<Ячейка_словаря>,             //одиночные слова
    pub составное: Vec<Ячейка_словаря>,           //сложные и составные
    pub составное_важное: Vec<Ячейка_словаря>,    //сложные и составные (в 1 очередь)
    pub вездесущее: Vec<Ячейка_словаря>,          //сложные и составные
    pub неизменное: Vec<Ячейка_словаря>,          //
    pub огласовки: Vec<Ячейка_словаря>,           //
    pub неизменное_короткое: Vec<Ячейка_словаря>, //
    pub неизменное_длинное: Vec<Ячейка_словаря>,  //
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
    pub однобуквенные: [Ячейка_замены; 7],   //одиночные слова
    pub двубуквенные: [Ячейка_замены; 68],   //одиночные слова
    pub трехбуквенные: [Ячейка_замены; 123], //одиночные слова
    pub многобуквенные: [Ячейка_замены; 59], //одиночные слова
    pub целиковые: [Ячейка_замены; 251],     //одиночные слова
    pub исключения: [Ячейка_замены_с_исключением; 17],
    //
}
//

#[derive(Debug, Clone)]
pub enum Раздел_Словаря {
     простые,
     составные,
     составные_важные,
     огласовки,
     неизменные,
     неизменные_короткие,
     неизменные_длинные,
    вездесущие,
    не_является_разделом,
}

//
#[derive(Debug, Clone)]
pub struct Словарь_разделителей {
    pub ряд_1: [Ячейка_замены_с_разделителями; 37],   //одиночные слова
    //
}
//замена объявления
#[derive(Debug, Clone)]
pub struct Ячейка_замены_переносов {
    pub re_образец_конца: Regex,
    pub re_образец_начала: Regex,
    pub начало_простое: String,
    pub конец: String,
    // pub счёчтки:usize,
}
//замена объявления
#[derive(Debug, Clone)]
pub struct Ячейка_замены_объявления<'a> {
    pub начало: String,
    pub начало_re: Regex,
    pub вложение: &'a Ячейка_замены_переносов,
    // pub счёчтки:usize,
}
//словарь
#[derive(Debug, Clone)]
pub struct Ячейка_замены {
    pub искомое_слово: String,
    pub re_образец: Regex,
    pub замена: String,
    // pub счёчтки:usize,
}
impl Default for Ячейка_замены {
    fn default() -> Self {
        Self {
            искомое_слово: "".to_string(),
            re_образец: Regex::new(r"(?i)").unwrap(),
            замена: "".to_string(),
        }
    }
}

//словарь
#[derive(Debug, Clone)]
pub struct Ячейка_замены_с_разделителями {
    pub искомое_слово: String,
    pub re_исключение: Vec<Regex>,
    pub re_образец_для_поиска: Regex,
    pub замена: String,
    // pub счёчтки:usize,
    pub re_образец_для_замены: Regex,
}
impl Default for Ячейка_замены_с_разделителями {
    fn default() -> Self {
        Self {
            искомое_слово: "".to_string(),
            re_исключение: Vec::new(),//Regex::new(r"(?i)").unwrap(),
            re_образец_для_поиска: Regex::new(r"(?i)").unwrap(),
            замена: "".to_string(),
            re_образец_для_замены:Regex::new(r"(?i)").unwrap(),
        }
    }
}
//словарь
#[derive(Debug, Clone)]
pub struct Ячейка_замены_с_исключением {
    pub искомое_слово: String,
    pub re_исключение: Vec<Regex>,
    pub re_образец_для_поиска: Regex,
    pub замена: String,
    // pub счёчтки:usize,
}
impl Default for Ячейка_замены_с_исключением {
    fn default() -> Self {
        Self {
            искомое_слово: "".to_string(),
            re_исключение: Vec::new(),//Regex::new(r"(?i)").unwrap(),
            re_образец_для_поиска: Regex::new(r"(?i)").unwrap(),
            замена: "".to_string(),
        }
    }
}
//
#[derive(Debug)]
pub struct Счётчик_разделителей {
    pub подсчёт: Vec<AtomicUsize>,  //одиночные слова
   // pub с_заглавной: Vec<AtomicUsize>,   //одиночные слова
 
}
//
#[derive(Debug)]
pub struct Счётчик_замен {
    pub однобуквенные: Vec<AtomicUsize>,  //одиночные слова
    pub двубуквенные: Vec<AtomicUsize>,   //одиночные слова
    pub трехбуквенные: Vec<AtomicUsize>,  //одиночные слова
    pub многобуквенные: Vec<AtomicUsize>, //одиночные слова
    pub целиковые: Vec<AtomicUsize>,      //одиночные слова
    pub исключения: Vec<AtomicUsize>,     //одиночные слова
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
    pub огласовки: foldhash::HashMap<String, HashSet<usize>>,
    pub неизменное_короткое: foldhash::HashMap<String, HashSet<usize>>,
    pub неизменное_длинное: foldhash::HashMap<String, HashSet<usize>>,
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Полный_Словарь {
    //одиночные
    pub простое: Vec<Ячейка_словаря>, //одиночные слова
    //сложные
    pub составное: Vec<Ячейка_словаря>, //сложные и составные
    //сложные в 1 очередь
    pub составное_важное: Vec<Ячейка_словаря>, //сложные и составные (в 1 очередь)
    //вездесущие слова в 1 очередь
    pub вездесущее: Vec<Ячейка_словаря>, //сложные и составные
    //неизменные
    pub неизменное: Vec<Ячейка_словаря>, //сложные и составные
    pub огласовки: Vec<Ячейка_словаря>,  //сложные и составные
    pub неизменное_длинное: Vec<Ячейка_словаря>, //сложные и составные
    pub неизменное_короткое: Vec<Ячейка_словаря>, //сложные и составные
}
// Сначала объявите трейт Clear
pub trait Clear {
    fn clear(&mut self);
}
impl Clear for Полный_Словарь {
    fn clear(&mut self) {
        self.простое.clear();
        self.составное.clear();
        self.составное_важное.clear();
        self.вездесущее.clear();
        self.неизменное.clear();
        self.огласовки.clear();
        self.неизменное_длинное.clear();
        self.неизменное_короткое.clear();
    }
}

#[derive(Debug)]
pub struct Счётчики_Словаря {
    pub простое: Vec<AtomicUsize>,             //одиночные слова
    pub составное: Vec<AtomicUsize>,           //одиночные слова
    pub составное_важное: Vec<AtomicUsize>,    //одиночные слова
    pub вездесущее: Vec<AtomicUsize>,          //одиночные слова
    pub неизменное: Vec<AtomicUsize>,          //одиночные слова
    pub огласовки: Vec<AtomicUsize>,           //одиночные слова
    pub неизменное_короткое: Vec<AtomicUsize>, //одиночные слова
    pub неизменное_длинное: Vec<AtomicUsize>,  //одиночные слова
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Быстрый_Словарь {
    //одиночные
    pub простое: Vec<String>, //одиночные слова
}

#[derive(Debug, Default, Clone)]
pub struct Куча_Слова_Замены {
    pub слово: String,
    pub вложения: String,
}

fn main() {}
