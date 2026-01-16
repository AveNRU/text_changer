#![crate_type = "lib"]

use foldhash::{HashMap, HashSet, HashSetExt, fast::RandomState};
use regex::Regex;
use std::sync::atomic::AtomicUsize;

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
pub enum Кодировка {
    windows_1251,
    utf8,
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
    pub двубуквенные: [Ячейка_замены; 69],   //одиночные слова
    pub трехбуквенные: [Ячейка_замены; 123], //одиночные слова
    pub многобуквенные: [Ячейка_замены; 59], //одиночные слова
    pub целиковые: [Ячейка_замены; 251],     //одиночные слова
    pub исключения: [Ячейка_замены_с_исключением; 16],
}
//замена объявления
#[derive(Debug, Clone)]
pub struct Ячейка_замены_объявления {
    pub начало: String,
    pub re_образец_конца: Regex,
    pub re_образец_начала: Regex,
    pub начало_простое: String,
    pub конец: String,
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
pub struct Ячейка_замены_с_исключением {
    pub искомое_слово: String,
    pub re_исключение: Regex,
    pub re_образец: Regex,
    pub замена: String,
    // pub счёчтки:usize,
}
impl Default for Ячейка_замены_с_исключением {
    fn default() -> Self {
        Self {
            искомое_слово: "".to_string(),
            re_исключение: Regex::new(r"(?i)").unwrap(),
            re_образец: Regex::new(r"(?i)").unwrap(),
            замена: "".to_string(),
        }
    }
}

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

/*
#[derive(Debug, Clone)]
pub struct Образцы_для_разбиения_html {
    pub образцы: [String;56],
}

impl Default for Образцы_для_разбиения_html {
    fn default() -> Self {
        Self {
            образцы: [
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
                r#"<!--]-->"#.to_string(),
                r#"</h3>"#.to_string(),
                r#"</h2>"#.to_string(),
                r#"</h1>"#.to_string(),
                r#"</em>"#.to_string(),
                r#"<!--[-->"#.to_string(),
                r#"<!---->"#.to_string(),
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
            ],
        }
    }
}
*/

fn main() {}
