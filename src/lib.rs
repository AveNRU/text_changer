#![allow(non_snake_case, non_camel_case_types)]
//use foldhash::{rapidhash::fast::RapidHashMap,  fast::RandomState,*};
//use rapidhash::*;
use console::style;
//use gpui_kit::component::yellow;
use rapidhash::fast::RapidHashSet;
use regex::Regex;
use rust_xlsxwriter::{ColNum, Format, IntoExcelData, RowNum, Worksheet, XlsxError};
use std::fmt::{self};
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;
use std::sync::atomic::AtomicUsize;
pub const ВСЕГО_ШАГОВ: usize = 5;
//
#[derive(Debug, Clone)]
pub enum Вид_окончаний {
    Запрещённое,
    Недостающее,
}
impl Вид_окончаний {
    pub fn получить_имя_страницы_для_xlsx(&self) -> String {
        return match self {
            Вид_окончаний::Запрещённое => "Запрещённое".to_string(),
            Вид_окончаний::Недостающее => "Недостающиее".to_string(),
        };
    }
}

#[derive(Debug, Clone)]
pub enum Ошибка_Сохранения {
    Запись_в_Буфер, // save_to_buffer
    Чтение_Буфера,  // прочитать_xlsx_из_буфера
    Чтение_Диска,   // прочитать_xlsx_с_диска
    Запись,         // содержимое.save
    Создание_папки, // содержимое.save
}
impl fmt::Display for Ошибка_Сохранения {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Запись_в_Буфер => write!(f, "Ошибка записи в буфер"),
            Self::Чтение_Буфера => write!(f, "Ошибка чтения из буфера"),
            Self::Чтение_Диска => write!(f, "Ошибка чтения с диска"),
            Self::Запись => write!(f, "Ошибка записи"),
            Self::Создание_папки => write!(f, "Ошибка создания папки"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ошибка_при_вложении_книги {
    Пустой_ряд_байт,  // save_to_buffer
    Пустой_ряд_строк, // прочитать_xlsx_из_буфера
    Пустые_оба_ряда,  // прочитать_xlsx_с_диска
    Не_определено,    // содержимое.save
}
impl fmt::Display for Ошибка_при_вложении_книги {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Пустой_ряд_байт => write!(f, "Пустой ряд байт"),
            Self::Пустой_ряд_строк => write!(f, "Пустой ряд строк"),
            Self::Пустые_оба_ряда => write!(f, "Пустые оба ряда"),
            Self::Не_определено => write!(f, "Не определено"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Итог_Сохранения {
    Совпало,
    Перезаписано,
    Создано,
}
#[derive(Debug, Clone)]
pub enum Итог_Cравнения_Файлов {
    Полное_Совпадение,
    Не_Существует_Файл_в_папке,
    Пустой_Файл_в_ОЗУ,
    Пустой_Файл_в_папке,
    Данные_не_совпадают,
    Не_удалось_открыть_файл_в_папке,
    Не_Определено,
}
impl fmt::Display for Итог_Cравнения_Файлов {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Полное_Совпадение => write!(f, "Полное совпадение"),
            Self::Не_Существует_Файл_в_папке => {
                write!(f, "Файл не существует в папке")
            }
            Self::Пустой_Файл_в_ОЗУ => write!(f, "Пустой файл в ОЗУ"),
            Self::Пустой_Файл_в_папке => write!(f, "Пустой файл в папке"),
            Self::Данные_не_совпадают => write!(f, "Данные не совпадают"),
            Self::Не_Определено => write!(f, "Не определено"),
            Self::Не_удалось_открыть_файл_в_папке => {
                write!(f, "Не Не удалось открыть файл в папке")
            }
        }
    }
}
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
    Jpe,
    Svg,
    Avif,
    Webp,
    Wmf,
    Wpg,
    Eps,
    Tiff,
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
    Xml,
}
#[derive(Debug, Clone)]
pub enum Вид_Word {
    Doc,
    Docx,
}
#[derive(Debug, Clone)]
pub enum Вид_Excel {
    Xls,
    Xlsx,
}
#[derive(Debug, Clone)]
pub enum Вид_Шрифтов {
    Tif,
    Ttf,
    Otf,
}
#[derive(Debug, Clone)]
pub enum Вид_Разметки_Паутины {
    Php,
    Html,
    Htm,
    Md,
    Yml,
    Fs,
    Xhtml,
    Mhtml,
    Mht,
    Opf,
    Ncx,
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
pub enum Вид_рекламы_HTML {
    Ru,
}
#[derive(Debug, Clone)]
pub enum Основной_Вид_Расширения {
    Книга(Вид_Книги),
    Архив(Вид_Архива),
    Изображение(Вид_Изображения),
    Видео(Вид_Видео),
    Разметка_Паутины(Вид_Разметки_Паутины),
    Мусорная_Разметка(Вид_Мусорные_Разметки_Паутины),
    Word(Вид_Word),
    Excel(Вид_Excel),
    XML(Вид_XML),
    RTF,
    Реклама_html(Вид_рекламы_HTML),
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
    pub fn книга_ли(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Книга(_) =>  true,
            Основной_Вид_Расширения::Простая_письменность(_)=>true,
             Основной_Вид_Расширения::Разметка_Паутины(_)=>true,
              Основной_Вид_Расширения::XML(_)=>true,
                Основной_Вид_Расширения::Приказы(_)=>true,
                      Основной_Вид_Расширения::JS(_)=>true,
                        Основной_Вид_Расширения::Мусорная_Разметка(_)=>true,

            _=>false,
        }
    }
    pub fn шрифты(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Шрифты(_) => true,

            _ => false,
        }
    }
    pub fn разметка_паутины(&self) -> bool {
        match self {
            //Основной_Вид_Расширения::Книга(_) =>  true,
            //Основной_Вид_Расширения::Простая_письменность(_)=>true,
            Основной_Вид_Расширения::Разметка_Паутины(_) => {
                true
            }
            _ => false,
        }
    }
    pub fn простая_письменность(&self) -> bool {
        match self {
            //Основной_Вид_Расширения::Книга(_) =>  true,
            Основной_Вид_Расширения::Простая_письменность(_)=>true,
            /*Основной_Вид_Расширения::Разметка_Паутины(_) => {
                true
            }*/
            _ => false,
        }
    }
    pub fn реклама_html(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Реклама_html(_) => true,
            _ => false,
        }
    }
    pub fn мусорная_разметки_паутины(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Мусорная_Разметка(_) => {
                true
            }
            _ => false,
        }
    }
    pub fn js(&self) -> bool {
        match self {
            Основной_Вид_Расширения::JS(_) => true,
            _ => false,
        }
    }
    pub fn txt(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Простая_письменность(Txt) => true,
            _ => false,
        }
    }
    pub fn без_названия(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Без_Названия => true,
            _ => false,
        }
    }
    pub fn пусто(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Пусто => true,
            _ => false,
        }
    }
    /*pub fn fb2_mht_md_htm_yml_fs_и_т_д(&self) -> bool {
        self.fb2() || self.md_yml_fs() || self.htm_html_xhtml_ncx()
    }*/
    pub fn fb2(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Книга(содержимое) => {
                match содержимое {
                    Вид_Книги::Одиночная(Fb2) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn md(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Разметка_Паутины(
                содержимое,
            ) => match содержимое {
                Вид_Разметки_Паутины::Md => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn htm(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Разметка_Паутины(
                содержимое,
            ) => match содержимое {
                Вид_Разметки_Паутины::Htm => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn html(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Разметка_Паутины(
                содержимое,
            ) => match содержимое {
                Вид_Разметки_Паутины::Html => true,
                _ => false,
            },
            _ => false,
        }
    }
    //
    /*pub fn fb2(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Книга(содержимое) => {
                match содержимое {
                    Вид_Книги::Одиночная(Fb2) => true,
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
    }*/
    pub fn mhtml(&self) -> bool {
        match self {
            // Основной_Вид_Расширения::Разметка_Паутины(_)=>true,
            Основной_Вид_Расширения::Разметка_Паутины(
                содержимое,
            ) => match содержимое {
                Вид_Разметки_Паутины::Mhtml => true,

                _ => false,
            },
            _ => false,
        }
    }
    pub fn epub(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Книга(содержимое) => {
                match содержимое {
                    Вид_Книги::Архивная(Epub) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn fb3(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Книга(содержимое) => {
                match содержимое {
                    Вид_Книги::Архивная(Fb3) => true,
                    _ => false,
                }
            }
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
    pub fn htm_html_xhtml_ncx(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Разметка_Паутины(
                содержимое,
            ) => match содержимое {
                Вид_Разметки_Паутины::Htm
                | Вид_Разметки_Паутины::Html
                | Вид_Разметки_Паутины::Ncx
                | Вид_Разметки_Паутины::Xhtml => true,
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
    pub fn изображение(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Изображение(_) => true,
            _ => false,
        }
    }
    pub fn мусорное_расширение(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Изображение(_) => true,
            Основной_Вид_Расширения::Видео(_) => true,
            Основной_Вид_Расширения::Без_Названия => true,
            Основной_Вид_Расширения::Не_определено => true,
            Основной_Вид_Расширения::Pdf => true,
            Основной_Вид_Расширения::Прочее => true,
            Основной_Вид_Расширения::Excel(_) => true,
            Основной_Вид_Расширения::Word(_) => true,
            Основной_Вид_Расширения::Архив(_) => true,
            Основной_Вид_Расширения::Мусорная_Разметка(_) => {
                true
            }
            Основной_Вид_Расширения::Пусто => true,
            _ => false,
        }
    }
    pub fn doc_расширение(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Word(_) => true,

            _ => false,
        }
    }
    pub fn rtf(&self) -> bool {
        match self {
            Основной_Вид_Расширения::RTF => true,

            _ => false,
        }
    }
    pub fn не_определено(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Не_определено => true,

            _ => false,
        }
    }
}
#[derive(Debug, Clone)]
pub enum Умная_Строка {
    Пусто,
    Значение(String),
}
impl PartialEq for Умная_Строка {
    fn eq(&self, вторая: &Self) -> bool {
        match (self, вторая) {
            (Умная_Строка::Пусто, Умная_Строка::Пусто) => true,
            (Умная_Строка::Значение(s1), Умная_Строка::Значение(s2)) =>
            {
                // Можно добавить свою логику, например:
                // - сравнение без учёта регистра
                // - игнорирование пробелов
                // - семантическое сравнение
                s1.as_str() == s2.as_str()
            }
            _ => false,
        }
    }
}

// Вид_Слова
impl fmt::Display for Вид_Слова {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Слова::Исходное => write!(f, "исходное"),
            Вид_Слова::Замена => write!(f, "замена"),
        }
    }
}
// Вид_Видео
impl fmt::Display for Вид_рекламы_HTML {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_рекламы_HTML::Ru => write!(f, "ru"),
        }
    }
}

// Вид_Видео
impl fmt::Display for Вид_Видео {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Видео::Avi => write!(f, "avi"),
            Вид_Видео::Mkv => write!(f, "mkv"),
            Вид_Видео::Mp4 => write!(f, "mp4"),
            Вид_Видео::WebM => write!(f, "webm"),
        }
    }
}

// Вид_Звук
impl fmt::Display for Вид_Звук {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Звук::Wav => write!(f, "wav"),
            Вид_Звук::Mp3 => write!(f, "mp3"),
            Вид_Звук::Ogg => write!(f, "ogg"),
            Вид_Звук::Aac => write!(f, "aac"),
        }
    }
}

// Вид_Изображения
impl fmt::Display for Вид_Изображения {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Изображения::Jpeg => write!(f, "jpeg"),
            Вид_Изображения::Png => write!(f, "png"),
            Вид_Изображения::Bmp => write!(f, "bmp"),
            Вид_Изображения::Jpe => write!(f, "jpe"),
            Вид_Изображения::Gif => write!(f, "gif"),
            Вид_Изображения::Tif => write!(f, "tif"),
            Вид_Изображения::Tiff => write!(f, "tiff"),
            Вид_Изображения::Jpg => write!(f, "jpg"),
            Вид_Изображения::Svg => write!(f, "svg"),
            Вид_Изображения::Avif => write!(f, "avif"),
            Вид_Изображения::Webp => write!(f, "webp"),
            Вид_Изображения::Wmf => write!(f, "wmf"),
            Вид_Изображения::Wpg => write!(f, "wpg"),
            Вид_Изображения::Eps => write!(f, "eps"),
            Вид_Изображения::Emf => write!(f, "emf"),
        }
    }
}

// Вид_Архива
impl fmt::Display for Вид_Архива {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Архива::Zip => write!(f, "zip"),
            Вид_Архива::Rar => write!(f, "rar"),
            Вид_Архива::Gz => write!(f, "gz"),
            Вид_Архива::Gzip => write!(f, "gzip"),
        }
    }
}

// Вид_Мусорные_Разметки_Паутины
impl fmt::Display for Вид_Мусорные_Разметки_Паутины {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Мусорные_Разметки_Паутины::Css => write!(f, "css"),
            Вид_Мусорные_Разметки_Паутины::Thmx => write!(f, "thmx"),
        }
    }
}

// Вид_XML
impl fmt::Display for Вид_XML {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_XML::Rels => write!(f, "rels"),
            Вид_XML::Xml => write!(f, "xml"),
        }
    }
}

// Вид_Word
impl fmt::Display for Вид_Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Word::Doc => write!(f, "doc"),
            Вид_Word::Docx => write!(f, "docx"),
        }
    }
}

// Вид_Excel
impl fmt::Display for Вид_Excel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Excel::Xls => write!(f, "xls"),
            Вид_Excel::Xlsx => write!(f, "xlsx"),
        }
    }
}

// Вид_Шрифтов
impl fmt::Display for Вид_Шрифтов {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Шрифтов::Tif => write!(f, "tif"),
            Вид_Шрифтов::Ttf => write!(f, "ttf"),
            Вид_Шрифтов::Otf => write!(f, "otf"),
        }
    }
}

// Вид_Разметки_Паутины
impl fmt::Display for Вид_Разметки_Паутины {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Разметки_Паутины::Ncx => write!(f, "ncx"),
            Вид_Разметки_Паутины::Opf => write!(f, "opf"),
            Вид_Разметки_Паутины::Php => write!(f, "php"),
            Вид_Разметки_Паутины::Html => write!(f, "html"),
            Вид_Разметки_Паутины::Htm => write!(f, "htm"),
            Вид_Разметки_Паутины::Md => write!(f, "md"),
            Вид_Разметки_Паутины::Yml => write!(f, "yml"),
            Вид_Разметки_Паутины::Fs => write!(f, "fs"),
            Вид_Разметки_Паутины::Xhtml => write!(f, "xhtml"),
            Вид_Разметки_Паутины::Mhtml => write!(f, "mhtml"),
            Вид_Разметки_Паутины::Mht => write!(f, "mht"),
        }
    }
}

// Вид_Архивной_Книги
impl fmt::Display for Вид_Архивной_Книги {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Архивной_Книги::Epub => write!(f, "epub"),
            Вид_Архивной_Книги::Fb3 => write!(f, "fb3"),
        }
    }
}

// Вид_одичноной_книги
impl fmt::Display for Вид_одичноной_книги {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_одичноной_книги::Fb2 => write!(f, "fb2"),
        }
    }
}

// Вид_Книги
impl fmt::Display for Вид_Книги {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Книги::Архивная(вид) => write!(f, "архивная_книга({})", вид),
            Вид_Книги::Одиночная(вид) => write!(f, "одиночная_книга({})", вид),
        }
    }
}

// Вид_JS
impl fmt::Display for Вид_JS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_JS::Js => write!(f, "js"),
            Вид_JS::Mjs => write!(f, "mjs"),
            Вид_JS::Cjs => write!(f, "cjs"),
        }
    }
}

// Вид_Справи
impl fmt::Display for Вид_Справи {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Справи::Cnt => write!(f, "cnt"),
            Вид_Справи::Hlp => write!(f, "hlp"),
            Вид_Справи::Chm => write!(f, "chm"),
        }
    }
}

// Вид_приказов
impl fmt::Display for Вид_приказов {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_приказов::Tcl => write!(f, "tcl"),
            Вид_приказов::Fcg => write!(f, "fcg"),
            Вид_приказов::Cgi => write!(f, "cgi"),
        }
    }
}

// Основной_Вид_Расширения
impl fmt::Display for Основной_Вид_Расширения {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Основной_Вид_Расширения::Книга(вид) => {
                write!(f, "книга({})", вид)
            }
            Основной_Вид_Расширения::Реклама_html(вид) => {
                write!(f, "реклама html({})", вид)
            }
            Основной_Вид_Расширения::Архив(вид) => {
                write!(f, "архив({})", вид)
            }
            Основной_Вид_Расширения::Изображение(вид) => {
                write!(f, "изображение({})", вид)
            }
            Основной_Вид_Расширения::Видео(вид) => {
                write!(f, "видео({})", вид)
            }
            Основной_Вид_Расширения::Разметка_Паутины(вид) =>
            {
                write!(f, "разметка_паутины({})", вид)
            }
            Основной_Вид_Расширения::Мусорная_Разметка(
                вид,
            ) => write!(f, "мусорная_разметка({})", вид),
            Основной_Вид_Расширения::Word(вид) => {
                write!(f, "word({})", вид)
            }
            Основной_Вид_Расширения::Excel(вид) => {
                write!(f, "excel({})", вид)
            }
                Основной_Вид_Расширения::RTF => write!(f, "rtf"),
            Основной_Вид_Расширения::XML(вид_xml) => match вид_xml {
                Вид_XML::Xml=>
                write!(f, "xml"),
                Вид_XML::Rels=>
                write!(f, "rels")},
            Основной_Вид_Расширения::JS(вид) => write!(f, "js({})", вид),
            Основной_Вид_Расширения::Pdf => write!(f, "pdf"),
            Основной_Вид_Расширения::Прочее => write!(f, "прочее"),
            Основной_Вид_Расширения::Шрифты(вид) => {
                write!(f, "шрифты({})", вид)
            }
            Основной_Вид_Расширения::MIME => write!(f, "mime"),
             Основной_Вид_Расширения::Простая_письменность(вид) => write!(f, "простая_письменность({})", вид),
            Основной_Вид_Расширения::Приказы(вид) => {
                write!(f, "приказы({})", вид)
            }
            Основной_Вид_Расширения::Пусто => write!(f, "пусто"),
            Основной_Вид_Расширения::Без_Названия => {
                write!(f, "без_названия")
            }
            Основной_Вид_Расширения::Справка(вид) => {
                write!(f, "справка({})", вид)
            }
            Основной_Вид_Расширения::Не_определено => {
                write!(f, "не_определено")
            }
        }
    }
}
//
impl PartialEq<String> for Умная_Строка {
    fn eq(&self, вторая: &String) -> bool {
        match self {
            Умная_Строка::Пусто => вторая.is_empty(),
            Умная_Строка::Значение(s) => s.as_str() == вторая.as_str(),
        }
    }
}

impl PartialEq<Умная_Строка> for String {
    fn eq(&self, вторая: &Умная_Строка) -> bool {
        вторая.as_str() == self.as_str()
    }
}

impl From<Умная_Строка> for String {
    fn from(значение: Умная_Строка) -> Self {
        значение.to_string() // или любое преобразование
    }
}
impl IntoExcelData for Умная_Строка {
    //
    /* fn write(self, worksheet: &mut Worksheet, row: RowNum, col: ColNum) -> Result<&mut Worksheet, XlsxError> {
        worksheet.write_string(row, col, self.as_str()) // без to_string()
    }*/
    fn write(
        self,
        страница: &mut Worksheet,
        строка: RowNum,
        столбец: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        // Предполагаем, что у вашей структуры есть метод .as_str(), или вы просто хотите записать её отладочное представление.
        // Если структура напрямую не является строкой, используйте format!("{:?}", self) или другой метод сериализации.
        //let string_data = self.to_string(); // Или self.as_str()
        let string_data: &str = self.as_str(); // или self.to_string().as_str(), но лучше без выделения памяти
        страница.write_string(строка, столбец, string_data)
    }

    fn write_with_format<'a>(
        self,
        страница: &'a mut Worksheet,
        строка: RowNum,
        столбец: ColNum,
        format: &Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        //let string_data = self.to_string();
        let string_data: &str = self.as_str(); // или self.to_string().as_str(), но лучше без выделения памяти
        страница.write_string_with_format(строка, столбец, string_data, format)
    }
}
impl Default for Умная_Строка {
    fn default() -> Self {
        Умная_Строка::Пусто
    }
}
// Реализация преобразования из Vec<String> в Vec<Умная_Строка>
// Добавляем методы напрямую для Vec<Умная_Строка>
// Определяем свой трейт
// Определяем трейт
pub trait Умные_Строки_Ряд {
    fn в_умные(self) -> Vec<Умная_Строка>;
}

// impl для Vec<String>
impl Умные_Строки_Ряд for Vec<String> {
    fn в_умные(self) -> Vec<Умная_Строка> {
        self.into_iter()
            .map(Умная_Строка::создать_значение)
            .collect()
    }
}
pub fn в_умные_строки<S: Into<String>>(
    строки: Vec<S>
) -> Vec<Умная_Строка> {
    строки
        .into_iter()
        .map(|s| Умная_Строка::создать_значение(s.into()))
        .collect()
}
impl std::ops::Deref for Умная_Строка {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Умная_Строка::Пусто => "",
            Умная_Строка::Значение(s) => s.as_str(),
        }
    }
}
impl AsRef<str> for Умная_Строка {
    fn as_ref(&self) -> &str {
        match self {
            Умная_Строка::Пусто => "",
            Умная_Строка::Значение(s) => s.as_str(),
        }
    }
}
use convert_case::{Case, Casing};
impl Умная_Строка {
    // Замена всех вхождений подстроки
    pub fn replace(&self, from: &str, to: &str) -> Self {
        match self {
            Умная_Строка::Значение(s) => {
                Умная_Строка::создать_значение(s.replace(from, to))
            }
            Умная_Строка::Пусто => Умная_Строка::Пусто,
        }
    }

