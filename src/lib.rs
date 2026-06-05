#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//use foldhash::{rapidhash::fast::RapidHashMap,  fast::RandomState,*};
//use rapidhash::*;
use regex::Regex;
use std::hash::{Hash, Hasher};
use std::sync::atomic::AtomicUsize;
use vergen::DefaultConfig;
#[derive(Debug, Clone)]
pub enum Вид_Слова {
    Исходное,
    Замена,
}
#[derive(Debug, Clone)]
pub enum Вид_Видео {
    Avi,
    Mkv,
    Mp4,
    WebM,
}
#[derive(Debug, Clone)]
pub enum Вид_Звук {
    Wav,
    Mp3,
    Ogg,
    Aac,
}
#[derive(Debug, Clone)]
pub enum Вид_Изображения {
    Jpeg,
    Png,
    Bmp,
    Gif,
    Tif,
    Jpg,
    Svg,
    Avif,
    Webp,
    Wmf,
    Wpg,
    Eps,
    Emf,
}
#[derive(Debug, Clone)]
pub enum Вид_Архива {
    Zip,
    Rar,
    Gz,
    Gzip,
}
#[derive(Debug, Clone)]
pub enum Вид_Мусорные_Разметки_Паутины {
    Css,
    Thmx,
}
#[derive(Debug, Clone)]
pub enum Вид_XML {
    Rels,
}
#[derive(Debug, Clone)]
pub enum Вид_Word {
    Doc,
    Docx,
    Rtf,
}
#[derive(Debug, Clone)]
pub enum Вид_Excel {
    Xls,
    Xlsx,
}
#[derive(Debug, Clone)]
pub enum Вид_Шрифтов {
    Tif,
}
#[derive(Debug, Clone)]
pub enum Вид_Разметки_Паутины {
    Php,
    Html,
    Htm,
    Md,
    Yml,
    Fs,
    XHTML,
    Mhtml,
    Mht,
    Не_определено,
}
#[derive(Debug, Clone)]
pub enum Вид_Архивной_Книги {
    Epub,
    Fb3,
}
#[derive(Debug, Clone)]
pub enum Вид_одичноной_книги {
    Fb2,
}
#[derive(Debug, Clone)]
pub enum Вид_Книги {
    Архивная(Вид_Архивной_Книги),
    Одиночная(Вид_одичноной_книги),
}
#[derive(Debug, Clone)]
pub enum Вид_JS {
    Js,
    Mjs,
    Cjs,
}
#[derive(Debug, Clone)]
pub enum Вид_Справи {
    Cnt,
    Hlp,
    Chm,
}
#[derive(Debug, Clone)]
pub enum Вид_приказов {
    Tcl,
    Fcg,
    Cgi,
}
#[derive(Debug, Clone)]
pub enum Основной_Вид_Расширения {
    Книга(Вид_Книги),
    Архив(Вид_Архива),
    Изображение(Вид_Изображения),
    Видео(Вид_Видео),
    Разметка_Паутины(Вид_Разметки_Паутины),
    Мусорные_Разметка(Вид_Мусорные_Разметки_Паутины),
    Word(Вид_Word),
    Excel(Вид_Excel),
    XML,
    JS(Вид_JS),
    Pdf,
    Прочее,
    Шрифты(Вид_Шрифтов),
    MIME,
    Простая_письменность(Вид_простой_письменности),
    Приказы(Вид_приказов),
    Пусто,
    Без_Названия,
    Справка(Вид_Справи),
    Не_определено,
}
impl Основной_Вид_Расширения {
    pub fn fb2_mht(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Книга(содержимое) => {
                match содержимое {
                    Вид_Книги::Одиночная(_) => true,
                    _ => false,
                }
            }
            // Основной_Вид_Расширения::Разметка_Паутины(_)=>true,
            Основной_Вид_Расширения::Разметка_Паутины(
                содержимое,
            ) => match содержимое {
                Вид_Разметки_Паутины::Не_определено => false,
                Вид_Разметки_Паутины::Mht => true,
                Вид_Разметки_Паутины::Mhtml => true,

                _ => false,
            },
            _ => false,
        }
    }
    pub fn архив_книга(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Книга(содержимое) => {
                match содержимое {
                    Вид_Книги::Архивная(_) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn htm_html_xhtml(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Разметка_Паутины(
                содержимое,
            ) => match содержимое {
                Вид_Разметки_Паутины::Html | Вид_Разметки_Паутины::Htm => {
                    true
                }
                Вид_Разметки_Паутины::Html | Вид_Разметки_Паутины::Html => {
                    true
                }
                Вид_Разметки_Паутины::Html | Вид_Разметки_Паутины::XHTML => {
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
    pub fn md_yml_fs(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Разметка_Паутины(
                содержимое,
            ) => match содержимое {
                Вид_Разметки_Паутины::Md | Вид_Разметки_Паутины::Yml | Вид_Разметки_Паутины::Fs => {
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}
impl Default for Основной_Вид_Расширения {
    fn default() -> Self {
        Основной_Вид_Расширения::Не_определено
    }
}
#[derive(Debug, Clone)]
pub enum Вид_простой_письменности {
    Txt,
    Log,
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
    //
    pub словарь_запасной_чистый: String,
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
            //
            словарь_запасной_чистый: "./вывод/словари/".to_string(),
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
    pub вывод_кодировка: String,
}

impl Default for Пути_Вывода {
    fn default() -> Self {
        Self {
            //файлы пошли уже
            вывод_сообщений: "./вывод/сообщения.txt".to_string(),
            вывод_кучи_словаря: "./вывод/кучи/куча_".to_string(),
            вывод_кучи_словаря_ключи: "./вывод/кучи/куча_словарь_ключи_"
                .to_string(),
            вывод_книг_проверки_замен: "./вывод/проверка_после_замены_слов/"
                .to_string(),
            вывод_книги_запись_и_чтение: "./вывод/запись_проверка/"
                .to_string(),
            вывод_кодировка: "./вывод/остальное/кодировка.txt".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Сообщения_для_книги {
    pub имя_книги: String,
    pub путь_откудаво: String,
    pub расширение: String,
    pub сообщения: Vec<String>,
    pub расширение_подробно: Основной_Вид_Расширения,
}
//содержимое
#[derive(Debug, Default, Clone)]
pub struct Сообщения {
    pub общие: Vec<String>,
    pub запись_и_чтение: Vec<Сообщения_для_книги>,
    pub проверка_после_замен: Vec<Сообщения_для_книги>,
    pub чтение_книг: Vec<String>,
    pub кодировка: Vec<String>,
}
pub trait Возможности_Сообщений {
    fn вложить_оба(первый_ряд: Self, other: Self) -> Self;
}
impl Возможности_Сообщений for Сообщения {
    fn вложить_оба(первый_ряд: Self, other: Self) -> Self {
        let mut главный_ряд: Сообщения = Default::default();
        главный_ряд.общие.extend(other.общие);
        главный_ряд.запись_и_чтение.extend(other.запись_и_чтение);
        главный_ряд
            .проверка_после_замен
            .extend(other.проверка_после_замен);
        главный_ряд.чтение_книг.extend(other.чтение_книг);
        главный_ряд.кодировка.extend(other.кодировка);
        //
        главный_ряд.общие.extend(первый_ряд.общие);
        главный_ряд
            .запись_и_чтение
            .extend(первый_ряд.запись_и_чтение);
        главный_ряд
            .проверка_после_замен
            .extend(первый_ряд.проверка_после_замен);
        главный_ряд.чтение_книг.extend(первый_ряд.чтение_книг);
        главный_ряд.кодировка.extend(первый_ряд.кодировка);
        //
        главный_ряд
    }
}
impl Сообщения {
    pub fn вложить(&mut self, other: Сообщения) {
        self.общие.extend(other.общие);
        self.запись_и_чтение.extend(other.запись_и_чтение);
        self.проверка_после_замен.extend(other.проверка_после_замен);
        self.чтение_книг.extend(other.чтение_книг);
        self.кодировка.extend(other.кодировка);
    }
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
    pub расширение: String,
    pub расширение_подробно: Основной_Вид_Расширения, //формат
    pub архив: rapidhash::fast::RapidHashMap<String, Vec<u8>>, //для zip
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
    pub расширение_подробно: Основной_Вид_Расширения,
    //pub изображение: Vec<u8>, //если это картинки, нельзя в utf8 переводить
}

impl Default for Вложения {
    fn default() -> Self {
        Self {
            содержимое: Vec::new(),
            содержимое_в_байтах: Vec::new(),
            имя: "".to_string(),
            имя_без_пути: "".to_string(),
            кодировка: Кодировка::Не_определено,
            расширение_подробно:
                Основной_Вид_Расширения::Не_определено,
            //  счёчтки: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Вид_Словаря {
    Основной_Словарь,
    Запасной_Словарь,
    //
}
impl Display for Вид_Словаря {
    fn fmt(&self, образ: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Вид_Словаря::Запасной_Словарь => write!(образ, "Запасной"),
            Вид_Словаря::Основной_Словарь => write!(образ, "Основной"),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Имена_страниц {
    Простая_стр,
    Cоставное_стр,
    Составное_важное_стр,
    Огласовки_стр,
    Вездесущее_стр,
    Неизменные_стр,
    Неизменные_длинные_стр,
    Неизменные_короткие_стр,
    Запятые,
    //
}

use std::fmt;
use std::fmt::Display;

//
impl fmt::Display for Имена_страниц {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Имена_страниц::Простая_стр => write!(f, "Простые"),
            Имена_страниц::Cоставное_стр => write!(f, "Составные"),
            Имена_страниц::Составное_важное_стр => {
                write!(f, "Составные важные")
            }
            Имена_страниц::Огласовки_стр => write!(f, "Огласовки"),
            Имена_страниц::Вездесущее_стр => write!(f, "Вездесущие"),
            Имена_страниц::Неизменные_стр => write!(f, "Неизменные"),
            Имена_страниц::Неизменные_длинные_стр => {
                write!(f, "Неизменные длинные")
            }
            Имена_страниц::Неизменные_короткие_стр => {
                write!(f, "Неизменные короткие")
            }
            Имена_страниц::Запятые => {
                write!(f, "Запятые")
            }
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Кодировка {
    Windows_1251,
    Utf8,
    Windows_1252,
    Не_определено,
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
    pub запятые: Vec<Ячейка_словаря>,             //
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
// Ручная реализация PartialEq
// Ручная реализация PartialEq
impl PartialEq for Ячейка_словаря {
    fn eq(&self, other: &Self) -> bool {
        // Сравните все поля, которые должны определять уникальность
        self.искомое_слово == other.искомое_слово
            && self.замена == other.замена
            && self.re_образец.as_str() == other.re_образец.as_str()
    }
}

// Потом пустая реализация Eq (маркерный трейт)
impl Eq for Ячейка_словаря {} // 👈 ВОТ ТАК ПРАВИЛЬНО!

// Ручная реализация Hash
impl Hash for Ячейка_словаря {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.искомое_слово.hash(state);
        self.замена.hash(state);
        self.re_образец.as_str().hash(state);
    }
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

pub static СЛОВАРЬ_ПЕРЕНОСОВ_ОДНОБУКВЕННЫЕ: usize = 7;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_ДВУБУКВЕННЫЕ: usize = 68;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_ТРЕХБУКВЕННЫЕ: usize = 123;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_МНОГОБУКВЕННЫЕ: usize = 59;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_ЦЕЛИКОВЫЕ: usize = 250;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_ИСКЛЮЧЕНИЯ: usize = 17;
//
pub static РЯД_СО_ЗНАЧЕНИЯМИ: [usize; 6] = [
    СЛОВАРЬ_ПЕРЕНОСОВ_ОДНОБУКВЕННЫЕ,
    СЛОВАРЬ_ПЕРЕНОСОВ_ДВУБУКВЕННЫЕ,
    СЛОВАРЬ_ПЕРЕНОСОВ_ТРЕХБУКВЕННЫЕ,
    СЛОВАРЬ_ПЕРЕНОСОВ_МНОГОБУКВЕННЫЕ,
    СЛОВАРЬ_ПЕРЕНОСОВ_ЦЕЛИКОВЫЕ,
    СЛОВАРЬ_ПЕРЕНОСОВ_ИСКЛЮЧЕНИЯ,
];
//
#[derive(Debug, Clone)]
pub struct Словарь_Переносов {
    pub однобуквенные:
        [Ячейка_замены; СЛОВАРЬ_ПЕРЕНОСОВ_ОДНОБУКВЕННЫЕ], //одиночные слова
    pub двубуквенные: [Ячейка_замены; СЛОВАРЬ_ПЕРЕНОСОВ_ДВУБУКВЕННЫЕ], //одиночные слова
    pub трехбуквенные:
        [Ячейка_замены; СЛОВАРЬ_ПЕРЕНОСОВ_ТРЕХБУКВЕННЫЕ], //одиночные слова
    pub многобуквенные:
        [Ячейка_замены; СЛОВАРЬ_ПЕРЕНОСОВ_МНОГОБУКВЕННЫЕ], //одиночные слова
    pub целиковые: [Ячейка_замены; СЛОВАРЬ_ПЕРЕНОСОВ_ЦЕЛИКОВЫЕ],       //одиночные слова
    pub исключения:
        [Ячейка_замены_с_исключением; СЛОВАРЬ_ПЕРЕНОСОВ_ИСКЛЮЧЕНИЯ],
    //
    //
}
//
#[derive(Debug, Clone)]
pub enum Значение_Ячейки_XLSX {
    Пустое_значение,
    Строка(String),
    Ошибка(String),
    Разумное(bool),
    Целое(i64),
    Вещественное(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Раздел_Словаря {
    Простые,
    Составные,
    Составные_важные,
    Огласовки,
    Неизменные,
    Неизменные_короткие,
    Неизменные_длинные,
    Запятые,
    Вездесущие,
    Не_является_разделом,
}

impl fmt::Display for Раздел_Словаря {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Раздел_Словаря::Простые => write!(f, "Простые"),
            Раздел_Словаря::Составные => write!(f, "Составные"),
            Раздел_Словаря::Составные_важные => {
                write!(f, "Составные важные")
            }
            Раздел_Словаря::Огласовки => write!(f, "Огласовки"),
            Раздел_Словаря::Вездесущие => write!(f, "Вездесущие"),
            Раздел_Словаря::Неизменные => write!(f, "Неизменные"),
            Раздел_Словаря::Неизменные_длинные => {
                write!(f, "Неизменные длинные")
            }
            Раздел_Словаря::Запятые => {
                write!(f, "Запятые")
            }
            Раздел_Словаря::Неизменные_короткие => {
                write!(f, "Неизменные короткие")
            }
            Раздел_Словаря::Не_является_разделом => {
                write!(f, "не_является_разделом")
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum Примечания {
    html,
    js,
}
pub static РАЗМЕР_РАЗДЕЛИТЕЛЕЙ: usize = 214;

//

#[derive(Debug, Clone)]
pub struct Словарь_разделителей {
    pub ряд_1: [Ячейка_замены_с_разделителями; РАЗМЕР_РАЗДЕЛИТЕЛЕЙ], //одиночные слова
                                                                     //
}
//замена объявления
#[derive(Debug, Clone)]
pub struct Ячейка_замены_переносов {
    pub re_образец_конца: Regex,
    pub re_образец_начала: Regex,
    pub начало_простое: &'static str,
    pub конец: &'static str,
    // pub счёчтки:usize,
}
//замена объявления
#[derive(Debug, Clone)]
pub struct Ячейка_замены_объявления<'a> {
    pub начало: &'static str,
    pub начало_re: Regex,
    pub вложение: &'a Ячейка_замены_переносов,
    pub примечания: Примечания,
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
    pub ряд_re_исключений: Vec<Regex>,
    pub re_образец_для_поиска: Regex,
    pub замена: String,
    pub ряд_исключений: Vec<String>,
    // pub счёчтки:usize,
    pub re_образец_для_замены: Regex,
}
impl Default for Ячейка_замены_с_разделителями {
    fn default() -> Self {
        Self {
            искомое_слово: "".to_string(),
            ряд_re_исключений: Vec::new(), //Regex::new(r"(?i)").unwrap(),
            re_образец_для_поиска: Regex::new(r"(?i)").unwrap(),
            замена: "".to_string(),
            re_образец_для_замены: Regex::new(r"(?i)").unwrap(),
            ряд_исключений: Vec::new(),
        }
    }
}
//
pub trait Возможности_ячейки_замены_с_разделителями {
    fn добавить_re_исключения_изнутри(&self) -> Vec<Regex>;
    fn добавить_оставшиеся_поля(&mut self);
}
impl Возможности_ячейки_замены_с_разделителями
    for Ячейка_замены_с_разделителями
{
    fn добавить_re_исключения_изнутри(&self) -> Vec<Regex> {
        self.ряд_исключений
            .iter()
            .map(|ячейка| {
                //let исключение: Regex = LazyLock::new(|| Regex::new(исключение).unwrap();
                let исключение_2: String = format!(r#"({})"#, ячейка);
                Regex::new(&исключение_2).unwrap()
            })
            .collect()
    }
    //
    fn добавить_оставшиеся_поля(&mut self) {
        self.замена = format!("{}-", self.искомое_слово);
        self.re_образец_для_поиска =
            Regex::new(&format!(r#"\b{{start}}{}\w"#, self.искомое_слово)).unwrap();
        self.re_образец_для_замены = {
            Regex::new(&format!(
                r#"\b{{start}}({})([\w]{{4,}})"#,
                self.искомое_слово
            ))
            .unwrap()
        };
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
            re_исключение: Vec::new(), //Regex::new(r"(?i)").unwrap(),
            re_образец_для_поиска: Regex::new(r"(?i)").unwrap(),
            замена: "".to_string(),
        }
    }
}
//
#[derive(Debug, Clone)]
pub enum Правописание_слова {
    С_Заглавной,
    Все_Заглавные,
    Все_строчные,
    Исходное,
}
impl fmt::Display for Правописание_слова {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Правописание_слова::С_Заглавной => write!(f, "С заглавной"),
            Правописание_слова::Все_Заглавные => {
                write!(f, "Все заглавные")
            }
            Правописание_слова::Все_строчные => {
                write!(f, "Все строчные")
            }
            Правописание_слова::Исходное => write!(f, "Исходное"),
        }
    }
}
//
#[derive(Debug)]
pub struct Счётчик_разделителей {
    pub подсчёт: Vec<AtomicUsize>, //одиночные слова
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
    pub простое: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub составное: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub запятые: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub составное_важное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub вездесущее: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub неизменное: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub огласовки: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub неизменное_короткое:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub неизменное_длинное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
}
#[derive(Debug, Default, Clone)]
pub struct Куча_Словарь_Искомые {
    pub простое: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub составное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub составное_важное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub вездесущее: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub запятые: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub огласовки: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное_длинное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное_короткое: rapidhash::fast::RapidHashSet<String>, //одиночные слова
}
#[derive(Debug, Default, Clone)]
pub struct Куча_Словарь_Замены {
    pub простое: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub запятые: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub составное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub составное_важное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub вездесущее: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub огласовки: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное_длинное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное_короткое: rapidhash::fast::RapidHashSet<String>, //одиночные слова
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Полный_Словарь {
    //одиночные
    pub простое: Vec<Ячейка_словаря>, //одиночные слова
    //сложные
    pub составное: Vec<Ячейка_словаря>, //сложные и составные
    pub запятые: Vec<Ячейка_словаря>,   //сложные и составные
    //сложные в 1 очередь
    pub составное_важное: Vec<Ячейка_словаря>, //сложные и составные (в 1 очередь)
    //вездесущие слова в 1 очередь
    pub вездесущее: Vec<Ячейка_словаря>, //сложные и составные
    //неизменные
    pub неизменное: Vec<Ячейка_словаря>, //сложные и составные
    //
    pub огласовки: Vec<Ячейка_словаря>, //сложные и составные
    //
    pub неизменное_длинное: Vec<Ячейка_словаря>, //сложные и составные
    //
    pub неизменное_короткое: Vec<Ячейка_словаря>, //сложные и составные
}
pub const КОЛИЧЕСТВО_УРОВНЕЙ_СЛОВАРЯ_КУЧ: usize = 3;
//
#[derive(Debug, Default, Clone)]
pub struct Словарь_Куч {
    pub куча_словарь_искомые:
        [Куча_Словарь_Искомые; КОЛИЧЕСТВО_УРОВНЕЙ_СЛОВАРЯ_КУЧ],
    pub куча_словарь_замены:
        [Куча_Словарь_Замены; КОЛИЧЕСТВО_УРОВНЕЙ_СЛОВАРЯ_КУЧ],
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
    pub запятые: Vec<AtomicUsize>,             //одиночные слова
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Быстрый_Словарь {
    //одиночные
    pub простое: Vec<String>, //одиночные слова
}

#[derive(Debug, Default, Clone)]
pub struct Слова_с_Вложениями {
    pub слово: String,
    pub вложения: String,
}
