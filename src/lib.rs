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
            "{}ошибки_частей_речи.xlsx",
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
//
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Вид_ошибки_при_поиске_выдера_до_торжка {
    Не_соответствие_частей_речи(
        Ошибка_несоотвествия_части_речи_RE_окончание,
    ),
    Не_найден_образец_re_для_слова(String),
    Не_определено,
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
    Исключение(String, Вид_Окончания),
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

            Вид_ошибки_при_поиске_выдера_в_торжке::Исключение(слово,окоччание) => {
                write!(f, "|{}| {} {} {} ",
                    style(format!("|{}|",слово)).blue(),
                    format!(" его вид "),
                    style(format!("|{}|", окоччание)).blue(),
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
    Тн,
    // Лос,
    _Ж,
    Чн,
    _Н,
    //_В,
    _Р,
    _А,
    Ам,
    Ко_ка,
    Ск,
    Ва_Во,
    Ель,
    _Е,
    Ен,
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
    Ем,
    Ом,
    Ют,
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
    Ты,
    Чи,
    _И,
    _Л,
    Льн,
    //_К,
    Ъят,
    _Ш,
    _Ц,
    Ст,
    _С,
    Ств,
    Ям,
    Ти,
    _Д,
    Ян,
    Жн,
    Сл,
    //Лю_ля,
    Не_определено,
}
pub static ВСЕ_ВИДЫ_ОКОНЧАНИЙ: [Вид_Окончания; 63] = [
    // Вид_Окончания::_Лос,
    Вид_Окончания::Сен,
    Вид_Окончания::Ат,
    Вид_Окончания::Нут,
    Вид_Окончания::Сён,
    Вид_Окончания::Дан,
    Вид_Окончания::Пн,
    Вид_Окончания::Льн,
    Вид_Окончания::Тн,
    Вид_Окончания::Ет,
    Вид_Окончания::Ян,
    Вид_Окончания::Чн,
    Вид_Окончания::_И,
    Вид_Окончания::_Ж,
    Вид_Окончания::Жн,
    Вид_Окончания::_Н,
    Вид_Окончания::_О,
    Вид_Окончания::_А,
    //Вид_Окончания::_В,
    Вид_Окончания::_Р,
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
    Вид_Окончания::Ты,
    Вид_Окончания::Чи,
    Вид_Окончания::_Л,
    //Вид_Окончания::_К,
    Вид_Окончания::Ъят,
    Вид_Окончания::_Ш,
    Вид_Окончания::_Ц,
    Вид_Окончания::Ст,
    Вид_Окончания::_С,
    Вид_Окончания::Ств,
    Вид_Окончания::Ям,
    Вид_Окончания::Ти,
    Вид_Окончания::_Д,
    Вид_Окончания::_Ы,
    Вид_Окончания::_Ю,
    Вид_Окончания::Сл,
    //Вид_Окончания::_Л,
    //Вид_Окончания::Не_определено,
];
impl Вид_Окончания {
    pub fn получить_вид_окончания_по_строке(
        окончание: &str,
    ) -> Result<Self, String> {
        if КУЧА_ПОЛНАЯ_ОКОНЧАНИЙ_ВАН.contains(окончание) {
            return Ok(Вид_Окончания::Ван);
        };
        for раздел in РАЗДЕЛЫ_ОКОНЧАНИЙ_ПОЛНЫЕ.iter() {
            if раздел.куча_окончаний_полная.contains(окончание) {
                return Ok(раздел.вид_окончания);
            }
        }
        //
        Err(format!("Не найдено окончание в разделах|{окончание}|"))
    }
    pub fn ен_ён(&self) -> bool {
        match self {
            Вид_Окончания::Ён => true,
            Вид_Окончания::Ен => true,
            _ => false,
        }
    }
    pub fn ты_то(&self) -> bool {
        match self {
            Вид_Окончания::Ты => true,
            Вид_Окончания::Та_то => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Вид_Окончания {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Вид_Окончания::Сен => "Сен",
            Вид_Окончания::Сён => "Сён",
            Вид_Окончания::Дан => "Дан",
            Вид_Окончания::Нут => "Нут",
            Вид_Окончания::Пн => "Пн",
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
            //Вид_Окончания::_Л => "Лю_ля",
            Вид_Окончания::Ет => "Ет",
            Вид_Окончания::_О => "_О",
            Вид_Окончания::_И => "_И",
            Вид_Окончания::Ут => "Ут",
            Вид_Окончания::Ат => "Ат",
            Вид_Окончания::Ти => "Ти",
            Вид_Окончания::Чи => "Чи",
            Вид_Окончания::_С => "_С",
            Вид_Окончания::_Ц => "_Ц",
            Вид_Окончания::_У => "_У",
            Вид_Окончания::Ят => "Ят",
            Вид_Окончания::Ям => "Ям",
            Вид_Окончания::Ск => "Ск",
            Вид_Окончания::Ам => "Ам",
            Вид_Окончания::_Ж => "_Ж",
            //Вид_Окончания::_К => "_К",
            Вид_Окончания::_Р => "_Р",
            //Вид_Окончания::Ни => "Ни",
            Вид_Окончания::Ель => "Ель",
            Вид_Окончания::Ъят => "Ъят",
            Вид_Окончания::Нят => "Нят",
            Вид_Окончания::Нятн => "Нятн",
            Вид_Окончания::Ств => "Ств",
            Вид_Окончания::Ты => "Ты",
            // Вид_Окончания::Те => "Те",
            Вид_Окончания::Ва_Во => "Ва",
            Вид_Окончания::Ыт => "Ыт",
            Вид_Окончания::Ст => "Ст",
            Вид_Окончания::Та_то => "То",
            Вид_Окончания::Ым => "Ым",
            // Вид_Окончания::Ты => "Ты",
            // Вид_Окончания::_Ы => "Ыт",
            Вид_Окончания::_Ю => "_Ю",
            Вид_Окончания::_Д => "_Д",
            Вид_Окончания::_Ш => "_Ш",
            Вид_Окончания::_Щ => "_Щ",
            Вид_Окончания::_Л => "_Л",
            Вид_Окончания::Ит => "Ит",
            Вид_Окончания::Ов => "Ов",
            Вид_Окончания::Ев => "Ев",
            Вид_Окончания::Ив => "Ив",
            Вид_Окончания::_Н => "_Н",
            Вид_Окончания::_Ы => "_Ы",
            //Вид_Окончания::_В => "_В",
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
const ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА: &[Вид_части_речи] = &[Вид_части_речи::Не_определено];
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
    pub правописание: Буквы_Слова,
    pub строка: &'static str,
    pub вид_окончания: Вид_Окончания,
    pub части_речи: &'static [Вид_части_речи],
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
// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_Н
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Н: LazyLock<[RE_полное_окончание; 38]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+н$").unwrap(),
            re_замены: Regex::new(r"(?i)(н)$").unwrap(),
            строка: "н",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ни$").unwrap(),
            re_замены: Regex::new(r"(?i)(ни)$").unwrap(),
            строка: "ни",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ним$").unwrap(),
            re_замены: Regex::new(r"(?i)(ним)$").unwrap(),
            строка: "ним",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+не$").unwrap(),
            re_замены: Regex::new(r"(?i)(не)$").unwrap(),
            строка: "не",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+на$").unwrap(),
            re_замены: Regex::new(r"(?i)(на)$").unwrap(),
            строка: "на",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ная$").unwrap(),
            re_замены: Regex::new(r"(?i)(ная)$").unwrap(),
            строка: "ная",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ную$").unwrap(),
            re_замены: Regex::new(r"(?i)(ную)$").unwrap(),
            строка: "ную",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+но$").unwrap(),
            re_замены: Regex::new(r"(?i)(но)$").unwrap(),
            строка: "но",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ны$").unwrap(),
            re_замены: Regex::new(r"(?i)(ны)$").unwrap(),
            строка: "ны",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ном$").unwrap(),
            re_замены: Regex::new(r"(?i)(ном)$").unwrap(),
            строка: "ном",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ному$").unwrap(),
            re_замены: Regex::new(r"(?i)(ному)$").unwrap(),
            строка: "ному",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ного$").unwrap(),
            re_замены: Regex::new(r"(?i)(ного)$").unwrap(),
            строка: "ного",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ное$").unwrap(),
            re_замены: Regex::new(r"(?i)(ное)$").unwrap(),
            строка: "ное",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ной$").unwrap(),
            re_замены: Regex::new(r"(?i)(ной)$").unwrap(),
            строка: "ной",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ность$").unwrap(),
            re_замены: Regex::new(r"(?i)(ность)$").unwrap(),
            строка: "ность",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        // ВНИМАНИЕ: "ного" повторяется — дубликат удалён.
        // Если нужен именно второй экземпляр — верните и увеличьте размер до 25.
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностью)$").unwrap(),
            строка: "ностью",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ности$").unwrap(),
            re_замены: Regex::new(r"(?i)(ности)$").unwrap(),
            строка: "ности",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностей)$").unwrap(),
            строка: "ностей",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностям)$").unwrap(),
            строка: "ностям",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностями)$").unwrap(),
            строка: "ностями",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностях)$").unwrap(),
            строка: "ностях",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ным$").unwrap(),
            re_замены: Regex::new(r"(?i)(ным)$").unwrap(),
            строка: "ным",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ными$").unwrap(),
            re_замены: Regex::new(r"(?i)(ными)$").unwrap(),
            строка: "ными",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ный$").unwrap(),
            re_замены: Regex::new(r"(?i)(ный)$").unwrap(),
            строка: "ный",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ные$").unwrap(),
            re_замены: Regex::new(r"(?i)(ные)$").unwrap(),
            строка: "ные",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ных$").unwrap(),
            re_замены: Regex::new(r"(?i)(ных)$").unwrap(),
            строка: "ных",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нее$").unwrap(),
            re_замены: Regex::new(r"(?i)(нее)$").unwrap(),
            строка: "нее",
            вид_окончания: Вид_Окончания::_Н,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+нии$").unwrap(),
            re_замены: Regex::new(r"(?i)(нии)$").unwrap(),
            строка: "нии",
            вид_окончания: Вид_Окончания::_Н,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ними$").unwrap(),
            re_замены: Regex::new(r"(?i)(ними)$").unwrap(),
            строка: "ними",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ния$").unwrap(),
            re_замены: Regex::new(r"(?i)(ния)$").unwrap(),
            строка: "ния",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ниям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ниям$)$").unwrap(),
            строка: "ниям",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ниями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ниями$)$").unwrap(),
            строка: "ниями",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ниях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ниях$)$").unwrap(),
            строка: "ниях",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нию$").unwrap(),
            re_замены: Regex::new(r"(?i)(нию$)$").unwrap(),
            строка: "нию",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ний$").unwrap(),
            re_замены: Regex::new(r"(?i)(ний)$").unwrap(),
            строка: "ний",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ние$").unwrap(),
            re_замены: Regex::new(r"(?i)(ние)$").unwrap(),
            строка: "ние",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нием$").unwrap(),
            re_замены: Regex::new(r"(?i)(нием)$").unwrap(),
            строка: "нием",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+них$").unwrap(),
            re_замены: Regex::new(r"(?i)(них)$").unwrap(),
            строка: "них",
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Н: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "н",
            "ни",
            "них",
            "ним",
            "ними",
            "нии",
            "не",
            "на",
            "ная",
            "ную",
            "но",
            "ны",
            "ном",
            "ному",
            "ного",
            "ное",
            "ной",
            "ность",
            "ностью",
            "ности",
            "ностей",
            "ностям",
            "ностями",
            "ностях",
            "ным",
            "ными",
            "ный",
            "ные",
            "ных",
            "нее",
            "ния",
            "ниям",
            "ниями",
            "ниях",
            "нию",
            "ний",
            "ние",
            "нием",
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_СТ
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СТ: LazyLock<[RE_полное_окончание; 16]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+стая$").unwrap(),
            re_замены: Regex::new(r"(?i)(стая)$").unwrap(),
            строка: "стая",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+стее$").unwrap(),
            re_замены: Regex::new(r"(?i)(стее)$").unwrap(),
            строка: "стее",
            вид_окончания: Вид_Окончания::Ст,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+стую$").unwrap(),
            re_замены: Regex::new(r"(?i)(стую)$").unwrap(),
            строка: "стую",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+сте$").unwrap(),
            re_замены: Regex::new(r"(?i)(сте)$").unwrap(),
            строка: "сте",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+сти$").unwrap(),
            re_замены: Regex::new(r"(?i)(сти)$").unwrap(),
            строка: "сти",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+стем$").unwrap(),
            re_замены: Regex::new(r"(?i)(стем)$").unwrap(),
            строка: "стем",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+стему$").unwrap(),
            re_замены: Regex::new(r"(?i)(стему)$").unwrap(),
            строка: "стему",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+стей$").unwrap(),
            re_замены: Regex::new(r"(?i)(стей)$").unwrap(),
            строка: "стей",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сть$").unwrap(),
            re_замены: Regex::new(r"(?i)(сть)$").unwrap(),
            строка: "сть",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+стью$").unwrap(),
            re_замены: Regex::new(r"(?i)(стью)$").unwrap(),
            строка: "стью",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        // Дубликат "сти" удалён
        // Дубликат "стей" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+стям$").unwrap(),
            re_замены: Regex::new(r"(?i)(стям)$").unwrap(),
            строка: "стям",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+стями$").unwrap(),
            re_замены: Regex::new(r"(?i)(стями)$").unwrap(),
            строка: "стями",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+стях$").unwrap(),
            re_замены: Regex::new(r"(?i)(стях)$").unwrap(),
            строка: "стях",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+стим$").unwrap(),
            re_замены: Regex::new(r"(?i)(стим)$").unwrap(),
            строка: "стим",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+стими$").unwrap(),
            re_замены: Regex::new(r"(?i)(стими)$").unwrap(),
            строка: "стими",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+стий$").unwrap(),
            re_замены: Regex::new(r"(?i)(стий)$").unwrap(),
            строка: "стий",
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        // Дубликат "стях" удалён
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_СТ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            // одиночные
            "сть",
            "сти",
            "стей",
            "сте",
            "стем",
            "стему",
            "стям",
            "стями",
            "стях",
            "стим",
            "стими",
            "стий",
            // прилагательные (ж.р., ср.р.)
            "стая",
            "стее",
            "стую",
            "стью",
            "стее",
        ])
    });
// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ЕН
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЕН: LazyLock<[RE_полное_окончание; 57]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ен$").unwrap(),
            re_замены: Regex::new(r"(?i)(ен)$").unwrap(),
            строка: "ен",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+енее$").unwrap(),
            re_замены: Regex::new(r"(?i)(енее)$").unwrap(),
            строка: "енее",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+еннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(еннее)$").unwrap(),
            строка: "еннее",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еной$").unwrap(),
            re_замены: Regex::new(r"(?i)(еной)$").unwrap(),
            строка: "еной",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енна$").unwrap(),
            re_замены: Regex::new(r"(?i)(енна)$").unwrap(),
            строка: "енна",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ена$").unwrap(),
            re_замены: Regex::new(r"(?i)(ена)$").unwrap(),
            строка: "ена",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еная$").unwrap(),
            re_замены: Regex::new(r"(?i)(еная)$").unwrap(),
            строка: "еная",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енная$").unwrap(),
            re_замены: Regex::new(r"(?i)(енная)$").unwrap(),
            строка: "енная",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еную$").unwrap(),
            re_замены: Regex::new(r"(?i)(еную)$").unwrap(),
            строка: "еную",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енную$").unwrap(),
            re_замены: Regex::new(r"(?i)(енную)$").unwrap(),
            строка: "енную",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ены$").unwrap(),
            re_замены: Regex::new(r"(?i)(ены)$").unwrap(),
            строка: "ены",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енны$").unwrap(),
            re_замены: Regex::new(r"(?i)(енны)$").unwrap(),
            строка: "енны",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еном$").unwrap(),
            re_замены: Regex::new(r"(?i)(еном)$").unwrap(),
            строка: "еном",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енном$").unwrap(),
            re_замены: Regex::new(r"(?i)(енном)$").unwrap(),
            строка: "енном",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еному$").unwrap(),
            re_замены: Regex::new(r"(?i)(еному)$").unwrap(),
            строка: "еному",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енному$").unwrap(),
            re_замены: Regex::new(r"(?i)(енному)$").unwrap(),
            строка: "енному",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еного$").unwrap(),
            re_замены: Regex::new(r"(?i)(еного)$").unwrap(),
            строка: "еного",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енного$").unwrap(),
            re_замены: Regex::new(r"(?i)(енного)$").unwrap(),
            строка: "енного",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+ено$").unwrap(),
            re_замены: Regex::new(r"(?i)(ено)$").unwrap(),
            строка: "ено",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+енно$").unwrap(),
            re_замены: Regex::new(r"(?i)(енно)$").unwrap(),
            строка: "енно",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еное$").unwrap(),
            re_замены: Regex::new(r"(?i)(еное)$").unwrap(),
            строка: "еное",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енное$").unwrap(),
            re_замены: Regex::new(r"(?i)(енное)$").unwrap(),
            строка: "енное",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енной$").unwrap(),
            re_замены: Regex::new(r"(?i)(енной)$").unwrap(),
            строка: "енной",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+еность$").unwrap(),
            re_замены: Regex::new(r"(?i)(еность)$").unwrap(),
            строка: "еность",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+енность$").unwrap(),
            re_замены: Regex::new(r"(?i)(енность)$").unwrap(),
            строка: "енность",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+еностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(еностью)$").unwrap(),
            строка: "еностью",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+енностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностью)$").unwrap(),
            строка: "енностью",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ености$").unwrap(),
            re_замены: Regex::new(r"(?i)(ености)$").unwrap(),
            строка: "ености",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+енности$").unwrap(),
            re_замены: Regex::new(r"(?i)(енности)$").unwrap(),
            строка: "енности",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+еностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(еностей)$").unwrap(),
            строка: "еностей",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+енностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностей)$").unwrap(),
            строка: "енностей",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+еностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(еностям)$").unwrap(),
            строка: "еностям",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+енностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностям)$").unwrap(),
            строка: "енностям",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+еностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(еностями)$").unwrap(),
            строка: "еностями",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+енностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностями)$").unwrap(),
            строка: "енностями",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+еностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(еностях)$").unwrap(),
            строка: "еностях",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+енностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностях)$").unwrap(),
            строка: "енностях",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+еным$").unwrap(),
            re_замены: Regex::new(r"(?i)(еным)$").unwrap(),
            строка: "еным",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+енным$").unwrap(),
            re_замены: Regex::new(r"(?i)(енным)$").unwrap(),
            строка: "енным",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еными$").unwrap(),
            re_замены: Regex::new(r"(?i)(еными)$").unwrap(),
            строка: "еными",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енными$").unwrap(),
            re_замены: Regex::new(r"(?i)(енными)$").unwrap(),
            строка: "енными",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еный$").unwrap(),
            re_замены: Regex::new(r"(?i)(еный)$").unwrap(),
            строка: "еный",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енный$").unwrap(),
            re_замены: Regex::new(r"(?i)(енный)$").unwrap(),
            строка: "енный",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еные$").unwrap(),
            re_замены: Regex::new(r"(?i)(еные)$").unwrap(),
            строка: "еные",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енные$").unwrap(),
            re_замены: Regex::new(r"(?i)(енные)$").unwrap(),
            строка: "енные",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+еных$").unwrap(),
            re_замены: Regex::new(r"(?i)(еных)$").unwrap(),
            строка: "еных",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+енных$").unwrap(),
            re_замены: Regex::new(r"(?i)(енных)$").unwrap(),
            строка: "енных",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ения$").unwrap(),
            re_замены: Regex::new(r"(?i)(ения)$").unwrap(),
            строка: "ения",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ениям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ениям$)$").unwrap(),
            строка: "ениям",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ениями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ениями$)$").unwrap(),
            строка: "ениями",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ениях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ениях$)$").unwrap(),
            строка: "ениях",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ении$").unwrap(),
            re_замены: Regex::new(r"(?i)(ении$)$").unwrap(),
            строка: "ении",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ений$").unwrap(),
            re_замены: Regex::new(r"(?i)(ений)$").unwrap(),
            строка: "ений",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ение$").unwrap(),
            re_замены: Regex::new(r"(?i)(ение)$").unwrap(),
            строка: "ение",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ением$").unwrap(),
            re_замены: Regex::new(r"(?i)(ением)$").unwrap(),
            строка: "ением",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ениии$").unwrap(),
            re_замены: Regex::new(r"(?i)(ениии)$").unwrap(),
            строка: "ениии",
            вид_окончания: Вид_Окончания::Ен,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ению$").unwrap(),
            re_замены: Regex::new(r"(?i)(ению)$").unwrap(),
            строка: "ению",
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ПОЛНАЯ_ЕН: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "енна",
        "ена",
        "ен",
        //
        "ения",
        "ение",
        "ениям",
        "ениями",
        "ениии",
        "ениях",
        "ением",
        "ению",
        "ении",
        "ений",
        //
        "енее",
        "еннее",
        "ена",
        "еная",
        "енная",
        "еную",
        "енную",
        "ено",
        "ены",
        "енны",
        "еном",
        "енном",
        "еному",
        "енному",
        "еного",
        "енного",
        "енно",
        "еное",
        "енное",
        "еной",
        "енной",
        "еность",
        "енность",
        "еностью",
        "енностью",
        "ености",
        "енности",
        "еностей",
        "енностей",
        "еностям",
        "енностям",
        "еностями",
        "енностями",
        "еностях",
        "енностях",
        "еным",
        "енным",
        "еными",
        "енными",
        "еный",
        "енный",
        "еные",
        "енные",
        "енных",
        "еных",
    ])
});

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ОН
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ОН: LazyLock<[RE_полное_окончание; 46]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онее$").unwrap(),
            re_замены: Regex::new(r"(?i)(онее)$").unwrap(),
            строка: "онее",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+он$").unwrap(),
            re_замены: Regex::new(r"(?i)(он)$").unwrap(),
            строка: "он",
            вид_окончания: Вид_Окончания::Он,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+онна$").unwrap(),
            re_замены: Regex::new(r"(?i)(онна)$").unwrap(),
            строка: "онна",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+она$").unwrap(),
            re_замены: Regex::new(r"(?i)(она)$").unwrap(),
            строка: "она",
            вид_окончания: Вид_Окончания::Он,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оная$").unwrap(),
            re_замены: Regex::new(r"(?i)(оная)$").unwrap(),
            строка: "оная",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онная$").unwrap(),
            re_замены: Regex::new(r"(?i)(онная)$").unwrap(),
            строка: "онная",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оную$").unwrap(),
            re_замены: Regex::new(r"(?i)(оную)$").unwrap(),
            строка: "оную",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онную$").unwrap(),
            re_замены: Regex::new(r"(?i)(онную)$").unwrap(),
            строка: "онную",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+оно$").unwrap(),
            re_замены: Regex::new(r"(?i)(оно)$").unwrap(),
            строка: "оно",
            вид_окончания: Вид_Окончания::Он,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+оны$").unwrap(),
            re_замены: Regex::new(r"(?i)(оны)$").unwrap(),
            строка: "оны",
            вид_окончания: Вид_Окончания::Он,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оном$").unwrap(),
            re_замены: Regex::new(r"(?i)(оном)$").unwrap(),
            строка: "оном",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онном$").unwrap(),
            re_замены: Regex::new(r"(?i)(онном)$").unwrap(),
            строка: "онном",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оному$").unwrap(),
            re_замены: Regex::new(r"(?i)(оному)$").unwrap(),
            строка: "оному",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онному$").unwrap(),
            re_замены: Regex::new(r"(?i)(онному)$").unwrap(),
            строка: "онному",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онно$").unwrap(),
            re_замены: Regex::new(r"(?i)(онно)$").unwrap(),
            строка: "онно",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оного$").unwrap(),
            re_замены: Regex::new(r"(?i)(оного)$").unwrap(),
            строка: "оного",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онного$").unwrap(),
            re_замены: Regex::new(r"(?i)(онного)$").unwrap(),
            строка: "онного",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оное$").unwrap(),
            re_замены: Regex::new(r"(?i)(оное)$").unwrap(),
            строка: "оное",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онное$").unwrap(),
            re_замены: Regex::new(r"(?i)(онное)$").unwrap(),
            строка: "онное",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оной$").unwrap(),
            re_замены: Regex::new(r"(?i)(оной)$").unwrap(),
            строка: "оной",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онной$").unwrap(),
            re_замены: Regex::new(r"(?i)(онной)$").unwrap(),
            строка: "онной",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+оность$").unwrap(),
            re_замены: Regex::new(r"(?i)(оность)$").unwrap(),
            строка: "оность",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+онность$").unwrap(),
            re_замены: Regex::new(r"(?i)(онность)$").unwrap(),
            строка: "онность",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+оностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(оностью)$").unwrap(),
            строка: "оностью",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+онностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностью)$").unwrap(),
            строка: "онностью",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+оности$").unwrap(),
            re_замены: Regex::new(r"(?i)(оности)$").unwrap(),
            строка: "оности",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+онности$").unwrap(),
            re_замены: Regex::new(r"(?i)(онности)$").unwrap(),
            строка: "онности",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+оностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(оностей)$").unwrap(),
            строка: "оностей",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+онностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностей)$").unwrap(),
            строка: "онностей",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+оностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(оностям)$").unwrap(),
            строка: "оностям",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+онностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностям)$").unwrap(),
            строка: "онностям",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+оностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(оностями)$").unwrap(),
            строка: "оностями",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+онностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностями)$").unwrap(),
            строка: "онностями",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+оностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(оностях)$").unwrap(),
            строка: "оностях",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+онностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностях)$").unwrap(),
            строка: "онностях",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оным$").unwrap(),
            re_замены: Regex::new(r"(?i)(оным)$").unwrap(),
            строка: "оным",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онным$").unwrap(),
            re_замены: Regex::new(r"(?i)(онным)$").unwrap(),
            строка: "онным",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оными$").unwrap(),
            re_замены: Regex::new(r"(?i)(оными)$").unwrap(),
            строка: "оными",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онными$").unwrap(),
            re_замены: Regex::new(r"(?i)(онными)$").unwrap(),
            строка: "онными",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оный$").unwrap(),
            re_замены: Regex::new(r"(?i)(оный)$").unwrap(),
            строка: "оный",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онный$").unwrap(),
            re_замены: Regex::new(r"(?i)(онный)$").unwrap(),
            строка: "онный",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оные$").unwrap(),
            re_замены: Regex::new(r"(?i)(оные)$").unwrap(),
            строка: "оные",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онные$").unwrap(),
            re_замены: Regex::new(r"(?i)(онные)$").unwrap(),
            строка: "онные",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оных$").unwrap(),
            re_замены: Regex::new(r"(?i)(оных)$").unwrap(),
            строка: "оных",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+онных$").unwrap(),
            re_замены: Regex::new(r"(?i)(онных)$").unwrap(),
            строка: "онных",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+оннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(оннее)$").unwrap(),
            строка: "оннее",
            вид_окончания: Вид_Окончания::Он,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ОН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "он",
            "она",
            "онна",
            "оная",
            "онная",
            "оную",
            "онную",
            "оно",
            "оннее",
            "онее",
            "оны",
            "оном",
            "онном",
            "оному",
            "онному",
            "онно",
            "оного",
            "онного",
            "оное",
            "онное",
            "оной",
            "онной",
            "оность",
            "онность",
            "оностью",
            "онностью",
            "оности",
            "онности",
            "оностей",
            "онностей",
            "оностям",
            "онностям",
            "оностями",
            "онностями",
            "оностях",
            "онностях",
            "оным",
            "онным",
            "оными",
            "онными",
            "оный",
            "онный",
            "оные",
            "онные",
            "оных",
            "онных",
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ЁН
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЁН: LazyLock<[RE_полное_окончание; 45]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ён$").unwrap(),
            re_замены: Regex::new(r"(?i)(ён)$").unwrap(),
            строка: "ён",
            вид_окончания: Вид_Окончания::Ён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёнее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёнее)$").unwrap(),
            строка: "ёнее",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённее)$").unwrap(),
            строка: "ённее",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ённа$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённа)$").unwrap(),
            строка: "ённа",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ёна$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёна)$").unwrap(),
            строка: "ёна",
            вид_окончания: Вид_Окончания::Ён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ёная$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёная)$").unwrap(),
            строка: "ёная",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ённая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённая)$").unwrap(),
            строка: "ённая",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ёную$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёную)$").unwrap(),
            строка: "ёную",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ённую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённую)$").unwrap(),
            строка: "ённую",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ёном$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёном)$").unwrap(),
            строка: "ёном",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ённом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённом)$").unwrap(),
            строка: "ённом",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ёному$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёному)$").unwrap(),
            строка: "ёному",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ённому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённому)$").unwrap(),
            строка: "ённому",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ёного$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёного)$").unwrap(),
            строка: "ёного",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ённого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённого)$").unwrap(),
            строка: "ённого",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+ёно$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёно)$").unwrap(),
            строка: "ёно",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+ённо$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённо)$").unwrap(),
            строка: "ённо",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ёное$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёное)$").unwrap(),
            строка: "ёное",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ённое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённое)$").unwrap(),
            строка: "ённое",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ёной$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёной)$").unwrap(),
            строка: "ёной",
            вид_окончания: Вид_Окончания::Ён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ённой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённой)$").unwrap(),
            строка: "ённой",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёность$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёность)$").unwrap(),
            строка: "ёность",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённость)$").unwrap(),
            строка: "ённость",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёностью)$").unwrap(),
            строка: "ёностью",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостью)$").unwrap(),
            строка: "ённостью",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёности$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёности)$").unwrap(),
            строка: "ёности",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённости)$").unwrap(),
            строка: "ённости",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёностей)$").unwrap(),
            строка: "ёностей",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостей)$").unwrap(),
            строка: "ённостей",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёностям)$").unwrap(),
            строка: "ёностям",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостям)$").unwrap(),
            строка: "ённостям",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёностями)$").unwrap(),
            строка: "ёностями",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостями)$").unwrap(),
            строка: "ённостями",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёностях)$").unwrap(),
            строка: "ёностях",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостях)$").unwrap(),
            строка: "ённостях",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёным$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёным)$").unwrap(),
            строка: "ёным",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённым)$").unwrap(),
            строка: "ённым",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёными$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёными)$").unwrap(),
            строка: "ёными",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёнными$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёнными)$").unwrap(),
            строка: "ёнными",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёный$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёный)$").unwrap(),
            строка: "ёный",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённый)$").unwrap(),
            строка: "ённый",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёные$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёные)$").unwrap(),
            строка: "ёные",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённые)$").unwrap(),
            строка: "ённые",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ённых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённых)$").unwrap(),
            строка: "ённых",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ёных$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёных)$").unwrap(),
            строка: "ёных",
            вид_окончания: Вид_Окончания::Ён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});