    pub fn replace_all<'a, I>(&self, pairs: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        match self {
            Умная_Строка::Пусто => Умная_Строка::Пусто,
            Умная_Строка::Значение(s) => {
                let result = pairs
                    .into_iter()
                    .fold(s.clone(), |acc, (from, to)| acc.replace(from, to));
                Умная_Строка::Значение(result)
            }
        }
    }
    pub fn replace_range<R: std::ops::RangeBounds<usize>>(&mut self, range: R, replace_with: &str) {
        if let Умная_Строка::Значение(s) = self {
            s.replace_range(range, replace_with);
        }
        // для Пусто — ничего не делаем (или паникуем — на ваш выбор)
    }
    pub fn to_case(&self, case: Case) -> Self {
        match self {
            Умная_Строка::Значение(s) => {
                Умная_Строка::создать_значение(s.to_case(case))
            }
            Умная_Строка::Пусто => Умная_Строка::Пусто,
        }
    }
    //
    pub fn получить_значение_f32(&self) -> Result<f32, String> {
        match self {
            Умная_Строка::Пусто => {
                Err(format!("Нельзя извлечь f32 из пустой Умной строки"))
            }
            Умная_Строка::Значение(значение) => {
                match значение.parse::<f32>() {
                    Ok(успех) => return Ok(успех),
                    Err(ошибка) => {
                        return Err(format!(
                            "Не удалось извлечь f32 из Умной строки |{}| Ошибка: |{:?}|",
                            значение, ошибка
                        ));
                    }
                }
            }
        }
    }
    pub fn вложить_значение_либо_ошибка(&self) -> Result<String, ()> {
        match self {
            Умная_Строка::Пусто => Err(()),
            Умная_Строка::Значение(содержимое) => {
                Ok(содержимое.to_string())
            }
        }
    }
    //
    pub fn вложить_значение_XLSX_либо_ошибка(
        &mut self,
        ячейка: impl Into<Значение_Ячейки_XLSX>,
    ) -> Result<(), ()> {
        let ячейка = ячейка.into();
        //если не пусто - не вкладывать
        if self.не_пусто() {
            return Ok(());
        }
        //
        match ячейка {
            Значение_Ячейки_XLSX::Пустое_значение => {
                *self = Умная_Строка::Пусто;
                Ok(())
            }
            Значение_Ячейки_XLSX::Строка(содержимое) => {
                *self = Умная_Строка::создать_значение(содержимое);
                Ok(())
            }
            Значение_Ячейки_XLSX::Ошибка(содержимое) => {
                *self = Умная_Строка::создать_значение(содержимое);
                Ok(())
            }
            Значение_Ячейки_XLSX::Разумное(содержимое) => {
                *self = Умная_Строка::создать_значение(
                    &содержимое.to_string(),
                );
                Ok(())
            }
            Значение_Ячейки_XLSX::Целое(содержимое) => {
                *self = Умная_Строка::создать_значение(
                    &содержимое.to_string(),
                );
                Ok(())
            }
            Значение_Ячейки_XLSX::Вещественное(содержимое) => {
                *self = Умная_Строка::создать_значение(
                    &содержимое.to_string(),
                );
                Ok(())
            } // _ => Err(()),
        }
    }
    //
    pub fn as_str(&self) -> &str {
        match self {
            Умная_Строка::Пусто => "",
            Умная_Строка::Значение(значение) => значение.as_str(),
        }
    }
    pub fn создать_значение_из_XLSX(
        ячейка: impl Into<Значение_Ячейки_XLSX>,
    ) -> Self {
        /*static ОБРАЗЦЫ_RE: LazyLock<[Regex; 1]> =
        LazyLock::new(|| [Regex::new("(?i)Пусто$").unwrap()]);*/
        let ячейка = ячейка.into(); // получаем Значение_Ячейки_XLSX
        match ячейка {
            Значение_Ячейки_XLSX::Пустое_значение => {
                Умная_Строка::Пусто
            }
            Значение_Ячейки_XLSX::Строка(содержимое) => {
                Умная_Строка::создать_значение(содержимое)
            }
            Значение_Ячейки_XLSX::Ошибка(содержимое) => {
                Умная_Строка::создать_значение(содержимое)
            }
            Значение_Ячейки_XLSX::Разумное(содержимое) => {
                Умная_Строка::создать_значение(&содержимое.to_string())
            }
            Значение_Ячейки_XLSX::Целое(содержимое) => {
                Умная_Строка::создать_значение(&содержимое.to_string())
            }
            Значение_Ячейки_XLSX::Вещественное(содержимое) => {
                Умная_Строка::создать_значение(&содержимое.to_string())
            }
        }
    }

    pub fn создать_значение_из_XLSX_заменить_точки_на_нижние_подчёркивания(
        ячейка: impl Into<Значение_Ячейки_XLSX>,
    ) -> Self {
        let ячейка = ячейка.into(); // получаем Значение_Ячейки_XLSX
        match ячейка {
            Значение_Ячейки_XLSX::Пустое_значение => {
                Умная_Строка::Пусто
            }
            Значение_Ячейки_XLSX::Строка(содержимое) => {
                Умная_Строка::создать_значение(
                    содержимое.to_uppercase().replace(".", "_"),
                )
            }
            Значение_Ячейки_XLSX::Ошибка(содержимое) => {
                Умная_Строка::создать_значение(
                    содержимое.to_uppercase().replace(".", "_"),
                )
            }
            Значение_Ячейки_XLSX::Разумное(содержимое) => {
                Умная_Строка::создать_значение(&содержимое.to_string())
            }
            Значение_Ячейки_XLSX::Целое(содержимое) => {
                Умная_Строка::создать_значение(&содержимое.to_string())
            }
            Значение_Ячейки_XLSX::Вещественное(содержимое) => {
                Умная_Строка::создать_значение(&содержимое.to_string())
            }
        }
    }

    pub fn создать_значение(строка: impl Into<String>) -> Self {
        static ОБРАЗЦЫ_RE: LazyLock<[Regex; 1]> =
            LazyLock::new(|| [Regex::new("(?i)Пусто$").unwrap()]);
        let строка = строка.into();
        if строка.is_empty() {
            return Умная_Строка::Пусто;
        }
        for образец in ОБРАЗЦЫ_RE.iter() {
            if образец.is_match(строка.as_str()) {
                return Умная_Строка::Пусто;
            }
        }
        let исправленная = строка
            .replace('\u{00A0}', " ") // NBSP → пробел
            .replace('\u{2009}', " ") // тонкий → пробел
            .replace('\u{202F}', " ") // узкий неразрывный → пробел
            .replace('\u{200B}', "") // zero-width → удалить
            .replace('\u{FEFF}', ""); // BOM → удалить
        return Умная_Строка::Значение(исправленная.to_string());
    }
    pub fn создать_значение_можно_пусто(
        строка: impl Into<String>,
    ) -> Self {
        static ОБРАЗЦЫ_RE: LazyLock<[Regex; 1]> =
            LazyLock::new(|| [Regex::new("(?i)Пусто$").unwrap()]);
        let строка = строка.into();
        if строка.is_empty() {
            return Умная_Строка::Значение("".to_string());
        }
        for образец in ОБРАЗЦЫ_RE.iter() {
            if образец.is_match(строка.as_str()) {
                return Умная_Строка::Пусто;
            }
        }
        let исправленная = строка
            .replace('\u{00A0}', " ") // NBSP → пробел
            .replace('\u{2009}', " ") // тонкий → пробел
            .replace('\u{202F}', " ") // узкий неразрывный → пробел
            .replace('\u{200B}', "") // zero-width → удалить
            .replace('\u{FEFF}', ""); // BOM → удалить
        return Умная_Строка::Значение(исправленная.to_string());
    }

    pub fn создать_значение_из_str(строка: &str) -> Self {
        static ОБРАЗЦЫ_RE: LazyLock<[Regex; 1]> =
            LazyLock::new(|| [Regex::new("(?i)Пусто$").unwrap()]);

        if строка.is_empty() {
            return Умная_Строка::Пусто;
        }
        for образец in ОБРАЗЦЫ_RE.iter() {
            if образец.is_match(строка) {
                return Умная_Строка::Пусто;
            }
        }
        return Умная_Строка::Значение(строка.to_string());
    }
    pub fn получить_значение(&self) -> String {
        match self {
            Умная_Строка::Пусто => "Пусто".to_string(),
            Умная_Строка::Значение(содержимое) => {
                содержимое.to_string()
            }
        }
    }
    pub fn получить_ссылку(&self) -> &str {
        match self {
            Умная_Строка::Пусто => "Пусто",
            Умная_Строка::Значение(содержимое) => содержимое.as_str(),
        }
    }
    pub fn получить_ссылку_на_строку(&self) -> &String {
        static ПУСТО: LazyLock<String> = LazyLock::new(|| "Пусто".to_string());
        match self {
            Умная_Строка::Пусто => &ПУСТО,
            Умная_Строка::Значение(содержимое) => {
                &содержимое
            }
        }
    }

    pub fn есть_ли_значение(&self) -> bool {
        match self {
            Умная_Строка::Пусто => false,
            Умная_Строка::Значение(_) => true,
        }
    }
    pub fn не_пусто(&self) -> bool {
        match self {
            Умная_Строка::Пусто => false,
            Умная_Строка::Значение(_) => true,
        }
    }
    pub fn не_примечание(&self) -> bool {
        match self {
            Умная_Строка::Пусто => false,
            Умная_Строка::Значение(содердимое) => {
                match содердимое.as_str() {
                    "#" => false,
                    "# " => false,
                    _ => true,
                }
            }
        }
    }
    pub fn примечание(&self) -> bool {
        match self {
            Умная_Строка::Пусто => false,
            Умная_Строка::Значение(содердимое) => {
                match содердимое.as_str() {
                    "#" => true,
                    "# " => true,
                    _ => false,
                }
            }
        }
    }

    pub fn пусто(&self) -> bool {
        match self {
            Умная_Строка::Пусто => true,
            Умная_Строка::Значение(значение) => {
                //
                match значение.as_str() {
                    "Пусто" => true,
                    "" => true,
                    _ => false,
                }
            }
        }
    }
    pub fn is_some(&self) -> bool {
        matches!(self, Умная_Строка::Значение(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Умная_Строка::Пусто)
    }
}

impl From<&Значение_Ячейки_XLSX> for Значение_Ячейки_XLSX {
    fn from(ячейка: &Значение_Ячейки_XLSX) -> Self {
        ячейка.clone() // так как у вас уже есть #[derive(Clone)]
    }
}

impl fmt::Display for Умная_Строка {
    fn fmt(&self, образ: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Умная_Строка::Пусто => write!(образ, "Пусто"),
            Умная_Строка::Значение(содержимое) => {
                write!(образ, "{}", содержимое)
            }
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

impl fmt::Display for Вид_простой_письменности {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_простой_письменности::Txt => write!(f, "txt"),
            Вид_простой_письменности::Log => write!(f, "log"),
            //Вид_простой_письменности::Csv => write!(f, "csv"),
        }
    }
}
//пути
#[derive(Debug, Clone)]
pub struct Пути_Общие {
    pub книги: &'static str,
    pub словари: &'static str,
    pub переносы: &'static str,
    pub вывод_книги: &'static str,
    pub вывод_словари: &'static str,
    pub вывод: &'static str,
    pub вывод_книги_кучи: &'static str,
    pub вывод_книги_пропуски: &'static str,
    pub вывод_книги_проверка_после_замены_слов: &'static str,
    pub вывод_книги_запись_и_чтение: &'static str,
    pub вывод_книг_с_разделением: &'static str,
    //окончания
    pub вывод_окончания_слов: &'static str,

    //
    pub словарь_запасной_чистый: &'static str,
}
pub const ПУТИ_ОБЩИЕ: Пути_Общие = Пути_Общие {
    книги: "./книги/",
    словари: "./словари/",
    переносы: "./переносы/",
    вывод: "./вывод/",
    // вложенные
    вывод_словари: "./вывод/словари/",
    вывод_книги: "./вывод/книги/",
    вывод_книг_с_разделением: "./вывод/книги_с_разделениями/",
    вывод_книги_кучи: "./вывод/кучи/",
    вывод_книги_пропуски: "./вывод/пропуски/",
    вывод_книги_проверка_после_замены_слов: "./вывод/проверка_после_замены_слов/",
    вывод_книги_запись_и_чтение: "./вывод/запись_проверка/",
    вывод_окончания_слов: "./вывод/окончания_слов/",
    //
    словарь_запасной_чистый: "./вывод/словари/",
};
#[derive(Debug, Clone)]
pub struct Пути_Вывода_Словарей {
    pub запасной: String,
    pub все_вместе: String,
    pub словари_кучи: String,
    pub основной_недостающие: String,
    pub разделители: String,
    pub переносов_обычные: String,
    pub переносов_длинная_черта: String,
    pub переносов_пробел_преед_чертой: String,
}
pub static ПУТИ_ВЫВОДА_СЛОВАРЕЙ: LazyLock<Пути_Вывода_Словарей> =
    LazyLock::new(|| Пути_Вывода_Словарей {
        запасной: format!("{}Запасной.xlsx", ПУТИ_ОБЩИЕ.вывод_словари),
        все_вместе: format!("{}Все словари вместе.xlsx", ПУТИ_ОБЩИЕ.вывод_словари),
        словари_кучи: format!("{}Словари_кучи.xlsx", ПУТИ_ОБЩИЕ.вывод_словари),
        основной_недостающие: format!(
            "{}Основной_недостающие.xlsx",
            ПУТИ_ОБЩИЕ.вывод_словари
        ),
        разделители: format!("{}Словарь_разделителей_.xlsx", ПУТИ_ОБЩИЕ.вывод_словари),
        переносов_обычные: format!(
            "{}Словари_переносов_обычные.xlsx",
            ПУТИ_ОБЩИЕ.вывод_словари
        ),
        переносов_длинная_черта: format!(
            "{}Словари_переносов_длинное_тире.xlsx",
            ПУТИ_ОБЩИЕ.вывод_словари
        ),
        переносов_пробел_преед_чертой: format!(
            "{}Словари_переносов_пробел_перед_тире.xlsx",
            ПУТИ_ОБЩИЕ.вывод_словари
        ),
    });
#[derive(Debug, Clone)]
pub struct Пути_Вывода_Окончаний {
    pub окончания_недостающие: String,
    pub окончания_запрещённые: String,
    pub окончания_части_речи: String,
    //окончания
    pub ошибки_частей_речи_окончаний_слов: String,
}
pub static ПУТИ_ВЫВОДА_ОКОНЧАНИЙ: LazyLock<Пути_Вывода_Окончаний> =
    LazyLock::new(|| Пути_Вывода_Окончаний {
        окончания_недостающие: format!(
            "{}недостающие.xlsx",
            ПУТИ_ОБЩИЕ.вывод_окончания_слов
        ),
        окончания_запрещённые: format!(
            "{}запрещённые.xlsx",
            ПУТИ_ОБЩИЕ.вывод_окончания_слов
        ),
        окончания_части_речи: format!(
            "{}части_речи.xlsx",
            ПУТИ_ОБЩИЕ.вывод_окончания_слов
        ),
        ошибки_частей_речи_окончаний_слов: format!(
            "{}ошибки_окончаний_все.xlsx",
            ПУТИ_ОБЩИЕ.вывод_окончания_слов
        ),
    });
/*impl Default for Пути_Общие {
    fn default() -> Self {
        Self {
            книги: "./книги/",
            словари: "./словари/",
            переносы: "./перееносы/",
            вывод: "./вывод/",
            //вложенные
            вывод_словари: "./вывод/словари/",
            вывод_книги: "./вывод/книги/",
            вывод_книг_с_разделением: "./вывод/книги_с_разделениями/",
            вывод_книги_кучи: "./вывод/кучи/",
            вывод_книги_пропуски: "./вывод/пропуски/",
            вывод_книги_проверка_после_замены_слов: "./вывод/проверка_после_замены_слов/",
            вывод_книги_запись_и_чтение: "./вывод/запись_проверка/",
            вывод_окончания_слов: "./вывод/окончания_слов/",
            //
            словарь_запасной_чистый: "./вывод/словари/",
        }
    }
}*/
#[derive(Debug, Clone, Default)]
pub struct Книги_в_ОЗУ {
    //файлы
    pub книги: Vec<Книга_целиковая>,
    pub размер_в_озу: usize,
}

#[derive(Debug, Clone)]
pub struct Указатель_на_две_кучи_полные_словаря<'a> {
    //файлы
    pub искомая: &'a rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub замена: &'a rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
}
#[derive(Debug, Clone)]
pub struct Указатель_на_две_кучи_простых_словаря<'a> {
    //файлы
    pub искомая: &'a rapidhash::fast::RapidHashSet<String>,
    pub замена: &'a rapidhash::fast::RapidHashSet<String>,
}
#[derive(Debug, Clone, Default)]
pub struct Словари_с_кучами {
    //файлы
    pub сам: Полный_Словарь,
    pub кучи_полные: Словарь_Куч_полных,
    pub кучи_простые: Словарь_Куч_простых,
}
#[derive(Debug, Clone)]
pub struct Пути_Вывода {
    //файлы
    pub вывод_сообщений: &'static str,
    pub вывод_кучи_словаря: &'static str,
    pub вывод_кучи_словаря_ключи: &'static str,
    pub вывод_книг_проверки_замен: &'static str,
    pub вывод_книги_запись_и_чтение: &'static str,
    pub вывод_кодировка: &'static str,
    pub вывод_ошибок_поиска_окончаний: &'static str,
}
pub const ПУТИ_ВЫВОДА: Пути_Вывода = Пути_Вывода {
    // файлы пошли уже
    вывод_сообщений: "./вывод/сообщения.txt",
    вывод_кучи_словаря: "./вывод/кучи/куча_",
    вывод_кучи_словаря_ключи: "./вывод/кучи/куча_словарь_ключи_",
    вывод_книг_проверки_замен: "./вывод/проверка_после_замены_слов/",
    вывод_книги_запись_и_чтение: "./вывод/запись_проверка/",
    вывод_кодировка: "./вывод/остальное/кодировка.txt",
    вывод_ошибок_поиска_окончаний: "./вывод/словари/",
};
/*impl Default for Пути_Вывода {
    fn default() -> Self {
        Self {
            //файлы пошли уже
            вывод_сообщений: "./вывод/сообщения.txt",
            вывод_кучи_словаря: "./вывод/кучи/куча_",
            вывод_кучи_словаря_ключи: "./вывод/кучи/куча_словарь_ключи_",
            вывод_книг_проверки_замен: "./вывод/проверка_после_замены_слов/",
            вывод_книги_запись_и_чтение: "./вывод/запись_проверка/",
            вывод_кодировка: "./вывод/остальное/кодировка.txt",
            вывод_ошибок_поиска_окончаний: "./вывод/словари/",
        }
    }
}*/

#[derive(Debug, Default, Clone)]
pub struct Сообщения_для_книги {
    pub имя_книги: Умная_Строка,
    pub путь_откудаво: Умная_Строка,
    pub расширение: Расширение_полное,
    pub сообщения: rapidhash::fast::RapidHashSet<String>,
}
//содержимое
#[derive(Debug, Default, Clone)]
pub struct Сообщения_считывания_книг {
    pub общие: rapidhash::fast::RapidHashSet<String>,
    pub запись_и_чтение: Vec<Сообщения_для_книги>,
    pub проверка_после_замен: Vec<Сообщения_для_книги>,
    pub чтение_книг: rapidhash::fast::RapidHashSet<String>,
    pub кодировка: rapidhash::fast::RapidHashSet<String>,
}
impl Сообщения_считывания_книг {
    /// Собирает общие сообщения считывания из набора книг.
    pub fn из_стопки_книг(
        стопки_книг: &[Книга_с_сообщениями]
    ) -> Self {
        let mut итог: Self = Default::default();
        for книга_с_сообщениями in стопки_книг {
            let с = &книга_с_сообщениями.сообщения_файлов;
            итог.общие.extend(с.общие.iter().cloned());
            итог.чтение_книг.extend(с.чтение_книг.iter().cloned());
            итог.кодировка.extend(с.кодировка.iter().cloned());
            итог
                .запись_и_чтение
                .extend(с.запись_и_чтение.iter().cloned());
            итог
                .проверка_после_замен
                .extend(с.проверка_после_замен.iter().cloned());
        }
        итог
    }
}
pub trait Возможности_Сообщений {
    fn вложить_оба(первый_ряд: Self, other: Self) -> Self;
}
impl Возможности_Сообщений for Сообщения_считывания_книг {
    fn вложить_оба(первый_ряд: Self, other: Self) -> Self {
        let mut главный_ряд: Сообщения_считывания_книг = Default::default();
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
impl Сообщения_считывания_книг {
    pub fn вложить(&mut self, other: Сообщения_считывания_книг) {
        self.общие.extend(other.общие);
        self.запись_и_чтение.extend(other.запись_и_чтение);
        self.проверка_после_замен.extend(other.проверка_после_замен);
        self.чтение_книг.extend(other.чтение_книг);
        self.кодировка.extend(other.кодировка);
    }
}
#[derive(Debug, Clone)]
pub struct Книга_с_сообщениями {
    pub книга: Книга_целиковая,
    pub сообщения_файлов: Сообщения_считывания_книг,
    pub содержимое_папок: Содержимое_папок,
}
//содержимое
#[derive(Debug, Default, Clone)]
pub struct Содержимое_папок {
    pub файлы: rapidhash::fast::RapidHashSet<String>,
    pub ошибки: rapidhash::fast::RapidHashSet<String>,
    pub не_вложено: rapidhash::fast::RapidHashSet<String>,
}
impl Содержимое_папок {
    /// Собирает общее содержимое папок из набора книг:
    /// складывает в один набор файлы, ошибки и «не вложено».
    pub fn из_стопки_книг(
        стопки_книг: &[Книга_с_сообщениями]
    ) -> Self {
        let mut итог: Self = Default::default();
        for книга_с_сообщениями in стопки_книг {
            let сод = &книга_с_сообщениями.содержимое_папок;
            итог.файлы.extend(сод.файлы.iter().cloned());
            итог.ошибки.extend(сод.ошибки.iter().cloned());
            итог.не_вложено.extend(сод.не_вложено.iter().cloned());
        }
        итог
    }
}

//Стопка с путём до книги и содержимым виде вектора строк
#[derive(Debug, Default, Clone)]
pub struct Книга_целиковая {
    pub путь: Умная_Строка,           //путь до книги
    pub название_книги: Умная_Строка, //имя книги
    pub вложения: Vec<Вложения>,      //содержимое\
    pub расширение: Расширение_полное,
    pub архив: rapidhash::fast::RapidHashMap<String, Vec<u8>>, //для zip
    pub книга_ли: bool,
    //pub содержимое:Vec<String>,//сами строки
}
#[derive(Debug, Default, Clone)]
pub struct Расширение_полное {
    pub строка: Умная_Строка,
    pub вид: Основной_Вид_Расширения, //формат
}

//содержимое - имя файла и его содержимое
#[derive(Debug, Clone)]
pub struct Вложения {
    pub содержимое: Vec<Умная_Строка>, //содержимое
    pub содержимое_в_байтах: Vec<u8>,
    pub имя: Умная_Строка,
    pub путь_полный: Умная_Строка,
    pub имя_без_пути: Умная_Строка,
    pub кодировка: Кодировка,
    pub расширение: Расширение_полное,
    //pub изображение: Vec<u8>, //если это картинки, нельзя в utf8 переводить
}

impl Default for Вложения {
    fn default() -> Self {
        Self {
            содержимое: Vec::new(),
            расширение: Default::default(),
            содержимое_в_байтах: Vec::new(),
            путь_полный: Умная_Строка::Пусто,
            имя: Умная_Строка::Пусто,
            имя_без_пути: Умная_Строка::Пусто,
            кодировка: Кодировка::Не_определено,
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
    Cоставная_стр,
    Составные_важные_стр,
    Составные_длинные_стр,
    Огласовки_стр,
    Вездесущее_стр,
    Неизменные_стр,
    Неизменные_длинные_стр,
    Неизменные_короткие_стр,
    Запятые_стр,
    Запятые_длинные_стр,
    Перевести_стр,
    //
}

//use std::fmt;
use std::fmt::Display;
impl Имена_страниц {
    pub fn получить_имя_страницы_для_xlsx(&self) -> String {
        return match self {
            Имена_страниц::Перевести_стр => "Перевести".to_string(),
            Имена_страниц::Простая_стр => "Простые".to_string(),
            Имена_страниц::Cоставная_стр => "Составные".to_string(),
            Имена_страниц::Составные_важные_стр => {
                "Составные_важные".to_string()
            }
            Имена_страниц::Составные_длинные_стр => {
                "Составные_длинные".to_string()
            }
            Имена_страниц::Огласовки_стр => "Огласовки".to_string(),
            Имена_страниц::Вездесущее_стр => "Вездесущие".to_string(),
            Имена_страниц::Неизменные_стр => "Неизменные".to_string(),
            Имена_страниц::Неизменные_длинные_стр => {
                "Неизменные_длинные".to_string()
            }
            Имена_страниц::Неизменные_короткие_стр => {
                "Неизменные_короткие".to_string()
            }
            Имена_страниц::Запятые_стр => "Запятые".to_string(),
            Имена_страниц::Запятые_длинные_стр => {
                "Запятые_длинные".to_string()
            }
        };
    }
}
//
impl fmt::Display for Имена_страниц {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Имена_страниц::Перевести_стр => write!(f, "Перевести"),
            Имена_страниц::Простая_стр => write!(f, "Простые"),
            Имена_страниц::Cоставная_стр => write!(f, "Составные"),
            Имена_страниц::Составные_важные_стр => {
                write!(f, "Составные важные")
            }
            Имена_страниц::Составные_длинные_стр => {
                write!(f, "Составные длинные")
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
            Имена_страниц::Запятые_стр => {
                write!(f, "Запятые")
            }
            Имена_страниц::Запятые_длинные_стр => {
                write!(f, "Запятые_длинные")
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
impl fmt::Display for Кодировка {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Кодировка::Windows_1251 => write!(f, "Windows_1251"),
            Кодировка::Utf8 => write!(f, "Utf8"),
            Кодировка::Windows_1252 => {
                write!(f, "Windows_1252")
            }
            Кодировка::Не_определено => {
                write!(f, "Не_определено")
            }
        }
    }
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
    pub путь: Умная_Строка,       //путь до книги
    pub имя: Умная_Строка,        //имя книги
    pub разрешение: Умная_Строка, //формат
    //
    pub перевести: Vec<Ячейка_словаря>,           //одиночные слова
    pub простое: Vec<Ячейка_словаря>,             //одиночные слова
    pub составное: Vec<Ячейка_словаря>,           //сложные и составные (в 3 очередь)
    pub составное_важное: Vec<Ячейка_словаря>,    //сложные и составные (в 2 очередь)
    pub составное_длинное: Vec<Ячейка_словаря>,   //сложные и составные (в 1 очередь)
    pub вездесущее: Vec<Ячейка_словаря>,          //сложные и составные
    pub неизменное: Vec<Ячейка_словаря>,          //
    pub огласовки: Vec<Ячейка_словаря>,           //
    pub неизменное_короткое: Vec<Ячейка_словаря>, //
    pub неизменное_длинное: Vec<Ячейка_словаря>,  //
    pub запятые: Vec<Ячейка_словаря>,             //
    pub запятые_длннные: Vec<Ячейка_словаря>,     //
}

//словарь переносов
//словарь
#[derive(Debug, Clone)]
pub struct Ячейка_словаря {
    pub искомое_слово: Умная_Строка,
    pub re_образец: Regex,
    pub замена: Умная_Строка,
    // pub счёчтки:usize,
}
#[derive(Debug, Clone)]
pub struct Слово_с_заменой {
    pub искомое_слово: Умная_Строка,
    pub замена: Умная_Строка,
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
            искомое_слово: Умная_Строка::Пусто,
            re_образец: Regex::new(r"").unwrap(),
            замена: Умная_Строка::Пусто,
            //  счёчтки: 0,
        }
    }
}
// Ручная реализация PartialEq
// Ручная реализация PartialEq
impl PartialEq for Слово_с_заменой {
    fn eq(&self, other: &Self) -> bool {
        // Сравните все поля, которые должны определять уникальность
        self.искомое_слово == other.искомое_слово && self.замена == other.замена
    }
}

// Потом пустая реализация Eq (маркерный трейт)
impl Eq for Слово_с_заменой {} // 👈 ВОТ ТАК ПРАВИЛЬНО!

// Ручная реализация Hash
impl Hash for Слово_с_заменой {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.искомое_слово.hash(state);
        self.замена.hash(state);
    }
}

impl Default for Слово_с_заменой {
    fn default() -> Self {
        Self {
            искомое_слово: Умная_Строка::Пусто,

            замена: Умная_Строка::Пусто,
            //  счёчтки: 0,
        }
    }
}
pub static КОЛИЧЕСТВО_ПРОХОДОВ_СЛОВАРЯ: usize = 11;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_ОДНОБУКВЕННЫЕ: usize = 7;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_ДВУБУКВЕННЫЕ: usize = 77;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_ТРЕХБУКВЕННЫЕ: usize = 147;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_МНОГОБУКВЕННЫЕ: usize = 108;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_ЦЕЛИКОВЫЕ: usize = 299;
pub static СЛОВАРЬ_ПЕРЕНОСОВ_ИСКЛЮЧЕНИЯ: usize = 18;
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
    Перевести,
    Простые,
    Составные,
    Составные_важные,
    Составные_длинные,
    Огласовки,
    Неизменные,
    Неизменные_короткие,
    Неизменные_длинные,
    Запятые,
    Запятые_длинные,
    Вездесущие,
    Не_является_разделом,
}

impl fmt::Display for Раздел_Словаря {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Раздел_Словаря::Перевести => write!(f, "Перевести"),
            Раздел_Словаря::Простые => write!(f, "Простые"),
            Раздел_Словаря::Составные => write!(f, "Составные"),
            Раздел_Словаря::Составные_важные => {
                write!(f, "Составные важные")
            }
            Раздел_Словаря::Составные_длинные => {
                write!(f, "Составные длинные")
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
            Раздел_Словаря::Запятые_длинные => {
                write!(f, "Запятые длинные")
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
pub const РАЗМЕР_РАЗДЕЛИТЕЛЕЙ: usize = 352;

//
//
//use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

use crate::Вид_Архивной_Книги::{Epub, Fb3};
use crate::Вид_одичноной_книги::Fb2;
use crate::Вид_простой_письменности::Txt;
#[derive(Clone, Debug)] //Serialize, Deserialize,
//#[serde(default)] // Добавляем это для всей структуры
pub struct Словарь_разделителей {
    //pub содержимое: [Ячейка_замены_с_разделителями; РАЗМЕР_РАЗДЕЛИТЕЛЕЙ], //одиночные слова
    pub содержимое: Vec<Ячейка_замены_с_разделителями>, //одиночные слова
                                                        //
}
// Реализация Index для доступа по индексу
impl Index<usize> for Словарь_разделителей {
    type Output = Ячейка_замены_с_разделителями;

    fn index(&self, индекс: usize) -> &Self::Output {
        &self.содержимое[индекс]
    }
}
impl Словарь_разделителей {
    // Получить длину словаря
    pub fn len(&self) -> usize {
        self.содержимое.len()
    }

    // Проверить, пуст ли словарь (всегда false, т.к. размер фиксирован)
    pub fn is_empty(&self) -> bool {
        false
    }

    // Получить ссылку на ячейку с проверкой границ
    pub fn get(
        &self,
        индекс: usize,
    ) -> Option<&Ячейка_замены_с_разделителями> {
        self.содержимое.get(индекс)
    }

    // Итератор по ячейкам
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, Ячейка_замены_с_разделителями> {
        self.содержимое.iter()
    }
}

// Реализация IndexMut для изменения ячеек
impl IndexMut<usize> for Словарь_разделителей {
    fn index_mut(&mut self, индекс: usize) -> &mut Self::Output {
        &mut self.содержимое[индекс]
    }
}

impl Default for Словарь_разделителей {
    fn default() -> Self {
        Self {
            //содержимое: std::array::from_fn(|_| Default::default()),
            содержимое: Vec::default(),
        }
    }
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
pub struct Ячейка_замены_переносов_Epub {
    pub образцы_конца: Vec<Ячейка_замены_Epub>,
    pub образцы_начала: Vec<Ячейка_замены_Epub>,
    // pub счёчтки:usize,
}
/*#[derive(Debug, Clone)]
pub struct Ячейка_замены_Epub_начала {
    pub искомое_слово: Умная_Строка,
    pub re_образец: Regex,
    // pub счёчтки:usize,
}
impl Default for Ячейка_замены_Epub_начала {
    fn default() -> Self {
        Self {
            искомое_слово: Умная_Строка::default(),
            re_образец: Regex::new(r"(?i)").unwrap(),
        }
    }
}*/
#[derive(Debug, Clone)]
pub struct Ячейка_замены_Epub {
    pub искомое_слово: Умная_Строка,
    pub re_образец_поиска: Regex,
    pub re_образец_замены: Regex,
    // pub счёчтки:usize,
}
impl Default for Ячейка_замены_Epub {
    fn default() -> Self {
        Self {
            искомое_слово: Умная_Строка::default(),
            re_образец_поиска: Regex::new(r"(?i)").unwrap(),
            re_образец_замены: Regex::new(r"(?i)").unwrap(),
        }
    }
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
//замена объявления
#[derive(Debug, Clone, Default)]
pub struct Ячейка_замены_примечания {
    pub начало: &'static str,
    pub замена: &'static str,
    pub исключение: Vec<&'static str>,
    // pub счёчтки:usize,
}
//словарь
#[derive(Debug, Clone)]
pub struct Ячейка_замены {
    pub искомое_слово: Умная_Строка,
    pub re_образец: Regex,
    pub замена: Умная_Строка,
    // pub счёчтки:usize,
}
impl Default for Ячейка_замены {
    fn default() -> Self {
        Self {
            искомое_слово: Умная_Строка::default(),
            re_образец: Regex::new(r"(?i)").unwrap(),
            замена: Умная_Строка::default(),
        }
    }
}
//словарь
#[derive(Clone, Debug)]
//#[serde(default)] // Добавляем это для всей структуры
pub struct Образцы_Исключений_Разделителей<'a> {
    pub ряд_re_пропуски: &'a Vec<Regex>,
    pub ряд_re_обязательства: &'a Vec<Regex>,
}
#[derive(Clone, Debug)]
pub enum Вид_Слов_Разделителей {
    Пропуски,
    Обязательства,
}
impl fmt::Display for Вид_Слов_Разделителей {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Вид_Слов_Разделителей::Пропуски => write!(f, "Пропуски"),
            Вид_Слов_Разделителей::Обязательства => {
                write!(f, "Обязательства")
            }
        }
    }
}
//словарь
#[derive(Clone, Debug)]
//#[serde(default)] // Добавляем это для всей структуры
pub struct Ячейка_замены_с_разделителями {
    pub искомое_слово: Умная_Строка,
    // #[serde(skip)]
    pub re_образец_для_поиска: Regex,
    pub замена: Умная_Строка,
    //
    pub ряд_пропусков: Vec<Умная_Строка>,
    pub ряд_re_пропуски: Vec<Regex>,
    //
    pub ряд_обязательств: Vec<Умная_Строка>,
    pub ряд_re_обязательства: Vec<Regex>,
    // pub счёчтки:usize,
    //   #[serde(skip)]
    pub re_образец_для_замены: Regex,
}
static ПУСТОЙ_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)").unwrap());
impl Default for Ячейка_замены_с_разделителями {
    fn default() -> Self {
        Self {
            искомое_слово: Умная_Строка::default(),

            re_образец_для_поиска: ПУСТОЙ_REGEX.clone(),
            //
            замена: Умная_Строка::default(),
            re_образец_для_замены: ПУСТОЙ_REGEX.clone(),
            //
            ряд_пропусков: Vec::default(),
            ряд_re_пропуски: Vec::default(), //Regex::new(r"(?i)").unwrap(),
            //
            ряд_обязательств: Vec::default(),
            ряд_re_обязательства: Vec::default(),
        }
    }
}
//
//
pub const КОЛИЧЕСТВО_БУКВ_ПОСЛЕ_РАЗДЕЛИТЕЛЯ: usize = 3;
pub trait Возможности_ячейки_замены_с_разделителями {
    fn добавить_re_пропуски_изнутри(&self) -> Vec<Regex>;
    fn добавить_re_обязательства_изнутри(&self) -> Vec<Regex>;
    fn добавить_оставшиеся_поля(&mut self);
}
impl Возможности_ячейки_замены_с_разделителями
    for Ячейка_замены_с_разделителями
{
    fn добавить_re_пропуски_изнутри(&self) -> Vec<Regex> {
        self.ряд_пропусков
            .iter()
            .map(|ячейка| {
                //let исключение: Regex = LazyLock::new(|| Regex::new(исключение).unwrap();
                let пропуск: String = format!(r#"(\b{{start}}{})"#, ячейка);
                Regex::new(&пропуск).unwrap()
            })
            .collect()
    }
    fn добавить_re_обязательства_изнутри(&self) -> Vec<Regex> {
        self.ряд_обязательств
            .iter()
            .map(|ячейка| {
                //let исключение: Regex = LazyLock::new(|| Regex::new(исключение).unwrap();
                let обязательство: String = format!(r#"(\b{{start}}{})"#, ячейка);
                Regex::new(&обязательство).unwrap()
            })
            .collect()
    }
    //
    fn добавить_оставшиеся_поля(&mut self) {
        self.замена = Умная_Строка::создать_значение(
            format!("{}-", self.искомое_слово),
        );
        self.re_образец_для_поиска =
            Regex::new(&format!(r#"\b{{start}}{}\w"#, self.искомое_слово)).unwrap();
        self.re_образец_для_замены = {
            Regex::new(&format!(
                r#"\b{{start}}({})([\w]{{{КОЛИЧЕСТВО_БУКВ_ПОСЛЕ_РАЗДЕЛИТЕЛЯ},}})"#,
                self.искомое_слово
            ))
            .unwrap()
        };
    }
}
//словарь
#[derive(Debug, Clone)]
pub struct Ячейка_замены_с_исключением {
    pub искомое_слово: Умная_Строка,
    pub re_исключение: Vec<Regex>,
    pub re_образец_для_поиска: Regex,
    pub замена: Умная_Строка,
    // pub счёчтки:usize,
}
impl Default for Ячейка_замены_с_исключением {
    fn default() -> Self {
        Self {
            искомое_слово: Умная_Строка::Пусто,
            re_исключение: Vec::new(), //Regex::new(r"(?i)").unwrap(),
            re_образец_для_поиска: Regex::new(r"(?i)").unwrap(),
            замена: Умная_Строка::Пусто,
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
pub struct Куча_Словарь_полный {
    pub простое: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub составное: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub запятые: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub запятые_длинные:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub составное_важное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub составное_длинное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub вездесущее: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub неизменное: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub огласовки: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub неизменное_короткое:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    pub неизменное_длинное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Куча_Словарь_простой {
    pub простое: rapidhash::fast::RapidHashSet<String>,
    pub составное: rapidhash::fast::RapidHashSet<String>,
    pub запятые: rapidhash::fast::RapidHashSet<String>,
    pub запятые_длинные: rapidhash::fast::RapidHashSet<String>,
    pub составное_важное: rapidhash::fast::RapidHashSet<String>,
    pub составное_длинное: rapidhash::fast::RapidHashSet<String>,
    pub вездесущее: rapidhash::fast::RapidHashSet<String>,
    pub неизменное: rapidhash::fast::RapidHashSet<String>,
    pub огласовки: rapidhash::fast::RapidHashSet<String>,
    pub неизменное_короткое: rapidhash::fast::RapidHashSet<String>,
    pub неизменное_длинное: rapidhash::fast::RapidHashSet<String>,
}
#[derive(Debug, Default, Clone)]
pub struct Куча_Словарь_Искомые_полные {
    pub перевести: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub простое: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub составное: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub составное_важное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub составное_длинное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub вездесущее: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub запятые: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub запятые_длинные:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub неизменное: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub огласовки: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub неизменное_длинное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub неизменное_короткое:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
}
#[derive(Debug, Default, Clone)]
pub struct Куча_Словарь_Искомые_простые {
    pub перевести: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub простое: rapidhash::fast::RapidHashSet<String>,   //одиночные слова
    pub составное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub составное_важное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub составное_длинное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub вездесущее: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub запятые: rapidhash::fast::RapidHashSet<String>,   //одиночные слова
    pub запятые_длинные: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub огласовки: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное_длинное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное_короткое: rapidhash::fast::RapidHashSet<String>, //одиночные слова
}
#[derive(Debug, Default, Clone)]
pub struct Куча_Словарь_Замены_полные {
    pub перевести: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub простое: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub запятые: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub запятые_длинные:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub составное: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub составное_длинное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub составное_важное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub вездесущее: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub неизменное: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub огласовки: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub неизменное_длинное:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
    pub неизменное_короткое:
        rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>, //одиночные слова
}
#[derive(Debug, Default, Clone)]
pub struct Куча_Словарь_Замены_простые {
    pub перевести: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub простое: rapidhash::fast::RapidHashSet<String>,   //одиночные слова
    pub запятые: rapidhash::fast::RapidHashSet<String>,   //одиночные слова
    pub запятые_длинные: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub составное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub составное_длинное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub составное_важное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub вездесущее: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub огласовки: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное_длинное: rapidhash::fast::RapidHashSet<String>, //одиночные слова
    pub неизменное_короткое: rapidhash::fast::RapidHashSet<String>, //одиночные слова
}
//
pub struct Две_Кучи_Словаря<'a> {
    pub искомая: &'a rapidhash::fast::RapidHashSet<String>,
    pub куда_вкладывать:
        &'a mut rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
}

pub struct Куча_Словаря_с_описанием<'a> {
    pub сама: &'a rapidhash::fast::RapidHashSet<String>,
    pub вид: Правописание_слова,
}
pub struct Раздел_Словаря_с_Привязкой<'a> {
    pub главный: &'a Vec<Ячейка_словаря>,
    pub куча: [Куча_Словаря_с_описанием<'a>; 3],
    /*pub куча_строчная: &'a rapidhash::fast::RapidHashSet<String>, //&'a Две_Кучи_Словаря<'a>,
    pub куча_заглавная_первая: &'a rapidhash::fast::RapidHashSet<String>, //&'a Две_Кучи_Словаря<'a>,
    pub куча_все_заглавные: &'a rapidhash::fast::RapidHashSet<String>, //&'a Две_Кучи_Словаря<'a>,*/
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Подробный_вид_расширения {
    pub строка: Умная_Строка,
    pub вид: Основной_Вид_Расширения,
}
#[derive(Debug, Default, Clone)]
pub struct Подробный_вид_расширения_с_кучей {
    pub строка: Умная_Строка,
    pub вид: Основной_Вид_Расширения,
    pub куча: rapidhash::fast::RapidHashSet<String>,
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Слово_и_знаки {
    pub слово: Умная_Строка,
    pub знаки: Vec<char>,
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Два_Слова {
    pub все_строчные: Умная_Строка,
    pub с_заглавной: Умная_Строка,
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Три_Слова {
    pub все_строчные: Умная_Строка,
    pub с_заглавной: Умная_Строка,
    pub все_заглавные: Умная_Строка,
}
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Полный_Словарь {
    pub перевести: Vec<Ячейка_словаря>, //одиночные слова
    //одиночные
    pub простое: Vec<Ячейка_словаря>, //одиночные слова
    //сложные
    pub составное: Vec<Ячейка_словаря>, //сложные и составные
    pub составное_длинное: Vec<Ячейка_словаря>, //сложные и составные
    pub запятые: Vec<Ячейка_словаря>,   //сложные и составные
    pub запятые_длинные: Vec<Ячейка_словаря>, //сложные и составные
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
pub struct Словарь_Куч_полных {
    pub искомые:
        [Куча_Словарь_Искомые_полные; КОЛИЧЕСТВО_УРОВНЕЙ_СЛОВАРЯ_КУЧ],
    pub замены:
        [Куча_Словарь_Замены_полные; КОЛИЧЕСТВО_УРОВНЕЙ_СЛОВАРЯ_КУЧ],
}
//
/*#[derive(Debug, Clone, PartialEq)]
pub enum Вид_ошибки_при_поиске_в_торжке {
    Исключение,
    Ошибка(String),
}*/
//
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ошибка_несоотвествия_части_речи_Торжок {
    pub выдер_для_замены_изначальный: RE_полное_окончание,
    pub выдер_для_неруси_исходный: RE_полное_окончание,
    pub ячейка_торжка_купленная: Вид_Окончания_со_строкой,
    pub часть_речи_торжка: &'static [Вид_части_речи],
    //
    pub нерусь_слово_исходное: String,
    pub слово_замена_исходное: String,
    pub окончание_нерусь_итоговое: String,
}
impl std::fmt::Display
    for Ошибка_несоотвествия_части_речи_Торжок
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            style(format!("Не соответствие частей речи (Торжок) - ")).yellow(),
            //Ошибка_несоотвествия_части_речи,
            format!("Выдер изначальный строка  ",),
            style(format!("|{}| ", self.выдер_для_замены_изначальный.строка,)).blue(),
            format!(" вид части речи ",),
            style(format!(
                " |{}|",
                self.выдер_для_замены_изначальный
                    .части_речи
                    .вывести_в_строку()
            ))
            .blue(),
            format!("  Ячейка торжка  строка ",),
            style(format!("  |{}|", self.ячейка_торжка_купленная.строка,)).cyan(),
            format!(" часть речи ",),
            style(format!("|{}|", self.часть_речи_торжка.вывести_в_строку())).cyan(),
            //style(где_найдено).magenta(),
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ошибка_несоотвествия_части_речи_RE_окончание {
    pub выдер_изначальный: RE_полное_окончание,
    pub купленный_на_торжке: RE_полное_окончание,
    //
    pub нерусь_слово_исходное: String,
    pub слово_замена_исходное: String,
    pub окончание_нерусь_итоговое: String,
}
impl std::fmt::Display
    for Ошибка_несоотвествия_части_речи_RE_окончание
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            style(format!("Не соответствие частей речи (до Торжка) - ")).yellow(),
            //Ошибка_несоотвествия_части_речи,
            format!("Выдер изначальный строка  ",),
            style(format!("|{}| ", self.выдер_изначальный.строка,)).blue(),
            format!(" вид части речи ",),
            style(format!(
                " |{}|",
                self.выдер_изначальный.части_речи.вывести_в_строку()
            ))
            .blue(),
            format!("  Ячейка торжка  строка ",),
            style(format!("  |{}|", self.купленный_на_торжке.строка,)).cyan(),
            format!(" часть речи ",),
            style(format!(
                "|{}|",
                self.купленный_на_торжке.части_речи.вывести_в_строку()
            ))
            .cyan(),
            //style(где_найдено).magenta(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Вид_ошибки_при_поиске_выдера {
    До_Торжка(Вид_ошибки_при_поиске_выдера_до_торжка),
    В_Торжке(Вид_ошибки_при_поиске_выдера_в_торжке),
    После_Торжка(Вид_ошибки_при_поиске_выдера_после_торжка),
    Не_определено,
}
impl std::fmt::Display for Вид_ошибки_при_поиске_выдера {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::До_Торжка(ошибка) => {
                write!(
                    f,
                    "{}{}{}{}",
                    style(format!("Ошибка ")).yellow(),
                    style(format!("при поиске выдера ",)).red(),
                    style(format!("До Торжка - ")),
                    style(format!("{}", ошибка)),
                )
            }
            Self::В_Торжке(ошибка) => {
                // write!(f, "Ошибка - при поиске выдера - В Торжке: {}", ошибка)
                write!(
                    f,
                    "{}{}{}{}",
                    style(format!("Ошибка ")).yellow(),
                    style(format!("при поиске выдера ",)).red(),
                    style(format!("В Торжке - ")),
                    style(format!("{}", ошибка)),
                )
            }
            Self::После_Торжка(ошибка) => {
                // write!(f, "Ошибка - при поиске выдера - В Торжке: {}", ошибка)
                write!(
                    f,
                    "{}{}{}{}",
                    style(format!("Ошибка ")).yellow(),
                    style(format!("при поиске выдера ",)).red(),
                    style(format!("После Торжка - ")),
                    style(format!("{}", ошибка)),
                )
            }
            Self::Не_определено => {
                //f.write_str("Ошибка - при поиске выдера - Не определено")
                write!(
                    f,
                    "{}{}{}",
                    style(format!("Ошибка ")).yellow(),
                    style(format!("при поиске выдера ",)).red(),
                    style(format!("Не определено ")),
                    // style(format!("{}", ошибка)),
                )
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ошибка_не_подошедшего_образца_RE {
    pub слово_замена_из_ряда: String,
    pub окончание_для_выдера_в_замене: Вид_Окончания_со_строкой,
    pub образец_re_неруси_первичного_слова: RE_полное_окончание,
    pub слово_замена_исходное: String,
    pub неруское_слово_исходное: String,
}
impl fmt::Display for Ошибка_не_подошедшего_образца_RE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|{}| {} |{}| {} {} {} {} {} {} {} {} {} {}",
            //style(format!("Ошибка93:")).yellow(),
            // style(format!("Нет другого окончания в разделе Торжка ")),
            style(format!("{}", self.неруское_слово_исходное)).blue(),
            style(format!("слово-замена")),
            style(format!("|{}|", self.слово_замена_исходное)).cyan(),
            style(format!("образец_окончания_неруского_слова")).bold(),
            style(format!(
                "|{}|",
                self.образец_re_неруси_первичного_слова.строка
            ))
            .blue(),
            style(format!("Не удалось выдрать re")).bold(),
            style(format!(
                "|{}|",
                self.окончание_для_выдера_в_замене.re_выдер.as_str(),
            ))
            .true_color(127, 255, 212),
            style(format!("строка")).bold(),
            style(format!("|{}|", self.окончание_для_выдера_в_замене.строка,))
                .true_color(127, 255, 212),
            style(format!("из")).bold(),
            style(format!("|{}|", self.слово_замена_из_ряда)).true_color(127, 255, 212),
            style(format!(" окончание строка ")),
            style(format!(
                "|{}|",
                self.образец_re_неруси_первичного_слова.строка
            ))
            .red(),
        )
    }
}
//
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Вид_ошибки_при_поиске_выдера_до_торжка {
    Не_соответствие_частей_речи(
        Ошибка_несоотвествия_части_речи_RE_окончание,
    ),
    Не_найден_образец_re_для_слова(String),
    Не_определено,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Вид_ошибки_при_поиске_выдера_после_торжка {
    /*Не_соответствие_частей_речи(
        Ошибка_несоотвествия_части_речи_RE_окончание,
    ),*/
    // Не_найден_образец_re_для_слова(String),
    Не_подошёл_другой_re_образец(Ошибка_не_подошедшего_образца_RE),
    Не_подошёл_re_образец_при_этом_нет_другого_образца(
        Ошибка_не_подошедшего_образца_RE,
    ),
    Не_определено,
}
#[derive(Debug, Clone, Default)]
pub struct Ошибки_при_поиске_выдера_после_торжка {
    pub не_подошёл_другой_re_образец: Ряд_Замен_не_подошедщий_образец_re,
    pub не_подошёл_re_образец_при_этом_нет_другого_образца:
        Ряд_Замен_не_подошедщий_образец_re,
}

impl fmt::Display for Вид_ошибки_при_поиске_выдера_после_торжка {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Не_подошёл_re_образец_при_этом_нет_другого_образца(ошибка) => {
                write!(
                    f,
                    "{} {}",
                    style(format!("Нет других образцов RE в Торжке:")).yellow(),
                    ошибка
                )
            }
            Self::Не_подошёл_другой_re_образец(ошибка) => {
                write!(
                    f,
                    "{} {}",
                    style(format!("Не подошёл другой RE-образец из раздела Торжка:")).yellow(),
                    ошибка
                )
            }
            Self::Не_определено => write!(f, "Ошибка не определена"),
        }
    }
}

impl std::fmt::Display for Вид_ошибки_при_поиске_выдера_до_торжка {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Не_соответствие_частей_речи(ошибка) => {
                write!(f, "{}", ошибка)
                // Оператор '?' автоматически вернет ошибку, если запись не удалась
                /*writeln!(f, "Не соответствие частей речи:")?;
                writeln!(
                    f,
                    "  выдер_изначальный |{}| его часть речи |{}|",
                    ошибка.выдер_изначальный.окончание_строка,
                    ошибка.выдер_изначальный.части_речи.вывести_в_строку()
                )?;
                writeln!(
                    f,
                    "  купленный_на_торжке |{}| его часть речи |{}|",
                    ошибка.купленный_на_торжке.окончание_строка,
                    ошибка.купленный_на_торжке.части_речи.вывести_в_строку()
                ) // Последнее выражение возвращает fmt::Result, точка с запятой не нужна
                */
            }
            //
            Self::Не_найден_образец_re_для_слова(слово) => {
                write!(
                    f,
                    "{}{}",
                    style(format!("Не найден образец re для слова ")),
                    //Ошибка_несоотвествия_части_речи,
                    style(format!("{}", слово)).green(),
                )
            }
            Self::Не_определено => f.write_str("Не определено"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Замена_с_ошибкой_возврат {
    pub строка: String,
    pub ряд_ошибок: Vec<Вид_ошибки_при_поиске_выдера>,
}
#[derive(Debug, Clone, Default)]
pub struct Ряд_Замен_не_подошедщий_образец_re {
    pub ряд: Vec<Ошибка_не_подошедшего_образца_RE>,
    pub куча: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<String>>,
}
impl Ряд_Замен_не_подошедщий_образец_re {
    pub fn упорядочить(&mut self) -> Result<(), String> {
        //упорядочивание стопки по напряжению >> refdes >> имя цепи
        self.ряд.sort_by(|l, r| {
            l.образец_re_неруси_первичного_слова
                .строка
                .cmp(r.образец_re_неруси_первичного_слова.строка)
                .then(
                    l.окончание_для_выдера_в_замене
                        .строка
                        .cmp(&r.окончание_для_выдера_в_замене.строка),
                )
        });
        //
        //   //
        self.куча = rapidhash::fast::RapidHashMap::default();
        // self.цепи_земли.упорядочить();
        for (_указатель_ячейки, ячейка) in self.ряд.iter().enumerate() {
            //если вложено имя вывода
            if self.содержит_ошибку(&ячейка) {
                return Err(format!(
                    "Ячейка ряда: образец_re_неруси_первичного_слова.строка |{}| окончание_для_выдера_в_замене строка |{}| уже содержится в куче",
                    ячейка.образец_re_неруси_первичного_слова.строка,
                    ячейка.окончание_для_выдера_в_замене.строка,
                    //ячейка.ячейка_торжка.часть_речи.вывести_в_строку(),
                ));
            }
            //вложить вывод sigrity уже если есть стопка
            else {
                self.куча
                    .entry(ячейка.образец_re_неруси_первичного_слова.строка.to_string())
                    .or_insert(rapidhash::fast::RapidHashSet::default())
                    .insert(ячейка.окончание_для_выдера_в_замене.строка.to_string());
            }
            // если не вложено совсем
            /*else {
                self.куча
                    .entry(ячейка.имя_вывода.получить_значение())
                    .or_insert(rapidhash::fast::RapidHashSet::default())
                    .insert(ячейка.имя_вывода_sigrity.получить_значение());
            }*/
            //
        }
        return Ok(());
    }
    //
    pub fn содержит_ошибку(
        &self,
        ошибка_вкладываемая: &Ошибка_не_подошедшего_образца_RE,
    ) -> bool {
        //
        if self.куча.contains_key(
            ошибка_вкладываемая
                .образец_re_неруси_первичного_слова
                .строка,
        ) {
            //содержимое ячейки кучи в виде ключ-значение
            let содержимое_ячейки_кучи = self
                .куча
                .get_key_value(
                    ошибка_вкладываемая
                        .образец_re_неруси_первичного_слова
                        .строка,
                )
                .unwrap();
            //
            if содержимое_ячейки_кучи
                .1
                .contains(ошибка_вкладываемая.окончание_для_выдера_в_замене.строка)
            {
                return true;
            }
            //вложить вывод sigrity уже если есть стопка
            else {
                return false;
            }
        }
        return false;
        //
        /*self.куча
        .contains_key(&вывод.имя_вывода_sigrity.получить_значение())*/
    }
    pub fn len(&self) -> usize {
        // Возвращает длину вектора
        self.ряд.len()
    }
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, Ошибка_не_подошедшего_образца_RE> {
        self.ряд.iter()
    }
    //
    pub fn iter_mut(
        &mut self,
    ) -> std::slice::IterMut<'_, Ошибка_не_подошедшего_образца_RE> {
        self.ряд.iter_mut()
    }

    //
    pub fn вложить(
        &mut self,
        ошибка_вкладываемая: Ошибка_не_подошедшего_образца_RE,
    ) -> Result<(), String> {
        //
        //если не содержит потребителя - то вложить
        if !self.содержит_ошибку(&ошибка_вкладываемая) {
            //
            //let имя_вывода_изначальное = вложение.имя_вывода_sigrity.clone();

            self.ряд.push(ошибка_вкладываемая.clone());
            //
            if self.ряд.len() == 0 {
                return Err(format!("Пустой ряд"));
            }
            //
            if self.ряд[self.ряд.len() - 1] == ошибка_вкладываемая {
                self.куча
                    //.insert(имя_источника.получить_значение(), self.ряд.len() - 1);
                    .entry(
                        ошибка_вкладываемая
                            .образец_re_неруси_первичного_слова
                            .строка
                            .to_string(),
                    )
                    .or_insert(rapidhash::fast::RapidHashSet::default())
                    .insert(
                        ошибка_вкладываемая
                            .окончание_для_выдера_в_замене
                            .строка
                            .to_string(),
                    );
                return Ok(());
            } else {
                return Err(format!("Ошибка0"));
            }
        //
        }
        //неопределённое поведение - если потребитель уже содержится в куче
        else {
            let mut ряд_сообщений = vec![format!("\r\nПоиск ошибок уже вложенных")];
            let mut счётчик_совпадений: usize = 0;
            //println!("\r\nПоиск потребителей уже вложенных");
            //
            //for ячейка in self.куча.iter() {
            //
            for ячейка_ряда in self.ряд.iter() {
                // если refdes сходится
                if ячейка_ряда.образец_re_неруси_первичного_слова
                    == ошибка_вкладываемая.образец_re_неруси_первичного_слова
                {
                    //перебор refdes true
                    if ячейка_ряда.окончание_для_выдера_в_замене
                        == ошибка_вкладываемая.окончание_для_выдера_в_замене
                    //
                    {
                        /*if ячейка_ряда.часть_речи_торжка == ошибка_вкладываемая.часть_речи_торжка
                        {*/
                        println!(
                            "Ячейка ряда: выдер_изначальный.строка |{}| ячейка_торжка строка |{}| ",
                            ячейка_ряда.образец_re_неруси_первичного_слова.строка,
                            ячейка_ряда.окончание_для_выдера_в_замене.строка,
                            // ячейка_ряда.часть_речи_торжка.вывести_в_строку(),
                        );
                        //return;
                        счётчик_совпадений += 1;
                    } else {
                        ряд_сообщений.push(format!(
                            "Ячейка ряда: выдер_изначальный.строка |{}| ячейка_торжка строка |{}| ",
                            ячейка_ряда.образец_re_неруси_первичного_слова.строка,
                            ячейка_ряда.окончание_для_выдера_в_замене.строка,
                            // ячейка_ряда.часть_речи_торжка.вывести_в_строку(),
                            // ошибка_вкладываемая.часть_речи_торжка.вывести_в_строку(),
                        ));
                        //}
                    }
                }
            }
            // }
            //
            // Если не нашёлся по Refdex
            //
            match счётчик_совпадений {
                0 => {
                    //
                    for строка in ряд_сообщений.iter() {
                        println!("{}", строка);
                    }
                    //
                    return Err(format!(
                        "Ошибка4 неопределённое поведение \r\n |сам вывод |{:?}|",
                        ошибка_вкладываемая
                    ));
                }
                _ => {
                    return Err(format!(
                        "Ошибка5 потребитель уже вложен \r\n |сам вывод |{:?}|",
                        ошибка_вкладываемая
                    ));
                }
            }
        }

        //
    }
}
//
//
impl Index<usize> for Ряд_Замен_не_подошедщий_образец_re {
    type Output = Ошибка_не_подошедшего_образца_RE;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ряд[index]
    }
}
//
impl IndexMut<usize> for Ряд_Замен_не_подошедщий_образец_re {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ряд[index]
    }
}
#[derive(Debug, Clone, Default)]
pub struct Ряд_ошибок_Иключений_торжка {
    pub ряд: Vec<Ошибка_в_торжке_исключение>,
    pub куча: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<String>>,
}
impl Ряд_ошибок_Иключений_торжка {
    pub fn упорядочить(&mut self) -> Result<(), String> {
        //упорядочивание стопки по напряжению >> refdes >> имя цепи
        self.ряд.sort_by(|l, r| {
            l.образец_окончания_неруского_слова
                .cmp(&r.образец_окончания_неруского_слова)
                .then(
                    l.вид_окончания
                        .to_string()
                        .cmp(&r.вид_окончания.to_string()),
                )
        });
        //
        //   //
        self.куча = rapidhash::fast::RapidHashMap::default();
        // self.цепи_земли.упорядочить();
        for (_указатель_ячейки, ячейка) in self.ряд.iter().enumerate() {
            //если вложено имя вывода
            if self.содержит_ошибку(&ячейка) {
                return Err(format!(
                    "Ячейка ряда: образец_re_неруси_первичного_слова.строка |{}| окончание_для_выдера_в_замене строка |{}| уже содержится в куче |{}|",
                    ячейка.образец_окончания_неруского_слова,
                    ячейка.вид_окончания,
                    ячейка.изначальный_выдер_неруси,
                    //ячейка.ячейка_торжка.часть_речи.вывести_в_строку(),
                ));
            }
            //вложить вывод sigrity уже если есть стопка
            else {
                self.куча
                    .entry(ячейка.образец_окончания_неруского_слова.to_string())
                    .or_insert(rapidhash::fast::RapidHashSet::default())
                    .insert(ячейка.вид_окончания.to_string());
            }
            // если не вложено совсем
            /*else {
                self.куча
                    .entry(ячейка.имя_вывода.получить_значение())
                    .or_insert(rapidhash::fast::RapidHashSet::default())
                    .insert(ячейка.имя_вывода_sigrity.получить_значение());
            }*/
            //
        }
        return Ok(());
    }
    //
    pub fn содержит_ошибку(
        &self,
        ошибка_вкладываемая: &Ошибка_в_торжке_исключение,
    ) -> bool {
        //
        if self
            .куча
            .contains_key(&ошибка_вкладываемая.образец_окончания_неруского_слова)
        {
            //содержимое ячейки кучи в виде ключ-значение
            let содержимое_ячейки_кучи = self
                .куча
                .get_key_value(&ошибка_вкладываемая.образец_окончания_неруского_слова)
                .unwrap();
            //
            if содержимое_ячейки_кучи
                .1
                .contains(&ошибка_вкладываемая.вид_окончания.to_string())
            {
                return true;
            }
            //вложить вывод sigrity уже если есть стопка
            else {
                return false;
            }
        }
        return false;
        //
        /*self.куча
        .contains_key(&вывод.имя_вывода_sigrity.получить_значение())*/
    }
    pub fn len(&self) -> usize {
        // Возвращает длину вектора
        self.ряд.len()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Ошибка_в_торжке_исключение> {
        self.ряд.iter()
    }
    //
    pub fn iter_mut(
        &mut self,
    ) -> std::slice::IterMut<'_, Ошибка_в_торжке_исключение> {
        self.ряд.iter_mut()
    }

    //
    pub fn вложить(
        &mut self,
        ошибка_вкладываемая: Ошибка_в_торжке_исключение,
    ) -> Result<(), String> {
        //
        //если не содержит потребителя - то вложить
        if !self.содержит_ошибку(&ошибка_вкладываемая) {
            //
            //let имя_вывода_изначальное = вложение.имя_вывода_sigrity.clone();

            self.ряд.push(ошибка_вкладываемая.clone());
            //
            if self.ряд.len() == 0 {
                return Err(format!("Пустой ряд"));
            }
            //
            if self.ряд[self.ряд.len() - 1] == ошибка_вкладываемая {
                self.куча
                    //.insert(имя_источника.получить_значение(), self.ряд.len() - 1);
                    .entry(ошибка_вкладываемая.образец_окончания_неруского_слова)
                    .or_insert(rapidhash::fast::RapidHashSet::default())
                    .insert(ошибка_вкладываемая.вид_окончания.to_string());
                return Ok(());
            } else {
                return Err(format!("Ошибка0"));
            }
        //
        }
        //неопределённое поведение - если потребитель уже содержится в куче
        else {
            let mut ряд_сообщений = vec![format!("\r\nПоиск ошибок уже вложенных")];
            let mut счётчик_совпадений: usize = 0;
            //println!("\r\nПоиск потребителей уже вложенных");
            //
            //for ячейка in self.куча.iter() {
            //
            for ячейка_ряда in self.ряд.iter() {
                // если refdes сходится
                if ячейка_ряда.образец_окончания_неруского_слова
                    == ошибка_вкладываемая.образец_окончания_неруского_слова
                {
                    //перебор refdes true
                    if ячейка_ряда.вид_окончания == ошибка_вкладываемая.вид_окончания
                    //
                    {
                        /*if ячейка_ряда.часть_речи_торжка == ошибка_вкладываемая.часть_речи_торжка
                        {*/
                        println!(
                            "Ячейка ряда: выдер_изначальный.строка |{}| ячейка_торжка строка |{}| |{}| ",
                            ячейка_ряда.образец_окончания_неруского_слова,
                            ячейка_ряда.вид_окончания,
                            ячейка_ряда.изначальный_выдер_неруси,
                            // ячейка_ряда.часть_речи_торжка.вывести_в_строку(),
                        );
                        //return;
                        счётчик_совпадений += 1;
                    } else {
                        ряд_сообщений.push(format!(
                            "Ячейка ряда: выдер_изначальный.строка |{}| ячейка_торжка строка |{}| {} ",
                            ячейка_ряда.образец_окончания_неруского_слова,
                            ячейка_ряда.вид_окончания,
                            ячейка_ряда.изначальный_выдер_неруси,
                            // ячейка_ряда.часть_речи_торжка.вывести_в_строку(),
                            // ошибка_вкладываемая.часть_речи_торжка.вывести_в_строку(),
                        ));
                        //}
                    }
                }
            }
            // }
            //
            // Если не нашёлся по Refdex
            //
            match счётчик_совпадений {
                0 => {
                    //
                    for строка in ряд_сообщений.iter() {
                        println!("{}", строка);
                    }
                    //
                    return Err(format!(
                        "Ошибка4 неопределённое поведение \r\n |сам вывод |{:?}|",
                        ошибка_вкладываемая
                    ));
                }
                _ => {
                    return Err(format!(
                        "Ошибка5 потребитель уже вложен \r\n |сам вывод |{:?}|",
                        ошибка_вкладываемая
                    ));
                }
            }
        }

        //
    }
}
//
//
impl Index<usize> for Ряд_ошибок_Иключений_торжка {
    type Output = Ошибка_в_торжке_исключение;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ряд[index]
    }
}
//
impl IndexMut<usize> for Ряд_ошибок_Иключений_торжка {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ряд[index]
    }
}
//
#[derive(Debug, Clone, Default)]
pub struct Ряд_Замен_с_ошибкой_частей_речи {
    pub ряд: Vec<Ошибка_несоотвествия_части_речи_Торжок>,
    pub куча: rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<String>>,
}
impl Ряд_Замен_с_ошибкой_частей_речи {
    pub fn упорядочить(&mut self) -> Result<(), String> {
        //упорядочивание стопки по напряжению >> refdes >> имя цепи
        self.ряд.sort_by(|l, r| {
            l.выдер_для_замены_изначальный
                .строка
                .cmp(r.выдер_для_замены_изначальный.строка)
                .then(
                    l.ячейка_торжка_купленная
                        .строка
                        .cmp(&r.ячейка_торжка_купленная.строка),
                )
        });
        //
        //   //
        self.куча = rapidhash::fast::RapidHashMap::default();
        // self.цепи_земли.упорядочить();
        for (_указатель_ячейки, ячейка) in self.ряд.iter().enumerate() {
            //если вложено имя вывода
            if self.содержит_ошибку(&ячейка) {
                return Err(format!(
                    "Ячейка ряда: выдер_изначальный.строка |{}| ячейка_торжка строка |{}| части речи |{}| уже содержится в куче",
                    ячейка.выдер_для_замены_изначальный.строка,
                    ячейка
                        .выдер_для_замены_изначальный
                        .части_речи
                        .вывести_в_строку(),
                    ячейка.ячейка_торжка_купленная.строка,
                    //ячейка.ячейка_торжка.часть_речи.вывести_в_строку(),
                ));
            }
            //вложить вывод sigrity уже если есть стопка
            else {
                self.куча
                    .entry(ячейка.выдер_для_замены_изначальный.строка.to_string())
                    .or_insert(rapidhash::fast::RapidHashSet::default())
                    .insert(ячейка.ячейка_торжка_купленная.строка.to_string());
            }
            // если не вложено совсем
            /*else {
                self.куча
                    .entry(ячейка.имя_вывода.получить_значение())
                    .or_insert(rapidhash::fast::RapidHashSet::default())
                    .insert(ячейка.имя_вывода_sigrity.получить_значение());
            }*/
            //
        }
        return Ok(());
    }
    //
    pub fn содержит_ошибку(
        &self,
        ошибка_вкладываемая: &Ошибка_несоотвествия_части_речи_Торжок,
    ) -> bool {
        //
        if self
            .куча
            .contains_key(ошибка_вкладываемая.выдер_для_замены_изначальный.строка)
        {
            //содержимое ячейки кучи в виде ключ-значение
            let содержимое_ячейки_кучи = self
                .куча
                .get_key_value(ошибка_вкладываемая.выдер_для_замены_изначальный.строка)
                .unwrap();
            //
            if содержимое_ячейки_кучи
                .1
                .contains(ошибка_вкладываемая.ячейка_торжка_купленная.строка)
            {
                return true;
            }
            //вложить вывод sigrity уже если есть стопка
            else {
                return false;
            }
        }
        return false;
        //
        /*self.куча
        .contains_key(&вывод.имя_вывода_sigrity.получить_значение())*/
    }
    pub fn len(&self) -> usize {
        // Возвращает длину вектора
        self.ряд.len()
    }
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, Ошибка_несоотвествия_части_речи_Торжок> {
        self.ряд.iter()
    }
    //
    pub fn iter_mut(
        &mut self,
    ) -> std::slice::IterMut<'_, Ошибка_несоотвествия_части_речи_Торжок> {
        self.ряд.iter_mut()
    }

    //
    pub fn вложить(
        &mut self,
        ошибка_вкладываемая: Ошибка_несоотвествия_части_речи_Торжок,
    ) -> Result<(), String> {
        //
        //если не содержит потребителя - то вложить
        if !self.содержит_ошибку(&ошибка_вкладываемая) {
            //
            //let имя_вывода_изначальное = вложение.имя_вывода_sigrity.clone();

            self.ряд.push(ошибка_вкладываемая.clone());
            //
            if self.ряд.len() == 0 {
                return Err(format!("Пустой ряд"));
            }
            //
            if self.ряд[self.ряд.len() - 1] == ошибка_вкладываемая {
                self.куча
                    //.insert(имя_источника.получить_значение(), self.ряд.len() - 1);
                    .entry(
                        ошибка_вкладываемая
                            .выдер_для_замены_изначальный
                            .строка
                            .to_string(),
                    )
                    .or_insert(rapidhash::fast::RapidHashSet::default())
                    .insert(
                        ошибка_вкладываемая
                            .ячейка_торжка_купленная
                            .строка
                            .to_string(),
                    );
                return Ok(());
            } else {
                return Err(format!("Ошибка0"));
            }
        //
        }
        //неопределённое поведение - если потребитель уже содержится в куче
        else {
            let mut ряд_сообщений = vec![format!("\r\nПоиск ошибок уже вложенных")];
            let mut счётчик_совпадений: usize = 0;
            //println!("\r\nПоиск потребителей уже вложенных");
            //
            //for ячейка in self.куча.iter() {
            //
            for ячейка_ряда in self.ряд.iter() {
                // если refdes сходится
                if ячейка_ряда.выдер_для_замены_изначальный
                    == ошибка_вкладываемая.выдер_для_замены_изначальный
                {
                    //перебор refdes true
                    if ячейка_ряда.ячейка_торжка_купленная
                        == ошибка_вкладываемая.ячейка_торжка_купленная
                    //
                    {
                        if ячейка_ряда.часть_речи_торжка == ошибка_вкладываемая.часть_речи_торжка
                        {
                            println!(
                                "Ячейка ряда: выдер_изначальный.строка |{}| ячейка_торжка строка |{}| части речи |{}| ",
                                ячейка_ряда.выдер_для_замены_изначальный.строка,
                                ячейка_ряда.ячейка_торжка_купленная.строка,
                                ячейка_ряда.часть_речи_торжка.вывести_в_строку(),
                            );
                            //return;
                            счётчик_совпадений += 1;
                        } else {
                            ряд_сообщений.push(
                        format!(
                            "Ячейка ряда: выдер_изначальный.строка |{}| ячейка_торжка строка |{}| части речи |{}|, а части речи вкладываемой |{}|",
                            ячейка_ряда.выдер_для_замены_изначальный.строка,
                            ячейка_ряда.ячейка_торжка_купленная.строка,
                            ячейка_ряда.часть_речи_торжка.вывести_в_строку(),
                            ошибка_вкладываемая.часть_речи_торжка.вывести_в_строку(),
                        ));
                        }
                    }
                }
            }
            // }
            //
            // Если не нашёлся по Refdex
            //
            match счётчик_совпадений {
                0 => {
                    //
                    for строка in ряд_сообщений.iter() {
                        println!("{}", строка);
                    }
                    //
                    return Err(format!(
                        "Ошибка4 неопределённое поведение \r\n |сам вывод |{:?}|",
                        ошибка_вкладываемая
                    ));
                }
                _ => {
                    return Err(format!(
                        "Ошибка5 потребитель уже вложен \r\n |сам вывод |{:?}|",
                        ошибка_вкладываемая
                    ));
                }
            }
        }

        //
    }
}
//
//
impl Index<usize> for Ряд_Замен_с_ошибкой_частей_речи {
    type Output = Ошибка_несоотвествия_части_речи_Торжок;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ряд[index]
    }
}
//
impl IndexMut<usize> for Ряд_Замен_с_ошибкой_частей_речи {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ряд[index]
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ошибка_в_торжке_исключение {
    pub изначальный_выдер_неруси: String,
    pub неруское_слово_исходное: String,
    pub слово_замена_из_ряда: String,
    pub слово_замена_исходное: String,
    pub вид_окончания: Вид_Окончания,
    pub образец_окончания_неруского_слова: String,
}
//
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Вид_ошибки_при_поиске_выдера_в_торжке {
    //
    Не_найдено_окончание_в_разделе_торжка_но_есть_в_куче(
        String,
    ),
    Не_найден_вид_окончания_в_разделе_торжка_но_есть_в_куче(
        String,
        Вид_Окончания,
    ),
    Не_найден_образец_по_условиям_гласных_и_согласных(
        Вид_Окончания_со_строкой,
        RE_полное_окончание,
    ),
    Исключение(Ошибка_в_торжке_исключение),
    Не_соответствие_частей_речи(Ошибка_несоотвествия_части_речи_Торжок),
    Не_найден_в_кучах(String), //(Вид_Окончания),
    // Не_найден_в_куче_окончаний, //(Вид_Окончания),
    Не_определено,
}
impl std::fmt::Display for Вид_ошибки_при_поиске_выдера_в_торжке {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //use Возможноси_частей_речи;
        match self {
            Вид_ошибки_при_поиске_выдера_в_торжке::Не_найден_образец_по_условиям_гласных_и_согласных(выдер_изначальный_из_замены
                ,образец_в_торжке) => {
                write!(f, "{} |{} {} {} {} {} {} {} {} {} {} {} {}",
                      style(format!("Изначальный выдер не сошёлся в Торжке с окончанием по правилу согласных и гласных")).yellow(),
                        format!("Где Изначальный выдер из замены"),
                    style(format!("|{}|",выдер_изначальный_из_замены.строка)).blue(),
                    format!("двойная гласная"),
                   style(format!("|{}|",выдер_изначальный_из_замены.правописание.удвоенная_гласная,)).blue(),
                   format!("двойная согласная"),
                  style(format!("|{}|",выдер_изначальный_из_замены.правописание.удвоенная_согласная)).blue(),
                  //
                    format!("образец в Торжке"),
                    style(format!("|{}|",образец_в_торжке.строка)).blue(),
                    format!("двойная гласная"),
                   style(format!("|{}|",образец_в_торжке.правописание.удвоенная_гласная)).blue(),
                   format!("двойная согласная"),
                  style(format!("|{}|",образец_в_торжке.правописание.удвоенная_согласная)).blue(),


                )
            }

            Вид_ошибки_при_поиске_выдера_в_торжке::Исключение(ошибка) => {
                write!(f, "|{}| {} {} {} ",
                    style(format!("|{}|",ошибка.изначальный_выдер_неруси)).blue(),
                    format!(" его вид "),
                    style(format!("|{}|", ошибка.вид_окончания)).blue(),
                     format!(" содержится в исключениях Торжка"),


                )
            }
            Вид_ошибки_при_поиске_выдера_в_торжке::Не_найден_в_кучах(слово) => {
                write!(f, "{} |{}| {} ",
                          format!("Торжок не содержит окончание"),
                       style(format!("|{}|",слово)).blue(),
                        format!("в кучах"),
                )
            }
            Вид_ошибки_при_поиске_выдера_в_торжке::Не_соответствие_частей_речи(Ошибка_несоотвествия_части_речи)/*(где_найдено)*/ => {
                   write!(
                       f,
                       "{}",
                       //format!("Не соответствие частей речи - "),
                       Ошибка_несоотвествия_части_речи,
                         /*format!("выдер_изначальный |{}|",Ошибка_несоотвествия_части_речи.выдер_изначальный.вывести_в_строку()),
                            format!("купленный_на_торжке |{}|",Ошибка_несоотвествия_части_речи.часть_речи_торжка.вывести_в_строку()),*/
                      // style(где_найдено).magenta(),
                   )
               }
               Вид_ошибки_при_поиске_выдера_в_торжке::Не_найден_вид_окончания_в_разделе_торжка_но_есть_в_куче(слово,вид_окончания) => {
                      write!(
                          f,
                          "{} |{}| {} {} |{}| {}",
                             style("Вид окончания").bold(),
                              style(вид_окончания).blue(),
                          style("не найден в Торжке").yellow(),
                               style("| Cамо окончание, предоставленное под обмен"),
                           style(слово).blue(),
                                  style("предоставленное под обмен. Данное окончание имеется в кучах Торжка"),

                      )
                  }
            Вид_ошибки_при_поиске_выдера_в_торжке::Не_найдено_окончание_в_разделе_торжка_но_есть_в_куче(слово) => {
                   write!(
                       f,
                       "{} |{}| {}",
                       style("не найдено окончание "),
                       style(слово).magenta(),
                                style("в разделах Торжка, но есть в куче"),
                   )
               }
           /*Вид_ошибки_при_поиске_выдера_в_торжке::Не_найден_в_куче_окончаний(где_найдено) => {
                   write!(
                       f,
                       "{}|{}|",
                       style("не найдено в куче окончаний Торжка "),
                       style(где_найдено).magenta(),
                   )
               }*/
               /*Вид_ошибки_при_поиске_выдера_в_торжке::Не_найден_в_куче_окончаний => {
                       write!(
                           f,
                           "{}",
                           style("не найдено в куче окончаний Торжка "),
                       )
               }*/
            Вид_ошибки_при_поиске_выдера_в_торжке::Не_определено => {
                f.write_str("нет в Торжке")
            }
        }
    }
}
//
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Вид_Окончания {
    Сен,
    Сён,
    Нут,
    Пн,
    Дан,
    Сн_длинное,
    Ин,
    Ным_но,
    Тн,
    // Лос,
    Ж_сущ,
    Чн,
    _Н,
    _В,
    Р_сущ,
    _А,
    Ам,
    Ко_ка,
    Ок_сущ,
    Ск,
    Ва_Во,
    Ель,
    _Е,
    Ен,
    Рен,
    Ён,
    Ван,
    _О,
    Он,
    Ан,
    Ат,
    //Ни,
    Вш,
    // Авш,
    Ут,
    _У,
    Г_полн,
    Г_сущ,
    Ем,
    Ом,
    Ют,
    Ют_сущ,
    _Ю,
    Ит,
    Ет,
    Ив,
    Ов,
    Ев,
    Нят,
    Нятн,
    Ят,
    _Щ,
    // _Ы,
    Ыт,
    Ым,
    _Ы,
    Та_то,
    //Те,
    // Ты,
    Чи,
    _И,
    _Л,
    Льн,
    //_К,
    Ъят,
    _Ш,
    Ц_сущ,
    Ст,
    _С,
    Ств,
    Ям,
    Ти,
    Д_СУЩ,
    Ян,
    Жн,
    Сл,
    //Лю_ля,
    Не_определено,
}
pub static ВСЕ_ВИДЫ_ОКОНЧАНИЙ: [Вид_Окончания; 70] = [
    // Вид_Окончания::_Лос,
    Вид_Окончания::Ок_сущ,
    Вид_Окончания::Сен,
    Вид_Окончания::Ным_но,
    Вид_Окончания::Ат,
    Вид_Окончания::Сн_длинное,
    Вид_Окончания::Нут,
    Вид_Окончания::Ют_сущ,
    Вид_Окончания::Ин,
    Вид_Окончания::Рен,
    Вид_Окончания::Сён,
    Вид_Окончания::Дан,
    Вид_Окончания::Пн,
    Вид_Окончания::Льн,
    Вид_Окончания::Тн,
    Вид_Окончания::Ет,
    Вид_Окончания::Ян,
    Вид_Окончания::Чн,
    Вид_Окончания::_И,
    Вид_Окончания::Ж_сущ,
    Вид_Окончания::Жн,
    Вид_Окончания::_Н,
    Вид_Окончания::_О,
    Вид_Окончания::_А,
    Вид_Окончания::_В,
    Вид_Окончания::Р_сущ,
    Вид_Окончания::Ам,
    Вид_Окончания::Ко_ка,
    Вид_Окончания::Ск,
    Вид_Окончания::Ва_Во,
    Вид_Окончания::Ель,
    Вид_Окончания::_Е,
    Вид_Окончания::_У,
    Вид_Окончания::Ен,
    Вид_Окончания::Ён,
    Вид_Окончания::Ван,
    Вид_Окончания::Он,
    Вид_Окончания::Ан,
    Вид_Окончания::Ат,
    //Вид_Окончания::Ни,
    Вид_Окончания::Вш,
    // Вид_Окончания::Авш,
    Вид_Окончания::Ут,
    Вид_Окончания::Г_полн,
    Вид_Окончания::Г_сущ,
    Вид_Окончания::Ем,
    Вид_Окончания::Ом,
    Вид_Окончания::Ют,
    Вид_Окончания::Ит,
    Вид_Окончания::Ив,
    Вид_Окончания::Ов,
    Вид_Окончания::Ев,
    Вид_Окончания::Нят,
    Вид_Окончания::Нятн,
    Вид_Окончания::Ят,
    Вид_Окончания::_Щ,
    // _Ы — закомментирован в enum
    Вид_Окончания::Ыт,
    Вид_Окончания::Ым,
    Вид_Окончания::Та_то,
    //Вид_Окончания::Те,
    //Вид_Окончания::Ты,
    Вид_Окончания::Чи,
    Вид_Окончания::_Л,
    //Вид_Окончания::_К,
    Вид_Окончания::Ъят,
    Вид_Окончания::_Ш,
    Вид_Окончания::Ц_сущ,
    Вид_Окончания::Ст,
    Вид_Окончания::_С,
    Вид_Окончания::Ств,
    Вид_Окончания::Ям,
    Вид_Окончания::Ти,
    Вид_Окончания::Д_СУЩ,
    Вид_Окончания::_Ы,
    Вид_Окончания::_Ю,
    Вид_Окончания::Сл,
    //Вид_Окончания::_Л,
    //Вид_Окончания::Не_определено,
];
impl Вид_Окончания {
    /*pub fn получить_вид_окончания_по_строке(
        окончание: &str,
        разделы_окончаний: &Разделы_окончаний_Полные,
    ) -> Result<Self, String> {
        for раздел in разделы_окончаний.iter() {
            if раздел.куча_окончаний_полная.contains(окончание) {
                return Ok(раздел.вид_окончания);
            }
        }
        //
        Err(format!("Не найдено окончание в разделах|{окончание}|"))
    }*/
    pub fn ен_ён(&self) -> bool {
        match self {
            Вид_Окончания::Ён => true,
            Вид_Окончания::Ен => true,
            _ => false,
        }
    }
    pub fn ты_то(&self) -> bool {
        match self {
            //Вид_Окончания::Ты => true,
            Вид_Окончания::Та_то => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Вид_Окончания {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Вид_Окончания::Ок_сущ => "Ок_сущ",
            Вид_Окончания::Сен => "Сен",
            Вид_Окончания::Сн_длинное => "Сн_длинное",
            Вид_Окончания::Сён => "Сён",
            Вид_Окончания::Ют_сущ => "Ют_сущ",
            Вид_Окончания::Дан => "Дан",
            Вид_Окончания::Нут => "Нут",
            Вид_Окончания::Пн => "Пн",
            Вид_Окончания::Ин => "Ин",
            Вид_Окончания::Ным_но => "Ным_но",
            Вид_Окончания::Рен => "Рен",
            Вид_Окончания::Льн => "Лн",
            Вид_Окончания::Тн => "Тн",
            Вид_Окончания::Ян => "Ян",
            Вид_Окончания::Ко_ка => "Ко",
            Вид_Окончания::Сл => "Сл",
            Вид_Окончания::Жн => "Жн",
            Вид_Окончания::Чн => "Чн",
            Вид_Окончания::_Е => "_Е",
            Вид_Окончания::_А => "_А",
            Вид_Окончания::Г_полн => "Г_полн",
            Вид_Окончания::Г_сущ => "Г_одиночн",
            //Вид_Окончания::_Л => "Лю_ля",
            Вид_Окончания::Ет => "Ет",
            Вид_Окончания::_О => "_О",
            Вид_Окончания::_И => "_И",
            Вид_Окончания::Ут => "Ут",
            Вид_Окончания::Ат => "Ат",
            Вид_Окончания::Ти => "Ти",
            Вид_Окончания::Чи => "Чи",
            Вид_Окончания::_С => "_С",
            Вид_Окончания::Ц_сущ => "_Ц",
            Вид_Окончания::_У => "_У",
            Вид_Окончания::Ят => "Ят",
            Вид_Окончания::Ям => "Ям",
            Вид_Окончания::Ск => "Ск",
            Вид_Окончания::Ам => "Ам",
            Вид_Окончания::Ж_сущ => "_Ж",
            //Вид_Окончания::_К => "_К",
            Вид_Окончания::Р_сущ => "_Р",
            //Вид_Окончания::Ни => "Ни",
            Вид_Окончания::Ель => "Ель",
            Вид_Окончания::Ъят => "Ъят",
            Вид_Окончания::Нят => "Нят",
            Вид_Окончания::Нятн => "Нятн",
            Вид_Окончания::Ств => "Ств",
            //Вид_Окончания::Ты => "Ты",
            // Вид_Окончания::Те => "Те",
            Вид_Окончания::Ва_Во => "Ва",
            Вид_Окончания::Ыт => "Ыт",
            Вид_Окончания::Ст => "Ст",
            Вид_Окончания::Та_то => "То",
            Вид_Окончания::Ым => "Ым",
            // Вид_Окончания::Ты => "Ты",
            // Вид_Окончания::_Ы => "Ыт",
            Вид_Окончания::_Ю => "_Ю",
            Вид_Окончания::Д_СУЩ => "_Д",
            Вид_Окончания::_Ш => "_Ш",
            Вид_Окончания::_Щ => "_Щ",
            Вид_Окончания::_Л => "_Л",
            Вид_Окончания::Ит => "Ит",
            Вид_Окончания::Ов => "Ов",
            Вид_Окончания::Ев => "Ев",
            Вид_Окончания::Ив => "Ив",
            Вид_Окончания::_Н => "_Н",
            Вид_Окончания::_Ы => "_Ы",
            Вид_Окончания::_В => "_В",
            Вид_Окончания::Ем => "Ем",
            Вид_Окончания::Ом => "Ом",
            Вид_Окончания::Ен => "Ен",
            Вид_Окончания::Ют => "_Ю",
            Вид_Окончания::Ён => "Ён",
            Вид_Окончания::Ван => "ВАН",
            Вид_Окончания::Он => "Он",
            Вид_Окончания::Ан => "Ан",
            // Вид_Окончания::Авш => "Авш",
            Вид_Окончания::Вш => "Вш",
            Вид_Окончания::Не_определено => "Не_определено",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Словарь_Куч_простых {
    pub искомые:
        [Куча_Словарь_Искомые_простые; КОЛИЧЕСТВО_УРОВНЕЙ_СЛОВАРЯ_КУЧ],
    pub замены:
        [Куча_Словарь_Замены_простые; КОЛИЧЕСТВО_УРОВНЕЙ_СЛОВАРЯ_КУЧ],
}
// Сначала объявите трейт Clear
pub trait Clear {
    fn clear(&mut self);
}

impl Clear for Полный_Словарь {
    fn clear(&mut self) {
        self.запятые_длинные.clear();
        self.запятые.clear();
        self.простое.clear();
        self.составное.clear();
        self.составное_важное.clear();
        self.составное_длинное.clear();
        self.вездесущее.clear();
        self.неизменное.clear();
        self.огласовки.clear();
        self.неизменное_длинное.clear();
        self.неизменное_короткое.clear();
    }
}

#[derive(Debug, Default)]
pub struct Счётчики_Словаря {
    pub простое: Vec<AtomicUsize>,             //одиночные слова
    pub составное: Vec<AtomicUsize>,           //одиночные слова
    pub составное_важное: Vec<AtomicUsize>,    //одиночные слова
    pub составное_длинное: Vec<AtomicUsize>,   //одиночные слова
    pub вездесущее: Vec<AtomicUsize>,          //одиночные слова
    pub неизменное: Vec<AtomicUsize>,          //одиночные слова
    pub огласовки: Vec<AtomicUsize>,           //одиночные слова
    pub неизменное_короткое: Vec<AtomicUsize>, //одиночные слова
    pub неизменное_длинное: Vec<AtomicUsize>,  //одиночные слова
    pub запятые: Vec<AtomicUsize>,             //одиночные слова
    pub запятые_длинные: Vec<AtomicUsize>,     //одиночные слова
}
//impl Default for
//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct Быстрый_Словарь {
    //одиночные
    pub простое: Vec<Умная_Строка>, //одиночные слова
}

#[derive(Debug, Default, Clone)]
pub struct Слова_с_Вложениями {
    pub слово: Умная_Строка,
    pub вложения: Умная_Строка,
}
#[derive(Debug, Default, Clone)]
pub struct Прогон_замены {
    pub книги: Vec<Книга_целиковая>,
    pub сообщения: Сообщения_считывания_книг,
}
pub mod Кучи_Раздел {
    use std::sync::LazyLock;
    pub struct Имена_Страниц_Куча {
        pub простое: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub составное: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub составное_важное: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub составное_длинное: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub вездесущее: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub неизменное: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub огласовки: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub неизменное_короткое: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub неизменное_длинное: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub запятые: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub перевести: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
        pub запятые_длинные: rapidhash::fast::RapidHashSet<&'static str>, //одиночные слова
    }
    //
    pub static ИМЕНА_СТР_ВСЕ: LazyLock<Имена_Страниц_Куча> =
        LazyLock::new(|| Имена_Страниц_Куча {
            простое: ИМЕНА_СТР_ПРОСТЫЕ_СЛОВА.clone(),
            составное: ИМЕНА_СТР_СОСТАВНЫЕ_СЛОВА.clone(),
            составное_важное:
                ИМЕНА_СТР_СОСТАВНЫЕ_ВАЖНЫЕ_СЛОВА.clone(),
            составное_длинное:
                ИМЕНА_СТР_СОСТАВНЫЕ_ДЛИННЫЕ_СЛОВА.clone(),
            вездесущее: ИМЕНА_СТР_ВЕЗДЕСУЩИЕ_СЛОВА.clone(),
            неизменное: ИМЕНА_СТР_НЕИЗМЕННЫЕ_СЛОВА.clone(),
            огласовки: ИМЕНА_СТР_ОГЛАСОВКИ.clone(),
            неизменное_короткое:
                ИМЕНА_СТР_НЕИЗМЕННЫЕ_КОРОТКИЕ_СЛОВА.clone(),
            неизменное_длинное:
                ИМЕНА_СТР_НЕИЗМЕННЫЕ_ДЛИННЫЕ_СЛОВА.clone(),
            запятые: ИМЕНА_СТР_ЗАПЯТЫЕ.clone(),
            запятые_длинные: ИМЕНА_СТР_ЗАПЯТЫЕ_ДЛИННЫЕ.clone(),
            перевести: ИМЕНА_СТР_ПЕРЕВЕСТИ.clone(),
        });
    //
    impl Имена_Страниц_Куча {
        /// Проверяет, есть ли слово хотя бы в одном из наборов.
        pub fn содержит(&self, слово: &super::Умная_Строка) -> bool {
            if слово.не_пусто() {
                self.простое.contains(слово.получить_ссылку())
                    || self.составное.contains(слово.получить_ссылку())
                    || self.составное_важное.contains(слово.получить_ссылку())
                    || self.составное_длинное.contains(слово.получить_ссылку())
                    || self.вездесущее.contains(слово.получить_ссылку())
                    || self.неизменное.contains(слово.получить_ссылку())
                    || self.огласовки.contains(слово.получить_ссылку())
                    || self.неизменное_короткое.contains(слово.получить_ссылку())
                    || self.неизменное_длинное.contains(слово.получить_ссылку())
                    || self.запятые.contains(слово.получить_ссылку())
                    || self.запятые_длинные.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn определить_имя_страницы(
            &self,
            имя_страницы: &super::Умная_Строка,
        ) -> Result<super::Имена_страниц, String> {
            if !имя_страницы.не_пусто() {
                panic!("Пустое имя страницы");
            }
            let имя_страницы_ссылка = имя_страницы.получить_ссылку();

            if self.простое.contains(имя_страницы_ссылка) {
                return Ok(super::Имена_страниц::Простая_стр);
            }
            if self.составное.contains(имя_страницы_ссылка) {
                return Ok(super::Имена_страниц::Cоставная_стр);
            }
            if self.составное_длинное.contains(имя_страницы_ссылка)
            {
                return Ok(super::Имена_страниц::Составные_длинные_стр);
            }
            if self.составное_важное.contains(имя_страницы_ссылка) {
                return Ok(super::Имена_страниц::Составные_важные_стр);
            }
            if self.вездесущее.contains(имя_страницы_ссылка) {
                return Ok(super::Имена_страниц::Вездесущее_стр);
            }
            if self.неизменное.contains(имя_страницы_ссылка) {
                return Ok(super::Имена_страниц::Неизменные_стр);
            }
            if self.огласовки.contains(имя_страницы_ссылка) {
                return Ok(super::Имена_страниц::Огласовки_стр);
            }
            if self.неизменное_короткое.contains(имя_страницы_ссылка)
            {
                return Ok(super::Имена_страниц::Неизменные_короткие_стр);
            }
            if self.неизменное_длинное.contains(имя_страницы_ссылка)
            {
                return Ok(super::Имена_страниц::Неизменные_длинные_стр);
            }
            if self.запятые.contains(имя_страницы_ссылка) {
                return Ok(super::Имена_страниц::Запятые_стр);
            }
            if self.запятые_длинные.contains(имя_страницы_ссылка) {
                return Ok(super::Имена_страниц::Запятые_длинные_стр);
            }
            if self.перевести.contains(имя_страницы_ссылка) {
                return Ok(super::Имена_страниц::Перевести_стр);
            }

            Err(format!("Не определён вид страницы: |{имя_страницы}|"))
        }
        pub fn простое(&self, слово: &super::Умная_Строка) -> bool {
            if слово.не_пусто() {
                self.простое.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn составное(&self, слово: &super::Умная_Строка) -> bool {
            if слово.не_пусто() {
                self.составное.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn составное_важное(
            &self, слово: &super::Умная_Строка
        ) -> bool {
            if слово.не_пусто() {
                self.составное_важное.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn составное_длинное(
            &self, слово: &super::Умная_Строка
        ) -> bool {
            if слово.не_пусто() {
                self.составное_длинное.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn вездесущее(&self, слово: &super::Умная_Строка) -> bool {
            if слово.не_пусто() {
                self.вездесущее.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn неизменное(&self, слово: &super::Умная_Строка) -> bool {
            if слово.не_пусто() {
                self.неизменное.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn неизменное_короткое(
            &self, слово: &super::Умная_Строка
        ) -> bool {
            if слово.не_пусто() {
                self.неизменное_короткое.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn неизменное_длинное(
            &self, слово: &super::Умная_Строка
        ) -> bool {
            if слово.не_пусто() {
                self.неизменное_длинное.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn огласовки(&self, слово: &super::Умная_Строка) -> bool {
            if слово.не_пусто() {
                self.огласовки.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn запятые(&self, слово: &super::Умная_Строка) -> bool {
            if слово.не_пусто() {
                self.запятые.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn запятые_длинные(
            &self, слово: &super::Умная_Строка
        ) -> bool {
            if слово.не_пусто() {
                self.запятые_длинные.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
        pub fn перевести(&self, слово: &super::Умная_Строка) -> bool {
            if слово.не_пусто() {
                self.перевести.contains(слово.получить_ссылку())
            } else {
                false
            }
        }
    }
    //use rapidhash::*;

    //
    /* pub static ИМЕНА_СТР_ВСЕ: LazyLock<[rapidhash::fast::RapidHashSet<&'static str>; 12]> =
    LazyLock::new(|| {
        [
            &ИМЕНА_СТР_ПРОСТЫЕ_СЛОВА,
            &ИМЕНА_СТР_СОСТАВНЫЕ_СЛОВА,
            &ИМЕНА_СТР_СОСТАВНЫЕ_ДЛИННЫЕ_СЛОВА,
            &ИМЕНА_СТР_СОСТАВНЫЕ_ВАЖНЫЕ_СЛОВА,
            &ИМЕНА_СТР_ВЕЗДЕСУЩИЕ_СЛОВА,
            &ИМЕНА_СТР_НЕИЗМЕННЫЕ_СЛОВА,
            &ИМЕНА_СТР_НЕИЗМЕННЫЕ_КОРОТКИЕ_СЛОВА,
            &ИМЕНА_СТР_НЕИЗМЕННЫЕ_ДЛИННЫЕ_СЛОВА,
            &ИМЕНА_СТР_ОГЛАСОВКИ,
            &ИМЕНА_СТР_ЗАПЯТЫЕ,
            &ИМЕНА_СТР_ПЕРЕВЕСТИ,
            &ИМЕНА_СТР_ЗАПЯТЫЕ_ДЛИННЫЕ,
        ]
    });*/
    //
    pub static ИМЕНА_СТР_ПРОСТЫЕ_СЛОВА: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "Простые",
            "Простое",
            "одиночные",
            "одиночное",
            "Одиночные",
            "Одиночное",
            "простые",
            "простое",
            "простые слова",
            "Простые слова",
            "простые_слова",
            "Простые_слова",
        ])
    });
    //
    pub static ИМЕНА_СТР_СОСТАВНЫЕ_СЛОВА: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "Составные",
            "составные",
            "сложные слова",
            "Сложные слова",
            "сложные_слова",
            "Сложные_слова",
        ])
    });
    pub static ИМЕНА_СТР_СОСТАВНЫЕ_ДЛИННЫЕ_СЛОВА: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "Составные_длинные",
            "Составные_Длинные",
            "составные_длинные",
            "Составные длинные",
            "Составные Длинные",
            "составные длинные",
        ])
    });
    pub static ИМЕНА_СТР_СОСТАВНЫЕ_ВАЖНЫЕ_СЛОВА: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "Составные_важные",
            "Составные_Важные",
            "составные_важные",
            "Составные важные",
            "Составные Важные",
            "составные важные",
        ])
    });
    pub static ИМЕНА_СТР_ВЕЗДЕСУЩИЕ_СЛОВА: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "Вездесущие",
            "вездесущие",
            "вездесущие слова",
            "Вездесущие слова",
            "вездесущие_слова",
            "Вездесущие_слова",
        ])
    });
    pub static ИМЕНА_СТР_НЕИЗМЕННЫЕ_СЛОВА: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "Неизменные",
            "неизменные",
            "неизменные слова",
            "Неизменные слова",
            "неизменные_слова",
            "Неизменные_слова",
        ])
    });
    pub static ИМЕНА_СТР_НЕИЗМЕННЫЕ_КОРОТКИЕ_СЛОВА: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "неизменные_короткие",
            "Неизменные_короткие",
            "неизменные короткие",
            "Неизменные_Короткие",
            "Неизменные Короткие",
        ])
    });
    pub static ИМЕНА_СТР_НЕИЗМЕННЫЕ_ДЛИННЫЕ_СЛОВА: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "неизменные_длинные",
            "Неизменные_длинные",
            "Неизменные длинные",
            "Неизменные_Длинные",
            "Неизменные Длинные",
        ])
    });
    pub static ИМЕНА_СТР_ОГЛАСОВКИ: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["Огласовки", "огласовки"]));
    pub static ИМЕНА_СТР_ЗАПЯТЫЕ: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["Запятые", "запятые"]));
    pub static ИМЕНА_СТР_ПЕРЕВЕСТИ: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["Перевести", "перевести"]));
    pub static ИМЕНА_СТР_ЗАПЯТЫЕ_ДЛИННЫЕ: LazyLock<
        rapidhash::fast::RapidHashSet<&'static str>,
    > = LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "Запятые_длинные",
            "Запятые_Длинные",
            "запятые_длинные",
            "Запятые длинные",
            "Запятые Длинные",
            "запятые длинные",
        ])
    });
}
/*#[derive(Debug, Clone)]
pub struct Окончания_включающие {
    pub ячейка_re: &'static [RE_полное_окончание],
    pub срез_окончаний: &'static [&'static str],
}*/
/*#[derive(Debug, Clone)]
pub struct RE_Окончания_Поиск {
    pub re_поиска: Regex,
    pub re_замена: Regex,
    pub окончание_строка: &'static str,
    pub вид_окончания: Вид_Окончания,
}*/
pub const ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА: &[Вид_части_речи] = &[Вид_части_речи::Не_определено];
/*#[derive(Debug, Clone)]
pub struct Re_образцы {
    pub содержимое: RE_Окончания_Поиск,
    pub исключения_первой_очереди: &'static [RE_полное_окончание],
    pub исключения_второй_очереди: &'static [RE_полное_окончание],
}*/
#[derive(Debug, Clone)]
pub struct RE_полное_окончание {
    pub re_поиска: Regex,
    pub re_замены: Regex,
    pub правописание: Буквы_Слова_Словаря,
    pub строка: &'static str,
    pub вид_окончания: Вид_Окончания,
    pub части_речи: &'static [Вид_части_речи],
    pub особенности_образца: Особенности_образца_RE,
}
impl Default for RE_полное_окончание {
    fn default() -> Self {
        Self {
            re_поиска: Regex::new(r"").unwrap(),
            re_замены: Regex::new(r"").unwrap(),
            строка: "",
            вид_окончания: Вид_Окончания::Не_определено, // какой-то дефолтный вариант
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            правописание: Default::default(),
            особенности_образца: Default::default(),
        }
    }
}
impl PartialEq for RE_полное_окончание {
    fn eq(&self, other: &Self) -> bool {
        self.re_поиска.as_str() == other.re_поиска.as_str()
            && self.re_замены.as_str() == other.re_замены.as_str()
            && self.строка == other.строка
            && self.вид_окончания == other.вид_окончания
            && self.части_речи == other.части_речи
    }
}