pub static КУЧА_ОКОНЧАНИЙ_ЁН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ён",
            "ёнее",
            "ённее",
            "ённа",
            "ёна",
            "ёная",
            "ённая",
            "ёную",
            "ённую",
            "ёном",
            "ённом",
            "ёному",
            "ённому",
            "ёного",
            "ённого",
            "ённо",
            "ёно",
            "ёное",
            "ённое",
            "ёной",
            "ённой",
            "ёность",
            "ённость",
            "ёностью",
            "ённостью",
            "ёности",
            "ённости",
            "ёностей",
            "ённостей",
            "ёностям",
            "ённостям",
            "ёностями",
            "ённостями",
            "ёностях",
            "ённостях",
            "ёным",
            "ённым",
            "ёными",
            "ёнными",
            "ёный",
            "ённый",
            "ёные",
            "ённые",
            "ёных",
            "ённых",
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ИТ
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ИТ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ит$").unwrap(),
            re_замены: Regex::new(r"(?i)(ит)$").unwrap(),
            строка: "ит",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+итее$").unwrap(),
            re_замены: Regex::new(r"(?i)(итее)$").unwrap(),
            строка: "итее",
            вид_окончания: Вид_Окончания::Ит,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ита$").unwrap(),
            re_замены: Regex::new(r"(?i)(ита)$").unwrap(),
            строка: "ита",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итая$").unwrap(),
            re_замены: Regex::new(r"(?i)(итая)$").unwrap(),
            строка: "итая",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итую$").unwrap(),
            re_замены: Regex::new(r"(?i)(итую)$").unwrap(),
            строка: "итую",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ито$").unwrap(),
            re_замены: Regex::new(r"(?i)(ито)$").unwrap(),
            строка: "ито",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+иты$").unwrap(),
            re_замены: Regex::new(r"(?i)(иты)$").unwrap(),
            строка: "иты",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итом$").unwrap(),
            re_замены: Regex::new(r"(?i)(итом)$").unwrap(),
            строка: "итом",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+итому$").unwrap(),
            re_замены: Regex::new(r"(?i)(итому)$").unwrap(),
            строка: "итому",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        // Дубликат "ито" удалён (уже есть выше)
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итого$").unwrap(),
            re_замены: Regex::new(r"(?i)(итого)$").unwrap(),
            строка: "итого",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итое$").unwrap(),
            re_замены: Regex::new(r"(?i)(итое)$").unwrap(),
            строка: "итое",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итой$").unwrap(),
            re_замены: Regex::new(r"(?i)(итой)$").unwrap(),
            строка: "итой",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+итость$").unwrap(),
            re_замены: Regex::new(r"(?i)(итость)$").unwrap(),
            строка: "итость",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+итостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостью)$").unwrap(),
            строка: "итостью",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+итости$").unwrap(),
            re_замены: Regex::new(r"(?i)(итости)$").unwrap(),
            строка: "итости",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+итостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостей)$").unwrap(),
            строка: "итостей",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+итостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостям)$").unwrap(),
            строка: "итостям",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+итостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостями)$").unwrap(),
            строка: "итостями",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+итостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостях)$").unwrap(),
            строка: "итостях",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итым$").unwrap(),
            re_замены: Regex::new(r"(?i)(итым)$").unwrap(),
            строка: "итым",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(итыми)$").unwrap(),
            строка: "итыми",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итый$").unwrap(),
            re_замены: Regex::new(r"(?i)(итый)$").unwrap(),
            строка: "итый",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итые$").unwrap(),
            re_замены: Regex::new(r"(?i)(итые)$").unwrap(),
            строка: "итые",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+итых$").unwrap(),
            re_замены: Regex::new(r"(?i)(итых)$").unwrap(),
            строка: "итых",
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ИТ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ит",
            "итее",
            "ита",
            "итая",
            "итую",
            "ито",
            "иты",
            "итом",
            "итому",
            "итого",
            "итое",
            "итой",
            "итость",
            "итостью",
            "итости",
            "итостей",
            "итостям",
            "итостями",
            "итостях",
            "итым",
            "итыми",
            "итый",
            "итые",
            "итых",
        ])
    });
// РАЗДЕЛ_ОКОНЧАНИЙ_ИТ
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЕТ: LazyLock<[RE_полное_окончание; 26]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ет$").unwrap(),
            re_замены: Regex::new(r"(?i)(ет)$").unwrap(),
            строка: "ет",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+етее$").unwrap(),
            re_замены: Regex::new(r"(?i)(етее)$").unwrap(),
            строка: "етее",
            вид_окончания: Вид_Окончания::Ет,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ета$").unwrap(),
            re_замены: Regex::new(r"(?i)(ета)$").unwrap(),
            строка: "ета",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+етая$").unwrap(),
            re_замены: Regex::new(r"(?i)(етая)$").unwrap(),
            строка: "етая",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+етую$").unwrap(),
            re_замены: Regex::new(r"(?i)(етую)$").unwrap(),
            строка: "етую",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ето$").unwrap(),
            re_замены: Regex::new(r"(?i)(ето)$").unwrap(),
            строка: "ето",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+еты$").unwrap(),
            re_замены: Regex::new(r"(?i)(еты)$").unwrap(),
            строка: "еты",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+етом$").unwrap(),
            re_замены: Regex::new(r"(?i)(етом)$").unwrap(),
            строка: "етом",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+етому$").unwrap(),
            re_замены: Regex::new(r"(?i)(етому)$").unwrap(),
            строка: "етому",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        // Дубликат "ето" удалён (уже есть выше)
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+етого$").unwrap(),
            re_замены: Regex::new(r"(?i)(етого)$").unwrap(),
            строка: "етого",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+етое$").unwrap(),
            re_замены: Regex::new(r"(?i)(етое)$").unwrap(),
            строка: "етое",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+етой$").unwrap(),
            re_замены: Regex::new(r"(?i)(етой)$").unwrap(),
            строка: "етой",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+етость$").unwrap(),
            re_замены: Regex::new(r"(?i)(етость)$").unwrap(),
            строка: "етость",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+етостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(етостью)$").unwrap(),
            строка: "етостью",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+етости$").unwrap(),
            re_замены: Regex::new(r"(?i)(етости)$").unwrap(),
            строка: "етости",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+етостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(етостей)$").unwrap(),
            строка: "етостей",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+етостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(етостям)$").unwrap(),
            строка: "етостям",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+етостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(етостями)$").unwrap(),
            строка: "етостями",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+етостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(етостях)$").unwrap(),
            строка: "етостях",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+етым$").unwrap(),
            re_замены: Regex::new(r"(?i)(етым)$").unwrap(),
            строка: "етым",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+етыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(етыми)$").unwrap(),
            строка: "етыми",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+етый$").unwrap(),
            re_замены: Regex::new(r"(?i)(етый)$").unwrap(),
            строка: "етый",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+етые$").unwrap(),
            re_замены: Regex::new(r"(?i)(етые)$").unwrap(),
            строка: "етые",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+етых$").unwrap(),
            re_замены: Regex::new(r"(?i)(етых)$").unwrap(),
            строка: "етых",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ется$").unwrap(),
            re_замены: Regex::new(r"(?i)(ется)$").unwrap(),
            строка: "ется",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ете$").unwrap(),
            re_замены: Regex::new(r"(?i)(ете)$").unwrap(),
            строка: "ете",
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЕТ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "етее",
            "ет",
            "ете",
            "ется",
            "ета",
            "етая",
            "етую",
            "ето",
            "еты",
            "етом",
            "етому",
            "етого",
            "етое",
            "етой",
            "етость",
            "етостью",
            "етости",
            "етостей",
            "етостям",
            "етостями",
            "етостях",
            "етым",
            "етыми",
            "етый",
            "етые",
            "етых",
        ])
    });
// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ЕМ
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЕМ: LazyLock<[RE_полное_окончание; 26]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ем$").unwrap(),
            re_замены: Regex::new(r"(?i)(ем)$").unwrap(),
            строка: "ем",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+еми$").unwrap(),
            re_замены: Regex::new(r"(?i)(еми)$").unwrap(),
            строка: "еми",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+емее$").unwrap(),
            re_замены: Regex::new(r"(?i)(емее)$").unwrap(),
            строка: "емее",
            вид_окончания: Вид_Окончания::Ем,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ема$").unwrap(),
            re_замены: Regex::new(r"(?i)(ема)$").unwrap(),
            строка: "ема",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+емая$").unwrap(),
            re_замены: Regex::new(r"(?i)(емая)$").unwrap(),
            строка: "емая",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+емую$").unwrap(),
            re_замены: Regex::new(r"(?i)(емую)$").unwrap(),
            строка: "емую",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+емо$").unwrap(),
            re_замены: Regex::new(r"(?i)(емо)$").unwrap(),
            строка: "емо",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+емы$").unwrap(),
            re_замены: Regex::new(r"(?i)(емы)$").unwrap(),
            строка: "емы",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+емом$").unwrap(),
            re_замены: Regex::new(r"(?i)(емом)$").unwrap(),
            строка: "емом",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+емому$").unwrap(),
            re_замены: Regex::new(r"(?i)(емому)$").unwrap(),
            строка: "емому",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        // Дубликат "емо" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+емого$").unwrap(),
            re_замены: Regex::new(r"(?i)(емого)$").unwrap(),
            строка: "емого",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+емое$").unwrap(),
            re_замены: Regex::new(r"(?i)(емое)$").unwrap(),
            строка: "емое",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+емой$").unwrap(),
            re_замены: Regex::new(r"(?i)(емой)$").unwrap(),
            строка: "емой",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+емость$").unwrap(),
            re_замены: Regex::new(r"(?i)(емость)$").unwrap(),
            строка: "емость",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+емостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостью)$").unwrap(),
            строка: "емостью",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+емости$").unwrap(),
            re_замены: Regex::new(r"(?i)(емости)$").unwrap(),
            строка: "емости",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+емостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостей)$").unwrap(),
            строка: "емостей",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+емостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостям)$").unwrap(),
            строка: "емостям",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+емостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостями)$").unwrap(),
            строка: "емостями",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+емостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостях)$").unwrap(),
            строка: "емостях",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+емым$").unwrap(),
            re_замены: Regex::new(r"(?i)(емым)$").unwrap(),
            строка: "емым",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+емыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(емыми)$").unwrap(),
            строка: "емыми",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+емый$").unwrap(),
            re_замены: Regex::new(r"(?i)(емый)$").unwrap(),
            строка: "емый",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+емые$").unwrap(),
            re_замены: Regex::new(r"(?i)(емые)$").unwrap(),
            строка: "емые",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+емых$").unwrap(),
            re_замены: Regex::new(r"(?i)(емых)$").unwrap(),
            строка: "емых",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+емся$").unwrap(),
            re_замены: Regex::new(r"(?i)(емся)$").unwrap(),
            строка: "емся",
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЕМ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ем",
            "емся",
            "еми",
            "емее",
            "ема",
            "емая",
            "емую",
            "емо",
            "емы",
            "емом",
            "емому",
            "емого",
            "емое",
            "емой",
            "емость",
            "емостью",
            "емости",
            "емостей",
            "емостям",
            "емостями",
            "емостях",
            "емым",
            "емыми",
            "емый",
            "емые",
            "емых",
        ])
    });
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЧН: LazyLock<[RE_полное_окончание; 22]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чна$").unwrap(),
            re_замены: Regex::new(r"(?i)(чна)$").unwrap(),
            строка: "чна",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чная$").unwrap(),
            re_замены: Regex::new(r"(?i)(чная)$").unwrap(),
            строка: "чная",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чную$").unwrap(),
            re_замены: Regex::new(r"(?i)(чную)$").unwrap(),
            строка: "чную",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+чно$").unwrap(),
            re_замены: Regex::new(r"(?i)(чно)$").unwrap(),
            строка: "чно",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+чны$").unwrap(),
            re_замены: Regex::new(r"(?i)(чны)$").unwrap(),
            строка: "чны",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+чном$").unwrap(),
            re_замены: Regex::new(r"(?i)(чном)$").unwrap(),
            строка: "чном",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чному$").unwrap(),
            re_замены: Regex::new(r"(?i)(чному)$").unwrap(),
            строка: "чному",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        // Дубликат "чно" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чного$").unwrap(),
            re_замены: Regex::new(r"(?i)(чного)$").unwrap(),
            строка: "чного",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чное$").unwrap(),
            re_замены: Regex::new(r"(?i)(чное)$").unwrap(),
            строка: "чное",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чной$").unwrap(),
            re_замены: Regex::new(r"(?i)(чной)$").unwrap(),
            строка: "чной",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чность$").unwrap(),
            re_замены: Regex::new(r"(?i)(чность)$").unwrap(),
            строка: "чность",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(чностью)$").unwrap(),
            строка: "чностью",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чности$").unwrap(),
            re_замены: Regex::new(r"(?i)(чности)$").unwrap(),
            строка: "чности",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(чностей)$").unwrap(),
            строка: "чностей",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(чностям)$").unwrap(),
            строка: "чностям",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(чностями)$").unwrap(),
            строка: "чностями",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(чностях)$").unwrap(),
            строка: "чностях",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чным$").unwrap(),
            re_замены: Regex::new(r"(?i)(чным)$").unwrap(),
            строка: "чным",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чными$").unwrap(),
            re_замены: Regex::new(r"(?i)(чными)$").unwrap(),
            строка: "чными",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чный$").unwrap(),
            re_замены: Regex::new(r"(?i)(чный)$").unwrap(),
            строка: "чный",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чные$").unwrap(),
            re_замены: Regex::new(r"(?i)(чные)$").unwrap(),
            строка: "чные",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+чных$").unwrap(),
            re_замены: Regex::new(r"(?i)(чных)$").unwrap(),
            строка: "чных",
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
    ]
});

//
pub static КУЧА_ОКОНЧАНИЙ_ЧН: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "чна",
        "чная",
        "чную",
        "чно",
        "чны",
        "чном",
        "чному",
        "чного",
        "чное",
        "чной",
        "чность",
        "чностью",
        "чности",
        "чностей",
        "чностям",
        "чностями",
        "чностях",
        "чным",
        "чными",
        "чный",
        "чные",
        "чных",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЖН: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+жен$").unwrap(),
            re_замены: Regex::new(r"(?i)(жен)$").unwrap(),
            строка: "жен",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жными$").unwrap(),
            re_замены: Regex::new(r"(?i)(жными)$").unwrap(),
            строка: "жными",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+жным$").unwrap(),
            re_замены: Regex::new(r"(?i)(жным)$").unwrap(),
            строка: "жным",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+жнее$").unwrap(),
            re_замены: Regex::new(r"(?i)(жнее)$").unwrap(),
            строка: "жнее",
            вид_окончания: Вид_Окончания::Жн,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жна$").unwrap(),
            re_замены: Regex::new(r"(?i)(жна)$").unwrap(),
            строка: "жна",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жная$").unwrap(),
            re_замены: Regex::new(r"(?i)(жная)$").unwrap(),
            строка: "жная",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жную$").unwrap(),
            re_замены: Regex::new(r"(?i)(жную)$").unwrap(),
            строка: "жную",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+жно$").unwrap(),
            re_замены: Regex::new(r"(?i)(жно)$").unwrap(),
            строка: "жно",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+жны$").unwrap(),
            re_замены: Regex::new(r"(?i)(жны)$").unwrap(),
            строка: "жны",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+жном$").unwrap(),
            re_замены: Regex::new(r"(?i)(жном)$").unwrap(),
            строка: "жном",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жному$").unwrap(),
            re_замены: Regex::new(r"(?i)(жному)$").unwrap(),
            строка: "жному",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        // Дубликат "жно" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жного$").unwrap(),
            re_замены: Regex::new(r"(?i)(жного)$").unwrap(),
            строка: "жного",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жное$").unwrap(),
            re_замены: Regex::new(r"(?i)(жное)$").unwrap(),
            строка: "жное",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жной$").unwrap(),
            re_замены: Regex::new(r"(?i)(жной)$").unwrap(),
            строка: "жной",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жность$").unwrap(),
            re_замены: Regex::new(r"(?i)(жность)$").unwrap(),
            строка: "жность",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(жностью)$").unwrap(),
            строка: "жностью",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жности$").unwrap(),
            re_замены: Regex::new(r"(?i)(жности)$").unwrap(),
            строка: "жности",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(жностей)$").unwrap(),
            строка: "жностей",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(жностям)$").unwrap(),
            строка: "жностям",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(жностями)$").unwrap(),
            строка: "жностями",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(жностях)$").unwrap(),
            строка: "жностях",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жный$").unwrap(),
            re_замены: Regex::new(r"(?i)(жный)$").unwrap(),
            строка: "жный",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жные$").unwrap(),
            re_замены: Regex::new(r"(?i)(жные)$").unwrap(),
            строка: "жные",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+жных$").unwrap(),
            re_замены: Regex::new(r"(?i)(жных)$").unwrap(),
            строка: "жных",
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЖН: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "жным",
        "жен",
        "жными",
        "жнее",
        "жна",
        "жная",
        "жную",
        "жно",
        "жны",
        "жном",
        "жному",
        "жного",
        "жное",
        "жной",
        "жность",
        "жностью",
        "жности",
        "жностей",
        "жностям",
        "жностями",
        "жностях",
        "жным",
        "жными",
        "жный",
        "жные",
        "жных",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ТН: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тен$").unwrap(),
            re_замены: Regex::new(r"(?i)(тен)$").unwrap(),
            строка: "тен",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тными$").unwrap(),
            re_замены: Regex::new(r"(?i)(тными)$").unwrap(),
            строка: "тными",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+тным$").unwrap(),
            re_замены: Regex::new(r"(?i)(тным)$").unwrap(),
            строка: "тным",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+тнее$").unwrap(),
            re_замены: Regex::new(r"(?i)(тнее)$").unwrap(),
            строка: "тнее",
            вид_окончания: Вид_Окончания::Тн,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тна$").unwrap(),
            re_замены: Regex::new(r"(?i)(тна)$").unwrap(),
            строка: "тна",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тная$").unwrap(),
            re_замены: Regex::new(r"(?i)(тная)$").unwrap(),
            строка: "тная",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тную$").unwrap(),
            re_замены: Regex::new(r"(?i)(тную)$").unwrap(),
            строка: "тную",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+тно$").unwrap(),
            re_замены: Regex::new(r"(?i)(тно)$").unwrap(),
            строка: "тно",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+тны$").unwrap(),
            re_замены: Regex::new(r"(?i)(тны)$").unwrap(),
            строка: "тны",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+тном$").unwrap(),
            re_замены: Regex::new(r"(?i)(тном)$").unwrap(),
            строка: "тном",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тному$").unwrap(),
            re_замены: Regex::new(r"(?i)(тному)$").unwrap(),
            строка: "тному",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        // Дубликат "тно" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тного$").unwrap(),
            re_замены: Regex::new(r"(?i)(тного)$").unwrap(),
            строка: "тного",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тное$").unwrap(),
            re_замены: Regex::new(r"(?i)(тное)$").unwrap(),
            строка: "тное",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тной$").unwrap(),
            re_замены: Regex::new(r"(?i)(тной)$").unwrap(),
            строка: "тной",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тность$").unwrap(),
            re_замены: Regex::new(r"(?i)(тность)$").unwrap(),
            строка: "тность",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(тностью)$").unwrap(),
            строка: "тностью",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тности$").unwrap(),
            re_замены: Regex::new(r"(?i)(тности)$").unwrap(),
            строка: "тности",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(тностей)$").unwrap(),
            строка: "тностей",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(тностям)$").unwrap(),
            строка: "тностям",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(тностями)$").unwrap(),
            строка: "тностями",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(тностях)$").unwrap(),
            строка: "тностях",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тный$").unwrap(),
            re_замены: Regex::new(r"(?i)(тный)$").unwrap(),
            строка: "тный",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тные$").unwrap(),
            re_замены: Regex::new(r"(?i)(тные)$").unwrap(),
            строка: "тные",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+тных$").unwrap(),
            re_замены: Regex::new(r"(?i)(тных)$").unwrap(),
            строка: "тных",
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ТН: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "тным",
        "тен",
        "тными",
        "тнее",
        "тна",
        "тная",
        "тную",
        "тно",
        "тны",
        "тном",
        "тному",
        "тного",
        "тное",
        "тной",
        "тность",
        "тностью",
        "тности",
        "тностей",
        "тностям",
        "тностями",
        "тностях",
        "тным",
        "тными",
        "тный",
        "тные",
        "тных",
    ])
});
// РАЗДЕЛ_ОКОНЧАНИЙ_АМ
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЫМ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыми)$").unwrap(),
            строка: "ыми",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ым)$").unwrap(),
            строка: "ым",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ымее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымее)$").unwrap(),
            строка: "ымее",
            вид_окончания: Вид_Окончания::Ым,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ыма$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыма)$").unwrap(),
            строка: "ыма",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымая)$").unwrap(),
            строка: "ымая",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымую)$").unwrap(),
            строка: "ымую",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ымо$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымо)$").unwrap(),
            строка: "ымо",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ымы$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымы)$").unwrap(),
            строка: "ымы",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ымом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымом)$").unwrap(),
            строка: "ымом",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымому)$").unwrap(),
            строка: "ымому",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        // Дубликат "ымо" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымого)$").unwrap(),
            строка: "ымого",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымое)$").unwrap(),
            строка: "ымое",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымой)$").unwrap(),
            строка: "ымой",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ымость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымость)$").unwrap(),
            строка: "ымость",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ымостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостью)$").unwrap(),
            строка: "ымостью",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ымости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымости)$").unwrap(),
            строка: "ымости",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ымостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостей)$").unwrap(),
            строка: "ымостей",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ымостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостям)$").unwrap(),
            строка: "ымостям",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ымостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостями)$").unwrap(),
            строка: "ымостями",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ымостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостях)$").unwrap(),
            строка: "ымостях",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымым)$").unwrap(),
            строка: "ымым",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымыми)$").unwrap(),
            строка: "ымыми",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымый)$").unwrap(),
            строка: "ымый",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымые)$").unwrap(),
            строка: "ымые",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ымых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымых)$").unwrap(),
            строка: "ымых",
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЫМ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ым",
        "ыми",
        "ымее",
        "ыма",
        "ымая",
        "ымую",
        "ымо",
        "ымы",
        "ымом",
        "ымому",
        "ымого",
        "ымое",
        "ымой",
        "ымость",
        "ымостью",
        "ымости",
        "ымостей",
        "ымостям",
        "ымостями",
        "ымостях",
        "ымым",
        "ымыми",
        "ымый",
        "ымые",
        "ымых",
    ])
});
// РАЗДЕЛ_ОКОНЧАНИЙ_АМ