impl Eq for RE_полное_окончание {}
pub static ПУСТОЕ_RE_ИСКЛЮЧАЮЩЕЕ_ОКОНЧАНИЕ: LazyLock<
    [RE_полное_окончание; 0],
> = LazyLock::new(|| Default::default());
//
//вырезано
//
#[derive(Debug)]
pub struct Разделы_окончаний_Словаря {
    //pub в_одиночн: Раздел_Окончания,
    pub сн_длинное: Раздел_Окончания,
    pub ю_одиночн: Раздел_Окончания,
    pub е_одиночн: Раздел_Окончания,
    pub ы_одиночн: Раздел_Окончания,
    pub ж_сущ: Раздел_Окончания,
    pub о_одиночн: Раздел_Окончания,
    pub у_одиночн: Раздел_Окончания,
    pub сл: Раздел_Окончания,
    //pub к_одиночн: Раздел_Окончания,
    pub ель: Раздел_Окончания,
    pub ым: Раздел_Окончания,
    pub ют: Раздел_Окончания,
    pub ут_ую_уя: Раздел_Окончания,
    pub г_полн: Раздел_Окончания,
    pub ям: Раздел_Окончания,
    pub ван: Раздел_Окончания,
    pub ыт: Раздел_Окончания,
    pub ат: Раздел_Окончания,
    //pub лю_ля: Раздел_Окончания,
    pub пн: Раздел_Окончания,
    pub ят: Раздел_Окончания,
    pub ив: Раздел_Окончания,
    pub ов: Раздел_Окончания,
    pub ев: Раздел_Окончания,
    pub ъят: Раздел_Окончания,
    pub нят: Раздел_Окончания,
    pub ет: Раздел_Окончания,
    pub ит: Раздел_Окончания,
    pub ти: Раздел_Окончания,
    pub тн: Раздел_Окончания,
    pub чи: Раздел_Окончания,
    pub ко_ка: Раздел_Окончания,
    pub и_одиночн: Раздел_Окончания,
    pub л_полная: Раздел_Окончания,
    pub сен: Раздел_Окончания,
    pub сён: Раздел_Окончания,
    pub с_одиночн: Раздел_Окончания,
    pub д_сущ: Раздел_Окончания,
    pub ц_сущ: Раздел_Окончания,
    pub р_сущ: Раздел_Окончания,
    pub н_одиночн: Раздел_Окончания,
    pub а_одиночн: Раздел_Окончания,
    pub г_одиночн: Раздел_Окончания,
    pub ш: Раздел_Окончания,
    pub щ: Раздел_Окончания,
    pub ств: Раздел_Окончания,
    pub ст: Раздел_Окончания,
    pub ен: Раздел_Окончания,
    pub ён: Раздел_Окончания,
    pub он: Раздел_Окончания,
    pub ем: Раздел_Окончания,
    pub ам: Раздел_Окончания,
    pub ом: Раздел_Окончания,
    pub ан: Раздел_Окончания,
    pub ин: Раздел_Окончания,
    pub вш: Раздел_Окончания,
    //pub ни: Раздел_Окончания,
    pub ск: Раздел_Окончания,
    pub нятн: Раздел_Окончания,
    //pub ты: Раздел_Окончания,
    pub та_то: Раздел_Окончания,
    pub ва_во: Раздел_Окончания,
    pub ным_но: Раздел_Окончания,
    pub нут: Раздел_Окончания,
    pub дан: Раздел_Окончания,
    pub чн: Раздел_Окончания,
    pub ян: Раздел_Окончания,
    pub жн: Раздел_Окончания,
    pub ют_сущ: Раздел_Окончания,
    pub льн: Раздел_Окончания,
    pub рен: Раздел_Окончания,
    pub в_одиночн: Раздел_Окончания,
    pub ок_сущ: Раздел_Окончания,
}
pub static ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ЕН_ЁН: LazyLock<
    rapidhash::fast::RapidHashSet<Вид_Окончания>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([Вид_Окончания::Ён, Вид_Окончания::Ен])
});
/*pub static ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ТА_ТЫ: LazyLock<
    rapidhash::fast::RapidHashSet<Вид_Окончания>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([Вид_Окончания::Та_то, Вид_Окончания::Ты])
});*/
impl Разделы_окончаний_Словаря {
    /// Порядок: ван, ыт, ит, н, ст, ен, ён, он, ем, ом, ан, авш.
    pub fn iter(&self) -> impl Iterator<Item = &Раздел_Окончания> {
        [
            &self.ель,
            &self.чн,
            &self.ван,
            &self.дан,
            &self.ъят,
            &self.нятн,
            &self.нят,
            &self.ют,
            &self.ств,
            &self.пн,
            &self.рен,
            &self.ен,
            &self.ён,
            &self.ин,
            &self.ным_но, //перед ен  - так как образец ЕН есть внутри
            &self.ет,
            &self.сн_длинное,
            //
            &self.сён,
            &self.сен,
            &self.он,
            &self.сл,
            &self.ко_ка,
            &self.нут,
            &self.ем,
            &self.ев,
            &self.ти,
            &self.чи,
            &self.ов,
            // &self.ам,
            &self.ят,
            //&self.лю_ля,
            &self.ат,
            &self.ян,
            &self.ан,
            &self.ив,
            &self.ск,
            &self.вш,
            &self.щ,
            &self.ш,
            &self.ит,
            &self.ыт,
            // &self.ты,
            &self.та_то,
            &self.жн,
            &self.тн,
            //вторая очередь
            &self.ж_сущ,
            &self.р_сущ,
            &self.ут_ую_уя,
            &self.ом,
            &self.ым,
            &self.ям,
            //&self.ни,
            &self.ва_во,
            &self.льн,
            &self.л_полная,
            //
            &self.г_полн,
            &self.д_сущ,
            //&self.к_одиночн,
            &self.ц_сущ,
            //&self.в_одиночн,
            //
            &self.н_одиночн,
            &self.ст,
            &self.о_одиночн,
            &self.ю_одиночн,
            &self.у_одиночн,
            &self.ы_одиночн,
            &self.и_одиночн,
            &self.а_одиночн,
            &self.е_одиночн,
            //с одиночные самые последние
            &self.с_одиночн,
            &self.в_одиночн,
            &self.г_одиночн,
            &self.ок_сущ,
            &self.ют_сущ,
        ]
        .into_iter()
    }

    pub fn получить_ячейку_по_срезу_и_виду_окончания(
        &self,
        окончание_входное: &str,
        вид_окончания_входной: &Вид_Окончания,
    ) -> Result<RE_полное_окончание, ()> {
        //

        //
        for раздел in self.iter() {
            //if раздел.вид_окончания == *вид_окончания_входной {
            //если есть в куче окончаний
            if раздел.куча_окончаний_полная.contains(окончание_входное)
            {
                //поиск в разделе
                for ячейка in раздел.re_ячейки.iter() {
                    if ячейка.строка == окончание_входное {
                        if *вид_окончания_входной == ячейка.вид_окончания
                        {
                            return Ok(ячейка.clone());
                        }
                        //
                        else if ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ЕН_ЁН.contains(вид_окончания_входной)
                            && ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ЕН_ЁН.contains(&ячейка.вид_окончания)
                        {
                            return Ok(ячейка.clone());
                        } /*else if ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ТА_ТЫ.contains(вид_окончания_входной)
                        && ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ТА_ТЫ.contains(&ячейка.вид_окончания)
                        {
                        return Ok(ячейка.clone());
                        }*/
                    }
                }
            }
            //}
        }
        println!(
            "Не удалось получить ячейку из Разделов полных по срезу |{окончание_входное}| вид окончания входной |{}|",
            вид_окончания_входной
        );
        return Err(());
        //
    }
    pub fn определить_вид_части_речи_по_срезу_и_виду_окончания(
        &self,
        окончание_входное: &str,
        вид_окончания_входной: &Вид_Окончания,
    ) -> Result<&[Вид_части_речи], ()> {
        //

        //
        for раздел in self.iter() {
            //если есть в куче окончаний
            if раздел.куча_окончаний_полная.contains(окончание_входное)
            {
                //поиск в разделе
                for ячейка in раздел.re_ячейки.iter() {
                    if ячейка.строка == окончание_входное {
                        return Ok(ячейка.части_речи);
                    }
                    //если это окончания Ён и Ен
                    //
                    else if ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ЕН_ЁН.contains(вид_окончания_входной)
                        && ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ЕН_ЁН.contains(&ячейка.вид_окончания)
                    {
                        return Ok(ячейка.части_речи);
                    } /*else if ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ТА_ТЫ.contains(вид_окончания_входной)
                    && ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ТА_ТЫ.contains(&ячейка.вид_окончания)
                    {
                    return Ok(ячейка.части_речи);
                    }*/
                }
            }
        }
        return Err(());
        //
    }
    pub fn определить_вид_части_речи_по_ячейке_торжка(
        &self,
        ячейка_торжка: &Вид_Окончания_со_строкой,
    ) -> Result<&[Вид_части_речи], ()> {
        //

        //
        //
        for раздел in self.iter() {
            /*if раздел.вид_окончания == ячейка_торжка.вид_окончания
            {*/
            //если есть в куче окончаний
            if раздел.куча_окончаний_полная.contains(ячейка_торжка.строка)
            {
                //поиск в разделе
                for ячейка in раздел.re_ячейки.iter() {
                    //
                    if ячейка.строка == ячейка_торжка.строка {
                        return Ok(ячейка.части_речи);
                    }
                    //если это окончания Ён и Ен
                    //
                    else if ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ЕН_ЁН.contains(&ячейка_торжка.вид_окончания)
                        && ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ЕН_ЁН.contains(&ячейка.вид_окончания)
                    {
                        return Ok(ячейка.части_речи);
                    } /*else if ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ТА_ТЫ.contains(&ячейка_торжка.вид_окончания)
                    && ОКОНЧАНИЯ_ИСКЛЮЧЕНИЯ_ТА_ТЫ.contains(&ячейка.вид_окончания)
                    {
                    return Ok(ячейка.части_речи);
                    }*/
                }
            }
            //}
        }
        println!("Не нашло часть речи для |{}|", ячейка_торжка.строка);
        return Err(());
        //
    }
    pub fn все_ли_окончания_торжка_есть(
        &self,
        торжок: &[Раздел_Торжка],
    ) -> Result<&[Вид_части_речи], ()> {
        //
        for раздел_торжка in торжок.iter() {
            //
            'указатель_ячейки_торжка: for ячейка_торжка in
                раздел_торжка.ряд.iter()
            {
                //
                match self.определить_вид_части_речи_по_ячейке_торжка(
                    &ячейка_торжка,
                ) {
                    Ok(_) => continue 'указатель_ячейки_торжка,
                    Err(_) => {
                        println!(
                            "{} {} {} {} {} {}",
                            style(format!("Ошибка")).red(),
                            style(format!(
                                "В Полном разделе окончаний нет ячейки Торжка из первой очереди"
                            ))
                            .blue(),
                            style(format!("Строка -")),
                            style(format!("|{}|", ячейка_торжка.строка,)).blue(),
                            style(format!("вид_окончания -")),
                            style(format!("|{}|", ячейка_торжка.вид_окончания)).blue(),
                        );
                    }
                }
            }
            //
            //
            /*'указатель_ячейки_торжка: for ячейка_торжка in
                раздел_торжка.вторая_очередь.iter()
            {
                //
                match self.определить_вид_части_речи_по_ячейке_торжка(
                    &ячейка_торжка,
                ) {
                    Ok(_) => continue 'указатель_ячейки_торжка,
                    Err(_) => {
                        println!(
                            "{}{}{}{}{}{}",
                            style(format!("Ошибка ")).red(),
                            style(format!(
                                "В Полном разделе окончаний нет ячейки Торжка из второй очереди "
                            ))
                            .blue(),
                            style(format!("Строка - ")),
                            style(format!("|{}| ", ячейка_торжка.строка,)).blue(),
                            style(format!("вид_окончания - ")),
                            style(format!("|{}|", ячейка_торжка.вид_окончания)).blue(),
                        );
                    }
                }
            }*/
        }
        return Ok(ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА);
        //
    }
    pub fn проверка_на_наличие_окончаний(&self) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&Вид_Окончания> =
            rapidhash::fast::RapidHashSet::default();