pub static РАЗДЕЛ_ОКОНЧАНИЙ_АМ: LazyLock<[RE_полное_окончание; 29]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ам$").unwrap(),
            re_замены: Regex::new(r"(?i)(ам)$").unwrap(),
            строка: "ам",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+амее$").unwrap(),
            re_замены: Regex::new(r"(?i)(амее)$").unwrap(),
            строка: "амее",
            вид_окончания: Вид_Окончания::Ам,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ама$").unwrap(),
            re_замены: Regex::new(r"(?i)(ама)$").unwrap(),
            строка: "ама",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амая$").unwrap(),
            re_замены: Regex::new(r"(?i)(амая)$").unwrap(),
            строка: "амая",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амую$").unwrap(),
            re_замены: Regex::new(r"(?i)(амую)$").unwrap(),
            строка: "амую",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Местоимение, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+амо$").unwrap(),
            re_замены: Regex::new(r"(?i)(амо)$").unwrap(),
            строка: "амо",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+амы$").unwrap(),
            re_замены: Regex::new(r"(?i)(амы)$").unwrap(),
            строка: "амы",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амом$").unwrap(),
            re_замены: Regex::new(r"(?i)(амом)$").unwrap(),
            строка: "амом",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амому$").unwrap(),
            re_замены: Regex::new(r"(?i)(амому)$").unwrap(),
            строка: "амому",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        // Дубликат "амо" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амого$").unwrap(),
            re_замены: Regex::new(r"(?i)(амого)$").unwrap(),
            строка: "амого",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амое$").unwrap(),
            re_замены: Regex::new(r"(?i)(амое)$").unwrap(),
            строка: "амое",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+амой$").unwrap(),
            re_замены: Regex::new(r"(?i)(амой)$").unwrap(),
            строка: "амой",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+амость$").unwrap(),
            re_замены: Regex::new(r"(?i)(амость)$").unwrap(),
            строка: "амость",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+амостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостью)$").unwrap(),
            строка: "амостью",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+амости$").unwrap(),
            re_замены: Regex::new(r"(?i)(амости)$").unwrap(),
            строка: "амости",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+амостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостей)$").unwrap(),
            строка: "амостей",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+амостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостям)$").unwrap(),
            строка: "амостям",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+амостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостями)$").unwrap(),
            строка: "амостями",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+амостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостях)$").unwrap(),
            строка: "амостях",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амым$").unwrap(),
            re_замены: Regex::new(r"(?i)(амым)$").unwrap(),
            строка: "амым",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(амыми)$").unwrap(),
            строка: "амыми",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амый$").unwrap(),
            re_замены: Regex::new(r"(?i)(амый)$").unwrap(),
            строка: "амый",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амые$").unwrap(),
            re_замены: Regex::new(r"(?i)(амые)$").unwrap(),
            строка: "амые",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+амых$").unwrap(),
            re_замены: Regex::new(r"(?i)(амых)$").unwrap(),
            строка: "амых",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ами$").unwrap(),
            re_замены: Regex::new(r"(?i)(ами)$").unwrap(),
            строка: "ами",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+амим$").unwrap(),
            re_замены: Regex::new(r"(?i)(амим)$").unwrap(),
            строка: "амим",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+амих$").unwrap(),
            re_замены: Regex::new(r"(?i)(амих)$").unwrap(),
            строка: "амих",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+амся$").unwrap(),
            re_замены: Regex::new(r"(?i)(амся)$").unwrap(),
            строка: "амся",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+амими$").unwrap(),
            re_замены: Regex::new(r"(?i)(амими)$").unwrap(),
            строка: "амими",
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_АМ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ам",
        "амся",
        "ами",
        "амих",
        "амим",
        "амими",
        "амее",
        "ама",
        "амая",
        "амую",
        "амо",
        "амы",
        "амом",
        "амому",
        "амого",
        "амое",
        "амой",
        "амость",
        "амостью",
        "амости",
        "амостей",
        "амостям",
        "амостями",
        "амостях",
        "амым",
        "амыми",
        "амый",
        "амые",
        "амых",
    ])
});
// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ОМ
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ОМ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ом)$").unwrap(),
            строка: "ом",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+омее$").unwrap(),
            re_замены: Regex::new(r"(?i)(омее)$").unwrap(),
            строка: "омее",
            вид_окончания: Вид_Окончания::Ом,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ома$").unwrap(),
            re_замены: Regex::new(r"(?i)(ома)$").unwrap(),
            строка: "ома",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омая$").unwrap(),
            re_замены: Regex::new(r"(?i)(омая)$").unwrap(),
            строка: "омая",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омую$").unwrap(),
            re_замены: Regex::new(r"(?i)(омую)$").unwrap(),
            строка: "омую",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омо$").unwrap(),
            re_замены: Regex::new(r"(?i)(омо)$").unwrap(),
            строка: "омо",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омы$").unwrap(),
            re_замены: Regex::new(r"(?i)(омы)$").unwrap(),
            строка: "омы",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омом$").unwrap(),
            re_замены: Regex::new(r"(?i)(омом)$").unwrap(),
            строка: "омом",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омому$").unwrap(),
            re_замены: Regex::new(r"(?i)(омому)$").unwrap(),
            строка: "омому",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        // Дубликат "омо" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омого$").unwrap(),
            re_замены: Regex::new(r"(?i)(омого)$").unwrap(),
            строка: "омого",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омое$").unwrap(),
            re_замены: Regex::new(r"(?i)(омое)$").unwrap(),
            строка: "омое",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омой$").unwrap(),
            re_замены: Regex::new(r"(?i)(омой)$").unwrap(),
            строка: "омой",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+омость$").unwrap(),
            re_замены: Regex::new(r"(?i)(омость)$").unwrap(),
            строка: "омость",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+омостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостью)$").unwrap(),
            строка: "омостью",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+омости$").unwrap(),
            re_замены: Regex::new(r"(?i)(омости)$").unwrap(),
            строка: "омости",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+омостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостей)$").unwrap(),
            строка: "омостей",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+омостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостям)$").unwrap(),
            строка: "омостям",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+омостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостями)$").unwrap(),
            строка: "омостями",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+омостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостях)$").unwrap(),
            строка: "омостях",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омым$").unwrap(),
            re_замены: Regex::new(r"(?i)(омым)$").unwrap(),
            строка: "омым",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(омыми)$").unwrap(),
            строка: "омыми",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омый$").unwrap(),
            re_замены: Regex::new(r"(?i)(омый)$").unwrap(),
            строка: "омый",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омые$").unwrap(),
            re_замены: Regex::new(r"(?i)(омые)$").unwrap(),
            строка: "омые",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+омых$").unwrap(),
            re_замены: Regex::new(r"(?i)(омых)$").unwrap(),
            строка: "омых",
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ОМ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ом",
            "омее",
            "ома",
            "омая",
            "омую",
            "омо",
            "омы",
            "омом",
            "омому",
            "омого",
            "омое",
            "омой",
            "омость",
            "омостью",
            "омости",
            "омостей",
            "омостям",
            "омостями",
            "омостях",
            "омым",
            "омыми",
            "омый",
            "омые",
            "омых",
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ВАН
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ВАН: LazyLock<[RE_полное_окончание; 47]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ван$").unwrap(),
            re_замены: Regex::new(r"(?i)(ван)$").unwrap(),
            строка: "ван",
            вид_окончания: Вид_Окончания::Ван,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ванее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванее)$").unwrap(),
            строка: "ванее",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ваннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваннее)$").unwrap(),
            строка: "ваннее",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ванна$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванна)$").unwrap(),
            строка: "ванна",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вана$").unwrap(),
            re_замены: Regex::new(r"(?i)(вана)$").unwrap(),
            строка: "вана",
            вид_окончания: Вид_Окончания::Ван,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваная$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваная)$").unwrap(),
            строка: "ваная",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванная$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванная)$").unwrap(),
            строка: "ванная",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваную$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваную)$").unwrap(),
            строка: "ваную",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванную$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванную)$").unwrap(),
            строка: "ванную",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вано$").unwrap(),
            re_замены: Regex::new(r"(?i)(вано)$").unwrap(),
            строка: "вано",
            вид_окончания: Вид_Окончания::Ван,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ваны$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваны)$").unwrap(),
            строка: "ваны",
            вид_окончания: Вид_Окончания::Ван,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ванны$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванны)$").unwrap(),
            строка: "ванны",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваном$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваном)$").unwrap(),
            строка: "ваном",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванном$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванном)$").unwrap(),
            строка: "ванном",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваному$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваному)$").unwrap(),
            строка: "ваному",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванному$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванному)$").unwrap(),
            строка: "ванному",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваного$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваного)$").unwrap(),
            строка: "ваного",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванного$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванного)$").unwrap(),
            строка: "ванного",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+ванно$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванно)$").unwrap(),
            строка: "ванно",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваное$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваное)$").unwrap(),
            строка: "ваное",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванное$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванное)$").unwrap(),
            строка: "ванное",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванной$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванной)$").unwrap(),
            строка: "ванной",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ванность$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванность)$").unwrap(),
            строка: "ванность",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ваность$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваность)$").unwrap(),
            строка: "ваность",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ваностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваностью)$").unwrap(),
            строка: "ваностью",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ванностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностью)$").unwrap(),
            строка: "ванностью",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ваности$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваности)$").unwrap(),
            строка: "ваности",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ванности$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванности)$").unwrap(),
            строка: "ванности",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ваностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваностей)$").unwrap(),
            строка: "ваностей",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ванностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностей)$").unwrap(),
            строка: "ванностей",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ваностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваностям)$").unwrap(),
            строка: "ваностям",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ванностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностям)$").unwrap(),
            строка: "ванностям",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ваностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваностями)$").unwrap(),
            строка: "ваностями",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ванностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностями)$").unwrap(),
            строка: "ванностями",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ваностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваностях)$").unwrap(),
            строка: "ваностях",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ванностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностях)$").unwrap(),
            строка: "ванностях",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваным$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваным)$").unwrap(),
            строка: "ваным",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванным$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванным)$").unwrap(),
            строка: "ванным",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваными$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваными)$").unwrap(),
            строка: "ваными",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванными$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванными)$").unwrap(),
            строка: "ванными",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваный$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваный)$").unwrap(),
            строка: "ваный",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванный$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванный)$").unwrap(),
            строка: "ванный",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваные$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваные)$").unwrap(),
            строка: "ваные",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванные$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванные)$").unwrap(),
            строка: "ванные",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ванных$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванных)$").unwrap(),
            строка: "ванных",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваных$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваных)$").unwrap(),
            строка: "ваных",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ваной$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваной)$").unwrap(),
            строка: "ваной",
            вид_окончания: Вид_Окончания::Ван,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});
pub static КУЧА_ПОЛНАЯ_ОКОНЧАНИЙ_ВАН: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "ван",
        "ванее",
        "ваннее",
        "вана",
        "ванна",
        "вано",
        "ваны",
        "ванны",
        //
        "ваная",
        "ванная",
        "ваную",
        "ванную",
        //о
        "ваной",
        "ванной",
        "ванно",
        "ваное",
        "ванное",
        "ваного",
        "ванного",
        //глаголы
        "ванном",
        "ваном",
        "ваному",
        "ванному",
        //ст
        "ваность",
        "ванность",
        "ваностью",
        "ванностью",
        "ваности",
        "ванности",
        "ваностей",
        "ванностей",
        "ваностям",
        "ванностям",
        "ваностями",
        "ванностями",
        "ваностях",
        "ванностях",
        //ы
        "ваными",
        "ванными",
        "ваным",
        "ванным",
        "ваных",
        "ванных",
        "ваные",
        "ванные",
        "ванный",
        "ваный",
    ])
});
pub static КУЧА_ПОИСКА_ОКОНЧАНИЙ_ВАН: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "ван",
        "ваннее",
        "ванна",
        //"вано",
        "ванны",
        //
        "ванная",
        "ванную",
        //о
        //"ваной", //исключили чтобы не искало, так как добавляет в список недостающих окончаний в итого слово с 1 н - что является недопустимым
        "ванной",
        "ванно",
        "ванное",
        "ванного",
        //глаголы
        "ванном",
        "ванному",
        //ст
        "ванность",
        "ванностью",
        "ванности",
        "ванностей",
        "ванностям",
        "ванностями",
        "ванностях",
        //ы
        "ванными",
        "ванным",
        "ванных",
        "ванные",
        "ванный",
    ])
});
//
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ДАН: LazyLock<[RE_полное_окончание; 47]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+дан$").unwrap(),
            re_замены: Regex::new(r"(?i)(дан)$").unwrap(),
            строка: "дан",
            вид_окончания: Вид_Окончания::Дан,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+данее$").unwrap(),
            re_замены: Regex::new(r"(?i)(данее)$").unwrap(),
            строка: "данее",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+даннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(даннее)$").unwrap(),
            строка: "даннее",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+данна$").unwrap(),
            re_замены: Regex::new(r"(?i)(данна)$").unwrap(),
            строка: "данна",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+дана$").unwrap(),
            re_замены: Regex::new(r"(?i)(дана)$").unwrap(),
            строка: "дана",
            вид_окончания: Вид_Окончания::Дан,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даная$").unwrap(),
            re_замены: Regex::new(r"(?i)(даная)$").unwrap(),
            строка: "даная",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данная$").unwrap(),
            re_замены: Regex::new(r"(?i)(данная)$").unwrap(),
            строка: "данная",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даную$").unwrap(),
            re_замены: Regex::new(r"(?i)(даную)$").unwrap(),
            строка: "даную",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данную$").unwrap(),
            re_замены: Regex::new(r"(?i)(данную)$").unwrap(),
            строка: "данную",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+дано$").unwrap(),
            re_замены: Regex::new(r"(?i)(дано)$").unwrap(),
            строка: "дано",
            вид_окончания: Вид_Окончания::Дан,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+даны$").unwrap(),
            re_замены: Regex::new(r"(?i)(даны)$").unwrap(),
            строка: "даны",
            вид_окончания: Вид_Окончания::Дан,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+данны$").unwrap(),
            re_замены: Regex::new(r"(?i)(данны)$").unwrap(),
            строка: "данны",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даном$").unwrap(),
            re_замены: Regex::new(r"(?i)(даном)$").unwrap(),
            строка: "даном",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данном$").unwrap(),
            re_замены: Regex::new(r"(?i)(данном)$").unwrap(),
            строка: "данном",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даному$").unwrap(),
            re_замены: Regex::new(r"(?i)(даному)$").unwrap(),
            строка: "даному",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данному$").unwrap(),
            re_замены: Regex::new(r"(?i)(данному)$").unwrap(),
            строка: "данному",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даного$").unwrap(),
            re_замены: Regex::new(r"(?i)(даного)$").unwrap(),
            строка: "даного",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данного$").unwrap(),
            re_замены: Regex::new(r"(?i)(данного)$").unwrap(),
            строка: "данного",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+данно$").unwrap(),
            re_замены: Regex::new(r"(?i)(данно)$").unwrap(),
            строка: "данно",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даное$").unwrap(),
            re_замены: Regex::new(r"(?i)(даное)$").unwrap(),
            строка: "даное",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данное$").unwrap(),
            re_замены: Regex::new(r"(?i)(данное)$").unwrap(),
            строка: "данное",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данной$").unwrap(),
            re_замены: Regex::new(r"(?i)(данной)$").unwrap(),
            строка: "данной",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+данность$").unwrap(),
            re_замены: Regex::new(r"(?i)(данность)$").unwrap(),
            строка: "данность",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+даность$").unwrap(),
            re_замены: Regex::new(r"(?i)(даность)$").unwrap(),
            строка: "даность",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+даностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(даностью)$").unwrap(),
            строка: "даностью",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+данностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(данностью)$").unwrap(),
            строка: "данностью",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+даности$").unwrap(),
            re_замены: Regex::new(r"(?i)(даности)$").unwrap(),
            строка: "даности",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+данности$").unwrap(),
            re_замены: Regex::new(r"(?i)(данности)$").unwrap(),
            строка: "данности",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+даностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(даностей)$").unwrap(),
            строка: "даностей",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+данностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(данностей)$").unwrap(),
            строка: "данностей",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+даностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(даностям)$").unwrap(),
            строка: "даностям",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+данностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(данностям)$").unwrap(),
            строка: "данностям",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+даностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(даностями)$").unwrap(),
            строка: "даностями",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+данностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(данностями)$").unwrap(),
            строка: "данностями",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+даностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(даностях)$").unwrap(),
            строка: "даностях",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+данностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(данностях)$").unwrap(),
            строка: "данностях",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даным$").unwrap(),
            re_замены: Regex::new(r"(?i)(даным)$").unwrap(),
            строка: "даным",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данным$").unwrap(),
            re_замены: Regex::new(r"(?i)(данным)$").unwrap(),
            строка: "данным",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даными$").unwrap(),
            re_замены: Regex::new(r"(?i)(даными)$").unwrap(),
            строка: "даными",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данными$").unwrap(),
            re_замены: Regex::new(r"(?i)(данными)$").unwrap(),
            строка: "данными",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даный$").unwrap(),
            re_замены: Regex::new(r"(?i)(даный)$").unwrap(),
            строка: "даный",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данный$").unwrap(),
            re_замены: Regex::new(r"(?i)(данный)$").unwrap(),
            строка: "данный",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даные$").unwrap(),
            re_замены: Regex::new(r"(?i)(даные)$").unwrap(),
            строка: "даные",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данные$").unwrap(),
            re_замены: Regex::new(r"(?i)(данные)$").unwrap(),
            строка: "данные",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+данных$").unwrap(),
            re_замены: Regex::new(r"(?i)(данных)$").unwrap(),
            строка: "данных",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даных$").unwrap(),
            re_замены: Regex::new(r"(?i)(даных)$").unwrap(),
            строка: "даных",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+даной$").unwrap(),
            re_замены: Regex::new(r"(?i)(даной)$").unwrap(),
            строка: "даной",
            вид_окончания: Вид_Окончания::Дан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});
pub static КУЧА_ПОЛНАЯ_ОКОНЧАНИЙ_ДАН: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "дан",
        "данее",
        "даннее",
        "дана",
        "данна",
        "дано",
        "даны",
        "данны",
        //
        "даная",
        "данная",
        "даную",
        "данную",
        //о
        "даной",
        "данной",
        "данно",
        "даное",
        "данное",
        "даного",
        "данного",
        //глаголы
        "данном",
        "даном",
        "даному",
        "данному",
        //ст
        "даность",
        "данность",
        "даностью",
        "данностью",
        "даности",
        "данности",
        "даностей",
        "данностей",
        "даностям",
        "данностям",
        "даностями",
        "данностями",
        "даностях",
        "данностях",
        //ы
        "даными",
        "данными",
        "даным",
        "данным",
        "даных",
        "данных",
        "даные",
        "данные",
        "данный",
        "даный",
    ])
});
pub static КУЧА_ПОИСКА_ОКОНЧАНИЙ_ДАН: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "дан",
        "даннее",
        "данна",
        //"дано",
        "данны",
        //
        "данная",
        "данную",
        //о
        //"даной", //исключили чтобы не искало, так как добавляет в список недостающих окончаний в итого слово с 1 н - что является недопустимым
        "данной",
        "данно",
        "данное",
        "данного",
        //глаголы
        "данном",
        "данному",
        //ст
        "данность",
        "данностью",
        "данности",
        "данностей",
        "данностям",
        "данностями",
        "данностях",
        //ы
        "данными",
        "данным",
        "данных",
        "данные",
        "данный",
    ])
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ИВ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ив$").unwrap(),
            re_замены: Regex::new(r"(?i)(ив)$").unwrap(),
            строка: "ив",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивее)$").unwrap(),
            строка: "ивее",
            вид_окончания: Вид_Окончания::Ив,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ива$").unwrap(),
            re_замены: Regex::new(r"(?i)(ива)$").unwrap(),
            строка: "ива",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивая)$").unwrap(),
            строка: "ивая",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивую)$").unwrap(),
            строка: "ивую",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+иво$").unwrap(),
            re_замены: Regex::new(r"(?i)(иво)$").unwrap(),
            строка: "иво",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивы$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивы)$").unwrap(),
            строка: "ивы",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивом)$").unwrap(),
            строка: "ивом",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивому)$").unwrap(),
            строка: "ивому",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивого)$").unwrap(),
            строка: "ивого",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивое)$").unwrap(),
            строка: "ивое",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивой)$").unwrap(),
            строка: "ивой",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ивость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивость)$").unwrap(),
            строка: "ивость",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ивостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостью)$").unwrap(),
            строка: "ивостью",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ивости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивости)$").unwrap(),
            строка: "ивости",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ивостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостей)$").unwrap(),
            строка: "ивостей",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ивостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостям)$").unwrap(),
            строка: "ивостям",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ивостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостями)$").unwrap(),
            строка: "ивостями",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ивостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостях)$").unwrap(),
            строка: "ивостях",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивым)$").unwrap(),
            строка: "ивым",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивыми)$").unwrap(),
            строка: "ивыми",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивый)$").unwrap(),
            строка: "ивый",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивые)$").unwrap(),
            строка: "ивые",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ивых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивых)$").unwrap(),
            строка: "ивых",
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ИВ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ив",
            "ивее",
            "ива",
            "ивая",
            "ивую",
            "иво",
            "ивы",
            "ивом",
            "ивому",
            "ивого",
            // "иво" уже было выше — дубликат убран
            "ивое",
            "ивой",
            "ивость",
            "ивостью",
            "ивости",
            "ивостей",
            "ивостям",
            "ивостями",
            "ивостях",
            "ивым",
            "ивыми",
            "ивый",
            "ивые",
            "ивых",
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_АН
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_АН: LazyLock<[RE_полное_окончание; 47]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ан$").unwrap(),
            re_замены: Regex::new(r"(?i)(ан)$").unwrap(),
            строка: "ан",
            вид_окончания: Вид_Окончания::Ан,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ана$").unwrap(),
            re_замены: Regex::new(r"(?i)(ана)$").unwrap(),
            строка: "ана",
            вид_окончания: Вид_Окончания::Ан,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+анна$").unwrap(),
            re_замены: Regex::new(r"(?i)(анна)$").unwrap(),
            строка: "анна",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аная$").unwrap(),
            re_замены: Regex::new(r"(?i)(аная)$").unwrap(),
            строка: "аная",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анная$").unwrap(),
            re_замены: Regex::new(r"(?i)(анная)$").unwrap(),
            строка: "анная",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аную$").unwrap(),
            re_замены: Regex::new(r"(?i)(аную)$").unwrap(),
            строка: "аную",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анную$").unwrap(),
            re_замены: Regex::new(r"(?i)(анную)$").unwrap(),
            строка: "анную",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ано$").unwrap(),
            re_замены: Regex::new(r"(?i)(ано)$").unwrap(),
            строка: "ано",
            вид_окончания: Вид_Окончания::Ан,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+анее$").unwrap(),
            re_замены: Regex::new(r"(?i)(анее)$").unwrap(),
            строка: "анее",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+аннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(аннее)$").unwrap(),
            строка: "аннее",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аны$").unwrap(),
            re_замены: Regex::new(r"(?i)(аны)$").unwrap(),
            строка: "аны",
            вид_окончания: Вид_Окончания::Ан,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анны$").unwrap(),
            re_замены: Regex::new(r"(?i)(анны)$").unwrap(),
            строка: "анны",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аном$").unwrap(),
            re_замены: Regex::new(r"(?i)(аном)$").unwrap(),
            строка: "аном",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анном$").unwrap(),
            re_замены: Regex::new(r"(?i)(анном)$").unwrap(),
            строка: "анном",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аному$").unwrap(),
            re_замены: Regex::new(r"(?i)(аному)$").unwrap(),
            строка: "аному",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анному$").unwrap(),
            re_замены: Regex::new(r"(?i)(анному)$").unwrap(),
            строка: "анному",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анно$").unwrap(),
            re_замены: Regex::new(r"(?i)(анно)$").unwrap(),
            строка: "анно",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аного$").unwrap(),
            re_замены: Regex::new(r"(?i)(аного)$").unwrap(),
            строка: "аного",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анного$").unwrap(),
            re_замены: Regex::new(r"(?i)(анного)$").unwrap(),
            строка: "анного",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аное$").unwrap(),
            re_замены: Regex::new(r"(?i)(аное)$").unwrap(),
            строка: "аное",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анное$").unwrap(),
            re_замены: Regex::new(r"(?i)(анное)$").unwrap(),
            строка: "анное",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аной$").unwrap(),
            re_замены: Regex::new(r"(?i)(аной)$").unwrap(),
            строка: "аной",
            вид_окончания: Вид_Окончания::Ан,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анной$").unwrap(),
            re_замены: Regex::new(r"(?i)(анной)$").unwrap(),
            строка: "анной",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+аность$").unwrap(),
            re_замены: Regex::new(r"(?i)(аность)$").unwrap(),
            строка: "аность",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+анность$").unwrap(),
            re_замены: Regex::new(r"(?i)(анность)$").unwrap(),
            строка: "анность",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+аностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(аностью)$").unwrap(),
            строка: "аностью",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+анностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностью)$").unwrap(),
            строка: "анностью",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+аности$").unwrap(),
            re_замены: Regex::new(r"(?i)(аности)$").unwrap(),
            строка: "аности",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+анности$").unwrap(),
            re_замены: Regex::new(r"(?i)(анности)$").unwrap(),
            строка: "анности",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+аностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(аностей)$").unwrap(),
            строка: "аностей",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+анностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностей)$").unwrap(),
            строка: "анностей",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+аностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(аностям)$").unwrap(),
            строка: "аностям",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+анностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностям)$").unwrap(),
            строка: "анностям",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+аностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(аностями)$").unwrap(),
            строка: "аностями",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+анностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностями)$").unwrap(),
            строка: "анностями",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+аностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(аностях)$").unwrap(),
            строка: "аностях",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+анностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностях)$").unwrap(),
            строка: "анностях",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аным$").unwrap(),
            re_замены: Regex::new(r"(?i)(аным)$").unwrap(),
            строка: "аным",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анным$").unwrap(),
            re_замены: Regex::new(r"(?i)(анным)$").unwrap(),
            строка: "анным",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аными$").unwrap(),
            re_замены: Regex::new(r"(?i)(аными)$").unwrap(),
            строка: "аными",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анными$").unwrap(),
            re_замены: Regex::new(r"(?i)(анными)$").unwrap(),
            строка: "анными",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аный$").unwrap(),
            re_замены: Regex::new(r"(?i)(аный)$").unwrap(),
            строка: "аный",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анный$").unwrap(),
            re_замены: Regex::new(r"(?i)(анный)$").unwrap(),
            строка: "анный",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аные$").unwrap(),
            re_замены: Regex::new(r"(?i)(аные)$").unwrap(),
            строка: "аные",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анные$").unwrap(),
            re_замены: Regex::new(r"(?i)(анные)$").unwrap(),
            строка: "анные",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+аных$").unwrap(),
            re_замены: Regex::new(r"(?i)(аных)$").unwrap(),
            строка: "аных",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+анных$").unwrap(),
            re_замены: Regex::new(r"(?i)(анных)$").unwrap(),
            строка: "анных",
            вид_окончания: Вид_Окончания::Ан,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_АН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ан",
            "ана",
            "анна",
            "аная",
            "анная",
            "аную",
            "анную",
            "ано",
            "анее",
            "аннее",
            "аны",
            "анны",
            "аном",
            "анном",
            "аному",
            "анному",
            "анно",
            "аного",
            "анного",
            "аное",
            "анное",
            "аной",
            "анной",
            "аность",
            "анность",
            "аностью",
            "анностью",
            "аности",
            "анности",
            "аностей",
            "анностей",
            "аностям",
            "анностям",
            "аностями",
            "анностями",
            "аностях",
            "анностях",
            "аным",
            "анным",
            "аными",
            "анными",
            "аный",
            "анный",
            "аные",
            "анные",
            "аных",
            "анных",
        ])
    });
pub static КУЧА_ОКОНЧАНИЙ_ЯН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ян",
            "яна",
            "янная",
            "янную",
            "яно",
            "яннее",
            "яны",
            "яном",
            "янном",
            "янному",
            "янно",
            "янного",
            "янное",
            "яной",
            "янной",
            "янность",
            "янностью",
            "янности",
            "янностей",
            "янностям",
            "янностями",
            "янностях",
            "янным",
            "янными",
            "янный",
            "яные",
            "янные",
            "янных",
            "янна",
            "янны",
            "яных",
            "яными",
        ])
    });
// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_АВШ
// ============================================================
// // ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ЯН
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЯН: LazyLock<[RE_полное_окончание; 32]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ян$").unwrap(),
            re_замены: Regex::new(r"(?i)(ян)$").unwrap(),
            строка: "ян",
            вид_окончания: Вид_Окончания::Ян,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янна$").unwrap(),
            re_замены: Regex::new(r"(?i)(янна)$").unwrap(),
            строка: "янна",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+яна$").unwrap(),
            re_замены: Regex::new(r"(?i)(яна)$").unwrap(),
            строка: "яна",
            вид_окончания: Вид_Окончания::Ян,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янная$").unwrap(),
            re_замены: Regex::new(r"(?i)(янная)$").unwrap(),
            строка: "янная",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янную$").unwrap(),
            re_замены: Regex::new(r"(?i)(янную)$").unwrap(),
            строка: "янную",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+яно$").unwrap(),
            re_замены: Regex::new(r"(?i)(яно)$").unwrap(),
            строка: "яно",
            вид_окончания: Вид_Окончания::Ян,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+яннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(яннее)$").unwrap(),
            строка: "яннее",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янны$").unwrap(),
            re_замены: Regex::new(r"(?i)(янны)$").unwrap(),
            строка: "янны",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+яны$").unwrap(),
            re_замены: Regex::new(r"(?i)(яны)$").unwrap(),
            строка: "яны",
            вид_окончания: Вид_Окончания::Ян,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+яном$").unwrap(),
            re_замены: Regex::new(r"(?i)(яном)$").unwrap(),
            строка: "яном",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янном$").unwrap(),
            re_замены: Regex::new(r"(?i)(янном)$").unwrap(),
            строка: "янном",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янному$").unwrap(),
            re_замены: Regex::new(r"(?i)(янному)$").unwrap(),
            строка: "янному",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янно$").unwrap(),
            re_замены: Regex::new(r"(?i)(янно)$").unwrap(),
            строка: "янно",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янного$").unwrap(),
            re_замены: Regex::new(r"(?i)(янного)$").unwrap(),
            строка: "янного",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янное$").unwrap(),
            re_замены: Regex::new(r"(?i)(янное)$").unwrap(),
            строка: "янное",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+яной$").unwrap(),
            re_замены: Regex::new(r"(?i)(яной)$").unwrap(),
            строка: "яной",
            вид_окончания: Вид_Окончания::Ян,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янной$").unwrap(),
            re_замены: Regex::new(r"(?i)(янной)$").unwrap(),
            строка: "янной",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+янность$").unwrap(),
            re_замены: Regex::new(r"(?i)(янность)$").unwrap(),
            строка: "янность",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+янностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(янностью)$").unwrap(),
            строка: "янностью",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+янности$").unwrap(),
            re_замены: Regex::new(r"(?i)(янности)$").unwrap(),
            строка: "янности",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+янностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(янностей)$").unwrap(),
            строка: "янностей",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+янностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(янностям)$").unwrap(),
            строка: "янностям",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+янностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(янностями)$").unwrap(),
            строка: "янностями",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+янностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(янностях)$").unwrap(),
            строка: "янностях",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янным$").unwrap(),
            re_замены: Regex::new(r"(?i)(янным)$").unwrap(),
            строка: "янным",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+яными$").unwrap(),
            re_замены: Regex::new(r"(?i)(яными)$").unwrap(),
            строка: "яными",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янными$").unwrap(),
            re_замены: Regex::new(r"(?i)(янными)$").unwrap(),
            строка: "янными",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янный$").unwrap(),
            re_замены: Regex::new(r"(?i)(янный)$").unwrap(),
            строка: "янный",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янные$").unwrap(),
            re_замены: Regex::new(r"(?i)(янные)$").unwrap(),
            строка: "янные",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+яные$").unwrap(),
            re_замены: Regex::new(r"(?i)(яные)$").unwrap(),
            строка: "яные",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+янных$").unwrap(),
            re_замены: Regex::new(r"(?i)(янных)$").unwrap(),
            строка: "янных",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+яных$").unwrap(),
            re_замены: Regex::new(r"(?i)(яных)$").unwrap(),
            строка: "яных",
            вид_окончания: Вид_Окончания::Ян,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});
/*pub static РАЗДЕЛ_ОКОНЧАНИЙ_АВШ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ав$").unwrap(),
            re_замены: Regex::new(r"(?i)(ав)$").unwrap(),
            окончание_строка: "ав",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авша$").unwrap(),
            re_замены: Regex::new(r"(?i)(авша)$").unwrap(),
            окончание_строка: "авша",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшая$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшая)$").unwrap(),
            окончание_строка: "авшая",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшее$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшее)$").unwrap(),
            окончание_строка: "авшее",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшую$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшую)$").unwrap(),
            окончание_строка: "авшую",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авше$").unwrap(),
            re_замены: Regex::new(r"(?i)(авше)$").unwrap(),
            окончание_строка: "авше",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшее$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшее)$").unwrap(),
            окончание_строка: "авшее",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авши$").unwrap(),
            re_замены: Regex::new(r"(?i)(авши)$").unwrap(),
            окончание_строка: "авши",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшем$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшем)$").unwrap(),
            окончание_строка: "авшем",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшему$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшему)$").unwrap(),
            окончание_строка: "авшему",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшего$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшего)$").unwrap(),
            окончание_строка: "авшего",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшее$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшее)$").unwrap(),
            окончание_строка: "авшее",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшей$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшей)$").unwrap(),
            окончание_строка: "авшей",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+авшесть$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшесть)$").unwrap(),
            окончание_строка: "авшесть",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+авшестью$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестью)$").unwrap(),
            окончание_строка: "авшестью",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+авшести$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшести)$").unwrap(),
            окончание_строка: "авшести",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+авшестей$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестей)$").unwrap(),
            окончание_строка: "авшестей",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+авшестям$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестям)$").unwrap(),
            окончание_строка: "авшестям",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+авшестями$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестями)$").unwrap(),
            окончание_строка: "авшестями",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+авшестях$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестях)$").unwrap(),
            окончание_строка: "авшестях",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшим$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшим)$").unwrap(),
            окончание_строка: "авшим",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшими$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшими)$").unwrap(),
            окончание_строка: "авшими",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авший$").unwrap(),
            re_замены: Regex::new(r"(?i)(авший)$").unwrap(),
            окончание_строка: "авший",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авшие$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшие)$").unwrap(),
            окончание_строка: "авшие",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+авших$").unwrap(),
            re_замены: Regex::new(r"(?i)(авших)$").unwrap(),
            окончание_строка: "авших",
            вид_окончания: Вид_Окончания::Авш,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_АВШ: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "ав",
        "авша",
        "авшая",
        "авшее",
        "авшую",
        "авше",
        // "авшее" уже было выше — дубликат!
        // В вашем файле он есть дважды (позиции 4 и 7, 12)
        "авши",
        "авшем",
        "авшему",
        "авшего",
        "авшей",
        "авшесть",
        "авшестью",
        "авшести",
        "авшестей",
        "авшестям",
        "авшестями",
        "авшестях",
        "авшим",
        "авшими",
        "авший",
        "авшие",
        "авших",
    ])
});*/
//раздел ЫТ
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЫТ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ыт$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыт)$").unwrap(),
            строка: "ыт",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ыте$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыте)$").unwrap(),
            строка: "ыте",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ытее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытее)$").unwrap(),
            строка: "ытее",
            вид_окончания: Вид_Окончания::Ыт,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ыта$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыта)$").unwrap(),
            строка: "ыта",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытая)$").unwrap(),
            строка: "ытая",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытую)$").unwrap(),
            строка: "ытую",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ыто$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыто)$").unwrap(),
            строка: "ыто",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ыты$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыты)$").unwrap(),
            строка: "ыты",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытом)$").unwrap(),
            строка: "ытом",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытому)$").unwrap(),
            строка: "ытому",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        // Дубликат "ыто" удалён (уже есть выше)
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытого)$").unwrap(),
            строка: "ытого",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытое)$").unwrap(),
            строка: "ытое",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытой)$").unwrap(),
            строка: "ытой",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ытость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытость)$").unwrap(),
            строка: "ытость",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ытостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостью)$").unwrap(),
            строка: "ытостью",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ытости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытости)$").unwrap(),
            строка: "ытости",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ытостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостей)$").unwrap(),
            строка: "ытостей",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ытостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостям)$").unwrap(),
            строка: "ытостям",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ытостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостями)$").unwrap(),
            строка: "ытостями",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ытостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостях)$").unwrap(),
            строка: "ытостях",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытым)$").unwrap(),
            строка: "ытым",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытыми)$").unwrap(),
            строка: "ытыми",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытый)$").unwrap(),
            строка: "ытый",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытые)$").unwrap(),
            строка: "ытые",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ытых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытых)$").unwrap(),
            строка: "ытых",
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЫТ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ыт",
            "ыта",
            "ытая",
            "ытую",
            "ыто",
            "ытом",
            "ытому",
            "ытое",
            "ытой",
            "ытого",
            "ытость",
            "ытостью",
            "ытости",
            "ытостей",
            "ытостям",
            "ытостями",
            "ытостях",
            "ыты",
            "ытые",
            "ытым",
            "ытыми",
            "ытых",
            "ытый",
            "ыте",
            "ытее",
        ])
    });
//pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЩИ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| []);
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ТЫ: LazyLock<[RE_полное_окончание; 6]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тым$").unwrap(),
            re_замены: Regex::new(r"(?i)(тым)$").unwrap(),
            строка: "тым",
            вид_окончания: Вид_Окончания::Ты,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(тыми)$").unwrap(),
            строка: "тыми",
            вид_окончания: Вид_Окончания::Ты,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тый$").unwrap(),
            re_замены: Regex::new(r"(?i)(тый)$").unwrap(),
            строка: "тый",
            вид_окончания: Вид_Окончания::Ты,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тые$").unwrap(),
            re_замены: Regex::new(r"(?i)(тые)$").unwrap(),
            строка: "тые",
            вид_окончания: Вид_Окончания::Ты,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тых$").unwrap(),
            re_замены: Regex::new(r"(?i)(тых)$").unwrap(),
            строка: "тых",
            вид_окончания: Вид_Окончания::Ты,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ты$").unwrap(),
            re_замены: Regex::new(r"(?i)(ты)$").unwrap(),
            строка: "ты",
            вид_окончания: Вид_Окончания::Ты,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ТЫ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ты", "тые", "тым", "тыми", "тых", "тый", //"те", "тее",
        ])
    });
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ТИ: LazyLock<[RE_полное_окончание; 8]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тии$").unwrap(),
            re_замены: Regex::new(r"(?i)(тии)$").unwrap(),
            строка: "тии",
            вид_окончания: Вид_Окончания::Ти,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тий$").unwrap(),
            re_замены: Regex::new(r"(?i)(тий)$").unwrap(),
            строка: "тий",
            вид_окончания: Вид_Окончания::Ти,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тие$").unwrap(),
            re_замены: Regex::new(r"(?i)(тие)$").unwrap(),
            строка: "тие",
            вид_окончания: Вид_Окончания::Ти,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тию$").unwrap(),
            re_замены: Regex::new(r"(?i)(тию)$").unwrap(),
            строка: "тию",
            вид_окончания: Вид_Окончания::Ти,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тиям$").unwrap(),
            re_замены: Regex::new(r"(?i)(тиям)$").unwrap(),
            строка: "тиям",
            вид_окончания: Вид_Окончания::Ти,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тия$").unwrap(),
            re_замены: Regex::new(r"(?i)(тия)$").unwrap(),
            строка: "тия",
            вид_окончания: Вид_Окончания::Ти,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тиями$").unwrap(),
            re_замены: Regex::new(r"(?i)(тиями)$").unwrap(),
            строка: "тиями",
            вид_окончания: Вид_Окончания::Ти,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тиях$").unwrap(),
            re_замены: Regex::new(r"(?i)(тиях)$").unwrap(),
            строка: "тиях",
            вид_окончания: Вид_Окончания::Ти,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ТИ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter(["тий", "тие", "тии", "тию", "тиям", "тиями", "тиях", "тия"])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ТА_ТО: LazyLock<[RE_полное_окончание; 20]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+та$").unwrap(),
            re_замены: Regex::new(r"(?i)(та)$").unwrap(),
            строка: "та",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тех$").unwrap(),
            re_замены: Regex::new(r"(?i)(тех)$").unwrap(),
            строка: "тех",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+те$").unwrap(),
            re_замены: Regex::new(r"(?i)(те)$").unwrap(),
            строка: "те",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тая$").unwrap(),
            re_замены: Regex::new(r"(?i)(тая)$").unwrap(),
            строка: "тая",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тую$").unwrap(),
            re_замены: Regex::new(r"(?i)(тую)$").unwrap(),
            строка: "тую",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+т$").unwrap(),
            re_замены: Regex::new(r"(?i)(т)$").unwrap(),
            строка: "т",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+то$").unwrap(),
            re_замены: Regex::new(r"(?i)(то)$").unwrap(),
            строка: "то",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+той$").unwrap(),
            re_замены: Regex::new(r"(?i)(той)$").unwrap(),
            строка: "той",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тее$").unwrap(),
            re_замены: Regex::new(r"(?i)(тее)$").unwrap(),
            строка: "тее",
            вид_окончания: Вид_Окончания::Та_то,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тое$").unwrap(),
            re_замены: Regex::new(r"(?i)(тое)$").unwrap(),
            строка: "тое",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+тому$").unwrap(),
            re_замены: Regex::new(r"(?i)(тому)$").unwrap(),
            строка: "тому",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+том$").unwrap(),
            re_замены: Regex::new(r"(?i)(том)$").unwrap(),
            строка: "том",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+того$").unwrap(),
            re_замены: Regex::new(r"(?i)(того)$").unwrap(),
            строка: "того",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тость$").unwrap(),
            re_замены: Regex::new(r"(?i)(тость)$").unwrap(),
            строка: "тость",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостью)$").unwrap(),
            строка: "тостью",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тости$").unwrap(),
            re_замены: Regex::new(r"(?i)(тости)$").unwrap(),
            строка: "тости",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостей)$").unwrap(),
            строка: "тостей",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостям)$").unwrap(),
            строка: "тостям",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостями)$").unwrap(),
            строка: "тостями",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+тостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостях)$").unwrap(),
            строка: "тостях",
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ТА_ТО: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "т",
        "те",
        "той",
        "та",
        "тая",
        "тую",
        "то",
        "тее",
        "тое",
        "том",
        "тому",
        "того",
        "тость",
        "тостью",
        "тости",
        "тостей",
        "тостям",
        "тостями",
        "тостях",
        "тех",
    ])
});
//
/*pub static РАЗДЕЛ_ОКОНЧАНИЙ_ВА_ВО: LazyLock<[RE_полное_окончание; 15]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ва$").unwrap(),
            re_замены: Regex::new(r"(?i)(ва)$").unwrap(),
            окончание_строка: "ва",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вая$").unwrap(),
            re_замены: Regex::new(r"(?i)(вая)$").unwrap(),
            окончание_строка: "вая",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вую$").unwrap(),
            re_замены: Regex::new(r"(?i)(вую)$").unwrap(),
            окончание_строка: "вую",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+во$").unwrap(),
            re_замены: Regex::new(r"(?i)(во)$").unwrap(),
            окончание_строка: "во",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вой$").unwrap(),
            re_замены: Regex::new(r"(?i)(вой)$").unwrap(),
            окончание_строка: "вой",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вое$").unwrap(),
            re_замены: Regex::new(r"(?i)(вое)$").unwrap(),
            окончание_строка: "вое",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вом$").unwrap(),
            re_замены: Regex::new(r"(?i)(вом)$").unwrap(),
            окончание_строка: "вом",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вого$").unwrap(),
            re_замены: Regex::new(r"(?i)(вого)$").unwrap(),
            окончание_строка: "вого",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вость$").unwrap(),
            re_замены: Regex::new(r"(?i)(вость)$").unwrap(),
            окончание_строка: "вость",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+востью$").unwrap(),
            re_замены: Regex::new(r"(?i)(востью)$").unwrap(),
            окончание_строка: "востью",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вости$").unwrap(),
            re_замены: Regex::new(r"(?i)(вости)$").unwrap(),
            окончание_строка: "вости",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+востей$").unwrap(),
            re_замены: Regex::new(r"(?i)(востей)$").unwrap(),
            окончание_строка: "востей",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+востям$").unwrap(),
            re_замены: Regex::new(r"(?i)(востям)$").unwrap(),
            окончание_строка: "востям",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+востями$").unwrap(),
            re_замены: Regex::new(r"(?i)(востями)$").unwrap(),
            окончание_строка: "востями",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+востях$").unwrap(),
            re_замены: Regex::new(r"(?i)(востях)$").unwrap(),
            окончание_строка: "востях",
            вид_окончания: Вид_Окончания::Ва_Во,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ВА_ВО: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "вой",
        "ва",
        "вая",
        "вую",
        "во",
        "вое",
        "вом",
        "вого",
        "вость",
        "востью",
        "вости",
        "востей",
        "востям",
        "востями",
        "востях",
    ])
});*/
//
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СТВ: LazyLock<[RE_полное_окончание; 9]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ства$").unwrap(),
            re_замены: Regex::new(r"(?i)(ства)$").unwrap(),
            строка: "ства",
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ству$").unwrap(),
            re_замены: Regex::new(r"(?i)(ству)$").unwrap(),
            строка: "ству",
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+стве$").unwrap(),
            re_замены: Regex::new(r"(?i)(стве)$").unwrap(),
            строка: "стве",
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ств$").unwrap(),
            re_замены: Regex::new(r"(?i)(ств)$").unwrap(),
            строка: "ств",
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ствам$").unwrap(),
            re_замены: Regex::new(r"(?i)(ствам)$").unwrap(),
            строка: "ствам",
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ствами$").unwrap(),
            re_замены: Regex::new(r"(?i)(ствами)$").unwrap(),
            строка: "ствами",
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ство$").unwrap(),
            re_замены: Regex::new(r"(?i)(ство)$").unwrap(),
            строка: "ство",
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ством$").unwrap(),
            re_замены: Regex::new(r"(?i)(ством)$").unwrap(),
            строка: "ством",
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ствах$").unwrap(),
            re_замены: Regex::new(r"(?i)(ствах)$").unwrap(),
            строка: "ствах",
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
    ]
});

//

//
pub static КУЧА_ОКОНЧАНИЙ_СТВ: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "ства",
        "ству",
        "стве",
        "ств",
        "ство",
        "ствам",
        "ствами",
        "ством",
        "ствах",
    ])
});
pub static КУЧА_ОКОНЧАНИЙ_НЯТН: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "нятен",
        "нятнее",
        "нятна",
        "нятная",
        "нятную",
        "нятное",
        "нятны",
        "нятном",
        "нятному",
        "нятного",
        "нятно",
        "нятной",
        "нятность",
        "нятностью",
        "нятности",
        "нятностей",
        "нятностям",
        "нятностями",
        "нятностях",
        "нятным",
        "нятными",
        "нятный",
        "нятные",
        "нятных",
    ])
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_НЯТН: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нятнее$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятнее)$").unwrap(),
            строка: "нятнее",
            вид_окончания: Вид_Окончания::Нятн,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятен$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятен)$").unwrap(),
            строка: "нятен",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятна$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятна)$").unwrap(),
            строка: "нятна",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нятная$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятная)$").unwrap(),
            строка: "нятная",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятную$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятную)$").unwrap(),
            строка: "нятную",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятное$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятное)$").unwrap(),
            строка: "нятное",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятны$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятны)$").unwrap(),
            строка: "нятны",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятном$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятном)$").unwrap(),
            строка: "нятном",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятному$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятному)$").unwrap(),
            строка: "нятному",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятного$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятного)$").unwrap(),
            строка: "нятного",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нятно$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятно)$").unwrap(),
            строка: "нятно",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятной$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятной)$").unwrap(),
            строка: "нятной",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятность$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятность)$").unwrap(),
            строка: "нятность",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятностью)$").unwrap(),
            строка: "нятностью",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятности$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятности)$").unwrap(),
            строка: "нятности",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятностей)$").unwrap(),
            строка: "нятностей",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятностям)$").unwrap(),
            строка: "нятностям",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятностями)$").unwrap(),
            строка: "нятностями",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятностях)$").unwrap(),
            строка: "нятностях",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нятным$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятным)$").unwrap(),
            строка: "нятным",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятными$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятными)$").unwrap(),
            строка: "нятными",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятный$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятный)$").unwrap(),
            строка: "нятный",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятные$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятные)$").unwrap(),
            строка: "нятные",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятных$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятных)$").unwrap(),
            строка: "нятных",
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
    ]
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_НЯТ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нятее$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятее)$").unwrap(),
            строка: "нятее",
            вид_окончания: Вид_Окончания::Нят,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нят$").unwrap(),
            re_замены: Regex::new(r"(?i)(нят)$").unwrap(),
            строка: "нят",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нята$").unwrap(),
            re_замены: Regex::new(r"(?i)(нята)$").unwrap(),
            строка: "нята",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нятая$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятая)$").unwrap(),
            строка: "нятая",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятую$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятую)$").unwrap(),
            строка: "нятую",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятое$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятое)$").unwrap(),
            строка: "нятое",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+няты$").unwrap(),
            re_замены: Regex::new(r"(?i)(няты)$").unwrap(),
            строка: "няты",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятом$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятом)$").unwrap(),
            строка: "нятом",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятому$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятому)$").unwrap(),
            строка: "нятому",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятого$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятого)$").unwrap(),
            строка: "нятого",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нято$").unwrap(),
            re_замены: Regex::new(r"(?i)(нято)$").unwrap(),
            строка: "нято",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятой$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятой)$").unwrap(),
            строка: "нятой",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятость$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятость)$").unwrap(),
            строка: "нятость",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостью)$").unwrap(),
            строка: "нятостью",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятости$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятости)$").unwrap(),
            строка: "нятости",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостей)$").unwrap(),
            строка: "нятостей",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостям)$").unwrap(),
            строка: "нятостям",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостями)$").unwrap(),
            строка: "нятостями",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нятостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостях)$").unwrap(),
            строка: "нятостях",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нятым$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятым)$").unwrap(),
            строка: "нятым",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятыми)$").unwrap(),
            строка: "нятыми",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятый$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятый)$").unwrap(),
            строка: "нятый",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятые$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятые)$").unwrap(),
            строка: "нятые",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нятых$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятых)$").unwrap(),
            строка: "нятых",
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
    ]
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_УТ: LazyLock<[RE_полное_окончание; 37]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+утся$").unwrap(),
            re_замены: Regex::new(r"(?i)(утся)$").unwrap(),
            строка: "утся",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Глагол,
                Вид_части_речи::Причастие,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ут$").unwrap(),
            re_замены: Regex::new(r"(?i)(ут)$").unwrap(),
            строка: "ут",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+уте$").unwrap(),
            re_замены: Regex::new(r"(?i)(уте)$").unwrap(),
            строка: "уте",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ута$").unwrap(),
            re_замены: Regex::new(r"(?i)(ута)$").unwrap(),
            строка: "ута",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+утая$").unwrap(),
            re_замены: Regex::new(r"(?i)(утая)$").unwrap(),
            строка: "утая",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+утую$").unwrap(),
            re_замены: Regex::new(r"(?i)(утую)$").unwrap(),
            строка: "утую",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+утое$").unwrap(),
            re_замены: Regex::new(r"(?i)(утое)$").unwrap(),
            строка: "утое",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+уты$").unwrap(),
            re_замены: Regex::new(r"(?i)(уты)$").unwrap(),
            строка: "уты",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+утом$").unwrap(),
            re_замены: Regex::new(r"(?i)(утом)$").unwrap(),
            строка: "утом",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+утому$").unwrap(),
            re_замены: Regex::new(r"(?i)(утому)$").unwrap(),
            строка: "утому",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+утого$").unwrap(),
            re_замены: Regex::new(r"(?i)(утого)$").unwrap(),
            строка: "утого",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+уто$").unwrap(),
            re_замены: Regex::new(r"(?i)(уто)$").unwrap(),
            строка: "уто",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+утой$").unwrap(),
            re_замены: Regex::new(r"(?i)(утой)$").unwrap(),
            строка: "утой",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+утость$").unwrap(),
            re_замены: Regex::new(r"(?i)(утость)$").unwrap(),
            строка: "утость",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+утостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостью)$").unwrap(),
            строка: "утостью",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+утости$").unwrap(),
            re_замены: Regex::new(r"(?i)(утости)$").unwrap(),
            строка: "утости",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+утостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостей)$").unwrap(),
            строка: "утостей",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+утостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостям)$").unwrap(),
            строка: "утостям",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+утостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостями)$").unwrap(),
            строка: "утостями",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+утостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостях)$").unwrap(),
            строка: "утостях",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+утым$").unwrap(),
            re_замены: Regex::new(r"(?i)(утым)$").unwrap(),
            строка: "утым",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+утыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(утыми)$").unwrap(),
            строка: "утыми",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+утый$").unwrap(),
            re_замены: Regex::new(r"(?i)(утый)$").unwrap(),
            строка: "утый",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+утые$").unwrap(),
            re_замены: Regex::new(r"(?i)(утые)$").unwrap(),
            строка: "утые",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+утых$").unwrap(),
            re_замены: Regex::new(r"(?i)(утых)$").unwrap(),
            строка: "утых",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+уй$").unwrap(),
            re_замены: Regex::new(r"(?i)(уй)$").unwrap(),
            строка: "уй",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+уйте$").unwrap(),
            re_замены: Regex::new(r"(?i)(уйте)$").unwrap(),
            строка: "уйте",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Деепричастие],
            re_поиска: Regex::new(r"(?i)\w+уя$").unwrap(),
            re_замены: Regex::new(r"(?i)(уя)$").unwrap(),
            строка: "уя",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Деепричастие],
            re_поиска: Regex::new(r"(?i)\w+уясь$").unwrap(),
            re_замены: Regex::new(r"(?i)(уясь)$").unwrap(),
            строка: "уясь",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Деепричастие],
            re_поиска: Regex::new(r"(?i)\w+ую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ую)$").unwrap(),
            строка: "ую",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Деепричастие],
            re_поиска: Regex::new(r"(?i)\w+уюсь$").unwrap(),
            re_замены: Regex::new(r"(?i)(уюсь)$").unwrap(),
            строка: "уюсь",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+уют$").unwrap(),
            re_замены: Regex::new(r"(?i)(уют)$").unwrap(),
            строка: "уют",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+уются$").unwrap(),
            re_замены: Regex::new(r"(?i)(уются)$").unwrap(),
            строка: "уются",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ует$").unwrap(),
            re_замены: Regex::new(r"(?i)(ует)$").unwrap(),
            строка: "ует",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+уется$").unwrap(),
            re_замены: Regex::new(r"(?i)(уется)$").unwrap(),
            строка: "уется",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+уем$").unwrap(),
            re_замены: Regex::new(r"(?i)(уем)$").unwrap(),
            строка: "уем",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+уемся$").unwrap(),
            re_замены: Regex::new(r"(?i)(уемся)$").unwrap(),
            строка: "уемся",
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
    ]
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_У: LazyLock<[RE_полное_окончание; 1]> = LazyLock::new(|| {
    [RE_полное_окончание {
        части_речи: &[Вид_части_речи::Существительное],
        re_поиска: Regex::new(r"(?i)\w+у$").unwrap(),
        re_замены: Regex::new(r"(?i)(у)$").unwrap(),
        строка: "у",
        вид_окончания: Вид_Окончания::_У,
        ..Default::default()
    }]
});
pub static КУЧА_ОКОНЧАНИЙ_У: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter(["у"]));
pub static КУЧА_ОКОНЧАНИЙ_УТ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ут",
        "уют",
        "ует",
        "уем",
        "уемся",
        "уется",
        "уются",
        "ую",
        "уюсь",
        "уя",
        "уясь",
        "уй",
        "уйте",
        "утся",
        "уте",
        "ута",
        "утая",
        "утую",
        "утое",
        "уты",
        "утом",
        "утому",
        "утого",
        "уто",
        "утой",
        "утость",
        "утостью",
        "утости",
        "утостей",
        "утостям",
        "утостями",
        "утостях",
        "утым",
        "утыми",
        "утый",
        "утые",
        "утых",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЮТ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+юте$").unwrap(),
            re_замены: Regex::new(r"(?i)(юте)$").unwrap(),
            строка: "юте",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ют$").unwrap(),
            re_замены: Regex::new(r"(?i)(ют)$").unwrap(),
            строка: "ют",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ются$").unwrap(),
            re_замены: Regex::new(r"(?i)(ются)$").unwrap(),
            строка: "ются",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+юта$").unwrap(),
            re_замены: Regex::new(r"(?i)(юта)$").unwrap(),
            строка: "юта",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ютая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютая)$").unwrap(),
            строка: "ютая",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ютую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютую)$").unwrap(),
            строка: "ютую",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ютое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютое)$").unwrap(),
            строка: "ютое",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+юты$").unwrap(),
            re_замены: Regex::new(r"(?i)(юты)$").unwrap(),
            строка: "юты",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ютом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютом)$").unwrap(),
            строка: "ютом",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ютому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютому)$").unwrap(),
            строка: "ютому",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ютого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютого)$").unwrap(),
            строка: "ютого",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+юто$").unwrap(),
            re_замены: Regex::new(r"(?i)(юто)$").unwrap(),
            строка: "юто",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ютой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютой)$").unwrap(),
            строка: "ютой",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ютость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютость)$").unwrap(),
            строка: "ютость",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ютостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостью)$").unwrap(),
            строка: "ютостью",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ютости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютости)$").unwrap(),
            строка: "ютости",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ютостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостей)$").unwrap(),
            строка: "ютостей",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ютостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостям)$").unwrap(),
            строка: "ютостям",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ютостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостями)$").unwrap(),
            строка: "ютостями",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ютостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостях)$").unwrap(),
            строка: "ютостях",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ютым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютым)$").unwrap(),
            строка: "ютым",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ютыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютыми)$").unwrap(),
            строка: "ютыми",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ютый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютый)$").unwrap(),
            строка: "ютый",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ютые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютые)$").unwrap(),
            строка: "ютые",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ютых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютых)$").unwrap(),
            строка: "ютых",
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
    ]
});