        //перебор кучи окончаний
        //перебор раздела
        '_главный_указатель: for окончание in ВСЕ_ВИДЫ_ОКОНЧАНИЙ.iter()
        {
            let mut повторы: rapidhash::fast::RapidHashSet<&Вид_Окончания> =
                rapidhash::fast::RapidHashSet::default();
            //
            'круговорот_раздела: for раздел in self.iter() {
                //
                if раздел.вид_окончания == *окончание {
                    match повторы.contains(окончание) {
                        false => {
                            повторы.insert(&окончание);
                            continue 'круговорот_раздела;
                        }
                        true => {
                            println!(
                                "Повторно в Полных разделах есть вид окончания |{}|",
                                окончание
                            );
                        }
                    }
                }
                //
            }
            if повторы.is_empty() {
                куча_ошибок.insert(окончание);
            }
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("Нет разделов окончаний ")).blue(),
                style(format!(" -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    pub fn проверка_куч_полных_между_собой(&self) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for (указатель_главный, раздел) in self.iter().enumerate() {
            //совпадает ли окончание
            'круговорот_раздела_2: for (_, раздел_2) in self
                .iter()
                .enumerate()
                .filter(|(указатель_2, _)| *указатель_2 != указатель_главный)
            //
            {
                'круговорота_слова_1: for слово_1 in
                    раздел.куча_окончаний_полная.iter()
                {
                    if раздел_2.куча_окончаний_полная.contains(слово_1)
                    {
                        for ячейка1 in раздел.re_ячейки.iter() {
                            if ячейка1.строка == *слово_1 {
                                if ячейка1.особенности_образца.исключить_участие_в_поиске
                                {
                                    continue 'круговорота_слова_1;
                                }
                            }
                        }
                        for ячейка in раздел_2.re_ячейки.iter() {
                            if ячейка.строка == *слово_1 {
                                //если пропустить в поиске - то это исключеие, не участвует в поиске через RE
                                if ячейка.особенности_образца.исключить_участие_в_поиске
                                {
                                    continue 'круговорот_раздела_2;
                                }
                            }
                        }
                        println!(
                            "Куча |{}| содержит повторно оконание |{}| из кучи |{}|",
                            раздел_2.вид_окончания, слово_1, раздел.вид_окончания
                        );
                        куча_ошибок.insert(слово_1);
                    }
                }
            }
            //
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("проверка_куч_полных_между_собой ")).blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    pub fn проверка_раздлов_на_повторы_между_собой(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for (указатель_1, раздел_1) in self.iter().enumerate() {
            //совпадает ли окончание
            for (_, раздел_2) in self
                .iter()
                .enumerate()
                .filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                for слово_1 in раздел_1.куча_окончаний_полная.iter() {
                    if раздел_2.куча_окончаний_полная.contains(слово_1)
                    {
                        куча_ошибок.insert(слово_1);
                    }
                }
            }
            //
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("проверка_раздлов_на_повторы_между_собой ")).blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
}
//
#[derive(Debug)]
pub struct Раздел_Окончания {
    pub куча_окончаний_полная: &'static rapidhash::fast::RapidHashSet<&'static str>,
    pub куча_окончаний_поиска: &'static rapidhash::fast::RapidHashSet<&'static str>,
    pub re_ячейки: &'static [RE_полное_окончание],
    pub вид_окончания: Вид_Окончания,
    pub вид_частей_речи_в_разделе: &'static RapidHashSet<Вид_части_речи>,
}
//
pub static ПУСТАЯ_КУЧА_STR: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter([]));
//
pub static ПУСТАЯ_КУЧА_ВИДОВ_ЧАСТЕЙ_РЕЧИ: LazyLock<
    RapidHashSet<Вид_части_речи>,
> = LazyLock::new(|| RapidHashSet::from_iter([]));
//
pub static ПУСТОЙ_РЯД_REGEX: LazyLock<[RE_полное_окончание; 0]> = LazyLock::new(|| []);
//
impl Default for Раздел_Окончания {
    fn default() -> Self {
        Self {
            куча_окончаний_полная: &*ПУСТАЯ_КУЧА_STR,
            куча_окончаний_поиска: &*ПУСТАЯ_КУЧА_STR,
            re_ячейки: &*ПУСТОЙ_РЯД_REGEX,
            вид_окончания: Вид_Окончания::Не_определено,
            вид_частей_речи_в_разделе:
                &*ПУСТАЯ_КУЧА_ВИДОВ_ЧАСТЕЙ_РЕЧИ,
        }
    }
}
pub static КУЧА_ВИДОВ_СЛОВ_СУЩ: LazyLock<RapidHashSet<Вид_части_речи>> =
    LazyLock::new(|| RapidHashSet::from_iter([Вид_части_речи::Существительное]));
//
pub static ПУСТАЯ_КУЧА_ВИДОВ_СМЕШАННЫХ: LazyLock<
    RapidHashSet<Вид_Окончания>,
> = LazyLock::new(|| RapidHashSet::from_iter([]));
//
use stringzilla::sz;