pub static КУЧА_ОКОНЧАНИЙ_ЮТ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ют",
        "ются",
        "юте",
        "юта",
        "ютая",
        "ютую",
        "ютое",
        "юты",
        "ютом",
        "ютому",
        "ютого",
        "юто",
        "ютой",
        "ютость",
        "ютостью",
        "ютости",
        "ютостей",
        "ютостям",
        "ютостями",
        "ютостях",
        "ютым",
        "ютыми",
        "ютый",
        "ютые",
        "ютых",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Ю: LazyLock<[RE_полное_окончание; 1]> = LazyLock::new(|| {
    [RE_полное_окончание {
        части_речи: &[
            Вид_части_речи::Существительное,
            Вид_части_речи::Прилагательное,
        ],
        re_поиска: Regex::new(r"(?i)\w+ю$").unwrap(),
        re_замены: Regex::new(r"(?i)(ю)$").unwrap(),
        строка: "ю",
        вид_окончания: Вид_Окончания::_Ю,
        ..Default::default()
    }]
});
pub static КУЧА_ОКОНЧАНИЙ_Ю: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter(["ю"]));
//
pub static КУЧА_ОКОНЧАНИЙ_НЯТ: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "нят",
        "нятее",
        "нята",
        "нятая",
        "нятую",
        "нятое",
        "няты",
        "нятом",
        "нятому",
        "нятого",
        "нято",
        "нятой",
        "нятость",
        "нятостью",
        "нятости",
        "нятостей",
        "нятостям",
        "нятостями",
        "нятостях",
        "нятым",
        "нятыми",
        "нятый",
        "нятые",
        "нятых",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЪЯТ: LazyLock<[RE_полное_окончание; 23]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъят$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъят)$").unwrap(),
            строка: "ъят",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъята$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъята)$").unwrap(),
            строка: "ъята",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятая)$").unwrap(),
            строка: "ъятая",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятую)$").unwrap(),
            строка: "ъятую",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятое)$").unwrap(),
            строка: "ъятое",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъяты$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъяты)$").unwrap(),
            строка: "ъяты",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятом)$").unwrap(),
            строка: "ъятом",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятому)$").unwrap(),
            строка: "ъятому",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятого)$").unwrap(),
            строка: "ъятого",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъято$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъято)$").unwrap(),
            строка: "ъято",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятой)$").unwrap(),
            строка: "ъятой",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ъятость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятость)$").unwrap(),
            строка: "ъятость",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ъятостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостью)$").unwrap(),
            строка: "ъятостью",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ъятости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятости)$").unwrap(),
            строка: "ъятости",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ъятостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостей)$").unwrap(),
            строка: "ъятостей",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ъятостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостям)$").unwrap(),
            строка: "ъятостям",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ъятостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостями)$").unwrap(),
            строка: "ъятостями",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ъятостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостях)$").unwrap(),
            строка: "ъятостях",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятым)$").unwrap(),
            строка: "ъятым",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятыми)$").unwrap(),
            строка: "ъятыми",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятый)$").unwrap(),
            строка: "ъятый",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ъятые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятые)$").unwrap(),
            строка: "ъятые",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ъятых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятых)$").unwrap(),
            строка: "ъятых",
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЪЯТ: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "ъят",
        "ъята",
        "ъятая",
        "ъятую",
        "ъятое",
        "ъяты",
        "ъятом",
        "ъятому",
        "ъятого",
        "ъято",
        "ъятой",
        "ъятость",
        "ъятостью",
        "ъятости",
        "ъятостей",
        "ъятостям",
        "ъятостями",
        "ъятостях",
        "ъятым",
        "ъятыми",
        "ъятый",
        "ъятые",
        "ъятых",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Щ: LazyLock<[RE_полное_окончание; 37]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щею$").unwrap(),
            re_замены: Regex::new(r"(?i)(щею)$").unwrap(),
            строка: "щею",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ща$").unwrap(),
            re_замены: Regex::new(r"(?i)(ща)$").unwrap(),
            строка: "ща",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щая$").unwrap(),
            re_замены: Regex::new(r"(?i)(щая)$").unwrap(),
            строка: "щая",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щаяся$").unwrap(),
            re_замены: Regex::new(r"(?i)(щаяся)$").unwrap(),
            строка: "щаяся",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щую$").unwrap(),
            re_замены: Regex::new(r"(?i)(щую)$").unwrap(),
            строка: "щую",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щуюся$").unwrap(),
            re_замены: Regex::new(r"(?i)(щуюся)$").unwrap(),
            строка: "щуюся",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щее$").unwrap(),
            re_замены: Regex::new(r"(?i)(щее)$").unwrap(),
            строка: "щее",
            вид_окончания: Вид_Окончания::_Щ,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щееся$").unwrap(),
            re_замены: Regex::new(r"(?i)(щееся)$").unwrap(),
            строка: "щееся",
            вид_окончания: Вид_Окончания::_Щ,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щи$").unwrap(),
            re_замены: Regex::new(r"(?i)(щи)$").unwrap(),
            строка: "щи",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щеми$").unwrap(),
            re_замены: Regex::new(r"(?i)(щеми)$").unwrap(),
            строка: "щеми",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щем$").unwrap(),
            re_замены: Regex::new(r"(?i)(щем)$").unwrap(),
            строка: "щем",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щемся$").unwrap(),
            re_замены: Regex::new(r"(?i)(щемся)$").unwrap(),
            строка: "щемся",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щемуся$").unwrap(),
            re_замены: Regex::new(r"(?i)(щемуся)$").unwrap(),
            строка: "щемуся",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щему$").unwrap(),
            re_замены: Regex::new(r"(?i)(щему)$").unwrap(),
            строка: "щему",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щего$").unwrap(),
            re_замены: Regex::new(r"(?i)(щего)$").unwrap(),
            строка: "щего",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щегося$").unwrap(),
            re_замены: Regex::new(r"(?i)(щегося)$").unwrap(),
            строка: "щегося",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ще$").unwrap(),
            re_замены: Regex::new(r"(?i)(ще)$").unwrap(),
            строка: "ще",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щейся$").unwrap(),
            re_замены: Regex::new(r"(?i)(щейся)$").unwrap(),
            строка: "щейся",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щей$").unwrap(),
            re_замены: Regex::new(r"(?i)(щей)$").unwrap(),
            строка: "щей",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+щесть$").unwrap(),
            re_замены: Regex::new(r"(?i)(щесть)$").unwrap(),
            строка: "щесть",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+щестью$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестью)$").unwrap(),
            строка: "щестью",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+щести$").unwrap(),
            re_замены: Regex::new(r"(?i)(щести)$").unwrap(),
            строка: "щести",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+щестей$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестей)$").unwrap(),
            строка: "щестей",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+щестям$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестям)$").unwrap(),
            строка: "щестям",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+щестями$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестями)$").unwrap(),
            строка: "щестями",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+щестях$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестях)$").unwrap(),
            строка: "щестях",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щим$").unwrap(),
            re_замены: Regex::new(r"(?i)(щим)$").unwrap(),
            строка: "щим",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щими$").unwrap(),
            re_замены: Regex::new(r"(?i)(щими)$").unwrap(),
            строка: "щими",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щимися$").unwrap(),
            re_замены: Regex::new(r"(?i)(щимися)$").unwrap(),
            строка: "щимися",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щимих$").unwrap(),
            re_замены: Regex::new(r"(?i)(щимих)$").unwrap(),
            строка: "щимих",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щимихся$").unwrap(),
            re_замены: Regex::new(r"(?i)(щимихся)$").unwrap(),
            строка: "щимихся",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щий$").unwrap(),
            re_замены: Regex::new(r"(?i)(щий)$").unwrap(),
            строка: "щий",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щийся$").unwrap(),
            re_замены: Regex::new(r"(?i)(щийся)$").unwrap(),
            строка: "щийся",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щие$").unwrap(),
            re_замены: Regex::new(r"(?i)(щие)$").unwrap(),
            строка: "щие",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+щиеся$").unwrap(),
            re_замены: Regex::new(r"(?i)(щиеся)$").unwrap(),
            строка: "щиеся",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щих$").unwrap(),
            re_замены: Regex::new(r"(?i)(щих)$").unwrap(),
            строка: "щих",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+щ$").unwrap(),
            re_замены: Regex::new(r"(?i)(щ)$").unwrap(),
            строка: "щ",
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Щ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "щ",
            "ща",
            "ще",
            "щею",
            "щая",
            "щаяся",
            "щуюся",
            "щую",
            "щее",
            "щееся",
            "щи",
            "щем",
            "щемся",
            "щеми",
            "щему",
            "щемуся",
            "щего",
            "щегося",
            // "ще" уже было выше — дубликат убран
            "щей",
            "щейся",
            "щесть",
            "щестью",
            "щести",
            "щестей",
            "щестям",
            "щестями",
            "щестях",
            // "щи" уже было выше — дубликат убран
            "щим",
            "щими",
            "щимися",
            "щимих",
            "щимихся",
            "щий",
            "щийся",
            "щие",
            "щиеся",
            "щих",
        ])
    });
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ВШ: LazyLock<[RE_полное_окончание; 36]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+вшею$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшею)$").unwrap(),
            строка: "вшею",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+вша$").unwrap(),
            re_замены: Regex::new(r"(?i)(вша)$").unwrap(),
            строка: "вша",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вше$").unwrap(),
            re_замены: Regex::new(r"(?i)(вше)$").unwrap(),
            строка: "вше",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+вшая$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшая)$").unwrap(),
            строка: "вшая",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшаяся$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшаяся)$").unwrap(),
            строка: "вшаяся",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+вшую$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшую)$").unwrap(),
            строка: "вшую",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшуюся$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшуюся)$").unwrap(),
            строка: "вшуюся",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+вшее$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшее)$").unwrap(),
            строка: "вшее",
            вид_окончания: Вид_Окончания::Вш,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшееся$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшееся)$").unwrap(),
            строка: "вшееся",
            вид_окончания: Вид_Окончания::Вш,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Деепричастие],
            re_поиска: Regex::new(r"(?i)\w+вши$").unwrap(),
            re_замены: Regex::new(r"(?i)(вши)$").unwrap(),
            строка: "вши",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшеми$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшеми)$").unwrap(),
            строка: "вшеми",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+вшем$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшем)$").unwrap(),
            строка: "вшем",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшемся$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшемся)$").unwrap(),
            строка: "вшемся",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшемуся$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшемуся)$").unwrap(),
            строка: "вшемуся",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+вшему$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшему)$").unwrap(),
            строка: "вшему",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+вшего$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшего)$").unwrap(),
            строка: "вшего",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшегося$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшегося)$").unwrap(),
            строка: "вшегося",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшейся$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшейся)$").unwrap(),
            строка: "вшейся",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшей$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшей)$").unwrap(),
            строка: "вшей",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вшесть$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшесть)$").unwrap(),
            строка: "вшесть",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вшестью$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшестью)$").unwrap(),
            строка: "вшестью",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вшести$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшести)$").unwrap(),
            строка: "вшести",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вшестей$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшестей)$").unwrap(),
            строка: "вшестей",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вшестям$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшестям)$").unwrap(),
            строка: "вшестям",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вшестями$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшестями)$").unwrap(),
            строка: "вшестями",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вшестях$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшестях)$").unwrap(),
            строка: "вшестях",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшим$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшим)$").unwrap(),
            строка: "вшим",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшими$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшими)$").unwrap(),
            строка: "вшими",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшимися$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшимися)$").unwrap(),
            строка: "вшимися",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшимих$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшимих)$").unwrap(),
            строка: "вшимих",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшимихся$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшимихся)$").unwrap(),
            строка: "вшимихся",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вший$").unwrap(),
            re_замены: Regex::new(r"(?i)(вший)$").unwrap(),
            строка: "вший",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшийся$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшийся)$").unwrap(),
            строка: "вшийся",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшие$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшие)$").unwrap(),
            строка: "вшие",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вшиеся$").unwrap(),
            re_замены: Regex::new(r"(?i)(вшиеся)$").unwrap(),
            строка: "вшиеся",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вших$").unwrap(),
            re_замены: Regex::new(r"(?i)(вших)$").unwrap(),
            строка: "вших",
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ВШ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "вша",
            "вше",
            "вшею",
            "вшая",
            "вшаяся",
            "вшуюся",
            "вшую",
            "вшее",
            "вшееся",
            "вши",
            "вшем",
            "вшемся",
            "вшеми",
            "вшему",
            "вшемуся",
            "вшего",
            "вшегося",
            // "вше" уже было выше — дубликат убран
            "вшей",
            "вшейся",
            "вшесть",
            "вшестью",
            "вшести",
            "вшестей",
            "вшестям",
            "вшестями",
            "вшестях",
            // "вши" уже было выше — дубликат убран
            "вшим",
            "вшими",
            "вшимися",
            "вшимих",
            "вшимихся",
            "вший",
            "вшийся",
            "вшие",
            "вшиеся",
            "вших",
        ])
    });

//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЕЛЬ: LazyLock<[RE_полное_окончание; 30]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ельностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельностью)$").unwrap(),
            строка: "ельностью",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельную$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельную)$").unwrap(),
            строка: "ельную",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+елен$").unwrap(),
            re_замены: Regex::new(r"(?i)(елен)$").unwrap(),
            строка: "елен",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ель$").unwrap(),
            re_замены: Regex::new(r"(?i)(ель)$").unwrap(),
            строка: "ель",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Глагол,
                Вид_части_речи::Наречие,
            ],
            re_поиска: Regex::new(r"(?i)\w+ели$").unwrap(),
            re_замены: Regex::new(r"(?i)(ели)$").unwrap(),
            строка: "ели",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+елью$").unwrap(),
            re_замены: Regex::new(r"(?i)(елью)$").unwrap(),
            строка: "елью",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+елям$").unwrap(),
            re_замены: Regex::new(r"(?i)(елям)$").unwrap(),
            строка: "елям",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+елей$").unwrap(),
            re_замены: Regex::new(r"(?i)(елей)$").unwrap(),
            строка: "елей",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+елями$").unwrap(),
            re_замены: Regex::new(r"(?i)(елями)$").unwrap(),
            строка: "елями",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+елях$").unwrap(),
            re_замены: Regex::new(r"(?i)(елях)$").unwrap(),
            строка: "елях",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельная$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельная)$").unwrap(),
            строка: "ельная",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельна$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельна)$").unwrap(),
            строка: "ельна",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ельнее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельнее)$").unwrap(),
            строка: "ельнее",
            вид_окончания: Вид_Окончания::Ель,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельному$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельному)$").unwrap(),
            строка: "ельному",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельном$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельном)$").unwrap(),
            строка: "ельном",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельного$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельного)$").unwrap(),
            строка: "ельного",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельное$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельное)$").unwrap(),
            строка: "ельное",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельной$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельной)$").unwrap(),
            строка: "ельной",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельно$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельно)$").unwrap(),
            строка: "ельно",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ельность$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельность)$").unwrap(),
            строка: "ельность",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ельности$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельности)$").unwrap(),
            строка: "ельности",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ельностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельностей)$").unwrap(),
            строка: "ельностей",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ельностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельностям)$").unwrap(),
            строка: "ельностям",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ельностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельностями)$").unwrap(),
            строка: "ельностями",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ельностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельностях)$").unwrap(),
            строка: "ельностях",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельный$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельный)$").unwrap(),
            строка: "ельный",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельны$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельны)$").unwrap(),
            строка: "ельны",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельных$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельных)$").unwrap(),
            строка: "ельных",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельным$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельным)$").unwrap(),
            строка: "ельным",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ельными$").unwrap(),
            re_замены: Regex::new(r"(?i)(ельными)$").unwrap(),
            строка: "ельными",
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЕЛЬ: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "елен",
        "ель",
        "ельную",
        "ельностью",
        "ельностях",
        "ельностям",
        "ельностями",
        "ельна",
        "ельная",
        "ельно",
        "ельность",
        "ельности",
        "ельностей",
        "ельной",
        "ельное",
        "ельного",
        "ельном",
        "ельному",
        "ельны",
        "ельным",
        "ельными",
        "ельных",
        "ельный",
        "ельнее",
        "елей",
        "ели",
        "елью",
        "елям",
        "елями",
        "елях",
    ])
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Ш: LazyLock<[RE_полное_окончание; 22]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ша$").unwrap(),
            re_замены: Regex::new(r"(?i)(ша)$").unwrap(),
            строка: "ша",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ше$").unwrap(),
            re_замены: Regex::new(r"(?i)(ше)$").unwrap(),
            строка: "ше",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шая$").unwrap(),
            re_замены: Regex::new(r"(?i)(шая)$").unwrap(),
            строка: "шая",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шую$").unwrap(),
            re_замены: Regex::new(r"(?i)(шую)$").unwrap(),
            строка: "шую",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шее$").unwrap(),
            re_замены: Regex::new(r"(?i)(шее)$").unwrap(),
            строка: "шее",
            вид_окончания: Вид_Окончания::_Ш,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шем$").unwrap(),
            re_замены: Regex::new(r"(?i)(шем)$").unwrap(),
            строка: "шем",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шему$").unwrap(),
            re_замены: Regex::new(r"(?i)(шему)$").unwrap(),
            строка: "шему",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шего$").unwrap(),
            re_замены: Regex::new(r"(?i)(шего)$").unwrap(),
            строка: "шего",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шей$").unwrap(),
            re_замены: Regex::new(r"(?i)(шей)$").unwrap(),
            строка: "шей",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+шесть$").unwrap(),
            re_замены: Regex::new(r"(?i)(шесть)$").unwrap(),
            строка: "шесть",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+шестью$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестью)$").unwrap(),
            строка: "шестью",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+шести$").unwrap(),
            re_замены: Regex::new(r"(?i)(шести)$").unwrap(),
            строка: "шести",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+шестей$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестей)$").unwrap(),
            строка: "шестей",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+шестям$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестям)$").unwrap(),
            строка: "шестям",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+шестями$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестями)$").unwrap(),
            строка: "шестями",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+шестях$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестях)$").unwrap(),
            строка: "шестях",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ши$").unwrap(),
            re_замены: Regex::new(r"(?i)(ши)$").unwrap(),
            строка: "ши",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шим$").unwrap(),
            re_замены: Regex::new(r"(?i)(шим)$").unwrap(),
            строка: "шим",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шими$").unwrap(),
            re_замены: Regex::new(r"(?i)(шими)$").unwrap(),
            строка: "шими",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ший$").unwrap(),
            re_замены: Regex::new(r"(?i)(ший)$").unwrap(),
            строка: "ший",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+шие$").unwrap(),
            re_замены: Regex::new(r"(?i)(шие)$").unwrap(),
            строка: "шие",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ших$").unwrap(),
            re_замены: Regex::new(r"(?i)(ших)$").unwrap(),
            строка: "ших",
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Ш: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ша",
            "ше",
            "шая",
            "шую",
            "шее",
            "ши",
            "шем",
            "шему",
            "шего",
            // "ше" уже было выше — дубликат убран
            "шей",
            "шесть",
            "шестью",
            "шести",
            "шестей",
            "шестям",
            "шестями",
            "шестях",
            // "ши" уже было выше — дубликат убран
            "шим",
            "шими",
            "ший",
            "шие",
            "ших",
        ])
    });
//
/*pub static РАЗДЕЛ_ОКОНЧАНИЙ_НИ: LazyLock<[RE_полное_окончание; 8]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ния$").unwrap(),
            re_замены: Regex::new(r"(?i)(ния)$").unwrap(),
            окончание_строка: "ния",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ниям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ниям$)$").unwrap(),
            окончание_строка: "ниям",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ниями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ниями$)$").unwrap(),
            окончание_строка: "ниями",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ниях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ниях$)$").unwrap(),
            окончание_строка: "ниях",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+нию$").unwrap(),
            re_замены: Regex::new(r"(?i)(нию$)$").unwrap(),
            окончание_строка: "нию",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ний$").unwrap(),
            re_замены: Regex::new(r"(?i)(ний)$").unwrap(),
            окончание_строка: "ний",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ние$").unwrap(),
            re_замены: Regex::new(r"(?i)(ние)$").unwrap(),
            окончание_строка: "ние",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+нием$").unwrap(),
            re_замены: Regex::new(r"(?i)(нием)$").unwrap(),
            окончание_строка: "нием",
            вид_окончания: Вид_Окончания::Ни,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_НИ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter(["ния", "ниям", "ниями", "ниях", "нию", "ний", "ние", "нием"])
});*/
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Р: LazyLock<[RE_полное_окончание; 11]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+р$").unwrap(),
            re_замены: Regex::new(r"(?i)(р)$").unwrap(),
            строка: "р",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+рен$").unwrap(),
            re_замены: Regex::new(r"(?i)(рен)$").unwrap(),
            строка: "рен",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ре$").unwrap(),
            re_замены: Regex::new(r"(?i)(ре)$").unwrap(),
            строка: "ре",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+рах$").unwrap(),
            re_замены: Regex::new(r"(?i)(рах)$").unwrap(),
            строка: "рах",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ра$").unwrap(),
            re_замены: Regex::new(r"(?i)(ра)$").unwrap(),
            строка: "ра",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+рами$").unwrap(),
            re_замены: Regex::new(r"(?i)(рами)$").unwrap(),
            строка: "рами",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+рам$").unwrap(),
            re_замены: Regex::new(r"(?i)(рам)$").unwrap(),
            строка: "рам",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Наречие,
                Вид_части_речи::Глагол,
            ],
            re_поиска: Regex::new(r"(?i)\w+ру$").unwrap(),
            re_замены: Regex::new(r"(?i)(ру)$").unwrap(),
            строка: "ру",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ры$").unwrap(),
            re_замены: Regex::new(r"(?i)(ры)$").unwrap(),
            строка: "ры",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Наречие,
                Вид_части_речи::Числительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ром$").unwrap(),
            re_замены: Regex::new(r"(?i)(ром)$").unwrap(),
            строка: "ром",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ров$").unwrap(),
            re_замены: Regex::new(r"(?i)(ров)$").unwrap(),
            строка: "ров",
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Р: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "р", "ре", "рах", "ра", "рами", "рам", "ру", "ры", "ром", "ров", "рен",
    ])
});

/*pub static РАЗДЕЛ_ОКОНЧАНИЙ_В: LazyLock<[RE_полное_окончание; 9]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+в$").unwrap(),
            re_замены: Regex::new(r"(?i)(в)$").unwrap(),
            окончание_строка: "в",
            вид_окончания: Вид_Окончания::_В,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вы$").unwrap(),
            re_замены: Regex::new(r"(?i)(вы)$").unwrap(),
            окончание_строка: "вы",
            вид_окончания: Вид_Окончания::_В,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ве$").unwrap(),
            re_замены: Regex::new(r"(?i)(ве)$").unwrap(),
            окончание_строка: "ве",
            вид_окончания: Вид_Окончания::_В,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ва$").unwrap(),
            re_замены: Regex::new(r"(?i)(ва)$").unwrap(),
            окончание_строка: "ва",
            вид_окончания: Вид_Окончания::_В,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ву$").unwrap(),
            re_замены: Regex::new(r"(?i)(ву)$").unwrap(),
            окончание_строка: "ву",
            вид_окончания: Вид_Окончания::_В,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вов$").unwrap(),
            re_замены: Regex::new(r"(?i)(вов)$").unwrap(),
            окончание_строка: "вов",
            вид_окончания: Вид_Окончания::_В,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вам$").unwrap(),
            re_замены: Regex::new(r"(?i)(вам)$").unwrap(),
            окончание_строка: "вам",
            вид_окончания: Вид_Окончания::_В,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вами$").unwrap(),
            re_замены: Regex::new(r"(?i)(вами)$").unwrap(),
            окончание_строка: "вами",
            вид_окончания: Вид_Окончания::_В,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вах$").unwrap(),
            re_замены: Regex::new(r"(?i)(вах)$").unwrap(),
            окончание_строка: "вах",
            вид_окончания: Вид_Окончания::_В,
        },
    ]
});

pub static КУЧА_ОКОНЧАНИЙ_В: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter(["в", "вы", "ве", "ва", "ву", "вов", "вам", "вами", "вах"])
});
*/
pub static КУЧА_ОКОНЧАНИЙ_ВА_ВО: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "в",
        "вы",
        "ве",
        "ва",
        "ву",
        "вов",
        "вам",
        "вами",
        "вах",
        "вая",
        "вую",
        "во",
        "вой",
        "вое",
        "вом",
        "вого",
        "вость",
        "востью",
        "вости",
        "востей",
        "востям",
        "востями",
        "востях",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_В_ВА_ВО: LazyLock<[RE_полное_окончание; 23]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Глагол,
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+в$").unwrap(),
            re_замены: Regex::new(r"(?i)(в)$").unwrap(),
            строка: "в",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+вы$").unwrap(),
            re_замены: Regex::new(r"(?i)(вы)$").unwrap(),
            строка: "вы",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ве$").unwrap(),
            re_замены: Regex::new(r"(?i)(ве)$").unwrap(),
            строка: "ве",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ва$").unwrap(),
            re_замены: Regex::new(r"(?i)(ва)$").unwrap(),
            строка: "ва",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ву$").unwrap(),
            re_замены: Regex::new(r"(?i)(ву)$").unwrap(),
            строка: "ву",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вов$").unwrap(),
            re_замены: Regex::new(r"(?i)(вов)$").unwrap(),
            строка: "вов",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вам$").unwrap(),
            re_замены: Regex::new(r"(?i)(вам)$").unwrap(),
            строка: "вам",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вами$").unwrap(),
            re_замены: Regex::new(r"(?i)(вами)$").unwrap(),
            строка: "вами",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вах$").unwrap(),
            re_замены: Regex::new(r"(?i)(вах)$").unwrap(),
            строка: "вах",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Деепричастие,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+вая$").unwrap(),
            re_замены: Regex::new(r"(?i)(вая)$").unwrap(),
            строка: "вая",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Деепричастие,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+вую$").unwrap(),
            re_замены: Regex::new(r"(?i)(вую)$").unwrap(),
            строка: "вую",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+во$").unwrap(),
            re_замены: Regex::new(r"(?i)(во)$").unwrap(),
            строка: "во",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вой$").unwrap(),
            re_замены: Regex::new(r"(?i)(вой)$").unwrap(),
            строка: "вой",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вое$").unwrap(),
            re_замены: Regex::new(r"(?i)(вое)$").unwrap(),
            строка: "вое",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вом$").unwrap(),
            re_замены: Regex::new(r"(?i)(вом)$").unwrap(),
            строка: "вом",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+вого$").unwrap(),
            re_замены: Regex::new(r"(?i)(вого)$").unwrap(),
            строка: "вого",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вость$").unwrap(),
            re_замены: Regex::new(r"(?i)(вость)$").unwrap(),
            строка: "вость",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+востью$").unwrap(),
            re_замены: Regex::new(r"(?i)(востью)$").unwrap(),
            строка: "востью",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+вости$").unwrap(),
            re_замены: Regex::new(r"(?i)(вости)$").unwrap(),
            строка: "вости",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+востей$").unwrap(),
            re_замены: Regex::new(r"(?i)(востей)$").unwrap(),
            строка: "востей",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+востям$").unwrap(),
            re_замены: Regex::new(r"(?i)(востям)$").unwrap(),
            строка: "востям",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+востями$").unwrap(),
            re_замены: Regex::new(r"(?i)(востями)$").unwrap(),
            строка: "востями",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+востях$").unwrap(),
            re_замены: Regex::new(r"(?i)(востях)$").unwrap(),
            строка: "востях",
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
    ]
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СК: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ск$").unwrap(),
            re_замены: Regex::new(r"(?i)(ск)$").unwrap(),
            строка: "ск",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+скее$").unwrap(),
            re_замены: Regex::new(r"(?i)(скее)$").unwrap(),
            строка: "скее",
            вид_окончания: Вид_Окончания::Ск,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ска$").unwrap(),
            re_замены: Regex::new(r"(?i)(ска)$").unwrap(),
            строка: "ска",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Деепричастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ская$").unwrap(),
            re_замены: Regex::new(r"(?i)(ская)$").unwrap(),
            строка: "ская",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Деепричастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+скую$").unwrap(),
            re_замены: Regex::new(r"(?i)(скую)$").unwrap(),
            строка: "скую",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ско$").unwrap(),
            re_замены: Regex::new(r"(?i)(ско)$").unwrap(),
            строка: "ско",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ски$").unwrap(),
            re_замены: Regex::new(r"(?i)(ски)$").unwrap(),
            строка: "ски",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ском$").unwrap(),
            re_замены: Regex::new(r"(?i)(ском)$").unwrap(),
            строка: "ском",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+скому$").unwrap(),
            re_замены: Regex::new(r"(?i)(скому)$").unwrap(),
            строка: "скому",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        // Дубликат "ско" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ского$").unwrap(),
            re_замены: Regex::new(r"(?i)(ского)$").unwrap(),
            строка: "ского",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ское$").unwrap(),
            re_замены: Regex::new(r"(?i)(ское)$").unwrap(),
            строка: "ское",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ской$").unwrap(),
            re_замены: Regex::new(r"(?i)(ской)$").unwrap(),
            строка: "ской",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+скость$").unwrap(),
            re_замены: Regex::new(r"(?i)(скость)$").unwrap(),
            строка: "скость",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+скостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостью)$").unwrap(),
            строка: "скостью",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+скости$").unwrap(),
            re_замены: Regex::new(r"(?i)(скости)$").unwrap(),
            строка: "скости",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+скостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостей)$").unwrap(),
            строка: "скостей",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+скостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостям)$").unwrap(),
            строка: "скостям",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+скостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостями)$").unwrap(),
            строка: "скостями",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+скостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостях)$").unwrap(),
            строка: "скостях",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ским$").unwrap(),
            re_замены: Regex::new(r"(?i)(ским)$").unwrap(),
            строка: "ским",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+скими$").unwrap(),
            re_замены: Regex::new(r"(?i)(скими)$").unwrap(),
            строка: "скими",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ский$").unwrap(),
            re_замены: Regex::new(r"(?i)(ский)$").unwrap(),
            строка: "ский",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ские$").unwrap(),
            re_замены: Regex::new(r"(?i)(ские)$").unwrap(),
            строка: "ские",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ских$").unwrap(),
            re_замены: Regex::new(r"(?i)(ских)$").unwrap(),
            строка: "ских",
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_СК: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ск",
            "скее",
            "ска",
            "ская",
            "скую",
            "ско",
            "ски",
            "ском",
            "скому",
            "ского",
            "ское",
            "ской",
            "скость",
            "скостью",
            "скости",
            "скостей",
            "скостям",
            "скостями",
            "скостях",
            "ским",
            "скими",
            "ский",
            "ские",
            "ских",
        ])
    });
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЯМ: LazyLock<[RE_полное_окончание; 3]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ям)$").unwrap(),
            строка: "ям",
            вид_окончания: Вид_Окончания::Ям,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ями)$").unwrap(),
            строка: "ями",
            вид_окончания: Вид_Окончания::Ям,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ях)$").unwrap(),
            строка: "ях",
            вид_окончания: Вид_Окончания::Ям,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЯМ: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter(["ям", "ями", "ях"]));
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Ц: LazyLock<[RE_полное_окончание; 10]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ец$").unwrap(),
            re_замены: Regex::new(r"(?i)(ец)$").unwrap(),
            строка: "ец",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ца$").unwrap(),
            re_замены: Regex::new(r"(?i)(ца)$").unwrap(),
            строка: "ца",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+цу$").unwrap(),
            re_замены: Regex::new(r"(?i)(цу)$").unwrap(),
            строка: "цу",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+це$").unwrap(),
            re_замены: Regex::new(r"(?i)(це)$").unwrap(),
            строка: "це",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+цом$").unwrap(),
            re_замены: Regex::new(r"(?i)(цом)$").unwrap(),
            строка: "цом",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+цам$").unwrap(),
            re_замены: Regex::new(r"(?i)(цам)$").unwrap(),
            строка: "цам",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+цами$").unwrap(),
            re_замены: Regex::new(r"(?i)(цами)$").unwrap(),
            строка: "цами",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+цах$").unwrap(),
            re_замены: Regex::new(r"(?i)(цах)$").unwrap(),
            строка: "цах",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+цы$").unwrap(),
            re_замены: Regex::new(r"(?i)(цы)$").unwrap(),
            строка: "цы",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        // Дубликат "цо" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+цов$").unwrap(),
            re_замены: Regex::new(r"(?i)(цов)$").unwrap(),
            строка: "цов",
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Ц: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ец", "ца", "цу", "це", "цом", "цам", "цами", "цах", "цы", "цов",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ОВ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ов$").unwrap(),
            re_замены: Regex::new(r"(?i)(ов)$").unwrap(),
            строка: "ов",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+овее$").unwrap(),
            re_замены: Regex::new(r"(?i)(овее)$").unwrap(),
            строка: "овее",
            вид_окончания: Вид_Окончания::Ов,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ова$").unwrap(),
            re_замены: Regex::new(r"(?i)(ова)$").unwrap(),
            строка: "ова",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овая$").unwrap(),
            re_замены: Regex::new(r"(?i)(овая)$").unwrap(),
            строка: "овая",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овую$").unwrap(),
            re_замены: Regex::new(r"(?i)(овую)$").unwrap(),
            строка: "овую",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ово$").unwrap(),
            re_замены: Regex::new(r"(?i)(ово)$").unwrap(),
            строка: "ово",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овы$").unwrap(),
            re_замены: Regex::new(r"(?i)(овы)$").unwrap(),
            строка: "овы",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овом$").unwrap(),
            re_замены: Regex::new(r"(?i)(овом)$").unwrap(),
            строка: "овом",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овому$").unwrap(),
            re_замены: Regex::new(r"(?i)(овому)$").unwrap(),
            строка: "овому",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ового$").unwrap(),
            re_замены: Regex::new(r"(?i)(ового)$").unwrap(),
            строка: "ового",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овое$").unwrap(),
            re_замены: Regex::new(r"(?i)(овое)$").unwrap(),
            строка: "овое",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овой$").unwrap(),
            re_замены: Regex::new(r"(?i)(овой)$").unwrap(),
            строка: "овой",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+овость$").unwrap(),
            re_замены: Regex::new(r"(?i)(овость)$").unwrap(),
            строка: "овость",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+овостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостью)$").unwrap(),
            строка: "овостью",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+овости$").unwrap(),
            re_замены: Regex::new(r"(?i)(овости)$").unwrap(),
            строка: "овости",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+овостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостей)$").unwrap(),
            строка: "овостей",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+овостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостям)$").unwrap(),
            строка: "овостям",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+овостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостями)$").unwrap(),
            строка: "овостями",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+овостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостях)$").unwrap(),
            строка: "овостях",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овым$").unwrap(),
            re_замены: Regex::new(r"(?i)(овым)$").unwrap(),
            строка: "овым",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(овыми)$").unwrap(),
            строка: "овыми",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овый$").unwrap(),
            re_замены: Regex::new(r"(?i)(овый)$").unwrap(),
            строка: "овый",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овые$").unwrap(),
            re_замены: Regex::new(r"(?i)(овые)$").unwrap(),
            строка: "овые",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+овых$").unwrap(),
            re_замены: Regex::new(r"(?i)(овых)$").unwrap(),
            строка: "овых",
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ОВ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ов",
        "овее",
        "ова",
        "овая",
        "овую",
        "ово",
        "овы",
        "овом",
        "овому",
        "ового",
        "овое",
        "овой",
        "овость",
        "овостью",
        "овости",
        "овостей",
        "овостям",
        "овостями",
        "овостях",
        "овым",
        "овыми",
        "овый",
        "овые",
        "овых",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЕВ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ев$").unwrap(),
            re_замены: Regex::new(r"(?i)(ев)$").unwrap(),
            строка: "ев",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+евее$").unwrap(),
            re_замены: Regex::new(r"(?i)(евее)$").unwrap(),
            строка: "евее",
            вид_окончания: Вид_Окончания::Ев,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ева$").unwrap(),
            re_замены: Regex::new(r"(?i)(ева)$").unwrap(),
            строка: "ева",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евая$").unwrap(),
            re_замены: Regex::new(r"(?i)(евая)$").unwrap(),
            строка: "евая",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евую$").unwrap(),
            re_замены: Regex::new(r"(?i)(евую)$").unwrap(),
            строка: "евую",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евы$").unwrap(),
            re_замены: Regex::new(r"(?i)(евы)$").unwrap(),
            строка: "евы",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евом$").unwrap(),
            re_замены: Regex::new(r"(?i)(евом)$").unwrap(),
            строка: "евом",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евому$").unwrap(),
            re_замены: Regex::new(r"(?i)(евому)$").unwrap(),
            строка: "евому",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евого$").unwrap(),
            re_замены: Regex::new(r"(?i)(евого)$").unwrap(),
            строка: "евого",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+ево$").unwrap(),
            re_замены: Regex::new(r"(?i)(ево)$").unwrap(),
            строка: "ево",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евое$").unwrap(),
            re_замены: Regex::new(r"(?i)(евое)$").unwrap(),
            строка: "евое",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евой$").unwrap(),
            re_замены: Regex::new(r"(?i)(евой)$").unwrap(),
            строка: "евой",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+евость$").unwrap(),
            re_замены: Regex::new(r"(?i)(евость)$").unwrap(),
            строка: "евость",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+евостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостью)$").unwrap(),
            строка: "евостью",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+евости$").unwrap(),
            re_замены: Regex::new(r"(?i)(евости)$").unwrap(),
            строка: "евости",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+евостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостей)$").unwrap(),
            строка: "евостей",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+евостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостям)$").unwrap(),
            строка: "евостям",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+евостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостями)$").unwrap(),
            строка: "евостями",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+евостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостях)$").unwrap(),
            строка: "евостях",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евым$").unwrap(),
            re_замены: Regex::new(r"(?i)(евым)$").unwrap(),
            строка: "евым",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(евыми)$").unwrap(),
            строка: "евыми",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евый$").unwrap(),
            re_замены: Regex::new(r"(?i)(евый)$").unwrap(),
            строка: "евый",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евые$").unwrap(),
            re_замены: Regex::new(r"(?i)(евые)$").unwrap(),
            строка: "евые",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+евых$").unwrap(),
            re_замены: Regex::new(r"(?i)(евых)$").unwrap(),
            строка: "евых",
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЕВ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ев",
        "евее",
        "ева",
        "евая",
        "евую",
        "ево",
        "евы",
        "евом",
        "евому",
        "евого",
        "евое",
        "евой",
        "евость",
        "евостью",
        "евости",
        "евостей",
        "евостям",
        "евостями",
        "евостях",
        "евым",
        "евыми",
        "евый",
        "евые",
        "евых",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Д: LazyLock<[RE_полное_окончание; 10]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+д$").unwrap(),
            re_замены: Regex::new(r"(?i)(д)$").unwrap(),
            строка: "д",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+да$").unwrap(),
            re_замены: Regex::new(r"(?i)(да)$").unwrap(),
            строка: "да",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ду$").unwrap(),
            re_замены: Regex::new(r"(?i)(ду)$").unwrap(),
            строка: "ду",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+де$").unwrap(),
            re_замены: Regex::new(r"(?i)(де)$").unwrap(),
            строка: "де",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+дом$").unwrap(),
            re_замены: Regex::new(r"(?i)(дом)$").unwrap(),
            строка: "дом",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+дам$").unwrap(),
            re_замены: Regex::new(r"(?i)(дам)$").unwrap(),
            строка: "дам",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+дами$").unwrap(),
            re_замены: Regex::new(r"(?i)(дами)$").unwrap(),
            строка: "дами",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+дах$").unwrap(),
            re_замены: Regex::new(r"(?i)(дах)$").unwrap(),
            строка: "дах",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ды$").unwrap(),
            re_замены: Regex::new(r"(?i)(ды)$").unwrap(),
            строка: "ды",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        // Дубликат "до" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+дов$").unwrap(),
            re_замены: Regex::new(r"(?i)(дов)$").unwrap(),
            строка: "дов",
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
    ]
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_С: LazyLock<[RE_полное_окончание; 14]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ся$").unwrap(),
            re_замены: Regex::new(r"(?i)(ся)$").unwrap(),
            строка: "ся",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сный$").unwrap(),
            re_замены: Regex::new(r"(?i)(сный)$").unwrap(),
            строка: "сный",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сно$").unwrap(),
            re_замены: Regex::new(r"(?i)(сно)$").unwrap(),
            строка: "сно",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+с$").unwrap(),
            re_замены: Regex::new(r"(?i)(с)$").unwrap(),
            строка: "с",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+са$").unwrap(),
            re_замены: Regex::new(r"(?i)(са)$").unwrap(),
            строка: "са",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+су$").unwrap(),
            re_замены: Regex::new(r"(?i)(су)$").unwrap(),
            строка: "су",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+се$").unwrap(),
            re_замены: Regex::new(r"(?i)(се)$").unwrap(),
            строка: "се",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сом$").unwrap(),
            re_замены: Regex::new(r"(?i)(сом)$").unwrap(),
            строка: "сом",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сам$").unwrap(),
            re_замены: Regex::new(r"(?i)(сам)$").unwrap(),
            строка: "сам",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(
                r"(?i)\w+сами
                $",
            )
            .unwrap(),
            re_замены: Regex::new(r"(?i)(сами)$").unwrap(),
            строка: "сами",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сах$").unwrap(),
            re_замены: Regex::new(r"(?i)(сах)$").unwrap(),
            строка: "сах",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сы$").unwrap(),
            re_замены: Regex::new(r"(?i)(сы)$").unwrap(),
            строка: "сы",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        // Дубликат "со" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сов$").unwrap(),
            re_замены: Regex::new(r"(?i)(сов)$").unwrap(),
            строка: "сов",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сь$").unwrap(),
            re_замены: Regex::new(r"(?i)(сь)$").unwrap(),
            строка: "сь",
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_С: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "с", "сь", "са", "су", "се", "сом", "сам", "сами", "сах", "сы", "сов", "ся", "сно", "сный",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СЕН: LazyLock<[RE_полное_окончание; 42]> = LazyLock::new(|| {
    [
        // ---- причастия/прилагательные с "нн" ----
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенная$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенная)$").unwrap(),
            строка: "сенная",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенную$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенную)$").unwrap(),
            строка: "сенную",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенное$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенное)$").unwrap(),
            строка: "сенное",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенного$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенного)$").unwrap(),
            строка: "сенного",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенному$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенному)$").unwrap(),
            строка: "сенному",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенным$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенным)$").unwrap(),
            строка: "сенным",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенными$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенными)$").unwrap(),
            строка: "сенными",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сеными$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеными)$").unwrap(),
            строка: "сеными",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенном$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенном)$").unwrap(),
            строка: "сенном",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенные$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенные)$").unwrap(),
            строка: "сенные",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенных$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенных)$").unwrap(),
            строка: "сенных",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сенный$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенный)$").unwrap(),
            строка: "сенный",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеннее)$").unwrap(),
            строка: "сеннее",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        // ---- формы с одной "н" ----
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сен$").unwrap(),
            re_замены: Regex::new(r"(?i)(сен)$").unwrap(),
            строка: "сен",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сенна$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенна)$").unwrap(),
            строка: "сенна",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сена$").unwrap(),
            re_замены: Regex::new(r"(?i)(сена)$").unwrap(),
            строка: "сена",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сены$").unwrap(),
            re_замены: Regex::new(r"(?i)(сены)$").unwrap(),
            строка: "сены",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сено$").unwrap(),
            re_замены: Regex::new(r"(?i)(сено)$").unwrap(),
            строка: "сено",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сенно$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенно)$").unwrap(),
            строка: "сенно",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сенной$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенной)$").unwrap(),
            строка: "сенной",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеной$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеной)$").unwrap(),
            строка: "сеной",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеные$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеные)$").unwrap(),
            строка: "сеные",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеное$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеное)$").unwrap(),
            строка: "сеное",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеная$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеная)$").unwrap(),
            строка: "сеная",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеном$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеном)$").unwrap(),
            строка: "сеном",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеному$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеному)$").unwrap(),
            строка: "сеному",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеную$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеную)$").unwrap(),
            строка: "сеную",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеного$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеного)$").unwrap(),
            строка: "сеного",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеным$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеным)$").unwrap(),
            строка: "сеным",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеных$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеных)$").unwrap(),
            строка: "сеных",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сеный$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеный)$").unwrap(),
            строка: "сеный",
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сенее$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенее)$").unwrap(),
            строка: "сенее",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сенностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенностью)$").unwrap(),
            строка: "сенностью",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сенности$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенности)$").unwrap(),
            строка: "сенности",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сенностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенностей)$").unwrap(),
            строка: "сенностей",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сенностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенностям)$").unwrap(),
            строка: "сенностям",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сенностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(сенностями)$").unwrap(),
            строка: "сенностями",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сеностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеностью)$").unwrap(),
            строка: "сеностью",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сености$").unwrap(),
            re_замены: Regex::new(r"(?i)(сености)$").unwrap(),
            строка: "сености",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сеностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеностей)$").unwrap(),
            строка: "сеностей",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сеностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеностям)$").unwrap(),
            строка: "сеностям",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сеностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(сеностями)$").unwrap(),
            строка: "сеностями",
            вид_окончания: Вид_Окончания::Сен,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});