pub fn sz_найти(строка: &String, образец: &str) -> bool {
    if let Some(_указатель) = sz::find(&строка, образец) {
        return true;
    }
    return false;
}
//
impl Раздел_Окончания {
    pub fn проверка_образцов_разделов_на_удвоенные_согласные(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок_не_указано: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        let mut куча_ошибок_не_указано_но_не_является: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            let проверка_на_удвоенность: bool =
                проверка_на_удвоенные_согласные_для_слова(
                    ячейка.строка,
                );
            if проверка_на_удвоенность && !ячейка.правописание.удвоенная_согласная
            {
                куча_ошибок_не_указано.insert(ячейка.строка);
            }
            if !проверка_на_удвоенность && ячейка.правописание.удвоенная_согласная
            {
                куча_ошибок_не_указано_но_не_является.insert(ячейка.строка);
            }
            //
        }
        //
        //
        if куча_ошибок_не_указано.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    "Удвоенная согласная в слове, в поле указано что не является таким"
                ))
                .cyan(),
                style(format!("в куче - ")),
            );
            for сообщение in куча_ошибок_не_указано.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        if куча_ошибок_не_указано_но_не_является.len() > 0 {
            print!(
            "\r\n{} {} {}",
            style(format!("Ошибка")).red(),
            style(format!(
                "Удвоенная согласная в слове, в поле указано , но на самом деле не является таким"
            ))
            .cyan(),
            style(format!("в куче - ")),
        );
            for сообщение in куча_ошибок_не_указано_но_не_является.iter()
            {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        return Ok(());
    }
    pub fn проверка_образцов_разделов_на_наречие_и_одновреенно_другими_частями_речи(
        &self,
    ) -> Result<(), ()> {
        use Возможноси_частей_речи;
        let mut куча_ошибок_одновременно_сущ_и_прил_либо_прич_нареч: rapidhash::fast::RapidHashSet<
            &str,
        > = rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if ячейка.части_речи.contains(&Вид_части_речи::Наречие)
                && ячейка.части_речи.содержит_кроме_наречия()
            //
            {
                куча_ошибок_одновременно_сущ_и_прил_либо_прич_нареч.insert(ячейка.строка);
            }
            //
        }
        //
        //
        if куча_ошибок_одновременно_сущ_и_прил_либо_прич_нареч.len() > 0
        {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    "Ячейка одновременно наречие и другая часть речи"
                ))
                .cyan(),
                style(format!("в куче - ")),
            );
            for сообщение in
                куча_ошибок_одновременно_сущ_и_прил_либо_прич_нареч.iter()
            {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        //
        return Ok(());
    }
    pub fn проверка_образцов_разделов_на_сущ_и_одновреенно_прич_либо_прил_и_нареч(
        &self,
    ) -> Result<(), ()> {
        use Возможноси_частей_речи;
        let mut куча_ошибок_одновременно_сущ_и_прил_либо_прич_нареч: rapidhash::fast::RapidHashSet<
            &str,
        > = rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if ячейка.части_речи.contains(&Вид_части_речи::Существительное)
                && ячейка.части_речи.содержит_кроме_существительного()
            /*(ячейка.части_речи.contains(&Вид_части_речи::Прилагательное)
            || ячейка.части_речи.contains(&Вид_части_речи::Причастие)
            || ячейка.части_речи.contains(&Вид_части_речи::Наречие))*/
            //
            {
                куча_ошибок_одновременно_сущ_и_прил_либо_прич_нареч.insert(ячейка.строка);
            }
            //
        }
        //
        //
        if куча_ошибок_одновременно_сущ_и_прил_либо_прич_нареч.len() > 0
        {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    "Ячейка одновременно существительное и другая часть речи"
                ))
                .cyan(),
                style(format!("в куче - ")),
            );
            for сообщение in
                куча_ошибок_одновременно_сущ_и_прил_либо_прич_нареч.iter()
            {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        //
        return Ok(());
    }
    pub fn проверка_образцов_разделов_на_удвоенные_гласные(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок_не_указано: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        let mut куча_ошибок_не_указано_но_не_является: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            let проверка_на_удвоенность: bool =
                проверка_на_удвоенные_гласные_для_слова(
                    ячейка.строка,
                );
            if проверка_на_удвоенность && !ячейка.правописание.удвоенная_гласная
            {
                куча_ошибок_не_указано.insert(ячейка.строка);
            }
            if !проверка_на_удвоенность && ячейка.правописание.удвоенная_гласная
            {
                куча_ошибок_не_указано_но_не_является.insert(ячейка.строка);
            }
            //
        }
        //
        //
        if куча_ошибок_не_указано.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    "Удвоенная гласная в слове, в поле указано что не является таким"
                ))
                .cyan(),
                style(format!("в куче - ")),
            );
            for сообщение in куча_ошибок_не_указано.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        if куча_ошибок_не_указано_но_не_является.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    "Удвоенная гласная в слове, в поле указано , но на самом деле не является таким"
                ))
                .cyan(),
                style(format!("в куче - ")),
            );
            for сообщение in куча_ошибок_не_указано_но_не_является.iter()
            {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        return Ok(());
    }
    //
    pub fn проверка_соответствия_наличия_строки_в_re_заменах(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        'главный_указатель: for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if sz_найти(&ячейка.re_замены.to_string(), ячейка.строка) {
                continue 'главный_указатель;
            } else {
                куча_ошибок.insert(ячейка.строка);
            }
            //
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("окончание не найдено в re_замены ")).blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }

    pub fn проверка_соответствия_re_замены_на_окончание(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        static ОБРАЗЕЦ_ПОИСКА_В_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"\(\?i\)\("#).unwrap());
        //
        for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if ОБРАЗЕЦ_ПОИСКА_В_RE.is_match(ячейка.re_замены.as_str()) {
                // if sz_найти_str(ячейка.re_замены.as_str(), r#"(?i)("#) {
                continue;
            } else {
                куча_ошибок.insert(ячейка.re_замены.as_str());
            }
            //
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!(
                    r##"окончание в re_замены содержит не верный образец re (?i)( "##
                ))
                .blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    pub fn проверка_соответствия_re_поиска_на_полное_слово(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        static ОБРАЗЕЦ_ПОИСКА_В_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"\\w\+"#).unwrap());
        //
        for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if ОБРАЗЕЦ_ПОИСКА_В_RE.is_match(ячейка.re_поиска.as_str()) {
                //if sz_найти_str(ячейка.re_замены.as_str(), r#"\w+"#) {
                continue;
            } else {
                куча_ошибок.insert(ячейка.re_поиска.as_str());
            }
            //
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!(r##"окончание в re_поиска не содержит \w+ "##)).blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    pub fn проверка_соответствия_re_замены_строке(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        'главный_указатель: for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if ячейка.re_замены.is_match(&ячейка.строка) {
                continue 'главный_указатель;
            } else {
                куча_ошибок.insert(ячейка.строка);
            }
            //
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("окончание в re_замены не сошлось с образцом ")).blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    pub fn проверка_соответствия_наличия_строки_в_re_поиска(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        'главный_указатель: for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if sz_найти(&ячейка.re_поиска.to_string(), ячейка.строка) {
                continue 'главный_указатель;
            } else {
                куча_ошибок.insert(ячейка.строка);
            }
            //
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("окончание не найдено в re_поиска  ")).blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    pub fn проверка_соответствия_видов_окончаний(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        'главный_указатель: for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if self.вид_окончания == ячейка.вид_окончания {
                continue 'главный_указатель;
            }
            //
            куча_ошибок.insert(ячейка.строка);
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!(
                    "Вид окончания раздела различается с окончанием ряде  "
                ))
                .blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    pub fn проверка_кучи_окончаний_в_ячейках_ряда(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        'главный_указатель: for слово_из_кучи in self.куча_окончаний_полная.iter()
        {
            for ячейка in self.re_ячейки.iter() {
                //совпадает ли окончание
                if ячейка.строка == *слово_из_кучи {
                    continue 'главный_указатель;
                }
                //
            }
            куча_ошибок.insert(слово_из_кучи);
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("Окончания из кучи нет в ряде  ")).blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    pub fn проверка_образцов_с_кучей_полной(&self) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for ячейка in self.re_ячейки.iter() {
            if !self.куча_окончаний_поиска.contains(ячейка.строка) {
                куча_ошибок.insert(ячейка.строка);
            }
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}",
                style(format!("Ошибка ")).red(),
                style(format!(
                    "Нет искомого окончания из RE образцов  в куче полной"
                ))
                .blue(),
                // style(format!("")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    pub fn проверка_кучи_поиска_с_кучей_полной(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        'главный_указатель: for ячейка in self.куча_окончаний_поиска.iter()
        {
            //совпадает ли окончание
            if self.куча_окончаний_полная.contains(ячейка) {
                continue 'главный_указатель;
            }
            //
            куча_ошибок.insert(ячейка);
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}",
                style(format!("Ошибка ")).red(),
                style(format!(
                    "Нет искомого окончания из кучи поиска  в куче полной"
                ))
                .blue(),
                // style(format!("")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    pub fn проверка_ячеек_ряда_в_куче_окончаний(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок_кучи_полной: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        'главный_указатель: for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if self.куча_окончаний_полная.contains(&ячейка.строка) {
                continue 'главный_указатель;
            }
            //
            куча_ошибок_кучи_полной.insert(ячейка.строка);
            //совпадает ли окончание
        }
        //
        if куча_ошибок_кучи_полной.len() > 0 {
            print!(
                "\r\n{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("Нет искомого окончания  в куче ")).blue(),
                // style(format!("")),
            );
            for сообщение in куча_ошибок_кучи_полной.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    pub fn проверка_повтором_ячеек_ряда(&self) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for (указатель_1, ячейка1) in self.re_ячейки.iter().enumerate() {
            //совпадает ли окончание
            for (_, ячейка2) in self
                .re_ячейки
                .iter()
                .enumerate()
                .filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                if ячейка1.строка == ячейка2.строка {
                    куча_ошибок.insert(ячейка1.строка);
                }
            }

            //
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{} {} ",
                style(format!("Ошибка ")).red(),
                style(format!("Повтор строки в ряде")).blue(),
                // style(format!("")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
}
pub fn sz_найти_str(срез: &str, образец: &str) -> bool {
    if let Some(_указатель) = sz::find(срез, образец) {
        return true;
    }
    return false;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Вид_части_речи {
    Существительное,
    Прилагательное,
    Причастие,
    Числительное,
    Местоимение,
    Глагол,
    Наречие,
    Деепричастие,
    Не_определено,
    Предлоги,
    Междометия,
    Звуко_подражания,
    Послеслоги,
    Предикативы,
    Частицы,
    Союзы,
}

impl Вид_части_речи {
    pub fn сравнение_двух_частей_на_допустимость(
        образец_1: &Self,
        образец_2: &Self,
    ) -> bool {
        //return true;
        //
        if образец_1 == образец_2 {
            return true;
        }
        //
        if Self::прич_либо_прил_с_сущ(образец_1, образец_2) {
            return false;
        }
        if Self::прич_либо_прил(образец_1, образец_2) {
            return true;
        }

        false
    }
    pub fn прич_либо_прил_с_сущ(
        образец_1: &Self, образец_2: &Self
    ) -> bool {
        //
        if (образец_1.существительное() && образец_2.причастие_или_прилагательное_одно())
            || (образец_2.существительное() && образец_1.причастие_или_прилагательное_одно())
        {
            return true;
        }
        false
    }
    //
    pub fn прич_либо_прил(образец_1: &Self, образец_2: &Self) -> bool {
        //
        if образец_2.причастие_или_прилагательное_одно()
            && образец_1.причастие_или_прилагательное_одно()
        {
            return true;
        }
        false
    }

    pub fn причастие_или_прилагательное_одно(&self) -> bool {
        match self {
            Вид_части_речи::Причастие => true,
            Вид_части_речи::Прилагательное => true,
            _ => false,
        }
    }

    pub fn существительное(&self) -> bool {
        match self {
            Вид_части_речи::Существительное => true,
            _ => false,
        }
    }
    pub fn прилагательное(&self) -> bool {
        matches!(self, Self::Прилагательное)
    }

    pub fn причастие(&self) -> bool {
        matches!(self, Self::Причастие)
    }

    pub fn глагол(&self) -> bool {
        matches!(self, Self::Глагол)
    }

    /// Прилагательное или причастие — общая категория «определение».
    pub fn определение(&self) -> bool {
        matches!(self, Self::Прилагательное | Self::Причастие)
    }

    /// Не_определено — «не знаю», совместимо со всем.
    pub fn не_определено(&self) -> bool {
        matches!(self, Self::Не_определено)
    }
}
/*pub mod ui_gpui;
pub use ui_gpui::gpui_main::Данные_при_загрузке;*/
pub trait Возможноси_частей_речи {
    fn вывести_в_строку(&self) -> String;
    fn допустима_ли_покупка(
        образец_1: &Self,
        образец_2: &Self,
        данные_при_загрузке: &Данные_при_загрузке_lib,
    ) -> bool;
    fn содержит_кроме_наречия(&self) -> bool;
    fn содержит_кроме_существительного(&self) -> bool;
}
impl Возможноси_частей_речи for [Вид_части_речи] {
    //
    fn содержит_кроме_наречия(&self) -> bool {
        for ячейка in self.iter() {
            match ячейка {
                Вид_части_речи::Наречие => continue,
                _ => return true,
            }
        }
        false
    }
    fn содержит_кроме_существительного(&self) -> bool {
        for ячейка in self.iter() {
            match ячейка {
                Вид_части_речи::Существительное => continue,
                _ => return true,
            }
        }
        false
    }
    //
    fn вывести_в_строку(&self) -> String {
        let mut строка: String = String::new();
        //
        if self.len() == 1 {
            return format!("{}", self[0]);
        }
        //
        for ячейка in self.iter() {
            строка = format!("{}|{}|", строка, ячейка);
        }
        строка
    }
    fn допустима_ли_покупка(
        образец_1: &Self,
        образец_2: &Self,
        данные_при_загрузке: &Данные_при_загрузке_lib,
    ) -> bool {
        //let mut куча_условий: RapidHashSet<bool> = rapidhash::fast::RapidHashSet::default();
        //
        for ячейка_1 in образец_1.iter() {
            //
            for ячейка_2 in образец_2.iter() {
                //
                if Вид_части_речи::сравнение_двух_частей_на_допустимость(ячейка_1, ячейка_2)  {
                   // println!("\tистина - спех");
                    return true}
                //if ячейка_1.
            }
        }
        /*println!(
            "Неуспех при 1 |{}| 2 |{}|",
            образец_1.вывести_в_строку(),
            образец_2.вывести_в_строку()
        );*/
        //
        false
    }
}
#[derive(Debug, Clone, Default)]
pub struct Буквы_Слова_Словаря {
    pub удвоенная_согласная: bool,
    pub удвоенная_гласная: bool,
}
#[derive(Debug, Clone, Default)]
pub struct Буквы_Слова_Торжка {
    pub удвоенная_согласная: bool,
    pub удвоенная_гласная: bool,
    pub единственный_образец_в_разделе: bool,
    pub множество_образцов_в_разделе: bool,
    pub обмен_на_двойную_согласную: bool,
    pub обмен_на_двойную_гласную: bool,
}
impl Буквы_Слова_Торжка {
    pub fn допустимость_согласных_и_гласных(
        ячейка_торжка: &Self,
        образец_раздела: &Буквы_Слова_Словаря,
        данные_при_загрузке: &Данные_при_загрузке_lib,
    ) -> bool {
        //return true;
        //
        if ячейка_торжка.удвоенная_гласная == образец_раздела.удвоенная_гласная
            && ячейка_торжка.удвоенная_согласная == образец_раздела.удвоенная_согласная
        {
            if данные_при_загрузке.общий_ход {
                println!(
                    "{}",
                    style(format!("допустимость_согласных_и_гласных - 1")).green(),
                );
            };

            return true;
        }

        //
        if образец_раздела.удвоенная_согласная {
            //удвоенная согласная обмен
            if ячейка_торжка.обмен_на_двойную_согласную == образец_раздела.удвоенная_согласная
                || ячейка_торжка.обмен_на_двойную_согласную == образец_раздела.удвоенная_согласная
            {
                if данные_при_загрузке.общий_ход {
                    println!(
                        "{}",
                        style(format!("допустимость_согласных_и_гласных - 2")).yellow(),
                    );
                };
                return true;
            }
        }

        //удвоенная гласная обмен
        if образец_раздела.удвоенная_гласная {
            if ячейка_торжка.обмен_на_двойную_гласную == образец_раздела.удвоенная_гласная
                || ячейка_торжка.обмен_на_двойную_гласную == образец_раздела.удвоенная_гласная
            {
                if данные_при_загрузке.общий_ход {
                    println!(
                        "{}",
                        style(format!("допустимость_согласных_и_гласных - 3")).cyan(),
                    );
                };
                return true;
            }
        }

        /*//удвоенная согласная обмен
        if образец_1.обмен_на_двойную_согласную == образец_2.удвоенная_согласная
        {
            return true;
        }

        //удвоенная гласная обмен
        if образец_1.обмен_на_двойную_гласную == образец_2.удвоенная_гласная
        {
            return true;
        }*/
        //инае возврат
        false
    }
}
#[derive(Debug, Clone, Default)]
pub struct Особенности_образца_RE {
    pub исключить_участие_в_поиске: bool,
}
#[derive(Debug, Clone, Default)]
pub struct Особенности_образца_Торжка {
    pub возрат_повтор: bool,
    pub может_быть_в_нескольких_разделах: bool,
}
#[derive(Debug, Clone)]
pub struct Вид_Окончания_со_строкой {
    pub строка: &'static str,
    pub re_выдер: Regex,
    pub правописание: Буквы_Слова_Торжка,
    pub вид_окончания: Вид_Окончания,
    //pub часть_речи: Вид_части_речи,
    pub особенности_образца: Особенности_образца_Торжка,
}
// Ручная реализация PartialEq из-за типа Regex
impl PartialEq for Вид_Окончания_со_строкой {
    fn eq(&self, other: &Self) -> bool {
        self.строка == other.строка
            && self.вид_окончания == other.вид_окончания
            //&& self.часть_речи == other.часть_речи
            // Сравниваем регулярные выражения по их текстовому шаблону
            && self.re_выдер.as_str() == other.re_выдер.as_str()
    }
}
impl Eq for Вид_Окончания_со_строкой {}
impl std::fmt::Display for Вид_части_речи {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Вид_части_речи::Союзы => "Союзы",
            Вид_части_речи::Частицы => "Частицы",
            Вид_части_речи::Послеслоги => "Послеслоги",
            Вид_части_речи::Предикативы => "Предикативы",
            Вид_части_речи::Звуко_подражания => "Звуко_подражания",
            Вид_части_речи::Междометия => "Междометия",
            Вид_части_речи::Предлоги => "Предлоги",
            Вид_части_речи::Существительное => "Существительное",
            Вид_части_речи::Деепричастие => "Деепричастие",
            Вид_части_речи::Причастие => "Причастие",
            Вид_части_речи::Прилагательное => "Прилагательное",
            Вид_части_речи::Числительное => "Числительное",
            Вид_части_речи::Местоимение => "Местоимение",
            Вид_части_речи::Глагол => "Глагол",
            Вид_части_речи::Наречие => "Наречие",
            Вид_части_речи::Не_определено => "Не_определено",
        };
        f.write_str(s)
    }
}
impl Default for Вид_Окончания_со_строкой {
    fn default() -> Self {
        Self {
            строка: "",
            re_выдер: Regex::new(r"").unwrap(),
            вид_окончания: Вид_Окончания::Не_определено,
            //часть_речи: Вид_части_речи::Не_определено,
            правописание: Default::default(),
            особенности_образца: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Раздел_Торжка {
    pub ряд: Vec<Вид_Окончания_со_строкой>,
    // pub вторая_очередь: Vec<Вид_Окончания_со_строкой>,
    pub куча_окончаний: rapidhash::fast::RapidHashSet<&'static str>,

    pub окончания_исключения: rapidhash::fast::RapidHashSet<Вид_Окончания>,
}
pub trait Возможности_Торжка {
    fn проверка_на_повтор_окончаний(&self) -> Result<(), ()>;

    fn достать_соседнее_окончание(
        &self,
        выдер_из_торжка: &Вид_Окончания_со_строкой,
    ) -> Result<Вид_Окончания_со_строкой, String>;
    fn в_одном_ли_разделе_окончания(
        &self,
        выдер_из_торжка: &Вид_Окончания_со_строкой,
        re_выдер: &RE_полное_окончание,
    ) -> bool;
}
//
impl Возможности_Торжка for [Раздел_Торжка] {
    fn в_одном_ли_разделе_окончания(
        &self,
        выдер_из_торжка: &Вид_Окончания_со_строкой,
        re_выдер: &RE_полное_окончание,
    ) -> bool {
        for раздел in self.iter() {
            if раздел.куча_окончаний.contains(&выдер_из_торжка.строка)
                && раздел.куча_окончаний.contains(&re_выдер.строка)
            {
                return true;
            }
        }

        false
    }
    fn достать_соседнее_окончание(
        &self,
        выдер_из_торжка: &Вид_Окончания_со_строкой,
    ) -> Result<Вид_Окончания_со_строкой, String> {
        if выдер_из_торжка.правописание.единственный_образец_в_разделе
        {
            return Err(format!(
                "Образец в торжке |{}| вид окончания |{}| не имеет других случаев для выдера",
                выдер_из_торжка.строка, выдер_из_торжка.вид_окончания
            ));
        }
        //
        for раздел in self.iter()
        // .filter(|(указатель_1,_)|указатель_1!=)
        {
            if раздел.куча_окончаний.contains(выдер_из_торжка.строка)
            {
                //
                for ячейка in раздел.ряд.iter() {
                    if ячейка.вид_окончания == выдер_из_торжка.вид_окончания
                        && ячейка.строка != выдер_из_торжка.строка
                    {
                        return Ok(ячейка.clone());
                    }
                }
                //
                /*for ячейка in раздел.вторая_очередь.iter() {
                    if ячейка.вид_окончания == выдер_из_торжка.вид_окончания
                        && ячейка.строка != выдер_из_торжка.строка
                    {
                        return Ok(ячейка.clone());
                    }
                }*/
                //
            }
        }
        return Err(format!(
            "Образец в торжке |{}| вид окончания |{}| не нашло нигде",
            выдер_из_торжка.строка, выдер_из_торжка.вид_окончания
        ));
    }
    //
    fn проверка_на_повтор_окончаний(&self) -> Result<(), ()> {
        //
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<String> =
            rapidhash::fast::RapidHashSet::default();
        //
        for (указатель_1, раздел_1) in self.iter().enumerate()
        // .filter(|(указатель_1,_)|указатель_1!=)
        {
            for (_указатель_2, раздел_2) in self
                .iter()
                .enumerate()
                .filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            {
                'круговорот_слова_кучи: for слово_кучи_1 in
                    раздел_1.куча_окончаний.iter()
                {
                    //
                    for слово_раздела in раздел_1.ряд.iter() {
                        //
                        if слово_раздела.строка == *слово_кучи_1 {
                            //
                            if слово_раздела
                                .особенности_образца
                                .может_быть_в_нескольких_разделах
                            {
                                continue 'круговорот_слова_кучи;
                            }
                        }
                    }
                    //
                    /*for слово_раздела in раздел_1.вторая_очередь.iter()
                    {
                        //
                        if слово_раздела.строка == *слово_кучи_1 {
                            //
                            if слово_раздела
                                .особенности_образца
                                .может_быть_в_нескольких_разделах
                            {
                                continue 'круговорот_слова_кучи;
                            }
                        }
                    }*/
                    //
                    if раздел_2.куча_окончаний.contains(слово_кучи_1) {
                        if !раздел_2.является_ли_окончание_множественным_в_разделе(
                            слово_кучи_1,
                        ) {
                            куча_ошибок.insert(слово_кучи_1.to_string());
                        }
                    }
                }
            }
        }

        //
        if куча_ошибок.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" В другом ряде Торжка ")).cyan(),
                style(format!(" содержится повтор искомого окончания -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}|", сообщение)).green(),);
            }
            println!();
            // return Err(());
        }
        return Ok(());
    }
}