pub static КУЧА_ПОЛНАЯ_ОКОНЧАНИЙ_СЕН: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "сенностью",
        "сена",
        "сенна",
        "сен",
        "сено",
        "сенно",
        "сености",
        "сенности",
        "сеностям",
        "сенностям",
        "сеностями",
        "сенностями",
        "сеностей",
        "сенностей",
        "сеной",
        "сенной",
        "сеные",
        "сенные",
        "сеное",
        "сенное",
        "сеная",
        "сенная",
        "сеном",
        "сенном",
        "сеному",
        "сенному",
        "сеную",
        "сенную",
        "сеного",
        "сенного",
        "сеным",
        "сенным",
        "сеными",
        "сенными",
        "сены",
        "сенных",
        "сеных",
        "сенее",
        "сеннее",
        "сеный",
        "сенный",
        "сеностью",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СЁН: LazyLock<[RE_полное_окончание; 42]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сённо$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённо)$").unwrap(),
            строка: "сённо",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сёно$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёно)$").unwrap(),
            строка: "сёно",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сённостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённостью)$").unwrap(),
            строка: "сённостью",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сённости$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённости)$").unwrap(),
            строка: "сённости",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сённостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённостей)$").unwrap(),
            строка: "сённостей",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сённостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённостям)$").unwrap(),
            строка: "сённостям",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сённостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённостями)$").unwrap(),
            строка: "сённостями",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сёностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёностью)$").unwrap(),
            строка: "сёностью",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сёности$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёности)$").unwrap(),
            строка: "сёности",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сёностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёностей)$").unwrap(),
            строка: "сёностей",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сёностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёностям)$").unwrap(),
            строка: "сёностям",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сёностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёностями)$").unwrap(),
            строка: "сёностями",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        // ---- причастия/прилагательные с "нн" ----
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённая$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённая)$").unwrap(),
            строка: "сённая",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённую$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённую)$").unwrap(),
            строка: "сённую",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённое$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённое)$").unwrap(),
            строка: "сённое",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённого$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённого)$").unwrap(),
            строка: "сённого",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённому$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённому)$").unwrap(),
            строка: "сённому",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённым$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённым)$").unwrap(),
            строка: "сённым",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сёными$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёными)$").unwrap(),
            строка: "сёными",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сёнными$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёнными)$").unwrap(),
            строка: "сёнными",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённом$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённом)$").unwrap(),
            строка: "сённом",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённые$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённые)$").unwrap(),
            строка: "сённые",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённых$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённых)$").unwrap(),
            строка: "сённых",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сённый$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённый)$").unwrap(),
            строка: "сённый",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сённее$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённее)$").unwrap(),
            строка: "сённее",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        // ---- формы с одной "н" ----
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сён$").unwrap(),
            re_замены: Regex::new(r"(?i)(сён)$").unwrap(),
            строка: "сён",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сёна$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёна)$").unwrap(),
            строка: "сёна",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+сённа$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённа)$").unwrap(),
            строка: "сённа",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+сёны$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёны)$").unwrap(),
            строка: "сёны",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сённой$").unwrap(),
            re_замены: Regex::new(r"(?i)(сённой)$").unwrap(),
            строка: "сённой",
            правописание: Буквы_Слова {
                удвоенная_согласная: true,
                ..Default::default()
            },
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёной$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёной)$").unwrap(),
            строка: "сёной",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёные$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёные)$").unwrap(),
            строка: "сёные",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёное$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёное)$").unwrap(),
            строка: "сёное",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёная$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёная)$").unwrap(),
            строка: "сёная",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёном$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёном)$").unwrap(),
            строка: "сёном",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёному$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёному)$").unwrap(),
            строка: "сёному",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёную$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёную)$").unwrap(),
            строка: "сёную",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёного$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёного)$").unwrap(),
            строка: "сёного",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёным$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёным)$").unwrap(),
            строка: "сёным",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёных$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёных)$").unwrap(),
            строка: "сёных",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёный$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёный)$").unwrap(),
            строка: "сёный",
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+сёнее$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёнее)$").unwrap(),
            строка: "сёнее",
            вид_окончания: Вид_Окончания::Сён,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_СЁН: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "сённо",
        "сёно",
        "сёностям",
        "сёностями",
        "сён",
        "сёны",
        "сённости",
        "сённостям",
        "сённостями",
        "сёностей",
        "сёнее",
        "сённостей",
        "сёностью",
        "сённее",
        "сёности",
        "сённостью",
        "сёной",
        "сёна",
        "сённа",
        "сённой",
        "сёные",
        "сённые",
        "сёное",
        "сённое",
        "сёная",
        "сённая",
        "сёном",
        "сённом",
        "сёному",
        "сённому",
        "сёную",
        "сённую",
        "сёного",
        "сённого",
        "сёным",
        "сённым",
        "сёными",
        "сёнными",
        "сёных",
        "сённых",
        "сёный",
        "сённый",
    ])
});
/*pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЛЮ_ЛЯ: LazyLock<[RE_полное_окончание; 11]> = LazyLock::new(|| []);
pub static КУЧА_ОКОНЧАНИЙ_ЛЮ_ЛЯ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ля", "лю", "ли", /*"ле",*/ "лёй", "ль", "лям", "лями", "лях", "лась", "лись",
        "лось",
    ])
});*/
pub static КУЧА_ОКОНЧАНИЙ_Д: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "д", "да", "ду", "де", "дом", "дам", "дами", "дах", "ды", "дов",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЧИ: LazyLock<[RE_полное_окончание; 8]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чия$").unwrap(),
            re_замены: Regex::new(r"(?i)(чия)$").unwrap(),
            строка: "чия",
            вид_окончания: Вид_Окончания::Чи,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чие$").unwrap(),
            re_замены: Regex::new(r"(?i)(чие)$").unwrap(),
            строка: "чие",
            вид_окончания: Вид_Окончания::Чи,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чии$").unwrap(),
            re_замены: Regex::new(r"(?i)(чии)$").unwrap(),
            строка: "чии",
            вид_окончания: Вид_Окончания::Чи,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чий$").unwrap(),
            re_замены: Regex::new(r"(?i)(чий)$").unwrap(),
            строка: "чий",
            вид_окончания: Вид_Окончания::Чи,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чию$").unwrap(),
            re_замены: Regex::new(r"(?i)(чию)$").unwrap(),
            строка: "чию",
            вид_окончания: Вид_Окончания::Чи,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чиям$").unwrap(),
            re_замены: Regex::new(r"(?i)(чиям)$").unwrap(),
            строка: "чиям",
            вид_окончания: Вид_Окончания::Чи,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чиями$").unwrap(),
            re_замены: Regex::new(r"(?i)(чиями)$").unwrap(),
            строка: "чиями",
            вид_окончания: Вид_Окончания::Чи,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+чиях$").unwrap(),
            re_замены: Regex::new(r"(?i)(чиях)$").unwrap(),
            строка: "чиях",
            вид_окончания: Вид_Окончания::Чи,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЧИ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter(["чий", "чие", "чии", "чия", "чию", "чиям", "чиями", "чиях"])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЯТ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ят$").unwrap(),
            re_замены: Regex::new(r"(?i)(ят)$").unwrap(),
            строка: "ят",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ятее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятее)$").unwrap(),
            строка: "ятее",
            вид_окончания: Вид_Окончания::Ят,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ята$").unwrap(),
            re_замены: Regex::new(r"(?i)(ята)$").unwrap(),
            строка: "ята",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятая)$").unwrap(),
            строка: "ятая",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятую)$").unwrap(),
            строка: "ятую",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ято$").unwrap(),
            re_замены: Regex::new(r"(?i)(ято)$").unwrap(),
            строка: "ято",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+яты$").unwrap(),
            re_замены: Regex::new(r"(?i)(яты)$").unwrap(),
            строка: "яты",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятом)$").unwrap(),
            строка: "ятом",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятому)$").unwrap(),
            строка: "ятому",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        // Дубликат "ято" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятого)$").unwrap(),
            строка: "ятого",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятое)$").unwrap(),
            строка: "ятое",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятой)$").unwrap(),
            строка: "ятой",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ятость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятость)$").unwrap(),
            строка: "ятость",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ятостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостью)$").unwrap(),
            строка: "ятостью",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ятости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятости)$").unwrap(),
            строка: "ятости",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ятостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостей)$").unwrap(),
            строка: "ятостей",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ятостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостям)$").unwrap(),
            строка: "ятостям",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ятостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостями)$").unwrap(),
            строка: "ятостями",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ятостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостях)$").unwrap(),
            строка: "ятостях",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятым)$").unwrap(),
            строка: "ятым",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятыми)$").unwrap(),
            строка: "ятыми",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятый)$").unwrap(),
            строка: "ятый",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятые)$").unwrap(),
            строка: "ятые",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ятых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятых)$").unwrap(),
            строка: "ятых",
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЯТ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ят",
        "ятее",
        "ята",
        "ятая",
        "ятую",
        "ято",
        "яты",
        "ятом",
        "ятому",
        "ятого",
        "ятое",
        "ятой",
        "ятость",
        "ятостью",
        "ятости",
        "ятостей",
        "ятостям",
        "ятостями",
        "ятостях",
        "ятым",
        "ятыми",
        "ятый",
        "ятые",
        "ятых",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_АТ: LazyLock<[RE_полное_окончание; 27]> = LazyLock::new(|| {
    [
        /*  RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ат$").unwrap(),
            re_замены: Regex::new(r"(?i)(ат)$").unwrap(),
            окончание_строка: "ат",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атее$").unwrap(),
            re_замены: Regex::new(r"(?i)(атее)$").unwrap(),
            окончание_строка: "атее",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ата$").unwrap(),
            re_замены: Regex::new(r"(?i)(ата)$").unwrap(),
            окончание_строка: "ата",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атая$").unwrap(),
            re_замены: Regex::new(r"(?i)(атая)$").unwrap(),
            окончание_строка: "атая",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атую$").unwrap(),
            re_замены: Regex::new(r"(?i)(атую)$").unwrap(),
            окончание_строка: "атую",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+ато$").unwrap(),
            re_замены: Regex::new(r"(?i)(ато)$").unwrap(),
            окончание_строка: "ато",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+аты$").unwrap(),
            re_замены: Regex::new(r"(?i)(аты)$").unwrap(),
            окончание_строка: "аты",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атом$").unwrap(),
            re_замены: Regex::new(r"(?i)(атом)$").unwrap(),
            окончание_строка: "атом",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атому$").unwrap(),
            re_замены: Regex::new(r"(?i)(атому)$").unwrap(),
            окончание_строка: "атому",
            вид_окончания: Вид_Окончания::Ат,
        },
        // Дубликат "ато" удалён
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атого$").unwrap(),
            re_замены: Regex::new(r"(?i)(атого)$").unwrap(),
            окончание_строка: "атого",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атое$").unwrap(),
            re_замены: Regex::new(r"(?i)(атое)$").unwrap(),
            окончание_строка: "атое",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атой$").unwrap(),
            re_замены: Regex::new(r"(?i)(атой)$").unwrap(),
            окончание_строка: "атой",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атость$").unwrap(),
            re_замены: Regex::new(r"(?i)(атость)$").unwrap(),
            окончание_строка: "атость",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостью)$").unwrap(),
            окончание_строка: "атостью",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атости$").unwrap(),
            re_замены: Regex::new(r"(?i)(атости)$").unwrap(),
            окончание_строка: "атости",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостей)$").unwrap(),
            окончание_строка: "атостей",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостям)$").unwrap(),
            окончание_строка: "атостям",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостями)$").unwrap(),
            окончание_строка: "атостями",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостях)$").unwrap(),
            окончание_строка: "атостях",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атым$").unwrap(),
            re_замены: Regex::new(r"(?i)(атым)$").unwrap(),
            окончание_строка: "атым",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(атыми)$").unwrap(),
            окончание_строка: "атыми",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атый$").unwrap(),
            re_замены: Regex::new(r"(?i)(атый)$").unwrap(),
            окончание_строка: "атый",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атые$").unwrap(),
            re_замены: Regex::new(r"(?i)(атые)$").unwrap(),
            окончание_строка: "атые",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+атых$").unwrap(),
            re_замены: Regex::new(r"(?i)(атых)$").unwrap(),
            окончание_строка: "атых",
            вид_окончания: Вид_Окончания::Ат,
        },*/
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ать$").unwrap(),
            re_замены: Regex::new(r"(?i)(ать)$").unwrap(),
            строка: "ать",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+аться$").unwrap(),
            re_замены: Regex::new(r"(?i)(аться)$").unwrap(),
            строка: "аться",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атым$").unwrap(),
            re_замены: Regex::new(r"(?i)(атым)$").unwrap(),
            строка: "атым",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(атыми)$").unwrap(),
            строка: "атыми",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атый$").unwrap(),
            re_замены: Regex::new(r"(?i)(атый)$").unwrap(),
            строка: "атый",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атые$").unwrap(),
            re_замены: Regex::new(r"(?i)(атые)$").unwrap(),
            строка: "атые",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атых$").unwrap(),
            re_замены: Regex::new(r"(?i)(атых)$").unwrap(),
            строка: "атых",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+аты$").unwrap(),
            re_замены: Regex::new(r"(?i)(аты)$").unwrap(),
            строка: "аты",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ата$").unwrap(),
            re_замены: Regex::new(r"(?i)(ата)$").unwrap(),
            строка: "ата",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        /*RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+атех$").unwrap(),
            re_замены: Regex::new(r"(?i)(атех)$").unwrap(),
            строка: "атех",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },*/
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ате$").unwrap(),
            re_замены: Regex::new(r"(?i)(ате)$").unwrap(),
            строка: "ате",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атая$").unwrap(),
            re_замены: Regex::new(r"(?i)(атая)$").unwrap(),
            строка: "атая",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атую$").unwrap(),
            re_замены: Regex::new(r"(?i)(атую)$").unwrap(),
            строка: "атую",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ат$").unwrap(),
            re_замены: Regex::new(r"(?i)(ат)$").unwrap(),
            строка: "ат",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ато$").unwrap(),
            re_замены: Regex::new(r"(?i)(ато)$").unwrap(),
            строка: "ато",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атой$").unwrap(),
            re_замены: Regex::new(r"(?i)(атой)$").unwrap(),
            строка: "атой",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атее$").unwrap(),
            re_замены: Regex::new(r"(?i)(атее)$").unwrap(),
            строка: "атее",
            вид_окончания: Вид_Окончания::Ат,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атое$").unwrap(),
            re_замены: Regex::new(r"(?i)(атое)$").unwrap(),
            строка: "атое",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атому$").unwrap(),
            re_замены: Regex::new(r"(?i)(атому)$").unwrap(),
            строка: "атому",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атом$").unwrap(),
            re_замены: Regex::new(r"(?i)(атом)$").unwrap(),
            строка: "атом",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+атого$").unwrap(),
            re_замены: Regex::new(r"(?i)(атого)$").unwrap(),
            строка: "атого",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+атость$").unwrap(),
            re_замены: Regex::new(r"(?i)(атость)$").unwrap(),
            строка: "атость",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+атостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостью)$").unwrap(),
            строка: "атостью",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+атости$").unwrap(),
            re_замены: Regex::new(r"(?i)(атости)$").unwrap(),
            строка: "атости",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+атостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостей)$").unwrap(),
            строка: "атостей",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+атостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостям)$").unwrap(),
            строка: "атостям",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+атостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостями)$").unwrap(),
            строка: "атостями",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+атостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостях)$").unwrap(),
            строка: "атостях",
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_АТ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        //"ат",
        "ать",
        "аться",
        "атым",
        "атыми",
        "атый",
        "атые",
        "атых",
        "аты",
        "ата",
        "ате",
        "атая",
        "атую",
        "ат",
        "ато",
        "атой",
        "атее",
        "атое",
        "атому",
        "атом",
        "атого",
        "атость",
        "атостью",
        "атости",
        "атостей",
        "атостям",
        "атостями",
        "атостях",
        /*"атее",
        "ата",
        "атая",
        "атую",
        "ато",
        "аты",
        "атом",
        "атому",
        "атого",
        "атое",
        "атой",
        "атость",
        "атостью",
        "атости",
        "атостей",
        "атостям",
        "атостями",
        "атостях",
        "атым",
        "атыми",
        "атый",
        "атые",
        "атых",*/
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Л: LazyLock<[RE_полное_окончание; 23]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+л$").unwrap(),
            re_замены: Regex::new(r"(?i)(л)$").unwrap(),
            строка: "л",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ла$").unwrap(),
            re_замены: Regex::new(r"(?i)(ла)$").unwrap(),
            строка: "ла",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лу$").unwrap(),
            re_замены: Regex::new(r"(?i)(лу)$").unwrap(),
            строка: "лу",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ле$").unwrap(),
            re_замены: Regex::new(r"(?i)(ле)$").unwrap(),
            строка: "ле",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+лее$").unwrap(),
            re_замены: Regex::new(r"(?i)(лее)$").unwrap(),
            строка: "лее",
            вид_окончания: Вид_Окончания::_Л,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лом$").unwrap(),
            re_замены: Regex::new(r"(?i)(лом)$").unwrap(),
            строка: "лом",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лам$").unwrap(),
            re_замены: Regex::new(r"(?i)(лам)$").unwrap(),
            строка: "лам",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лами$").unwrap(),
            re_замены: Regex::new(r"(?i)(лами)$").unwrap(),
            строка: "лами",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лах$").unwrap(),
            re_замены: Regex::new(r"(?i)(лах)$").unwrap(),
            строка: "лах",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лы$").unwrap(),
            re_замены: Regex::new(r"(?i)(лы)$").unwrap(),
            строка: "лы",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+лось$").unwrap(),
            re_замены: Regex::new(r"(?i)(лось)$").unwrap(),
            строка: "лось",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+лись$").unwrap(),
            re_замены: Regex::new(r"(?i)(лись)$").unwrap(),
            строка: "лись",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+лась$").unwrap(),
            re_замены: Regex::new(r"(?i)(лась)$").unwrap(),
            строка: "лась",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ль$").unwrap(),
            re_замены: Regex::new(r"(?i)(ль)$").unwrap(),
            строка: "ль",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ля$").unwrap(),
            re_замены: Regex::new(r"(?i)(ля)$").unwrap(),
            строка: "ля",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лю$").unwrap(),
            re_замены: Regex::new(r"(?i)(лю)$").unwrap(),
            строка: "лю",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ли$").unwrap(),
            re_замены: Regex::new(r"(?i)(ли)$").unwrap(),
            строка: "ли",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        /*RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ле$").unwrap(),
            re_замены: Regex::new(r"(?i)(ле)$").unwrap(),
            окончание_строка: "ле",
            вид_окончания: Вид_Окончания::_Л,
        },*/
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лёй$").unwrap(),
            re_замены: Regex::new(r"(?i)(лёй)$").unwrap(),
            строка: "лёй",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        /*RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ль$").unwrap(),
            re_замены: Regex::new(r"(?i)(ль)$").unwrap(),
            окончание_строка: "ель",
            вид_окончания: Вид_Окончания::_Л,
        },*/
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лям$").unwrap(),
            re_замены: Regex::new(r"(?i)(лям)$").unwrap(),
            строка: "лям",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ло$").unwrap(),
            re_замены: Regex::new(r"(?i)(ло)$").unwrap(),
            строка: "ло",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лями$").unwrap(),
            re_замены: Regex::new(r"(?i)(лями)$").unwrap(),
            строка: "лями",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+лях$").unwrap(),
            re_замены: Regex::new(r"(?i)(лях)$").unwrap(),
            строка: "лях",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+лся$").unwrap(),
            re_замены: Regex::new(r"(?i)(лся)$").unwrap(),
            строка: "лся",
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        // Дубликат "ло" удалён
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Л: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "л", "ла", "ло", "лу", "ле", "лом", "лам", "лами", "лах", "лы", "лее", "ля", "лю", "ли",
        /*"ле",*/ "лёй", "ль", "лям", "лями", "лях", "лась", "лись", "лось", "лся",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЛЬН: LazyLock<[RE_полное_окончание; 23]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+льность$").unwrap(),
            re_замены: Regex::new(r"(?i)(льность)$").unwrap(),
            строка: "льность",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        // ВНИМАНИЕ: "льного" повторяется — дубликат удалён.
        // Если нужен именно второй экземпляр — верните и увеличьте размер до 25.
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+льностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(льностью)$").unwrap(),
            строка: "льностью",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+льности$").unwrap(),
            re_замены: Regex::new(r"(?i)(льности)$").unwrap(),
            строка: "льности",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+льностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(льностей)$").unwrap(),
            строка: "льностей",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+льностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(льностям)$").unwrap(),
            строка: "льностям",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+льностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(льностями)$").unwrap(),
            строка: "льностями",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+льностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(льностях)$").unwrap(),
            строка: "льностях",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        /*RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ле$").unwrap(),
            re_замены: Regex::new(r"(?i)(ле)$").unwrap(),
            окончание_строка: "ле",
            вид_окончания: Вид_Окончания::Льн,
        },*/
        // Дубликат "ло" удалён
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льным$").unwrap(),
            re_замены: Regex::new(r"(?i)(льным)$").unwrap(),
            строка: "льным",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льными$").unwrap(),
            re_замены: Regex::new(r"(?i)(льными)$").unwrap(),
            строка: "льными",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льный$").unwrap(),
            re_замены: Regex::new(r"(?i)(льный)$").unwrap(),
            строка: "льный",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льные$").unwrap(),
            re_замены: Regex::new(r"(?i)(льные)$").unwrap(),
            строка: "льные",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льных$").unwrap(),
            re_замены: Regex::new(r"(?i)(льных)$").unwrap(),
            строка: "льных",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+льнее$").unwrap(),
            re_замены: Regex::new(r"(?i)(льнее)$").unwrap(),
            строка: "льнее",
            вид_окончания: Вид_Окончания::Льн,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льна$").unwrap(),
            re_замены: Regex::new(r"(?i)(льна)$").unwrap(),
            строка: "льна",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льная$").unwrap(),
            re_замены: Regex::new(r"(?i)(льная)$").unwrap(),
            строка: "льная",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льную$").unwrap(),
            re_замены: Regex::new(r"(?i)(льную)$").unwrap(),
            строка: "льную",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льно$").unwrap(),
            re_замены: Regex::new(r"(?i)(льно)$").unwrap(),
            строка: "льно",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льны$").unwrap(),
            re_замены: Regex::new(r"(?i)(льны)$").unwrap(),
            строка: "льны",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льном$").unwrap(),
            re_замены: Regex::new(r"(?i)(льном)$").unwrap(),
            строка: "льном",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+льному$").unwrap(),
            re_замены: Regex::new(r"(?i)(льному)$").unwrap(),
            строка: "льному",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льного$").unwrap(),
            re_замены: Regex::new(r"(?i)(льного)$").unwrap(),
            строка: "льного",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льное$").unwrap(),
            re_замены: Regex::new(r"(?i)(льное)$").unwrap(),
            строка: "льное",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+льной$").unwrap(),
            re_замены: Regex::new(r"(?i)(льной)$").unwrap(),
            строка: "льной",
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЛЬН: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "льность",
        "льностью",
        "льности",
        "льностей",
        "льностям",
        "льностями",
        "льностях",
        "льным",
        "льными",
        "льный",
        "льные",
        "льных",
        "льнее",
        "льна",
        "льная",
        "льную",
        "льно",
        "льны",
        "льном",
        "льному",
        "льного",
        "льное",
        "льной",
    ])
});
/*pub static РАЗДЕЛ_ОКОНЧАНИЙ_К: LazyLock<[RE_полное_окончание; 10]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ок$").unwrap(),
            re_замены: Regex::new(r"(?i)(ок)$").unwrap(),
            окончание_строка: "ок",
            вид_окончания: Вид_Окончания::_К,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ка$").unwrap(),
            re_замены: Regex::new(r"(?i)(ка)$").unwrap(),
            окончание_строка: "ка",
            вид_окончания: Вид_Окончания::_К,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ку$").unwrap(),
            re_замены: Regex::new(r"(?i)(ку)$").unwrap(),
            окончание_строка: "ку",
            вид_окончания: Вид_Окончания::_К,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ке$").unwrap(),
            re_замены: Regex::new(r"(?i)(ке)$").unwrap(),
            окончание_строка: "ке",
            вид_окончания: Вид_Окончания::_К,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ком$").unwrap(),
            re_замены: Regex::new(r"(?i)(ком)$").unwrap(),
            окончание_строка: "ком",
            вид_окончания: Вид_Окончания::_К,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+кам$").unwrap(),
            re_замены: Regex::new(r"(?i)(кам)$").unwrap(),
            окончание_строка: "кам",
            вид_окончания: Вид_Окончания::_К,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ками$").unwrap(),
            re_замены: Regex::new(r"(?i)(ками)$").unwrap(),
            окончание_строка: "ками",
            вид_окончания: Вид_Окончания::_К,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ках$").unwrap(),
            re_замены: Regex::new(r"(?i)(ках)$").unwrap(),
            окончание_строка: "ках",
            вид_окончания: Вид_Окончания::_К,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ков$").unwrap(),
            re_замены: Regex::new(r"(?i)(ков)$").unwrap(),
            окончание_строка: "ков",
            вид_окончания: Вид_Окончания::_К,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ки$").unwrap(),
            re_замены: Regex::new(r"(?i)(ки)$").unwrap(),
            окончание_строка: "ки",
            вид_окончания: Вид_Окончания::_К,
        },
        // Дубликат "ко" удалён
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_К: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ок", "ка", "ку", "ке", "ком", "кам", "ками", "ках", "ков", "ки",
    ])
});*/
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Г_ПОЛН: LazyLock<[RE_полное_окончание; 23]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+г$").unwrap(),
            re_замены: Regex::new(r"(?i)(г)$").unwrap(),
            строка: "г",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+га$").unwrap(),
            re_замены: Regex::new(r"(?i)(га)$").unwrap(),
            строка: "га",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+гая$").unwrap(),
            re_замены: Regex::new(r"(?i)(гая)$").unwrap(),
            строка: "гая",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+гую$").unwrap(),
            re_замены: Regex::new(r"(?i)(гую)$").unwrap(),
            строка: "гую",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Местоимение,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+гое$").unwrap(),
            re_замены: Regex::new(r"(?i)(гое)$").unwrap(),
            строка: "гое",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ги$").unwrap(),
            re_замены: Regex::new(r"(?i)(ги)$").unwrap(),
            строка: "ги",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+гом$").unwrap(),
            re_замены: Regex::new(r"(?i)(гом)$").unwrap(),
            строка: "гом",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+гому$").unwrap(),
            re_замены: Regex::new(r"(?i)(гому)$").unwrap(),
            строка: "гому",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Местоимение],
            re_поиска: Regex::new(r"(?i)\w+гого$").unwrap(),
            re_замены: Regex::new(r"(?i)(гого)$").unwrap(),
            строка: "гого",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Наречие,
                Вид_части_речи::Местоимение,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+го$").unwrap(),
            re_замены: Regex::new(r"(?i)(го)$").unwrap(),
            строка: "го",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+гой$").unwrap(),
            re_замены: Regex::new(r"(?i)(гой)$").unwrap(),
            строка: "гой",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+гость$").unwrap(),
            re_замены: Regex::new(r"(?i)(гость)$").unwrap(),
            строка: "гость",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+гостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостью)$").unwrap(),
            строка: "гостью",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+гости$").unwrap(),
            re_замены: Regex::new(r"(?i)(гости)$").unwrap(),
            строка: "гости",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+гостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостей)$").unwrap(),
            строка: "гостей",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+гостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостям)$").unwrap(),
            строка: "гостям",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+гостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостями)$").unwrap(),
            строка: "гостями",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+гостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостях)$").unwrap(),
            строка: "гостях",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Местоимение],
            re_поиска: Regex::new(r"(?i)\w+гим$").unwrap(),
            re_замены: Regex::new(r"(?i)(гим)$").unwrap(),
            строка: "гим",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+гими$").unwrap(),
            re_замены: Regex::new(r"(?i)(гими)$").unwrap(),
            строка: "гими",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+гий$").unwrap(),
            re_замены: Regex::new(r"(?i)(гий)$").unwrap(),
            строка: "гий",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+гие$").unwrap(),
            re_замены: Regex::new(r"(?i)(гие)$").unwrap(),
            строка: "гие",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Местоимение],
            re_поиска: Regex::new(r"(?i)\w+гих$").unwrap(),
            re_замены: Regex::new(r"(?i)(гих)$").unwrap(),
            строка: "гих",
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Г_ПОЛН: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "г",
        "га",
        "гая",
        "гую",
        "гое",
        "ги",
        "гом",
        "гому",
        "гого",
        "го",
        "гой",
        "гость",
        "гостью",
        "гости",
        "гостей",
        "гостям",
        "гостями",
        "гостях",
        "гим",
        "гими",
        "гий",
        "гие",
        "гих",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СЛ: LazyLock<[RE_полное_окончание; 17]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+есена$").unwrap(),
            re_замены: Regex::new(r"(?i)(есена)$").unwrap(),
            строка: "есена",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+есен$").unwrap(),
            re_замены: Regex::new(r"(?i)(есен)$").unwrap(),
            строка: "есен",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+есён$").unwrap(),
            re_замены: Regex::new(r"(?i)(есён)$").unwrap(),
            строка: "есён",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ес$").unwrap(),
            re_замены: Regex::new(r"(?i)(ес)$").unwrap(),
            строка: "ес",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+есем$").unwrap(),
            re_замены: Regex::new(r"(?i)(есем)$").unwrap(),
            строка: "есем",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+есём$").unwrap(),
            re_замены: Regex::new(r"(?i)(есём)$").unwrap(),
            строка: "есём",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ёс$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёс)$").unwrap(),
            строка: "ёс",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сла$").unwrap(),
            re_замены: Regex::new(r"(?i)(сла)$").unwrap(),
            строка: "сла",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+слась$").unwrap(),
            re_замены: Regex::new(r"(?i)(слась)$").unwrap(),
            строка: "слась",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сло$").unwrap(),
            re_замены: Regex::new(r"(?i)(сло)$").unwrap(),
            строка: "сло",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+слось$").unwrap(),
            re_замены: Regex::new(r"(?i)(слось)$").unwrap(),
            строка: "слось",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+слись$").unwrap(),
            re_замены: Regex::new(r"(?i)(слись)$").unwrap(),
            строка: "слись",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сли$").unwrap(),
            re_замены: Regex::new(r"(?i)(сли)$").unwrap(),
            строка: "сли",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сет$").unwrap(),
            re_замены: Regex::new(r"(?i)(сет)$").unwrap(),
            строка: "сет",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сёт$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёт)$").unwrap(),
            строка: "сёт",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сут$").unwrap(),
            re_замены: Regex::new(r"(?i)(сут)$").unwrap(),
            строка: "сут",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+сутся$").unwrap(),
            re_замены: Regex::new(r"(?i)(сутся)$").unwrap(),
            строка: "сутся",
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        /*/ RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сти$").unwrap(),
            re_замены: Regex::new(r"(?i)(сти)$").unwrap(),
            окончание_строка: "сти",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стись$").unwrap(),
            re_замены: Regex::new(r"(?i)(стись)$").unwrap(),
            окончание_строка: "стись",
            вид_окончания: Вид_Окончания::Сл,
        },*/
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_СЛ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "есена",
        "есен",
        "есён",
        "ес",
        "есем",
        "есём",
        "ёс",
        "сла",
        "слась",
        "сло",
        "слось",
        "слись",
        "сли",
        "сет",
        "сёт",
        "сут",
        "сутся",
        /*"сти",
        "стись",*/
    ])
});
/*pub static РАЗДЕЛ_ОКОНЧАНИЙ_КО_КА: LazyLock<[RE_полное_окончание; 11]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+кой$").unwrap(),
            re_замены: Regex::new(r"(?i)(кой)$").unwrap(),
            окончание_строка: "кой",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ка$").unwrap(),
            re_замены: Regex::new(r"(?i)(ка)$").unwrap(),
            окончание_строка: "ка",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ки$").unwrap(),
            re_замены: Regex::new(r"(?i)(ки)$").unwrap(),
            окончание_строка: "ки",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ке$").unwrap(),
            re_замены: Regex::new(r"(?i)(ке)$").unwrap(),
            окончание_строка: "ке",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ко$").unwrap(),
            re_замены: Regex::new(r"(?i)(ко)$").unwrap(),
            окончание_строка: "ко",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ку$").unwrap(),
            re_замены: Regex::new(r"(?i)(ку)$").unwrap(),
            окончание_строка: "ку",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+кою$").unwrap(),
            re_замены: Regex::new(r"(?i)(кою)$").unwrap(),
            окончание_строка: "кою",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ок$").unwrap(),
            re_замены: Regex::new(r"(?i)(ок)$").unwrap(),
            окончание_строка: "ок",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+кам$").unwrap(),
            re_замены: Regex::new(r"(?i)(кам)$").unwrap(),
            окончание_строка: "кам",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ками$").unwrap(),
            re_замены: Regex::new(r"(?i)(ками)$").unwrap(),
            окончание_строка: "ками",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ках$").unwrap(),
            re_замены: Regex::new(r"(?i)(ках)$").unwrap(),
            окончание_строка: "ках",
            вид_окончания: Вид_Окончания::Ко_ка,
        },
    ]
});

pub static КУЧА_ОКОНЧАНИЙ_КО_КА: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "кой", "ка", "ки", "ке", "ко", "ку", "кою", "ок", "кам", "ками", "ках",
    ])
});*/
pub static РАЗДЕЛ_ОКОНЧАНИЙ_КО_КА_К: LazyLock<[RE_полное_окончание; 30]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+костей$").unwrap(),
            re_замены: Regex::new(r"(?i)(костей)$").unwrap(),
            строка: "костей",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+кое$").unwrap(),
            re_замены: Regex::new(r"(?i)(кое)$").unwrap(),
            строка: "кое",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+кого$").unwrap(),
            re_замены: Regex::new(r"(?i)(кого)$").unwrap(),
            строка: "кого",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+кой$").unwrap(),
            re_замены: Regex::new(r"(?i)(кой)$").unwrap(),
            строка: "кой",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+кая$").unwrap(),
            re_замены: Regex::new(r"(?i)(кая)$").unwrap(),
            строка: "кая",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+кости$").unwrap(),
            re_замены: Regex::new(r"(?i)(кости)$").unwrap(),
            строка: "кости",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+костях$").unwrap(),
            re_замены: Regex::new(r"(?i)(костях)$").unwrap(),
            строка: "костях",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+кость$").unwrap(),
            re_замены: Regex::new(r"(?i)(кость)$").unwrap(),
            строка: "кость",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+костью$").unwrap(),
            re_замены: Regex::new(r"(?i)(костью)$").unwrap(),
            строка: "костью",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+костям$").unwrap(),
            re_замены: Regex::new(r"(?i)(костям)$").unwrap(),
            строка: "костям",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+костями$").unwrap(),
            re_замены: Regex::new(r"(?i)(костями)$").unwrap(),
            строка: "костями",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ок$").unwrap(),
            re_замены: Regex::new(r"(?i)(ок)$").unwrap(),
            строка: "ок",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ка$").unwrap(),
            re_замены: Regex::new(r"(?i)(ка)$").unwrap(),
            строка: "ка",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+кую$").unwrap(),
            re_замены: Regex::new(r"(?i)(кую)$").unwrap(),
            строка: "кую",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ку$").unwrap(),
            re_замены: Regex::new(r"(?i)(ку)$").unwrap(),
            строка: "ку",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ке$").unwrap(),
            re_замены: Regex::new(r"(?i)(ке)$").unwrap(),
            строка: "ке",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ком$").unwrap(),
            re_замены: Regex::new(r"(?i)(ком)$").unwrap(),
            строка: "ком",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+кам$").unwrap(),
            re_замены: Regex::new(r"(?i)(кам)$").unwrap(),
            строка: "кам",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ками$").unwrap(),
            re_замены: Regex::new(r"(?i)(ками)$").unwrap(),
            строка: "ками",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ках$").unwrap(),
            re_замены: Regex::new(r"(?i)(ках)$").unwrap(),
            строка: "ках",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ков$").unwrap(),
            re_замены: Regex::new(r"(?i)(ков)$").unwrap(),
            строка: "ков",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+кими$").unwrap(),
            re_замены: Regex::new(r"(?i)(кими)$").unwrap(),
            строка: "кими",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ким$").unwrap(),
            re_замены: Regex::new(r"(?i)(ким)$").unwrap(),
            строка: "ким",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ких$").unwrap(),
            re_замены: Regex::new(r"(?i)(ких)$").unwrap(),
            строка: "ких",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+ки$").unwrap(),
            re_замены: Regex::new(r"(?i)(ки)$").unwrap(),
            строка: "ки",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+кие$").unwrap(),
            re_замены: Regex::new(r"(?i)(кие)$").unwrap(),
            строка: "кие",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+кий$").unwrap(),
            re_замены: Regex::new(r"(?i)(кий)$").unwrap(),
            строка: "кий",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Существительное,
                Вид_части_речи::Наречие,
            ],
            re_поиска: Regex::new(r"(?i)\w+ко$").unwrap(),
            re_замены: Regex::new(r"(?i)(ко)$").unwrap(),
            строка: "ко",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+кому$").unwrap(),
            re_замены: Regex::new(r"(?i)(кому)$").unwrap(),
            строка: "кому",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+кою$").unwrap(),
            re_замены: Regex::new(r"(?i)(кою)$").unwrap(),
            строка: "кою",
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_КО_КА_К: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "костей",
        "ких",
        "кое",
        "кие",
        "кий",
        // из КО_КА
        "кому",
        "кой",
        "ка",
        "ки",
        "кий",
        "ке",
        "ко",
        "ку",
        "кую",
        "кою",
        "ок",
        "кам",
        "ками",
        "ках",
        "кими",
        "ким", // из К (добавляются те, которых не было)
        "ком",
        "ков",
        "кой",
        "костями",
        "костям",
        "костью",
        "кость",
        "кая",
        "кости",
        "костях",
        "кого",
    ])
});
pub static КУЧА_ОКОНЧАНИЙ_Ж: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "жу", "ж", "жи", "же", "жей", "жею", "жю", "жам", "жами", "жах",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Ж: LazyLock<[RE_полное_окончание; 10]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+жу$").unwrap(),
            re_замены: Regex::new(r"(?i)(жу)$").unwrap(),
            строка: "жу",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ж$").unwrap(),
            re_замены: Regex::new(r"(?i)(ж)$").unwrap(),
            строка: "ж",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жи$").unwrap(),
            re_замены: Regex::new(r"(?i)(жи)$").unwrap(),
            строка: "жи",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+же$").unwrap(),
            re_замены: Regex::new(r"(?i)(же)$").unwrap(),
            строка: "же",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жей$").unwrap(),
            re_замены: Regex::new(r"(?i)(жей)$").unwrap(),
            строка: "жей",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА,
            re_поиска: Regex::new(r"(?i)\w+жею$").unwrap(),
            re_замены: Regex::new(r"(?i)(жею)$").unwrap(),
            строка: "жею",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Наречие,
                Вид_части_речи::Глагол,
            ],
            re_поиска: Regex::new(r"(?i)\w+жю$").unwrap(),
            re_замены: Regex::new(r"(?i)(жю)$").unwrap(),
            строка: "жю",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жам$").unwrap(),
            re_замены: Regex::new(r"(?i)(жам)$").unwrap(),
            строка: "жам",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жами$").unwrap(),
            re_замены: Regex::new(r"(?i)(жами)$").unwrap(),
            строка: "жами",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+жах$").unwrap(),
            re_замены: Regex::new(r"(?i)(жах)$").unwrap(),
            строка: "жах",
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
    ]
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_И: LazyLock<[RE_полное_окончание; 10]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+и$").unwrap(),
            re_замены: Regex::new(r"(?i)(и)$").unwrap(),
            строка: "и",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+им$").unwrap(),
            re_замены: Regex::new(r"(?i)(им)$").unwrap(),
            строка: "им",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ими$").unwrap(),
            re_замены: Regex::new(r"(?i)(ими)$").unwrap(),
            строка: "ими",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+их$").unwrap(),
            re_замены: Regex::new(r"(?i)(их)$").unwrap(),
            строка: "их",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ихся$").unwrap(),
            re_замены: Regex::new(r"(?i)(ихся)$").unwrap(),
            строка: "ихся",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ий$").unwrap(),
            re_замены: Regex::new(r"(?i)(ий)$").unwrap(),
            строка: "ий",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+ие$").unwrap(),
            re_замены: Regex::new(r"(?i)(ие)$").unwrap(),
            строка: "ие",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+иеся$").unwrap(),
            re_замены: Regex::new(r"(?i)(иеся)$").unwrap(),
            строка: "иеся",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+ийся$").unwrap(),
            re_замены: Regex::new(r"(?i)(ийся)$").unwrap(),
            строка: "ийся",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+имися$").unwrap(),
            re_замены: Regex::new(r"(?i)(имися)$").unwrap(),
            строка: "имися",
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
    ]
});
//
pub static КУЧА_ОКОНЧАНИЙ_И: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "и",
        "им",
        "ими",
        "их",
        "ихся",
        "ий",
        "ие",
        "ие",
        "иеся",
        "ийся",
        "имися",
    ])
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Ы: LazyLock<[RE_полное_окончание; 1]> = LazyLock::new(|| {
    [RE_полное_окончание {
        части_речи: &[
            Вид_части_речи::Существительное,
            Вид_части_речи::Причастие,
            Вид_части_речи::Прилагательное,
        ],
        re_поиска: Regex::new(r"(?i)\w+ы$").unwrap(),
        re_замены: Regex::new(r"(?i)(ы)$").unwrap(),
        строка: "ы",
        вид_окончания: Вид_Окончания::_Ы,
        ..Default::default()
    }]
});
pub static КУЧА_ОКОНЧАНИЙ_Ы: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter(["ы"]));
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Е: LazyLock<[RE_полное_окончание; 3]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Наречие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+е$").unwrap(),
            re_замены: Regex::new(r"(?i)(е)$").unwrap(),
            строка: "е",
            вид_окончания: Вид_Окончания::_Е,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ей)$").unwrap(),
            строка: "ей",
            вид_окончания: Вид_Окончания::_Е,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+еей$").unwrap(),
            re_замены: Regex::new(r"(?i)(еей)$").unwrap(),
            строка: "еей",
            вид_окончания: Вид_Окончания::_Е,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Е: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter(["е", "ей", "еей"]));
pub static РАЗДЕЛ_ОКОНЧАНИЙ_О: LazyLock<[RE_полное_окончание; 1]> = LazyLock::new(|| {
    [RE_полное_окончание {
        части_речи: &[
            Вид_части_речи::Прилагательное,
            Вид_части_речи::Существительное,
            Вид_части_речи::Наречие,
        ],
        re_поиска: Regex::new(r"(?i)\w+ой$").unwrap(),
        re_замены: Regex::new(r"(?i)(ой)$").unwrap(),
        строка: "ой",
        вид_окончания: Вид_Окончания::_О,
        ..Default::default()
    }]
});
pub static КУЧА_ОКОНЧАНИЙ_О: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter(["ой"]));
pub static РАЗДЕЛ_ОКОНЧАНИЙ_А: LazyLock<[RE_полное_окончание; 2]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+а$").unwrap(),
            re_замены: Regex::new(r"(?i)(а)$").unwrap(),
            строка: "а",
            вид_окончания: Вид_Окончания::_А,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+ах$").unwrap(),
            re_замены: Regex::new(r"(?i)(ах)$").unwrap(),
            строка: "ах",
            вид_окончания: Вид_Окончания::_А,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_А: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter(["а", "ах"]));