impl Раздел_Торжка {
    pub fn является_ли_окончание_множественным_в_разделе(
        &self,
        окончание: &str,
    ) -> bool {
        //
        for ячейка in self.ряд.iter() {
            if ячейка.строка == окончание {
                if ячейка.правописание.множество_образцов_в_разделе
                {
                    return true;
                }
            }
        }
        /*for ячейка in self.вторая_очередь.iter() {
            if ячейка.строка == окончание {
                if ячейка.правописание.множество_образцов_в_разделе
                {
                    return true;
                }
            }
        }*/
        false
    }
    pub fn проверка_окончаний_на_недопустимость_ложного_указания_множественных_образцов(
        &self,
    ) -> Result<(), ()> {
        //
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        'главный_круговорот: for (указатель_1, ячейка_1) in
            self.ряд.iter().enumerate()
        {
            //
            if ячейка_1.правописание.множество_образцов_в_разделе
            {
                //
                for (_, ячейка_2) in self
                    .ряд
                    .iter()
                    .enumerate()
                    .filter(|(указатель_2, _)| *указатель_2 != указатель_1)
                {
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        continue 'главный_круговорот;
                    }
                }
                куча_ошибок.insert(ячейка_1.строка);
            }
        }
        //
        if куча_ошибок.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}",
                style(format!("Ошибка")).red(),
                style(format!(
                    " образец заявлен как множество в разделе - на самом деле он один"
                ))
                .cyan(),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}|", сообщение)).green(),);
            }
            println!();
            // return Err(());
        }
        return Ok(());
    }
    pub fn проверка_окончаний_на_недопустимость_не_указания_множественных_образцов(
        &self,
    ) -> Result<(), ()> {
        //
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        for (указатель_1, ячейка_1) in self.ряд.iter().enumerate() {
            //
            if ячейка_1.правописание.единственный_образец_в_разделе
            {
                //
                for (_, ячейка_2) in self
                    .ряд
                    .iter()
                    .enumerate()
                    .filter(|(указатель_2, _)| *указатель_2 != указатель_1)
                {
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        куча_ошибок.insert(ячейка_1.строка);
                        куча_ошибок.insert(ячейка_2.строка);
                    }
                }
            }
        }
        //
        if куча_ошибок.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}",
                style(format!("Ошибка")).red(),
                style(format!(
                    " образец заявлен как один в разделе - на самом деле не один"
                ))
                .cyan(),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}|", сообщение)).green(),);
            }
            println!();
            // return Err(());
        }
        return Ok(());
    }
    pub fn проверка_окончаний_на_недопустимость_одновременно_единственный_и_множественный_образец(
        &self,
    ) -> Result<(), ()> {
        //
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        for ячейка in self.ряд.iter() {
            if ячейка.правописание.единственный_образец_в_разделе
                && ячейка.правописание.множество_образцов_в_разделе
            {
                куча_ошибок.insert(ячейка.строка);
            }
        }
        //
        /*for ячейка in self.вторая_очередь.iter() {
            if ячейка.правописание.единственный_образец_в_разделе
                && ячейка.правописание.множество_образцов_в_разделе
            {
                куча_ошибок.insert(ячейка.строка);
            }
        }*/
        //
        if куча_ошибок.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" образец содержит недопустимость в право-писании")).cyan(),
                style(format!(
                    " является одновременно множественным и одиночным -"
                )),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}|", сообщение)).green(),);
            }
            println!();
            // return Err(());
        }
        return Ok(());
    }
    //
    pub fn много_ли_окончаний_в_разделе(
        &self,
        вид_окончания: &Вид_Окончания,
    ) -> usize {
        let mut счётчик: usize = 0;
        //
        for ячейка in self.ряд.iter() {
            if ячейка.вид_окончания == *вид_окончания {
                счётчик += 1;
            }
        }
        /*for ячейка in self.вторая_очередь.iter() {
            if ячейка.вид_окончания == *вид_окончания {
                счётчик += 1;
            }
        }*/
        //
        /*match счётчик > 1 {
            true => return  счётчик),
            false => return Err(()),
        }*/
        return счётчик;
    }
    //
    /*pub fn проверка_что_образец_re_соответствует_строке_вторичной_очереди(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<String> =
            rapidhash::fast::RapidHashSet::default();
        //
        if self.окончания_исключения.len() == 0 {
            return Ok(());
        }
        for ячейка in self.вторая_очередь.iter() {
            if !ячейка.re_выдер.is_match(ячейка.строка) {
                куча_ошибок.insert(ячейка.re_выдер.to_string());
            }
        }
        //
        if куча_ошибок.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" RE образец  вторичного ряда")).cyan(),
                style(format!(" не сходится с образцами строки -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}|", сообщение)).green(),);
            }
            println!();
            // return Err(());
        }
        return Ok(());
    }*/
    //
    pub fn проверка_что_образец_re_соответствует_строке_первичной_очереди(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<String> =
            rapidhash::fast::RapidHashSet::default();
        //
        if self.окончания_исключения.len() == 0 {
            return Ok(());
        }
        for ячейка in self.ряд.iter() {
            if !ячейка.re_выдер.is_match(ячейка.строка) {
                куча_ошибок.insert(ячейка.re_выдер.to_string());
            }
        }
        if куча_ошибок.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" RE образец  первичного ряда")).blue(),
                style(format!(" не сходится с образцами строки -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            //return Err(());
        }
        return Ok(());
    }
    pub fn проверка_соответствия_много_образцов_в_разделе(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        for (указатель_1, ячейка_1) in self.ряд.iter().enumerate() {
            //совпадает ли окончание
            if ячейка_1.правописание.множество_образцов_в_разделе
            {
                let mut счётчик: usize = 0;
                //первая очередь
                for (_, ячейка_2) in self
                    .ряд
                    .iter()
                    .enumerate()
                    .filter(|(указатель_2, _)| указатель_1 != *указатель_2)
                {
                    //
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        счётчик += 1;
                    }
                }
                //вторая очередь
                /*for ячейка_2 in self.вторая_очередь.iter()
                //.filter(|(указатель_1, ячейка_2)| *указатель_1 != указатель_2)
                {
                    //
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        счётчик += 1;
                    }
                }*/
                //вложение окончания если счётчик меньше 2
                if счётчик < 1 {
                    куча_ошибок.insert(ячейка_1.строка);
                }
            }
            //
        }
        //проверка второй очереди
        /*for (указатель_1, ячейка_1) in self.вторая_очередь.iter().enumerate()
        {
            //совпадает ли окончание
            if ячейка_1.правописание.множество_образцов_в_разделе
            {
                let mut счётчик: usize = 0;
                //первая очередь
                for ячейка_2 in self.ряд.iter() {
                    //
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        счётчик += 1;
                    }
                }
                //вторая очередь
                for (_, ячейка_2) in self
                    .вторая_очередь
                    .iter()
                    .enumerate()
                    .filter(|(указатель_2, _)| указатель_1 != *указатель_2)
                {
                    //
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        счётчик += 1;
                    }
                }
                //вложение окончания если счётчик меньше 2
                if счётчик < 1 {
                    куча_ошибок.insert(ячейка_1.строка);
                }
            }
            //
        }*/
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    r##"окончания заявлены как множественные, на деле же - меньше 2 раз встречаются"##
                ))
                .blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    pub fn проверка_соответствия_вида_окончания_в_разделах(
        &self,
        разделы: &Разделы_окончаний_Словаря,
    ) -> Result<(), ()> {
        static ИСКЛЮЧЕНИЯ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
            LazyLock::new(|| {
                rapidhash::fast::RapidHashSet::from_iter([
                    "енна", "ена", "енны", "ены", "ён", "ёна", "ённа", "ёны",
                ])
            });
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        for ячейка in self.ряд.iter() {
            let ссылка = разделы
                .получить_ячейку_по_срезу_и_виду_окончания(
                    ячейка.строка,
                    &ячейка.вид_окончания,
                )
                .unwrap();
            if ссылка.вид_окончания != ячейка.вид_окончания {
                if !ИСКЛЮЧЕНИЯ.contains(ячейка.строка) {
                    куча_ошибок.insert(ячейка.строка);
                }
            }
        }
        //проверка второй очереди
        /*for ячейка in self.вторая_очередь.iter() {
            let ссылка = РАЗДЕЛЫ_ОКОНЧАНИЙ_ПОЛНЫЕ
                .получить_ячейку_по_срезу_и_виду_окончания(
                    ячейка.строка,
                    &ячейка.вид_окончания,
                )
                .unwrap();
            if ссылка.вид_окончания != ячейка.вид_окончания {
                if !ИСКЛЮЧЕНИЯ.contains(ячейка.строка) {
                    куча_ошибок.insert(ячейка.строка);
                }
            }
        }*/
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    r##"окончания Торжка не соответствуют виду окончаний из РАЗДЕЛОВ ПОЛНЫХ"##
                ))
                .blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    pub fn проверка_соответствия_один_образец_в_разделе(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        for (указатель_1, ячейка_1) in self.ряд.iter().enumerate() {
            //совпадает ли окончание
            if ячейка_1.правописание.единственный_образец_в_разделе
            {
                let mut счётчик: usize = 0;
                //первая очередь
                for (_, ячейка_2) in self
                    .ряд
                    .iter()
                    .enumerate()
                    .filter(|(указатель_2, _)| указатель_1 != *указатель_2)
                {
                    //
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        счётчик += 1;
                    }
                }
                //вторая очередь
                /*for ячейка_2 in self.вторая_очередь.iter()
                //.filter(|(указатель_1, ячейка_2)| *указатель_1 != указатель_2)
                {
                    //
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        счётчик += 1;
                    }
                }*/
                //вложение окончания если счётчик меньше 2
                if счётчик > 0 {
                    куча_ошибок.insert(ячейка_1.строка);
                }
            }
            //
        }
        //проверка второй очереди
        /*for (указатель_1, ячейка_1) in self.вторая_очередь.iter().enumerate()
        {
            //совпадает ли окончание
            if ячейка_1.правописание.единственный_образец_в_разделе
            {
                let mut счётчик: usize = 0;
                //первая очередь
                for ячейка_2 in self.ряд.iter() {
                    //
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        счётчик += 1;
                    }
                }
                //вторая очередь
                for (_, ячейка_2) in self
                    .вторая_очередь
                    .iter()
                    .enumerate()
                    .filter(|(указатель_2, _)| указатель_1 != *указатель_2)
                {
                    //
                    if ячейка_2.вид_окончания == ячейка_1.вид_окончания
                    {
                        счётчик += 1;
                    }
                }
                //вложение окончания если счётчик меньше 2
                if счётчик > 0 {
                    куча_ошибок.insert(ячейка_1.строка);
                }
            }
            //
        }*/
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка ")).red(),
                style(format!(
                    r##"окончания заявлены как единсвенные, на деле же их больше 1 раза встречаются"##
                ))
                .blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    pub fn проверка_соответствия_re_выдера_первой_очереди_на_образец(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        static ОБРАЗЕЦ_ПОИСКА_В_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"\(\?i\)\("#).unwrap());
        //
        for ячейка in self.ряд.iter() {
            //совпадает ли окончание
            if ОБРАЗЕЦ_ПОИСКА_В_RE.is_match(ячейка.re_выдер.as_str()) {
                continue;
            } else {
                куча_ошибок.insert(ячейка.re_выдер.as_str());
            }
            //
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка ")).red(),
                style(format!(
                    r##"окончание в re_замены [первая очередь] содержит не верный образец re (?i)( "##
                ))
                .blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    /*pub fn проверка_что_окончания_вторичного_ряда_не_содержаться_в_исключениях(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&Вид_Окончания> =
            rapidhash::fast::RapidHashSet::default();
        if self.окончания_исключения.len() == 0 {
            return Ok(());
        }
        for ячейка in self.вторая_очередь.iter() {
            if self.окончания_исключения.contains(&ячейка.вид_окончания)
            {
                куча_ошибок.insert(&ячейка.вид_окончания);
            }
        }
        if куча_ошибок.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}{}{} {}",
                style(format!("Ошибка")).red(),
                style(format!(" Окончания  вторичного ряда ")).blue(),
                style(format!(
                    " есть в исключениях , первое окончание для поиска раздела - "
                )),
                style(format!("|{}|", self.вторая_очередь[0].строка)).cyan(),
                style(format!("перечень исключений")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            // return Err(());
        }
        return Ok(());
    }*/
    //
    pub fn проверка_что_окончания_внутри_не_пересекаются(
        &self,
    ) -> Result<(), ()> {
        //
        let mut куча_ошибок_строка: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        let mut куча_ошибок_re_выдер: rapidhash::fast::RapidHashSet<String> =
            rapidhash::fast::RapidHashSet::default();
        let mut куча_ошибок_вид_окончания: rapidhash::fast::RapidHashSet<&Вид_Окончания> =
            rapidhash::fast::RapidHashSet::default();
        //
        /*if self.окончания_исключения.len() == 0 {
            return Ok(());
        }*/
        //
        /*for (указатель_1, ячейка) in self
            .вторая_очередь
            .iter()
            .enumerate()
            .filter(|(_, ячейка)| !ячейка.особенности_образца.возрат_повтор)
        {
            //сначала с первой очередью
            for (указатель_2, ячейка_2) in self
                .вторая_очередь
                .iter()
                .enumerate()
                .filter(|(_, ячейка)| !ячейка.особенности_образца.возрат_повтор)
            //.filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                if указатель_1 == указатель_2 {
                    continue;
                }
                if ячейка.строка == ячейка_2.строка {
                    /*if ячейка.строка == "то" {
                        println!("Вошло ТО |{}| вид №1", ячейка.вид_окончания);
                        println!("Вошло ТО |{}| вид №2", ячейка.вид_окончания);
                    }*/
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_строка.insert(&ячейка.строка);
                    }
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                    }
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                    }
                }
            }
            //сначала с первой очередью
            for (_, ячейка_2) in self
                .ряд
                .iter()
                .enumerate()
                .filter(|(_, ячейка)| !ячейка.особенности_образца.возрат_повтор)
            //.filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                if ячейка.строка == ячейка_2.строка {
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_строка.insert(&ячейка.строка);
                    }
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str()
                    {
                        if !ён_ен_ли(&ячейка, &ячейка_2) {
                            куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                        }
                    }
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    //
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                    }
                    //
                }
            }
            //
        }*/
        //
        for (указатель_1, ячейка) in self
            .ряд
            .iter()
            .enumerate()
            .filter(|(_, ячейка)| !ячейка.особенности_образца.возрат_повтор)
        {
            //сначала с первой очередью
            for (указатель_2, ячейка_2) in self
                .ряд
                .iter()
                .enumerate()
                .filter(|(_, ячейка)| !ячейка.особенности_образца.возрат_повтор)
            // .filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                /*if ячейка.строка == "то" {
                    println!("Вошло ТО |{}| вид №1", ячейка.вид_окончания);
                    println!("Вошло ТО |{}| вид №2", ячейка.вид_окончания);
                }*/
                if указатель_1 == указатель_2 {
                    continue;
                }
                if ячейка.строка == ячейка_2.строка {
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_строка.insert(&ячейка.строка);
                    }
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                    }
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                    }
                }
            }
            //сначала с первой очередью
            /*for (_, ячейка_2) in self
                .вторая_очередь
                .iter()
                .enumerate()
                .filter(|(_, ячейка)| !ячейка.особенности_образца.возрат_повтор)
            //.filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                if ячейка.строка == ячейка_2.строка {
                    //
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_строка.insert(&ячейка.строка);
                    }
                    //
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    //
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                    }
                    //
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    //
                    if !ён_ен_ли(&ячейка, &ячейка_2) {
                        куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                    }
                    //
                }
            }*/
            //
        }
        //
        if куча_ошибок_строка.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" строки внутри раздела совпадают ")).blue(),
                //  style(format!("|{}|", self.первая_очередь[0].строка)).cyan(),
            );
            for сообщение in куча_ошибок_строка.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            //return Err(());
        }
        //
        if куча_ошибок_re_выдер.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" re_выдер внутри раздела совпадают ")).blue(),
                // style(format!("|{}|", self.первая_очередь[0].строка)).cyan(),
            );
            for сообщение in куча_ошибок_строка.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            //return Err(());
        }
        //
        if куча_ошибок_вид_окончания.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" виды окончания внутри раздела совпадают ")).blue(),
                // style(format!("|{}|", self.первая_очередь[0].строка)).cyan(),
            );
            for сообщение in куча_ошибок_строка.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            println!();
            //return Err(());
        }
        return Ok(());
        //
        fn ён_ен_ли(
            ячейка_1: &Вид_Окончания_со_строкой,
            ячейка_2: &Вид_Окончания_со_строкой,
        ) -> bool {
            if ячейка_1.вид_окончания.ен_ён() && ячейка_2.вид_окончания.ен_ён()
            {
                true
            } else {
                false
            }
        }
    }
    //
    pub fn проверка_что_окончания_первичного_ряда_не_содержаться_в_исключениях(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&Вид_Окончания> =
            rapidhash::fast::RapidHashSet::default();
        if self.окончания_исключения.len() == 0 {
            return Ok(());
        }
        //
        for ячейка in self.ряд.iter() {
            if self.окончания_исключения.contains(&ячейка.вид_окончания)
            {
                куча_ошибок.insert(&ячейка.вид_окончания);
            }
        }
        if куча_ошибок.is_empty() {
            return Ok(());
        } else {
            print!(
                "\r\n{}{}{}{} {}",
                style(format!("Ошибка")).red(),
                style(format!(" Окончания  первичного ряда ")).blue(),
                style(format!(
                    " есть в исключениях , первое окончание для поиска раздела  "
                )),
                style(format!("|{}|", self.ряд[0].строка)).cyan(),
                style(format!("перечень исключений")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            //println!();
            //return Err(());
        }
        return Ok(());
    }
    pub fn проверка_образцов_торжка_на_удвоенные_согласные(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок_не_указано: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        let mut куча_ошибок_не_указано_но_не_является: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for ячейка in self.ряд.iter() {
            //совпадает ли окончание
            let проверка_на_удвоенность: bool =
                проверка_на_удвоенные_согласные_для_слова(
                    ячейка.строка,
                );
            if проверка_на_удвоенность && !ячейка.правописание.удвоенная_согласная
            {
                куча_ошибок_не_указано.insert(ячейка.строка);
            }
            if !проверка_на_удвоенность && ячейка.правописание.удвоенная_согласная
            {
                куча_ошибок_не_указано_но_не_является.insert(ячейка.строка);
            }
            //
        }
        //
        /*for ячейка in self.вторая_очередь.iter() {
            //совпадает ли окончание
            let проверка_на_удвоенность: bool =
                проверка_на_удвоенные_согласные_для_слова(
                    ячейка.строка,
                );
            if проверка_на_удвоенность && !ячейка.правописание.удвоенная_согласная
            {
                куча_ошибок_не_указано.insert(ячейка.строка);
            }
            if !проверка_на_удвоенность && ячейка.правописание.удвоенная_согласная
            {
                куча_ошибок_не_указано_но_не_является.insert(ячейка.строка);
            }
            //
        }*/
        //
        if куча_ошибок_не_указано.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    "Удвоенная согласная в слове, в поле указано что не является таким"
                ))
                .cyan(),
                style(format!("в куче - ")),
            );
            for сообщение in куча_ошибок_не_указано.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        if куча_ошибок_не_указано_но_не_является.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    "Удвоенная согласная в слове, в поле указано , но на самом деле не является таким"
                ))
                .cyan(),
                style(format!("в куче - ")),
            );
            for сообщение in куча_ошибок_не_указано_но_не_является.iter()
            {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        return Ok(());
    }
    pub fn проверка_образцов_торжка_на_удвоенные_гласные(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок_не_указано: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        let mut куча_ошибок_не_указано_но_не_является: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        for ячейка in self.ряд.iter() {
            //совпадает ли окончание
            let проверка_на_удвоенность: bool =
                проверка_на_удвоенные_гласные_для_слова(
                    ячейка.строка,
                );
            if проверка_на_удвоенность && !ячейка.правописание.удвоенная_гласная
            {
                куча_ошибок_не_указано.insert(ячейка.строка);
            }
            if !проверка_на_удвоенность && ячейка.правописание.удвоенная_гласная
            {
                куча_ошибок_не_указано_но_не_является.insert(ячейка.строка);
            }
            //
        }
        //
        /*for ячейка in self.вторая_очередь.iter() {
            //совпадает ли окончание
            let проверка_на_удвоенность: bool =
                проверка_на_удвоенные_гласные_для_слова(
                    ячейка.строка,
                );
            if проверка_на_удвоенность && !ячейка.правописание.удвоенная_гласная
            {
                куча_ошибок_не_указано.insert(ячейка.строка);
            }
            if !проверка_на_удвоенность && ячейка.правописание.удвоенная_гласная
            {
                куча_ошибок_не_указано_но_не_является.insert(ячейка.строка);
            }
            //
        }*/
        //
        if куча_ошибок_не_указано.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    "Удвоенная гласная в слове, в поле указано что не является таким"
                ))
                .cyan(),
                style(format!("в куче - ")),
            );
            for сообщение in куча_ошибок_не_указано.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        if куча_ошибок_не_указано_но_не_является.len() > 0 {
            print!(
                "\r\n{} {} {}",
                style(format!("Ошибка")).red(),
                style(format!(
                    "Удвоенная гласная в слове, в поле указано , но на самом деле не является таким"
                ))
                .cyan(),
                style(format!("в куче - ")),
            );
            for сообщение in куча_ошибок_не_указано_но_не_является.iter()
            {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        }
        //
        return Ok(());
    }
    //
    pub fn проверка_первой_очереди_в_куче_окончаний(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        'главный_указатель: for раздел in self.ряд.iter() {
            //совпадает ли окончание
            if self.куча_окончаний.contains(&раздел.строка) {
                continue 'главный_указатель;
            }

            //
            куча_ошибок.insert(раздел.строка);
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" Нет искомого окончания  первичного ряда")).cyan(),
                style(format!(" в куче - ")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    /*pub fn проверка_второй_очереди_в_куче_окончаний(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        'главный_указатель: for раздел in self.вторая_очередь.iter()
        {
            //совпадает ли окончание
            if self.куча_окончаний.contains(&раздел.строка) {
                continue 'главный_указатель;
            }

            //
            куча_ошибок.insert(раздел.строка);
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("Нет искомого окончания  вторичного ряда ")).blue(),
                style(format!("в куче -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }*/
    //
    pub fn проверка_кучи_окончаний_в_рядах(&self) -> Result<(), ()> {
        //
        pub static ИСКЛЮЧЕНИЯ_ЁН: LazyLock<RapidHashSet<&'static str>> =
            LazyLock::new(|| RapidHashSet::from_iter(["ённа", "ёна", "ёны", "ённы"]));
        //
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        'главный_указатель: for образец_из_кучи in self.куча_окончаний.iter()
        {
            //перебор раздела
            for раздел in self.ряд.iter() {
                //совпадает ли окончание
                if раздел.строка == *образец_из_кучи {
                    continue 'главный_указатель;
                }
            }
            //перебор раздела
            /*for раздел in self.вторая_очередь.iter() {
                //совпадает ли окончание
                if раздел.строка == *образец_из_кучи {
                    continue 'главный_указатель;
                }
            }*/
            //исключения - которые объявлены, но нет в ряде не включаем. Ённа объявлена - но возвращается вместо неё енна
            if !ИСКЛЮЧЕНИЯ_ЁН.contains(образец_из_кучи) {
                куча_ошибок.insert(образец_из_кучи);
            }
        }
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("Нет искомого окончания из кучи ")).blue(),
                style(format!("в рядах -")),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            //return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    /*pub fn проверка_кучи_окончаний_со_второй_очередью(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        'главный_указатель: for образец_из_кучи in self.куча_окончаний.iter()
        {
            //перебор раздела
            for раздел in self.первая_очередь.iter() {
                //совпадает ли окончание
                if раздел.строка == *образец_из_кучи {
                    continue 'главный_указатель;
                }
            }
            //
            куча_ошибок.insert(образец_из_кучи);
        }
        //
        if куча_ошибок.len() > 0 {
            println!(
                "{}{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" Нет искомого окончания из кучи")).cyan(),
                style(format!("во вторичном ряде")),
            );
            for сообщение in куча_ошибок.iter() {
                println!("{}", style(format!("|{}|", сообщение)).green(),);
            }
            return Err(());
        } else {
            return Ok(());
        }
    }*/
}

#[derive(Debug, Clone)]
pub struct Исключение_поиска_выдера {
    pub вид_окончания: Вид_Окончания,
    pub куча_окончаний: rapidhash::fast::RapidHashSet<&'static str>,
    pub куча_обрезков_слов: rapidhash::fast::RapidHashSet<&'static str>,
}
/*
#[derive(Debug, Clone)]
pub struct Исключения_поиска_выдера {
    pub ван: Vec<Исключение_поиска_выдера>,
}

impl Исключения_поиска_выдера {
    pub fn есть_ли_в_исключениях(
        &self,
        обрезок_слова_замены: &str,
        образец_окончания_неруского_слова: &str,
        искомое_слово_с_окончанием: &String,
    ) -> bool {
        let вид_окончания: Вид_Окончания =
            Вид_Окончания::получить_вид_окончания_по_строке(
                образец_окончания_неруского_слова,
            )
            .unwrap();
        true
        //
        for раздел in РАЗДЕЛЫ_ОКОНЧАНИЙ_ПОЛНЫЕ.iter() {
           if раздел.куча.contains(образец_окончания_неруского_слова)
        }
        //
        match вид_окончания {
            Вид_Окончания::Ван => self
                .есть_ли_в_ван(обрезок_слова_замены, образец_окончания_неруского_слова),
            _ => panic!(
                "ошибка при попытке определить окончания, где обрезок_слова_замены |{}| образец_окончания |{}| искомое_слово_с_окончанием |{}|",
                обрезок_слова_замены, образец_окончания_неруского_слова, искомое_слово_с_окончанием
            ), //for раздел in self.iter() {}
        }
        //
    }
    pub fn есть_ли_в_ван(
        &self, обрезок_слова: &str, образец_окончания: &str
    ) -> bool {
        let вид_окончания: Вид_Окончания =
            Вид_Окончания::получить_вид_окончания_по_строке(
                образец_окончания,
            )
            .unwrap();
        //
        match вид_окончания {
            Вид_Окончания::Ван => (),
            _ => panic!(
                "Ван = ошибка при |{}| образец_окончания |{}|",
                обрезок_слова, образец_окончания
            ), //for раздел in self.iter() {}
        };
        //перебор в ВАН
        for раздел in self.ван.iter() {
            //если есть обрезок слова и само окончания
            if раздел.куча_обрезков_слов.contains(обрезок_слова)
                && раздел.куча_окончаний.contains(образец_окончания)
            {
                return true;
            }
        }
        //
        false
    }
}*/
pub fn проверка_на_удвоенные_согласные_для_слова(
    слово: &str,
) -> bool {
    use rapidhash::fast::RapidHashSet;
    /*const СОГЛАСНЫЕ: [char; 21] = [
        'б', 'в', 'г', 'д', 'ж', 'з', 'й', 'к', 'л', 'м', 'н', 'п', 'р', 'с', 'т', 'ф', 'х', 'ц',
        'ч', 'ш', 'щ',
    ];*/
    static СОГЛАСНЫЕ: LazyLock<RapidHashSet<char>> = LazyLock::new(|| {
        RapidHashSet::from_iter([
            'б', 'в', 'г', 'д', 'ж', 'з', 'й', 'к', 'л', 'м', 'н', 'п', 'р', 'с', 'т', 'ф', 'х',
            'ц', 'ч', 'ш', 'щ',
        ])
    });
    //
    let буквы: Vec<char> = слово.chars().collect();
    //
    for указатель_1 in 0..буквы.len() {
        //
        let следующий_указатель: usize = указатель_1 + 1;
        let буква_1: &char = &буквы[указатель_1];
        //
        if СОГЛАСНЫЕ.contains(буква_1) {
            //проверка выхода за пределы границ
            match безопасный_доступ_в_ряде(&буквы, следующий_указатель)
            {
                //нет следующей ячейки
                false => (),
                //есть следующая ячейка
                true => {
                    let буква_2: &char = &буквы[следующий_указатель];
                    //
                    if СОГЛАСНЫЕ.contains(буква_2) {
                        //
                        if буква_1 == буква_2 {
                            //

                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}
pub fn проверка_на_удвоенные_гласные_для_слова(
    слово: &str,
) -> bool {
    use rapidhash::fast::RapidHashSet;
    /*const СОГЛАСНЫЕ: [char; 21] = [
        'б', 'в', 'г', 'д', 'ж', 'з', 'й', 'к', 'л', 'м', 'н', 'п', 'р', 'с', 'т', 'ф', 'х', 'ц',
        'ч', 'ш', 'щ',
    ];*/
    static ГЛАСНЫЕ: LazyLock<RapidHashSet<char>> = LazyLock::new(|| {
        RapidHashSet::from_iter(['а', 'е', 'ё', 'и', 'о', 'у', 'ы', 'э', 'ю', 'я'])
    });
    //
    let буквы: Vec<char> = слово.chars().collect();
    //
    for указатель_1 in 0..буквы.len() {
        //
        let следующий_указатель: usize = указатель_1 + 1;
        let буква_1: &char = &буквы[указатель_1];
        //
        if ГЛАСНЫЕ.contains(буква_1) {
            //проверка выхода за пределы границ
            match безопасный_доступ_в_ряде(&буквы, следующий_указатель)
            {
                //нет следующей ячейки
                false => (),
                //есть следующая ячейка
                true => {
                    let буква_2: &char = &буквы[следующий_указатель];
                    //
                    if ГЛАСНЫЕ.contains(буква_2) {
                        //
                        if буква_1 == буква_2 {
                            //

                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

pub fn безопасный_доступ_в_ряде<T>(
    вектор: &[T],
    указатель_страницы: usize,
) -> bool {
    match вектор.get(указатель_страницы) {
        Some(_) => true,
        None => false,
    }
}

//
//
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy)]
pub struct Данные_при_загрузке_lib {
    pub включить_перевод: bool,
    pub включить_разделители: bool,
    pub вывод_всех_словарей: bool,
    pub вывод_словарей_куч: bool,
    pub вывод_словаря_переносов: bool,
    pub вывод_словаря_разделителей: bool,
    pub сохранить_книги_с_разделителями: bool,
    pub вывод_найденных_повторов_слов_в_заменах: bool,
    pub вывод_пропусков: bool,
    pub общий_ход: bool,
    pub загрузить_словари: bool,
    pub не_загружать_основные_словари: bool,
    pub загрузить_переведенные_слова: bool,
    pub проверка_недостающих_запрещённых_окончаний: bool,
}

impl Default for Данные_при_загрузке_lib {
    fn default() -> Self {
        Self {
            включить_перевод: false,
            включить_разделители: false,
            вывод_всех_словарей: false,
            вывод_словарей_куч: false,
            вывод_словаря_переносов: false,
            вывод_словаря_разделителей: false,
            сохранить_книги_с_разделителями: false,
            вывод_найденных_повторов_слов_в_заменах: false,
            вывод_пропусков: false,
            общий_ход: false,
            загрузить_словари: true,
            не_загружать_основные_словари: false,
            загрузить_переведенные_слова: false,
            проверка_недостающих_запрещённых_окончаний:
                false,
        }
    }
}