//------------------------------------------------------
pub static РАЗДЕЛЫ_ОКОНЧАНИЙ_ПОЛНЫЕ: LazyLock<Разделы_окончаний_Полные> =
    LazyLock::new(|| Разделы_окончаний_Полные {
        /*/ в_одиночн: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_В,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_В,
            вид_окончания: Вид_Окончания::_В,
        },*/
        г_полн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Г_ПОЛН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Г_ПОЛН,
            вид_окончания: Вид_Окончания::Г_полн,
            ..Default::default()
        },
        /*к_одиночн: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_К,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_К,
            вид_окончания: Вид_Окончания::_К,
        },*/
        ет: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЕТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЕТ,
            вид_окончания: Вид_Окончания::Ет,
            ..Default::default()
        },
        ко_ка: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_КО_КА_К,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_КО_КА_К,
            вид_окончания: Вид_Окончания::Ко_ка,
            ..Default::default()
        },
        ель: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЕЛЬ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЕЛЬ,
            вид_окончания: Вид_Окончания::Ель,
            ..Default::default()
        },
        ван: Раздел_Окончания {
            куча_окончаний_полная:
                &*КУЧА_ПОЛНАЯ_ОКОНЧАНИЙ_ВАН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ВАН,
            вид_окончания: Вид_Окончания::Ван,
            куча_окончаний_поиска:
                &*КУЧА_ПОИСКА_ОКОНЧАНИЙ_ВАН,
            ..Default::default()
        },
        дан: Раздел_Окончания {
            куча_окончаний_полная:
                &*КУЧА_ПОЛНАЯ_ОКОНЧАНИЙ_ДАН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ДАН,
            вид_окончания: Вид_Окончания::Дан,
            //  &*КУЧА_ПОИСКА_ОКОНЧАНИЙ_ВАН,
            ..Default::default()
        },
        ыт: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЫТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЫТ,
            вид_окончания: Вид_Окончания::Ыт,
            ..Default::default()
        },
        ым: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЫМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЫМ,
            вид_окончания: Вид_Окончания::Ым,
            ..Default::default()
        },
        ут_ую_уя: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_УТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_УТ,
            вид_окончания: Вид_Окончания::Ут,
            ..Default::default()
        },
        у_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_У,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_У,
            вид_окончания: Вид_Окончания::_У,
            ..Default::default()
        },
        ют: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЮТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЮТ,
            вид_окончания: Вид_Окончания::Ют,
            ..Default::default()
        },
        ит: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ИТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ИТ,
            вид_окончания: Вид_Окончания::Ит,
            ..Default::default()
        },
        ти: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ТИ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ТИ,
            вид_окончания: Вид_Окончания::Ти,
            ..Default::default()
        },
        тн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ТН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ТН,
            вид_окончания: Вид_Окончания::Тн,
            ..Default::default()
        },
        пн: Раздел_Окончания {
            куча_окончаний_полная:
                &*КУЧА_ПОЛНАЯ_ОКОНЧАНИЙ_ПН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ПН,
            вид_окончания: Вид_Окончания::Пн,
            ..Default::default()
        },
        ят: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЯТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЯТ,
            вид_окончания: Вид_Окончания::Ят,
            ..Default::default()
        },
        сен: Раздел_Окончания {
            куча_окончаний_полная:
                &*КУЧА_ПОЛНАЯ_ОКОНЧАНИЙ_СЕН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СЕН,
            вид_окончания: Вид_Окончания::Сен,
            ..Default::default()
        },
        сён: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_СЁН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СЁН,
            вид_окончания: Вид_Окончания::Сён,
            ..Default::default()
        },
        с_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_С,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_С,
            вид_окончания: Вид_Окончания::_С,
            ..Default::default()
        },
        ат: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_АТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_АТ,
            вид_окончания: Вид_Окончания::Ат,
            ..Default::default()
        },
        чи: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЧИ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЧИ,
            вид_окончания: Вид_Окончания::Чи,
            ..Default::default()
        },
        ив: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ИВ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ИВ,
            вид_окончания: Вид_Окончания::Ив,
            ..Default::default()
        },
        н_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Н,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Н,
            вид_окончания: Вид_Окончания::_Н,
            ..Default::default()
        },
        ш: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Ш,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Ш,
            вид_окончания: Вид_Окончания::_Ш,
            ..Default::default()
        },
        щ: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Щ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Щ,
            вид_окончания: Вид_Окончания::_Щ,
            ..Default::default()
        },
        ъят: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЪЯТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЪЯТ,
            вид_окончания: Вид_Окончания::Ъят,
            ..Default::default()
        },
        нятн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_НЯТН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_НЯТН,
            вид_окончания: Вид_Окончания::Нятн,
            ..Default::default()
        },
        нят: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_НЯТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_НЯТ,
            вид_окончания: Вид_Окончания::Нят,
            ..Default::default()
        },
        ст: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_СТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СТ,
            вид_окончания: Вид_Окончания::Ст,
            ..Default::default()
        },
        ств: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_СТВ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СТВ,
            вид_окончания: Вид_Окончания::Ств,
            ..Default::default()
        },
        ен: Раздел_Окончания {
            куча_окончаний_полная:
                &*КУЧА_ОКОНЧАНИЙ_ПОЛНАЯ_ЕН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЕН,
            вид_окончания: Вид_Окончания::Ен,
            ..Default::default()
        },
        /*лю_ля: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЛЮ_ЛЯ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЛЮ_ЛЯ,
            вид_окончания: Вид_Окончания::_Л,
        },*/
        ён: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЁН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЁН,
            вид_окончания: Вид_Окончания::Ён,
            ..Default::default()
        },
        ов: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ОВ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ОВ,
            вид_окончания: Вид_Окончания::Ов,
            ..Default::default()
        },
        ев: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЕВ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЕВ,
            вид_окончания: Вид_Окончания::Ев,
            ..Default::default()
        },
        он: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ОН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ОН,
            вид_окончания: Вид_Окончания::Он,
            ..Default::default()
        },
        ем: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЕМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЕМ,
            вид_окончания: Вид_Окончания::Ем,
            ..Default::default()
        },
        ам: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_АМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_АМ,
            вид_окончания: Вид_Окончания::Ам,
            ..Default::default()
        },
        ом: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ОМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ОМ,
            вид_окончания: Вид_Окончания::Ом,
            ..Default::default()
        },
        ан: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_АН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_АН,
            вид_окончания: Вид_Окончания::Ан,
            ..Default::default()
        },
        ян: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЯН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЯН,
            вид_окончания: Вид_Окончания::Ян,
            ..Default::default()
        },
        ям: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЯМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЯМ,
            вид_окончания: Вид_Окончания::Ям,
            ..Default::default()
        },
        /*авш: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_АВШ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_АВШ,
            вид_окончания: Вид_Окончания::Авш,
        },*/
        вш: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ВШ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ВШ,
            вид_окончания: Вид_Окончания::Вш,
            ..Default::default()
        },
        ты: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ТЫ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ТЫ,
            вид_окончания: Вид_Окончания::Ты,
            ..Default::default()
        },
        та_то: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ТА_ТО,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ТА_ТО,
            вид_окончания: Вид_Окончания::Та_то,
            ..Default::default()
        },
        /*ни: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_НИ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_НИ,
            вид_окончания: Вид_Окончания::Ни,
        },*/
        д_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Д,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Д,
            вид_окончания: Вид_Окончания::_Д,
            ..Default::default()
        },
        и_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_И,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_И,
            вид_окончания: Вид_Окончания::_И,
            ..Default::default()
        },
        р_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Р,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Р,
            вид_окончания: Вид_Окончания::_Р,
            ..Default::default()
        },
        л_полная: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Л,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Л,
            вид_окончания: Вид_Окончания::_Л,
            ..Default::default()
        },
        льн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЛЬН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЛЬН,
            вид_окончания: Вид_Окончания::Льн,
            ..Default::default()
        },
        ж_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Ж,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Ж,
            вид_окончания: Вид_Окончания::_Ж,
            ..Default::default()
        },
        ва_во: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ВА_ВО,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_В_ВА_ВО,
            вид_окончания: Вид_Окончания::Ва_Во,
            ..Default::default()
        },
        нут: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_НУТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_НУТ,
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        ц_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Ц,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Ц,
            вид_окончания: Вид_Окончания::_Ц,
            ..Default::default()
        },
        ск: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_СК,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СК,
            вид_окончания: Вид_Окончания::Ск,
            ..Default::default()
        },
        сл: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_СЛ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СЛ,
            вид_окончания: Вид_Окончания::Сл,
            ..Default::default()
        },
        ы_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Ы,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Ы,
            вид_окончания: Вид_Окончания::_Ы,
            ..Default::default()
        },
        е_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Е,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Е,
            вид_окончания: Вид_Окончания::_Е,
            ..Default::default()
        },
        о_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_О,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_О,
            вид_окончания: Вид_Окончания::_О,
            ..Default::default()
        },
        ю_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_Ю,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Ю,
            вид_окончания: Вид_Окончания::_Ю,
            ..Default::default()
        },
        а_одиночн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_А,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_А,
            вид_окончания: Вид_Окончания::_А,
            ..Default::default()
        },
        чн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЧН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЧН,
            вид_окончания: Вид_Окончания::Чн,
            ..Default::default()
        },
        жн: Раздел_Окончания {
            куча_окончаний_полная: &*КУЧА_ОКОНЧАНИЙ_ЖН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЖН,
            вид_окончания: Вид_Окончания::Жн,
            ..Default::default()
        },
    });
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ПН: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пен$").unwrap(),
            re_замены: Regex::new(r"(?i)(пен)$").unwrap(),
            строка: "пен",
            вид_окончания: Вид_Окончания::Пн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Прилагательное, Вид_части_речи::Наречие],
            re_поиска: Regex::new(r"(?i)\w+пнее$").unwrap(),
            re_замены: Regex::new(r"(?i)(пнее)$").unwrap(),
            строка: "пнее",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                удвоенная_гласная: true,
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пна$").unwrap(),
            re_замены: Regex::new(r"(?i)(пна)$").unwrap(),
            строка: "пна",
            вид_окончания: Вид_Окончания::Пн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пная$").unwrap(),
            re_замены: Regex::new(r"(?i)(пная)$").unwrap(),
            строка: "пная",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пную$").unwrap(),
            re_замены: Regex::new(r"(?i)(пную)$").unwrap(),
            строка: "пную",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пны$").unwrap(),
            re_замены: Regex::new(r"(?i)(пны)$").unwrap(),
            строка: "пны",
            вид_окончания: Вид_Окончания::Пн,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пном$").unwrap(),
            re_замены: Regex::new(r"(?i)(пном)$").unwrap(),
            строка: "пном",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пному$").unwrap(),
            re_замены: Regex::new(r"(?i)(пному)$").unwrap(),
            строка: "пному",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пного$").unwrap(),
            re_замены: Regex::new(r"(?i)(пного)$").unwrap(),
            строка: "пного",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
                Вид_части_речи::Причастие,
            ],
            re_поиска: Regex::new(r"(?i)\w+пно$").unwrap(),
            re_замены: Regex::new(r"(?i)(пно)$").unwrap(),
            строка: "пно",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пное$").unwrap(),
            re_замены: Regex::new(r"(?i)(пное)$").unwrap(),
            строка: "пное",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пной$").unwrap(),
            re_замены: Regex::new(r"(?i)(пной)$").unwrap(),
            строка: "пной",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пность$").unwrap(),
            re_замены: Regex::new(r"(?i)(пность)$").unwrap(),
            строка: "пность",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(пностью)$").unwrap(),
            строка: "пностью",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пности$").unwrap(),
            re_замены: Regex::new(r"(?i)(пности)$").unwrap(),
            строка: "пности",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(пностей)$").unwrap(),
            строка: "пностей",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(пностям)$").unwrap(),
            строка: "пностям",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(пностями)$").unwrap(),
            строка: "пностями",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+пностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(пностях)$").unwrap(),
            строка: "пностях",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пным$").unwrap(),
            re_замены: Regex::new(r"(?i)(пным)$").unwrap(),
            строка: "пным",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пными$").unwrap(),
            re_замены: Regex::new(r"(?i)(пными)$").unwrap(),
            строка: "пными",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пный$").unwrap(),
            re_замены: Regex::new(r"(?i)(пный)$").unwrap(),
            строка: "пный",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пные$").unwrap(),
            re_замены: Regex::new(r"(?i)(пные)$").unwrap(),
            строка: "пные",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+пных$").unwrap(),
            re_замены: Regex::new(r"(?i)(пных)$").unwrap(),
            строка: "пных",
            вид_окончания: Вид_Окончания::Пн,
            правописание: Буквы_Слова {
                ..Default::default()
            },
            ..Default::default()
        },
    ]
});
pub static КУЧА_ПОЛНАЯ_ОКОНЧАНИЙ_ПН: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "пен",
        "пнее",
        "пна",
        "пно",
        "пны",
        //
        "пная",
        "пную",
        //о
        "пной",
        "пное",
        "пного",
        //глаголы
        "пном",
        "пному",
        //ст
        "пность",
        "пностью",
        "пности",
        "пностей",
        "пностям",
        "пностями",
        "пностях",
        //ы
        "пными",
        "пным",
        "пных",
        "пные",
        "пный",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_НУТ: LazyLock<[RE_полное_окончание; 33]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Глагол,
                Вид_части_речи::Причастие,
                Вид_части_речи::Существительное,
            ],
            re_поиска: Regex::new(r"(?i)\w+нут$").unwrap(),
            re_замены: Regex::new(r"(?i)(нут)$").unwrap(),
            строка: "нут",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+нете$").unwrap(),
            re_замены: Regex::new(r"(?i)(нете)$").unwrap(),
            строка: "нете",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+нёте$").unwrap(),
            re_замены: Regex::new(r"(?i)(нёте)$").unwrap(),
            строка: "нёте",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Существительное,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+нута$").unwrap(),
            re_замены: Regex::new(r"(?i)(нута)$").unwrap(),
            строка: "нута",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нутая$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутая)$").unwrap(),
            строка: "нутая",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нутую$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутую)$").unwrap(),
            строка: "нутую",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нутое$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутое)$").unwrap(),
            строка: "нутое",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+нуты$").unwrap(),
            re_замены: Regex::new(r"(?i)(нуты)$").unwrap(),
            строка: "нуты",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нутом$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутом)$").unwrap(),
            строка: "нутом",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нутому$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутому)$").unwrap(),
            строка: "нутому",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нутого$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутого)$").unwrap(),
            строка: "нутого",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[
                Вид_части_речи::Причастие,
                Вид_части_речи::Наречие,
                Вид_части_речи::Прилагательное,
            ],
            re_поиска: Regex::new(r"(?i)\w+нуто$").unwrap(),
            re_замены: Regex::new(r"(?i)(нуто)$").unwrap(),
            строка: "нуто",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие, Вид_части_речи::Прилагательное],
            re_поиска: Regex::new(r"(?i)\w+нутой$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутой)$").unwrap(),
            строка: "нутой",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нутость$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутость)$").unwrap(),
            строка: "нутость",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нутостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутостью)$").unwrap(),
            строка: "нутостью",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нутости$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутости)$").unwrap(),
            строка: "нутости",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нутостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутостей)$").unwrap(),
            строка: "нутостей",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нутостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутостям)$").unwrap(),
            строка: "нутостям",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нутостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутостями)$").unwrap(),
            строка: "нутостями",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Существительное],
            re_поиска: Regex::new(r"(?i)\w+нутостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутостях)$").unwrap(),
            строка: "нутостях",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нутым$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутым)$").unwrap(),
            строка: "нутым",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нутыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутыми)$").unwrap(),
            строка: "нутыми",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нутый$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутый)$").unwrap(),
            строка: "нутый",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нутые$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутые)$").unwrap(),
            строка: "нутые",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Причастие],
            re_поиска: Regex::new(r"(?i)\w+нутых$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутых)$").unwrap(),
            строка: "нутых",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        /*
         *  RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ни$").unwrap(),
            re_замены: Regex::new(r"(?i)(ни)$").unwrap(),
            строка: "ни",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },*/
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+ните$").unwrap(),
            re_замены: Regex::new(r"(?i)(ните)$").unwrap(),
            строка: "ните",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Деепричастие],
            re_поиска: Regex::new(r"(?i)\w+ну$").unwrap(),
            re_замены: Regex::new(r"(?i)(ну)$").unwrap(),
            строка: "ну",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Деепричастие],
            re_поиска: Regex::new(r"(?i)\w+нусь$").unwrap(),
            re_замены: Regex::new(r"(?i)(нусь)$").unwrap(),
            строка: "нусь",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+нутся$").unwrap(),
            re_замены: Regex::new(r"(?i)(нутся)$").unwrap(),
            строка: "нутся",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+нет$").unwrap(),
            re_замены: Regex::new(r"(?i)(нет)$").unwrap(),
            строка: "нет",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+нется$").unwrap(),
            re_замены: Regex::new(r"(?i)(нется)$").unwrap(),
            строка: "нется",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+нем$").unwrap(),
            re_замены: Regex::new(r"(?i)(нем)$").unwrap(),
            строка: "нем",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
        RE_полное_окончание {
            части_речи: &[Вид_части_речи::Глагол],
            re_поиска: Regex::new(r"(?i)\w+немся$").unwrap(),
            re_замены: Regex::new(r"(?i)(немся)$").unwrap(),
            строка: "немся",
            вид_окончания: Вид_Окончания::Нут,
            ..Default::default()
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_НУТ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        // из раздела
        "нутся",
        "нут",
        "нете",
        "нёте",
        "нута",
        "нутая",
        "нутую",
        "нутое",
        "нуты",
        "нутом",
        "нутому",
        "нутого",
        "нуто",
        "нутой",
        "нутость",
        "нутостью",
        "нутости",
        "нутостей",
        "нутостям",
        "нутостями",
        "нутостях",
        "нутым",
        "нутыми",
        "нутый",
        "нутые",
        "нутых",
        //"ни",
        "ните",
        "ну",
        "нусь",
        "нет",
        "нется",
        "нем",
        "немся",
    ])
});
//
#[derive(Debug)]
pub struct Разделы_окончаний_Полные {
    //pub в_одиночн: Раздел_Окончания,
    pub ю_одиночн: Раздел_Окончания,
    pub е_одиночн: Раздел_Окончания,
    pub ы_одиночн: Раздел_Окончания,
    pub ж_одиночн: Раздел_Окончания,
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
    pub д_одиночн: Раздел_Окончания,
    pub ц_одиночн: Раздел_Окончания,
    pub р_одиночн: Раздел_Окончания,
    pub н_одиночн: Раздел_Окончания,
    pub а_одиночн: Раздел_Окончания,
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
    pub вш: Раздел_Окончания,
    //pub ни: Раздел_Окончания,
    pub ск: Раздел_Окончания,
    pub нятн: Раздел_Окончания,
    pub ты: Раздел_Окончания,
    pub та_то: Раздел_Окончания,
    pub ва_во: Раздел_Окончания,
    pub нут: Раздел_Окончания,
    pub дан: Раздел_Окончания,
    pub чн: Раздел_Окончания,
    pub ян: Раздел_Окончания,
    pub жн: Раздел_Окончания,
    pub льн: Раздел_Окончания,
}
impl Разделы_окончаний_Полные {
    /// Порядок: ван, ыт, ит, н, ст, ен, ён, он, ем, ом, ан, авш.
    pub fn iter(&self) -> impl Iterator<Item = &Раздел_Окончания> {
        [
            &self.ж_одиночн,
            &self.р_одиночн,
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
            &self.ен,
            &self.ет,
            &self.сён,
            &self.сен,
            &self.ён,
            &self.он,
            &self.сл,
            &self.ко_ка,
            &self.нут,
            &self.ем,
            &self.ев,
            &self.ти,
            &self.чи,
            &self.ов,
            &self.ам,
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
            &self.ты,
            &self.та_то,
            &self.жн,
            &self.тн,
            //вторая очередь
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
            &self.д_одиночн,
            //&self.к_одиночн,
            &self.ц_одиночн,
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
        ]
        .into_iter()
    }
    pub fn получить_ячейку_по_срезу(
        &self,
        окончание_входное: &str,
    ) -> Result<RE_полное_окончание, ()> {
        //
        for раздел in self.iter() {
            //если есть в куче окончаний
            if раздел.куча_окончаний_полная.contains(окончание_входное)
            {
                //поиск в разделе
                for ячейка in раздел.re_ячейки.iter() {
                    if ячейка.строка == окончание_входное {
                        return Ok(ячейка.clone());
                    }
                }
            }
        }
        return Err(());
        //
    }
    pub fn определить_вид_части_речи_по_срезу(
        &self,
        окончание_входное: &str,
    ) -> Result<&[Вид_части_речи], ()> {
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
        for раздел in self.iter() {
            //если есть в куче окончаний
            if раздел.куча_окончаний_полная.contains(ячейка_торжка.строка)
            {
                //поиск в разделе
                for ячейка in раздел.re_ячейки.iter() {
                    //
                    if ячейка.строка == ячейка_торжка.строка {
                        return Ok(ячейка.части_речи);
                    }
                }
            }
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
                раздел_торжка.первая_очередь.iter()
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
            'указатель_ячейки_торжка: for ячейка_торжка in
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
            }
        }
        return Ok(ЧАСТИ_РЕЧИ_НЕОПРЕДЕЛЕНА);
        //
    }
    pub fn проверка_на_наличие_окончаний(&self) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&Вид_Окончания> =
            rapidhash::fast::RapidHashSet::default();

        //перебор кучи окончаний
        //перебор раздела
        'главный_указатель: for окончание in ВСЕ_ВИДЫ_ОКОНЧАНИЙ.iter()
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
            for (_, раздел_2) in self
                .iter()
                .enumerate()
                .filter(|(указатель_2, _)| *указатель_2 != указатель_главный)
            //
            {
                for слово_1 in раздел.куча_окончаний_полная.iter() {
                    if раздел_2.куча_окончаний_полная.contains(слово_1)
                    {
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
}
//
pub static ПУСТАЯ_КУЧА_STR: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter([]));
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
        }
    }
}
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
pub trait Возможноси_частей_речи {
    fn вывести_в_строку(&self) -> String;
    fn допустима_ли_покупка(
        образец_1: &Self, образец_2: &Self
    ) -> bool;
}
impl Возможноси_частей_речи for [Вид_части_речи] {
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
        образец_1: &Self, образец_2: &Self
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
pub struct Буквы_Слова {
    pub удвоенная_согласная: bool,
    pub удвоенная_гласная: bool,
    pub единственный_образец_в_разделе: bool,
    pub множество_образцов_в_разделе: bool,
    pub обмен_на_двойную_согласную: bool,
    pub обмен_на_двойную_гласную: bool,
}
impl Буквы_Слова {
    pub fn допустимость_согласных_и_гласных(
        образец_1: &Self,
        образец_2: &Self,
    ) -> bool {
        //return true;
        //
        if образец_1.удвоенная_гласная == образец_2.удвоенная_гласная
            && образец_1.удвоенная_согласная == образец_2.удвоенная_согласная
        {
            return true;
        }
        //удвоенная согласная обмен
        if образец_1.удвоенная_согласная == образец_2.обмен_на_двойную_согласную
            || образец_1.обмен_на_двойную_согласную == образец_2.удвоенная_согласная
        {
            return true;
        }
        //удвоенная гласная обмен
        if образец_1.удвоенная_гласная == образец_2.обмен_на_двойную_гласную
            || образец_1.обмен_на_двойную_гласную == образец_2.удвоенная_гласная
        {
            return true;
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
#[derive(Debug, Clone)]
pub struct Вид_Окончания_со_строкой {
    pub строка: &'static str,
    pub re_выдер: Regex,
    pub правописание: Буквы_Слова,
    pub вид_окончания: Вид_Окончания,
    pub часть_речи: Вид_части_речи,
}
// Ручная реализация PartialEq из-за типа Regex
impl PartialEq for Вид_Окончания_со_строкой {
    fn eq(&self, other: &Self) -> bool {
        self.строка == other.строка
            && self.вид_окончания == other.вид_окончания
            && self.часть_речи == other.часть_речи
            // Сравниваем регулярные выражения по их текстовому шаблону
            && self.re_выдер.as_str() == other.re_выдер.as_str()
    }
}
impl Eq for Вид_Окончания_со_строкой {}
impl std::fmt::Display for Вид_части_речи {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
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
            часть_речи: Вид_части_речи::Не_определено,
            правописание: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Раздел_Торжка {
    pub первая_очередь: Vec<Вид_Окончания_со_строкой>,
    pub вторая_очередь: Vec<Вид_Окончания_со_строкой>,
    pub куча_окончаний: rapidhash::fast::RapidHashSet<&'static str>,

    pub окончания_исключения: rapidhash::fast::RapidHashSet<Вид_Окончания>,
}
pub trait Возможности_Торжка {
    fn проверка_на_повтор_окончаний(&self) -> Result<(), ()>;
}
impl Возможности_Торжка for [Раздел_Торжка] {
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
                for слово_кучи_1 in раздел_1.куча_окончаний.iter() {
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
        for ячейка in self.первая_очередь.iter() {
            if ячейка.строка == окончание {
                if ячейка.правописание.множество_образцов_в_разделе
                {
                    return true;
                }
            }
        }
        for ячейка in self.вторая_очередь.iter() {
            if ячейка.строка == окончание {
                if ячейка.правописание.множество_образцов_в_разделе
                {
                    return true;
                }
            }
        }
        false
    }
    pub fn проверка_окончаний_на_недопустимость_одновременно_единственный_и_множественный_образец(
        &self,
    ) -> Result<(), ()> {
        //
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        for ячейка in self.первая_очередь.iter() {
            if ячейка.правописание.единственный_образец_в_разделе
                && ячейка.правописание.множество_образцов_в_разделе
            {
                куча_ошибок.insert(ячейка.строка);
            }
        }
        //
        for ячейка in self.вторая_очередь.iter() {
            if ячейка.правописание.единственный_образец_в_разделе
                && ячейка.правописание.множество_образцов_в_разделе
            {
                куча_ошибок.insert(ячейка.строка);
            }
        }
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
        for ячейка in self.первая_очередь.iter() {
            if ячейка.вид_окончания == *вид_окончания {
                счётчик += 1;
            }
        }
        for ячейка in self.вторая_очередь.iter() {
            if ячейка.вид_окончания == *вид_окончания {
                счётчик += 1;
            }
        }
        //
        /*match счётчик > 1 {
            true => return  счётчик),
            false => return Err(()),
        }*/
        return счётчик;
    }
    //
    pub fn проверка_что_образец_re_соответствует_строке_вторичной_очереди(
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
            // return Err(());
        }
        return Ok(());
    }
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
        for ячейка in self.первая_очередь.iter() {
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
            //return Err(());
        }
        return Ok(());
    }
    pub fn проверка_соответствия_re_выдера_второй_очереди_на_образец(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        static ОБРАЗЕЦ_ПОИСКА_В_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"\(\?i\)\("#).unwrap());
        //
        for ячейка in self.вторая_очередь.iter() {
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
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!(
                    r##"окончание в re_замены [вторая очередь] содержит не верный образец re (?i)( "##
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
    pub fn проверка_соответствия_re_выдера_первой_очереди_на_образец(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        static ОБРАЗЕЦ_ПОИСКА_В_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"\(\?i\)\("#).unwrap());
        //
        for ячейка in self.первая_очередь.iter() {
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
                "\r\n{}{}{}",
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
            // return Err(());
        } else {
            return Ok(());
        }
        return Ok(());
    }
    //
    pub fn проверка_что_окончания_вторичного_ряда_не_содержаться_в_исключениях(
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
            // return Err(());
        }
        return Ok(());
    }
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
        for (указатель_1, ячейка) in self.вторая_очередь.iter().enumerate()
        {
            //сначала с первой очередью
            for (указатель_2, ячейка_2) in self.вторая_очередь.iter().enumerate()
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
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_строка.insert(&ячейка.строка);
                    }
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                    }
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                    }
                }
            }
            //сначала с первой очередью
            for (_, ячейка_2) in self.первая_очередь.iter().enumerate()
            //.filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                if ячейка.строка == ячейка_2.строка {
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_строка.insert(&ячейка.строка);
                    }
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str()
                    {
                        if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                        {
                            куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                        }
                    }
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    //
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                    }
                    //
                }
            }
            //
        }
        //
        for (указатель_1, ячейка) in self.первая_очередь.iter().enumerate()
        {
            //сначала с первой очередью
            for (указатель_2, ячейка_2) in self.первая_очередь.iter().enumerate()
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
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_строка.insert(&ячейка.строка);
                    }
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                    }
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                    }
                }
            }
            //сначала с первой очередью
            for (_, ячейка_2) in self.вторая_очередь.iter().enumerate()
            //.filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                if ячейка.строка == ячейка_2.строка {
                    //
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_строка.insert(&ячейка.строка);
                    }
                    //
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    //
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                    }
                    //
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    //
                    if !ён_ен_ли(&ячейка, &ячейка_2) && !ты_то_ли(&ячейка, &ячейка_2)
                    {
                        куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                    }
                    //
                }
            }
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
            //println!();
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
            //println!();
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
            //println!();
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
        fn ты_то_ли(
            ячейка_1: &Вид_Окончания_со_строкой,
            ячейка_2: &Вид_Окончания_со_строкой,
        ) -> bool {
            if ячейка_1.вид_окончания.ты_то() && ячейка_2.вид_окончания.ты_то()
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
        for ячейка in self.первая_очередь.iter() {
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
                style(format!("|{}|", self.первая_очередь[0].строка)).cyan(),
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
        for ячейка in self.первая_очередь.iter() {
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
        for ячейка in self.вторая_очередь.iter() {
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
        for ячейка in self.первая_очередь.iter() {
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
        for ячейка in self.вторая_очередь.iter() {
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
        'главный_указатель: for раздел in self.первая_очередь.iter()
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
    pub fn проверка_второй_очереди_в_куче_окончаний(
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
    }
    //
    pub fn проверка_кучи_окончаний_в_рядах(&self) -> Result<(), ()> {
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
            //перебор раздела
            for раздел in self.вторая_очередь.iter() {
                //совпадает ли окончание
                if раздел.строка == *образец_из_кучи {
                    continue 'главный_указатель;
                }
            }
            //
            куча_ошибок.insert(образец_из_кучи);
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
