#![allow(non_snake_case, non_camel_case_types)]
//use foldhash::{rapidhash::fast::RapidHashMap,  fast::RandomState,*};
//use rapidhash::*;
use console::style;
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

#[derive(Debug, Clone)]
pub enum Итог_Сохранения {
    Совпало,
    Перезаписано,
    Создано,
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
    pub вывод_окончания_слов: &'static str,
    //
    pub словарь_запасной_чистый: &'static str,
}

impl Default for Пути_Общие {
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
}
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

impl Default for Пути_Вывода {
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
}

#[derive(Debug, Default, Clone)]
pub struct Сообщения_для_книги {
    pub имя_книги: Умная_Строка,
    pub путь_откудаво: Умная_Строка,
    pub расширение: Расширение_полное,
    pub сообщения: Vec<Умная_Строка>,
}
//содержимое
#[derive(Debug, Default, Clone)]
pub struct Сообщения {
    pub общие: Vec<Умная_Строка>,
    pub запись_и_чтение: Vec<Сообщения_для_книги>,
    pub проверка_после_замен: Vec<Сообщения_для_книги>,
    pub чтение_книг: Vec<Умная_Строка>,
    pub кодировка: Vec<Умная_Строка>,
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
    pub файлы: Vec<Умная_Строка>,
    pub ошибки: Vec<Умная_Строка>,
    pub не_вложено: Vec<Умная_Строка>,
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
#[derive(Debug, Clone, PartialEq)]
pub enum Вид_ошибки_при_поиске_в_торжке {
    Исключение,
    Ошибка(String),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Вид_ошибки_при_поиске_выдера_в_торжке {
    Не_найдено_окончание_в_разделе,
    Не_найден_в_куче_окончаний,
    Не_определено,
}
impl std::fmt::Display for Вид_ошибки_при_поиске_выдера_в_торжке {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Вид_ошибки_при_поиске_выдера_в_торжке::Не_найдено_окончание_в_разделе =>
                "не найденоо окончание в разделе Торжка",
            Вид_ошибки_при_поиске_выдера_в_торжке::Не_найден_в_куче_окончаний =>
                "не найдено в куче окончаний Торжка",
            Вид_ошибки_при_поиске_выдера_в_торжке::Не_определено =>
                "нет в Торжке",
        };
        f.write_str(s)
    }
}
//
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Вид_Окончания {
    _Ж,
    _Н,
    _В,
    _Р,
    Ам,
    Ко_ка,
    Ск,
    Ва_Во,
    Ель,
    Ен,
    Ён,
    Ван,
    Он,
    Ан,
    Ат,
    Ни,
    Авш,
    Ут,
    Г_полн,
    Ем,
    Ом,
    Ют,
    Ит,
    Ив,
    Ов,
    Ев,
    Нят,
    Ят,
    _Щ,
    // _Ы,
    Ыт,
    Ым,
    Та_то,
    //Те,
    Ты,
    Чи,
    _Л,
    _К,
    Ъят,
    _Ш,
    _Ц,
    Ст,
    _С,
    Ств,
    Ям,
    Ти,
    _Д,
    Сл,
    Лю_ля,
    Не_определено,
}
pub static ВСЕ_ВИДЫ_ОКОНЧАНИЙ: [Вид_Окончания; 47] = [
    Вид_Окончания::_Ж,
    Вид_Окончания::_Н,
    Вид_Окончания::_В,
    Вид_Окончания::_Р,
    Вид_Окончания::Ам,
    Вид_Окончания::Ко_ка,
    Вид_Окончания::Ск,
    Вид_Окончания::Ва_Во,
    Вид_Окончания::Ель,
    Вид_Окончания::Ен,
    Вид_Окончания::Ён,
    Вид_Окончания::Ван,
    Вид_Окончания::Он,
    Вид_Окончания::Ан,
    Вид_Окончания::Ат,
    Вид_Окончания::Ни,
    Вид_Окончания::Авш,
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
    Вид_Окончания::_К,
    Вид_Окончания::Ъят,
    Вид_Окончания::_Ш,
    Вид_Окончания::_Ц,
    Вид_Окончания::Ст,
    Вид_Окончания::_С,
    Вид_Окончания::Ств,
    Вид_Окончания::Ям,
    Вид_Окончания::Ти,
    Вид_Окончания::_Д,
    Вид_Окончания::Сл,
    Вид_Окончания::Лю_ля,
    //Вид_Окончания::Не_определено,
];
impl Вид_Окончания {
    pub fn получить_вид_окончания_по_строке(
        окончание: &str,
    ) -> Result<Self, String> {
        if КУЧА_ОКОНЧАНИЙ_ВАН.contains(окончание) {
            return Ok(Вид_Окончания::Ван);
        };
        for раздел in РАЗДЕЛЫ_ОКОНЧАНИЙ_ПОЛНЫЕ.iter() {
            if раздел.куча_окончаний.contains(окончание) {
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
}

impl std::fmt::Display for Вид_Окончания {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Вид_Окончания::Ко_ка => "Ко",
            Вид_Окончания::Сл => "Сл",
            Вид_Окончания::Г_полн => "Г_полн",
            Вид_Окончания::Лю_ля => "Лю_ля",
            Вид_Окончания::Ут => "Ут",
            Вид_Окончания::Ат => "Ат",
            Вид_Окончания::Ти => "Ти",
            Вид_Окончания::Чи => "Чи",
            Вид_Окончания::_С => "_С",
            Вид_Окончания::_Ц => "_Ц",
            Вид_Окончания::Ят => "Ят",
            Вид_Окончания::Ям => "Ям",
            Вид_Окончания::Ск => "Ск",
            Вид_Окончания::Ам => "Ам",
            Вид_Окончания::_Ж => "_Ж",
            Вид_Окончания::_К => "_К",
            Вид_Окончания::_Р => "_Р",
            Вид_Окончания::Ни => "Ни",
            Вид_Окончания::Ель => "Ель",
            Вид_Окончания::Ъят => "Ъят",
            Вид_Окончания::Нят => "Нят",
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
            Вид_Окончания::_Д => "_Д",
            Вид_Окончания::_Ш => "_Ш",
            Вид_Окончания::_Щ => "_Щ",
            Вид_Окончания::_Л => "_Л",
            Вид_Окончания::Ит => "Ит",
            Вид_Окончания::Ов => "Ов",
            Вид_Окончания::Ев => "Ев",
            Вид_Окончания::Ив => "Ив",
            Вид_Окончания::_Н => "_Н",
            Вид_Окончания::_В => "_В",
            Вид_Окончания::Ем => "Ем",
            Вид_Окончания::Ом => "Ом",
            Вид_Окончания::Ен => "Ен",
            Вид_Окончания::Ют => "Ют",
            Вид_Окончания::Ён => "Ён",
            Вид_Окончания::Ван => "ВАН",
            Вид_Окончания::Он => "Он",
            Вид_Окончания::Ан => "Ан",
            Вид_Окончания::Авш => "Авш",
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
    pub сообщения: Сообщения,
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
            "одиночные",
            "Одиночные",
            "простые",
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
    pub окончание_строка: &'static str,
    pub вид_окончания: Вид_Окончания,
}
impl Default for RE_полное_окончание {
    fn default() -> Self {
        Self {
            re_поиска: Regex::new(r"").unwrap(),
            re_замены: Regex::new(r"").unwrap(),
            окончание_строка: "",
            вид_окончания: Вид_Окончания::Не_определено, // какой-то дефолтный вариант
        }
    }
}
pub static ПУСТОЕ_RE_ИСКЛЮЧАЮЩЕЕ_ОКОНЧАНИЕ: LazyLock<
    [RE_полное_окончание; 0],
> = LazyLock::new(|| Default::default());
//
// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_Н
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Н: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)н$").unwrap(),
            re_замены: Regex::new(r"(?i)(н)$").unwrap(),
            окончание_строка: "н",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)не$").unwrap(),
            re_замены: Regex::new(r"(?i)(не)$").unwrap(),
            окончание_строка: "не",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)на$").unwrap(),
            re_замены: Regex::new(r"(?i)(на)$").unwrap(),
            окончание_строка: "на",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ная$").unwrap(),
            re_замены: Regex::new(r"(?i)(ная)$").unwrap(),
            окончание_строка: "ная",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ную$").unwrap(),
            re_замены: Regex::new(r"(?i)(ную)$").unwrap(),
            окончание_строка: "ную",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)но$").unwrap(),
            re_замены: Regex::new(r"(?i)(но)$").unwrap(),
            окончание_строка: "но",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ны$").unwrap(),
            re_замены: Regex::new(r"(?i)(ны)$").unwrap(),
            окончание_строка: "ны",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ном$").unwrap(),
            re_замены: Regex::new(r"(?i)(ном)$").unwrap(),
            окончание_строка: "ном",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ному$").unwrap(),
            re_замены: Regex::new(r"(?i)(ному)$").unwrap(),
            окончание_строка: "ному",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ного$").unwrap(),
            re_замены: Regex::new(r"(?i)(ного)$").unwrap(),
            окончание_строка: "ного",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ное$").unwrap(),
            re_замены: Regex::new(r"(?i)(ное)$").unwrap(),
            окончание_строка: "ное",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ной$").unwrap(),
            re_замены: Regex::new(r"(?i)(ной)$").unwrap(),
            окончание_строка: "ной",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ность$").unwrap(),
            re_замены: Regex::new(r"(?i)(ность)$").unwrap(),
            окончание_строка: "ность",
            вид_окончания: Вид_Окончания::_Н,
        },
        // ВНИМАНИЕ: "ного" повторяется — дубликат удалён.
        // Если нужен именно второй экземпляр — верните и увеличьте размер до 25.
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностью)$").unwrap(),
            окончание_строка: "ностью",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ности$").unwrap(),
            re_замены: Regex::new(r"(?i)(ности)$").unwrap(),
            окончание_строка: "ности",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностей)$").unwrap(),
            окончание_строка: "ностей",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностям)$").unwrap(),
            окончание_строка: "ностям",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностями)$").unwrap(),
            окончание_строка: "ностями",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ностях)$").unwrap(),
            окончание_строка: "ностях",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ным$").unwrap(),
            re_замены: Regex::new(r"(?i)(ным)$").unwrap(),
            окончание_строка: "ным",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ными$").unwrap(),
            re_замены: Regex::new(r"(?i)(ными)$").unwrap(),
            окончание_строка: "ными",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ный$").unwrap(),
            re_замены: Regex::new(r"(?i)(ный)$").unwrap(),
            окончание_строка: "ный",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ные$").unwrap(),
            re_замены: Regex::new(r"(?i)(ные)$").unwrap(),
            окончание_строка: "ные",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ных$").unwrap(),
            re_замены: Regex::new(r"(?i)(ных)$").unwrap(),
            окончание_строка: "ных",
            вид_окончания: Вид_Окончания::_Н,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)нее$").unwrap(),
            re_замены: Regex::new(r"(?i)(нее)$").unwrap(),
            окончание_строка: "нее",
            вид_окончания: Вид_Окончания::_Н,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Н: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "н",
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
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_СТ
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СТ: LazyLock<[RE_полное_окончание; 17]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стая$").unwrap(),
            re_замены: Regex::new(r"(?i)(стая)$").unwrap(),
            окончание_строка: "стая",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стее$").unwrap(),
            re_замены: Regex::new(r"(?i)(стее)$").unwrap(),
            окончание_строка: "стее",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стую$").unwrap(),
            re_замены: Regex::new(r"(?i)(стую)$").unwrap(),
            окончание_строка: "стую",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сте$").unwrap(),
            re_замены: Regex::new(r"(?i)(сте)$").unwrap(),
            окончание_строка: "сте",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стее$").unwrap(),
            re_замены: Regex::new(r"(?i)(стее)$").unwrap(),
            окончание_строка: "стее",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сти$").unwrap(),
            re_замены: Regex::new(r"(?i)(сти)$").unwrap(),
            окончание_строка: "сти",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стем$").unwrap(),
            re_замены: Regex::new(r"(?i)(стем)$").unwrap(),
            окончание_строка: "стем",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стему$").unwrap(),
            re_замены: Regex::new(r"(?i)(стему)$").unwrap(),
            окончание_строка: "стему",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стей$").unwrap(),
            re_замены: Regex::new(r"(?i)(стей)$").unwrap(),
            окончание_строка: "стей",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сть$").unwrap(),
            re_замены: Regex::new(r"(?i)(сть)$").unwrap(),
            окончание_строка: "сть",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стью$").unwrap(),
            re_замены: Regex::new(r"(?i)(стью)$").unwrap(),
            окончание_строка: "стью",
            вид_окончания: Вид_Окончания::Ст,
        },
        // Дубликат "сти" удалён
        // Дубликат "стей" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стям$").unwrap(),
            re_замены: Regex::new(r"(?i)(стям)$").unwrap(),
            окончание_строка: "стям",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стями$").unwrap(),
            re_замены: Regex::new(r"(?i)(стями)$").unwrap(),
            окончание_строка: "стями",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стях$").unwrap(),
            re_замены: Regex::new(r"(?i)(стях)$").unwrap(),
            окончание_строка: "стях",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стим$").unwrap(),
            re_замены: Regex::new(r"(?i)(стим)$").unwrap(),
            окончание_строка: "стим",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стими$").unwrap(),
            re_замены: Regex::new(r"(?i)(стими)$").unwrap(),
            окончание_строка: "стими",
            вид_окончания: Вид_Окончания::Ст,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стий$").unwrap(),
            re_замены: Regex::new(r"(?i)(стий)$").unwrap(),
            окончание_строка: "стий",
            вид_окончания: Вид_Окончания::Ст,
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЕН: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ен$").unwrap(),
            re_замены: Regex::new(r"(?i)(ен)$").unwrap(),
            окончание_строка: "ен",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+еннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(еннее)$").unwrap(),
            окончание_строка: "еннее",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ена$").unwrap(),
            re_замены: Regex::new(r"(?i)(ена)$").unwrap(),
            окончание_строка: "ена",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енная$").unwrap(),
            re_замены: Regex::new(r"(?i)(енная)$").unwrap(),
            окончание_строка: "енная",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енную$").unwrap(),
            re_замены: Regex::new(r"(?i)(енную)$").unwrap(),
            окончание_строка: "енную",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ено$").unwrap(),
            re_замены: Regex::new(r"(?i)(ено)$").unwrap(),
            окончание_строка: "ено",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ены$").unwrap(),
            re_замены: Regex::new(r"(?i)(ены)$").unwrap(),
            окончание_строка: "ены",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енном$").unwrap(),
            re_замены: Regex::new(r"(?i)(енном)$").unwrap(),
            окончание_строка: "енном",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енному$").unwrap(),
            re_замены: Regex::new(r"(?i)(енному)$").unwrap(),
            окончание_строка: "енному",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енного$").unwrap(),
            re_замены: Regex::new(r"(?i)(енного)$").unwrap(),
            окончание_строка: "енного",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енно$").unwrap(),
            re_замены: Regex::new(r"(?i)(енно)$").unwrap(),
            окончание_строка: "енно",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енное$").unwrap(),
            re_замены: Regex::new(r"(?i)(енное)$").unwrap(),
            окончание_строка: "енное",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енной$").unwrap(),
            re_замены: Regex::new(r"(?i)(енной)$").unwrap(),
            окончание_строка: "енной",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енность$").unwrap(),
            re_замены: Regex::new(r"(?i)(енность)$").unwrap(),
            окончание_строка: "енность",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностью)$").unwrap(),
            окончание_строка: "енностью",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енности$").unwrap(),
            re_замены: Regex::new(r"(?i)(енности)$").unwrap(),
            окончание_строка: "енности",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностей)$").unwrap(),
            окончание_строка: "енностей",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностям)$").unwrap(),
            окончание_строка: "енностям",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностями)$").unwrap(),
            окончание_строка: "енностями",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(енностях)$").unwrap(),
            окончание_строка: "енностях",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енным$").unwrap(),
            re_замены: Regex::new(r"(?i)(енным)$").unwrap(),
            окончание_строка: "енным",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енными$").unwrap(),
            re_замены: Regex::new(r"(?i)(енными)$").unwrap(),
            окончание_строка: "енными",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енный$").unwrap(),
            re_замены: Regex::new(r"(?i)(енный)$").unwrap(),
            окончание_строка: "енный",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енные$").unwrap(),
            re_замены: Regex::new(r"(?i)(енные)$").unwrap(),
            окончание_строка: "енные",
            вид_окончания: Вид_Окончания::Ен,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+енных$").unwrap(),
            re_замены: Regex::new(r"(?i)(енных)$").unwrap(),
            окончание_строка: "енных",
            вид_окончания: Вид_Окончания::Ен,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЕН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ен",
            "еннее",
            "ена",
            "енная",
            "енную",
            "ено",
            "ены",
            "енном",
            "енному",
            "енного",
            "енно",
            "енное",
            "енной",
            "енность",
            "енностью",
            "енности",
            "енностей",
            "енностям",
            "енностями",
            "енностях",
            "енным",
            "енными",
            "енный",
            "енные",
            "енных",
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ОН
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ОН: LazyLock<[RE_полное_окончание; 26]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онее$").unwrap(),
            re_замены: Regex::new(r"(?i)(онее)$").unwrap(),
            окончание_строка: "онее",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+он$").unwrap(),
            re_замены: Regex::new(r"(?i)(он)$").unwrap(),
            окончание_строка: "он",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+она$").unwrap(),
            re_замены: Regex::new(r"(?i)(она)$").unwrap(),
            окончание_строка: "она",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онная$").unwrap(),
            re_замены: Regex::new(r"(?i)(онная)$").unwrap(),
            окончание_строка: "онная",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онную$").unwrap(),
            re_замены: Regex::new(r"(?i)(онную)$").unwrap(),
            окончание_строка: "онную",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+оно$").unwrap(),
            re_замены: Regex::new(r"(?i)(оно)$").unwrap(),
            окончание_строка: "оно",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+оны$").unwrap(),
            re_замены: Regex::new(r"(?i)(оны)$").unwrap(),
            окончание_строка: "оны",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онном$").unwrap(),
            re_замены: Regex::new(r"(?i)(онном)$").unwrap(),
            окончание_строка: "онном",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онному$").unwrap(),
            re_замены: Regex::new(r"(?i)(онному)$").unwrap(),
            окончание_строка: "онному",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онно$").unwrap(),
            re_замены: Regex::new(r"(?i)(онно)$").unwrap(),
            окончание_строка: "онно",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онного$").unwrap(),
            re_замены: Regex::new(r"(?i)(онного)$").unwrap(),
            окончание_строка: "онного",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онное$").unwrap(),
            re_замены: Regex::new(r"(?i)(онное)$").unwrap(),
            окончание_строка: "онное",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онной$").unwrap(),
            re_замены: Regex::new(r"(?i)(онной)$").unwrap(),
            окончание_строка: "онной",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онность$").unwrap(),
            re_замены: Regex::new(r"(?i)(онность)$").unwrap(),
            окончание_строка: "онность",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностью)$").unwrap(),
            окончание_строка: "онностью",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онности$").unwrap(),
            re_замены: Regex::new(r"(?i)(онности)$").unwrap(),
            окончание_строка: "онности",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностей)$").unwrap(),
            окончание_строка: "онностей",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностям)$").unwrap(),
            окончание_строка: "онностям",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностями)$").unwrap(),
            окончание_строка: "онностями",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(онностях)$").unwrap(),
            окончание_строка: "онностях",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онным$").unwrap(),
            re_замены: Regex::new(r"(?i)(онным)$").unwrap(),
            окончание_строка: "онным",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онными$").unwrap(),
            re_замены: Regex::new(r"(?i)(онными)$").unwrap(),
            окончание_строка: "онными",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онный$").unwrap(),
            re_замены: Regex::new(r"(?i)(онный)$").unwrap(),
            окончание_строка: "онный",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онные$").unwrap(),
            re_замены: Regex::new(r"(?i)(онные)$").unwrap(),
            окончание_строка: "онные",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+онных$").unwrap(),
            re_замены: Regex::new(r"(?i)(онных)$").unwrap(),
            окончание_строка: "онных",
            вид_окончания: Вид_Окончания::Он,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+оннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(оннее)$").unwrap(),
            окончание_строка: "оннее",
            вид_окончания: Вид_Окончания::Он,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ОН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "он",
            "она",
            "онная",
            "онную",
            "оно",
            "онее",
            "оны",
            "онном",
            "онному",
            "онно",
            "онного",
            "онное",
            "онной",
            "онность",
            "онностью",
            "онности",
            "онностей",
            "онностям",
            "онностями",
            "онностях",
            "онным",
            "онными",
            "онный",
            "онные",
            "онных",
            "оннее",
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ЁН
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЁН: LazyLock<[RE_полное_окончание; 22]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ён$").unwrap(),
            re_замены: Regex::new(r"(?i)(ён)$").unwrap(),
            окончание_строка: "ён",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённее)$").unwrap(),
            окончание_строка: "ённее",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённая)$").unwrap(),
            окончание_строка: "ённая",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённую)$").unwrap(),
            окончание_строка: "ённую",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённом)$").unwrap(),
            окончание_строка: "ённом",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённому)$").unwrap(),
            окончание_строка: "ённому",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённого)$").unwrap(),
            окончание_строка: "ённого",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённо$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённо)$").unwrap(),
            окончание_строка: "ённо",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённое)$").unwrap(),
            окончание_строка: "ённое",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённой)$").unwrap(),
            окончание_строка: "ённой",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённость)$").unwrap(),
            окончание_строка: "ённость",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостью)$").unwrap(),
            окончание_строка: "ённостью",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённости)$").unwrap(),
            окончание_строка: "ённости",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостей)$").unwrap(),
            окончание_строка: "ённостей",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостям)$").unwrap(),
            окончание_строка: "ённостям",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостями)$").unwrap(),
            окончание_строка: "ённостями",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённостях)$").unwrap(),
            окончание_строка: "ённостях",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённым)$").unwrap(),
            окончание_строка: "ённым",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ёнными$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёнными)$").unwrap(),
            окончание_строка: "ёнными",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённый)$").unwrap(),
            окончание_строка: "ённый",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённые)$").unwrap(),
            окончание_строка: "ённые",
            вид_окончания: Вид_Окончания::Ён,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ённых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ённых)$").unwrap(),
            окончание_строка: "ённых",
            вид_окончания: Вид_Окончания::Ён,
        },
    ]
});

pub static КУЧА_ОКОНЧАНИЙ_ЁН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ён",
            "ённее",
            "ённая",
            "ённую",
            "ённом",
            "ённому",
            "ённого",
            "ённо",
            "ённое",
            "ённой",
            "ённость",
            "ённостью",
            "ённости",
            "ённостей",
            "ённостям",
            "ённостями",
            "ённостях",
            "ённым",
            "ёнными",
            "ённый",
            "ённые",
            "ённых",
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ИТ
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ИТ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ит$").unwrap(),
            re_замены: Regex::new(r"(?i)(ит)$").unwrap(),
            окончание_строка: "ит",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итее$").unwrap(),
            re_замены: Regex::new(r"(?i)(итее)$").unwrap(),
            окончание_строка: "итее",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ита$").unwrap(),
            re_замены: Regex::new(r"(?i)(ита)$").unwrap(),
            окончание_строка: "ита",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итая$").unwrap(),
            re_замены: Regex::new(r"(?i)(итая)$").unwrap(),
            окончание_строка: "итая",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итую$").unwrap(),
            re_замены: Regex::new(r"(?i)(итую)$").unwrap(),
            окончание_строка: "итую",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ито$").unwrap(),
            re_замены: Regex::new(r"(?i)(ито)$").unwrap(),
            окончание_строка: "ито",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+иты$").unwrap(),
            re_замены: Regex::new(r"(?i)(иты)$").unwrap(),
            окончание_строка: "иты",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итом$").unwrap(),
            re_замены: Regex::new(r"(?i)(итом)$").unwrap(),
            окончание_строка: "итом",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итому$").unwrap(),
            re_замены: Regex::new(r"(?i)(итому)$").unwrap(),
            окончание_строка: "итому",
            вид_окончания: Вид_Окончания::Ит,
        },
        // Дубликат "ито" удалён (уже есть выше)
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итого$").unwrap(),
            re_замены: Regex::new(r"(?i)(итого)$").unwrap(),
            окончание_строка: "итого",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итое$").unwrap(),
            re_замены: Regex::new(r"(?i)(итое)$").unwrap(),
            окончание_строка: "итое",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итой$").unwrap(),
            re_замены: Regex::new(r"(?i)(итой)$").unwrap(),
            окончание_строка: "итой",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итость$").unwrap(),
            re_замены: Regex::new(r"(?i)(итость)$").unwrap(),
            окончание_строка: "итость",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостью)$").unwrap(),
            окончание_строка: "итостью",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итости$").unwrap(),
            re_замены: Regex::new(r"(?i)(итости)$").unwrap(),
            окончание_строка: "итости",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостей)$").unwrap(),
            окончание_строка: "итостей",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостям)$").unwrap(),
            окончание_строка: "итостям",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостями)$").unwrap(),
            окончание_строка: "итостями",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(итостях)$").unwrap(),
            окончание_строка: "итостях",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итым$").unwrap(),
            re_замены: Regex::new(r"(?i)(итым)$").unwrap(),
            окончание_строка: "итым",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(итыми)$").unwrap(),
            окончание_строка: "итыми",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итый$").unwrap(),
            re_замены: Regex::new(r"(?i)(итый)$").unwrap(),
            окончание_строка: "итый",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итые$").unwrap(),
            re_замены: Regex::new(r"(?i)(итые)$").unwrap(),
            окончание_строка: "итые",
            вид_окончания: Вид_Окончания::Ит,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+итых$").unwrap(),
            re_замены: Regex::new(r"(?i)(итых)$").unwrap(),
            окончание_строка: "итых",
            вид_окончания: Вид_Окончания::Ит,
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

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_ЕМ
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЕМ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ем$").unwrap(),
            re_замены: Regex::new(r"(?i)(ем)$").unwrap(),
            окончание_строка: "ем",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емее$").unwrap(),
            re_замены: Regex::new(r"(?i)(емее)$").unwrap(),
            окончание_строка: "емее",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ема$").unwrap(),
            re_замены: Regex::new(r"(?i)(ема)$").unwrap(),
            окончание_строка: "ема",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емая$").unwrap(),
            re_замены: Regex::new(r"(?i)(емая)$").unwrap(),
            окончание_строка: "емая",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емую$").unwrap(),
            re_замены: Regex::new(r"(?i)(емую)$").unwrap(),
            окончание_строка: "емую",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емо$").unwrap(),
            re_замены: Regex::new(r"(?i)(емо)$").unwrap(),
            окончание_строка: "емо",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емы$").unwrap(),
            re_замены: Regex::new(r"(?i)(емы)$").unwrap(),
            окончание_строка: "емы",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емом$").unwrap(),
            re_замены: Regex::new(r"(?i)(емом)$").unwrap(),
            окончание_строка: "емом",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емому$").unwrap(),
            re_замены: Regex::new(r"(?i)(емому)$").unwrap(),
            окончание_строка: "емому",
            вид_окончания: Вид_Окончания::Ем,
        },
        // Дубликат "емо" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емого$").unwrap(),
            re_замены: Regex::new(r"(?i)(емого)$").unwrap(),
            окончание_строка: "емого",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емое$").unwrap(),
            re_замены: Regex::new(r"(?i)(емое)$").unwrap(),
            окончание_строка: "емое",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емой$").unwrap(),
            re_замены: Regex::new(r"(?i)(емой)$").unwrap(),
            окончание_строка: "емой",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емость$").unwrap(),
            re_замены: Regex::new(r"(?i)(емость)$").unwrap(),
            окончание_строка: "емость",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостью)$").unwrap(),
            окончание_строка: "емостью",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емости$").unwrap(),
            re_замены: Regex::new(r"(?i)(емости)$").unwrap(),
            окончание_строка: "емости",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостей)$").unwrap(),
            окончание_строка: "емостей",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостям)$").unwrap(),
            окончание_строка: "емостям",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостями)$").unwrap(),
            окончание_строка: "емостями",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(емостях)$").unwrap(),
            окончание_строка: "емостях",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емым$").unwrap(),
            re_замены: Regex::new(r"(?i)(емым)$").unwrap(),
            окончание_строка: "емым",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(емыми)$").unwrap(),
            окончание_строка: "емыми",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емый$").unwrap(),
            re_замены: Regex::new(r"(?i)(емый)$").unwrap(),
            окончание_строка: "емый",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емые$").unwrap(),
            re_замены: Regex::new(r"(?i)(емые)$").unwrap(),
            окончание_строка: "емые",
            вид_окончания: Вид_Окончания::Ем,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+емых$").unwrap(),
            re_замены: Regex::new(r"(?i)(емых)$").unwrap(),
            окончание_строка: "емых",
            вид_окончания: Вид_Окончания::Ем,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЕМ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ем",
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЫМ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ым)$").unwrap(),
            окончание_строка: "ым",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымее)$").unwrap(),
            окончание_строка: "ымее",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ыма$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыма)$").unwrap(),
            окончание_строка: "ыма",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымая)$").unwrap(),
            окончание_строка: "ымая",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымую)$").unwrap(),
            окончание_строка: "ымую",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымо$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымо)$").unwrap(),
            окончание_строка: "ымо",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымы$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымы)$").unwrap(),
            окончание_строка: "ымы",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымом)$").unwrap(),
            окончание_строка: "ымом",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымому)$").unwrap(),
            окончание_строка: "ымому",
            вид_окончания: Вид_Окончания::Ым,
        },
        // Дубликат "ымо" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымого)$").unwrap(),
            окончание_строка: "ымого",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымое)$").unwrap(),
            окончание_строка: "ымое",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымой)$").unwrap(),
            окончание_строка: "ымой",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымость)$").unwrap(),
            окончание_строка: "ымость",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостью)$").unwrap(),
            окончание_строка: "ымостью",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымости)$").unwrap(),
            окончание_строка: "ымости",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостей)$").unwrap(),
            окончание_строка: "ымостей",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостям)$").unwrap(),
            окончание_строка: "ымостям",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостями)$").unwrap(),
            окончание_строка: "ымостями",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымостях)$").unwrap(),
            окончание_строка: "ымостях",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымым)$").unwrap(),
            окончание_строка: "ымым",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымыми)$").unwrap(),
            окончание_строка: "ымыми",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымый)$").unwrap(),
            окончание_строка: "ымый",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымые)$").unwrap(),
            окончание_строка: "ымые",
            вид_окончания: Вид_Окончания::Ым,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ымых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ымых)$").unwrap(),
            окончание_строка: "ымых",
            вид_окончания: Вид_Окончания::Ым,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЫМ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ым",
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_АМ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ам$").unwrap(),
            re_замены: Regex::new(r"(?i)(ам)$").unwrap(),
            окончание_строка: "ам",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амее$").unwrap(),
            re_замены: Regex::new(r"(?i)(амее)$").unwrap(),
            окончание_строка: "амее",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ама$").unwrap(),
            re_замены: Regex::new(r"(?i)(ама)$").unwrap(),
            окончание_строка: "ама",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амая$").unwrap(),
            re_замены: Regex::new(r"(?i)(амая)$").unwrap(),
            окончание_строка: "амая",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амую$").unwrap(),
            re_замены: Regex::new(r"(?i)(амую)$").unwrap(),
            окончание_строка: "амую",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амо$").unwrap(),
            re_замены: Regex::new(r"(?i)(амо)$").unwrap(),
            окончание_строка: "амо",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амы$").unwrap(),
            re_замены: Regex::new(r"(?i)(амы)$").unwrap(),
            окончание_строка: "амы",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амом$").unwrap(),
            re_замены: Regex::new(r"(?i)(амом)$").unwrap(),
            окончание_строка: "амом",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амому$").unwrap(),
            re_замены: Regex::new(r"(?i)(амому)$").unwrap(),
            окончание_строка: "амому",
            вид_окончания: Вид_Окончания::Ам,
        },
        // Дубликат "амо" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амого$").unwrap(),
            re_замены: Regex::new(r"(?i)(амого)$").unwrap(),
            окончание_строка: "амого",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амое$").unwrap(),
            re_замены: Regex::new(r"(?i)(амое)$").unwrap(),
            окончание_строка: "амое",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амой$").unwrap(),
            re_замены: Regex::new(r"(?i)(амой)$").unwrap(),
            окончание_строка: "амой",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амость$").unwrap(),
            re_замены: Regex::new(r"(?i)(амость)$").unwrap(),
            окончание_строка: "амость",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостью)$").unwrap(),
            окончание_строка: "амостью",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амости$").unwrap(),
            re_замены: Regex::new(r"(?i)(амости)$").unwrap(),
            окончание_строка: "амости",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостей)$").unwrap(),
            окончание_строка: "амостей",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостям)$").unwrap(),
            окончание_строка: "амостям",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостями)$").unwrap(),
            окончание_строка: "амостями",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(амостях)$").unwrap(),
            окончание_строка: "амостях",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амым$").unwrap(),
            re_замены: Regex::new(r"(?i)(амым)$").unwrap(),
            окончание_строка: "амым",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(амыми)$").unwrap(),
            окончание_строка: "амыми",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амый$").unwrap(),
            re_замены: Regex::new(r"(?i)(амый)$").unwrap(),
            окончание_строка: "амый",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амые$").unwrap(),
            re_замены: Regex::new(r"(?i)(амые)$").unwrap(),
            окончание_строка: "амые",
            вид_окончания: Вид_Окончания::Ам,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+амых$").unwrap(),
            re_замены: Regex::new(r"(?i)(амых)$").unwrap(),
            окончание_строка: "амых",
            вид_окончания: Вид_Окончания::Ам,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_АМ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ам",
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
            re_поиска: Regex::new(r"(?i)\w+ом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ом)$").unwrap(),
            окончание_строка: "ом",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омее$").unwrap(),
            re_замены: Regex::new(r"(?i)(омее)$").unwrap(),
            окончание_строка: "омее",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ома$").unwrap(),
            re_замены: Regex::new(r"(?i)(ома)$").unwrap(),
            окончание_строка: "ома",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омая$").unwrap(),
            re_замены: Regex::new(r"(?i)(омая)$").unwrap(),
            окончание_строка: "омая",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омую$").unwrap(),
            re_замены: Regex::new(r"(?i)(омую)$").unwrap(),
            окончание_строка: "омую",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омо$").unwrap(),
            re_замены: Regex::new(r"(?i)(омо)$").unwrap(),
            окончание_строка: "омо",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омы$").unwrap(),
            re_замены: Regex::new(r"(?i)(омы)$").unwrap(),
            окончание_строка: "омы",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омом$").unwrap(),
            re_замены: Regex::new(r"(?i)(омом)$").unwrap(),
            окончание_строка: "омом",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омому$").unwrap(),
            re_замены: Regex::new(r"(?i)(омому)$").unwrap(),
            окончание_строка: "омому",
            вид_окончания: Вид_Окончания::Ом,
        },
        // Дубликат "омо" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омого$").unwrap(),
            re_замены: Regex::new(r"(?i)(омого)$").unwrap(),
            окончание_строка: "омого",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омое$").unwrap(),
            re_замены: Regex::new(r"(?i)(омое)$").unwrap(),
            окончание_строка: "омое",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омой$").unwrap(),
            re_замены: Regex::new(r"(?i)(омой)$").unwrap(),
            окончание_строка: "омой",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омость$").unwrap(),
            re_замены: Regex::new(r"(?i)(омость)$").unwrap(),
            окончание_строка: "омость",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостью)$").unwrap(),
            окончание_строка: "омостью",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омости$").unwrap(),
            re_замены: Regex::new(r"(?i)(омости)$").unwrap(),
            окончание_строка: "омости",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостей)$").unwrap(),
            окончание_строка: "омостей",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостям)$").unwrap(),
            окончание_строка: "омостям",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостями)$").unwrap(),
            окончание_строка: "омостями",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(омостях)$").unwrap(),
            окончание_строка: "омостях",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омым$").unwrap(),
            re_замены: Regex::new(r"(?i)(омым)$").unwrap(),
            окончание_строка: "омым",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(омыми)$").unwrap(),
            окончание_строка: "омыми",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омый$").unwrap(),
            re_замены: Regex::new(r"(?i)(омый)$").unwrap(),
            окончание_строка: "омый",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омые$").unwrap(),
            re_замены: Regex::new(r"(?i)(омые)$").unwrap(),
            окончание_строка: "омые",
            вид_окончания: Вид_Окончания::Ом,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+омых$").unwrap(),
            re_замены: Regex::new(r"(?i)(омых)$").unwrap(),
            окончание_строка: "омых",
            вид_окончания: Вид_Окончания::Ом,
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ВАН: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ван$").unwrap(),
            re_замены: Regex::new(r"(?i)(ван)$").unwrap(),
            окончание_строка: "ван",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ваннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваннее)$").unwrap(),
            окончание_строка: "ваннее",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вана$").unwrap(),
            re_замены: Regex::new(r"(?i)(вана)$").unwrap(),
            окончание_строка: "вана",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванная$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванная)$").unwrap(),
            окончание_строка: "ванная",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванную$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванную)$").unwrap(),
            окончание_строка: "ванную",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+вано$").unwrap(),
            re_замены: Regex::new(r"(?i)(вано)$").unwrap(),
            окончание_строка: "вано",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ваны$").unwrap(),
            re_замены: Regex::new(r"(?i)(ваны)$").unwrap(),
            окончание_строка: "ваны",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванном$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванном)$").unwrap(),
            окончание_строка: "ванном",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванному$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванному)$").unwrap(),
            окончание_строка: "ванному",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванного$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванного)$").unwrap(),
            окончание_строка: "ванного",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванно$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванно)$").unwrap(),
            окончание_строка: "ванно",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванное$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванное)$").unwrap(),
            окончание_строка: "ванное",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванной$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванной)$").unwrap(),
            окончание_строка: "ванной",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванность$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванность)$").unwrap(),
            окончание_строка: "ванность",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностью)$").unwrap(),
            окончание_строка: "ванностью",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванности$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванности)$").unwrap(),
            окончание_строка: "ванности",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностей)$").unwrap(),
            окончание_строка: "ванностей",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностям)$").unwrap(),
            окончание_строка: "ванностям",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностями)$").unwrap(),
            окончание_строка: "ванностями",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванностях)$").unwrap(),
            окончание_строка: "ванностях",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванным$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванным)$").unwrap(),
            окончание_строка: "ванным",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванными$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванными)$").unwrap(),
            окончание_строка: "ванными",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванный$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванный)$").unwrap(),
            окончание_строка: "ванный",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванные$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванные)$").unwrap(),
            окончание_строка: "ванные",
            вид_окончания: Вид_Окончания::Ван,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ванных$").unwrap(),
            re_замены: Regex::new(r"(?i)(ванных)$").unwrap(),
            окончание_строка: "ванных",
            вид_окончания: Вид_Окончания::Ван,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ВАН: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "ван",
        "ваннее",
        "вана",
        "вано",
        "ваны",
        //
        "ванная",
        "ванную",
        //о
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ИВ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ив$").unwrap(),
            re_замены: Regex::new(r"(?i)(ив)$").unwrap(),
            окончание_строка: "ив",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивее)$").unwrap(),
            окончание_строка: "ивее",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ива$").unwrap(),
            re_замены: Regex::new(r"(?i)(ива)$").unwrap(),
            окончание_строка: "ива",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивая)$").unwrap(),
            окончание_строка: "ивая",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивую)$").unwrap(),
            окончание_строка: "ивую",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+иво$").unwrap(),
            re_замены: Regex::new(r"(?i)(иво)$").unwrap(),
            окончание_строка: "иво",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивы$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивы)$").unwrap(),
            окончание_строка: "ивы",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивом)$").unwrap(),
            окончание_строка: "ивом",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивому)$").unwrap(),
            окончание_строка: "ивому",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивого)$").unwrap(),
            окончание_строка: "ивого",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+иво$").unwrap(),
            re_замены: Regex::new(r"(?i)(иво)$").unwrap(),
            окончание_строка: "иво",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивое)$").unwrap(),
            окончание_строка: "ивое",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивой)$").unwrap(),
            окончание_строка: "ивой",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивость)$").unwrap(),
            окончание_строка: "ивость",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостью)$").unwrap(),
            окончание_строка: "ивостью",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивости)$").unwrap(),
            окончание_строка: "ивости",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостей)$").unwrap(),
            окончание_строка: "ивостей",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостям)$").unwrap(),
            окончание_строка: "ивостям",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостями)$").unwrap(),
            окончание_строка: "ивостями",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивостях)$").unwrap(),
            окончание_строка: "ивостях",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивым)$").unwrap(),
            окончание_строка: "ивым",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивыми)$").unwrap(),
            окончание_строка: "ивыми",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивый)$").unwrap(),
            окончание_строка: "ивый",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивые)$").unwrap(),
            окончание_строка: "ивые",
            вид_окончания: Вид_Окончания::Ив,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ивых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ивых)$").unwrap(),
            окончание_строка: "ивых",
            вид_окончания: Вид_Окончания::Ив,
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_АН: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ан$").unwrap(),
            re_замены: Regex::new(r"(?i)(ан)$").unwrap(),
            окончание_строка: "ан",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ана$").unwrap(),
            re_замены: Regex::new(r"(?i)(ана)$").unwrap(),
            окончание_строка: "ана",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анная$").unwrap(),
            re_замены: Regex::new(r"(?i)(анная)$").unwrap(),
            окончание_строка: "анная",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анную$").unwrap(),
            re_замены: Regex::new(r"(?i)(анную)$").unwrap(),
            окончание_строка: "анную",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ано$").unwrap(),
            re_замены: Regex::new(r"(?i)(ано)$").unwrap(),
            окончание_строка: "ано",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+аннее$").unwrap(),
            re_замены: Regex::new(r"(?i)(аннее)$").unwrap(),
            окончание_строка: "аннее",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+аны$").unwrap(),
            re_замены: Regex::new(r"(?i)(аны)$").unwrap(),
            окончание_строка: "аны",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анном$").unwrap(),
            re_замены: Regex::new(r"(?i)(анном)$").unwrap(),
            окончание_строка: "анном",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анному$").unwrap(),
            re_замены: Regex::new(r"(?i)(анному)$").unwrap(),
            окончание_строка: "анному",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анно$").unwrap(),
            re_замены: Regex::new(r"(?i)(анно)$").unwrap(),
            окончание_строка: "анно",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анного$").unwrap(),
            re_замены: Regex::new(r"(?i)(анного)$").unwrap(),
            окончание_строка: "анного",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анное$").unwrap(),
            re_замены: Regex::new(r"(?i)(анное)$").unwrap(),
            окончание_строка: "анное",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анной$").unwrap(),
            re_замены: Regex::new(r"(?i)(анной)$").unwrap(),
            окончание_строка: "анной",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анность$").unwrap(),
            re_замены: Regex::new(r"(?i)(анность)$").unwrap(),
            окончание_строка: "анность",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анностью$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностью)$").unwrap(),
            окончание_строка: "анностью",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анности$").unwrap(),
            re_замены: Regex::new(r"(?i)(анности)$").unwrap(),
            окончание_строка: "анности",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анностей$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностей)$").unwrap(),
            окончание_строка: "анностей",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анностям$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностям)$").unwrap(),
            окончание_строка: "анностям",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анностями$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностями)$").unwrap(),
            окончание_строка: "анностями",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анностях$").unwrap(),
            re_замены: Regex::new(r"(?i)(анностях)$").unwrap(),
            окончание_строка: "анностях",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анным$").unwrap(),
            re_замены: Regex::new(r"(?i)(анным)$").unwrap(),
            окончание_строка: "анным",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анными$").unwrap(),
            re_замены: Regex::new(r"(?i)(анными)$").unwrap(),
            окончание_строка: "анными",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анный$").unwrap(),
            re_замены: Regex::new(r"(?i)(анный)$").unwrap(),
            окончание_строка: "анный",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анные$").unwrap(),
            re_замены: Regex::new(r"(?i)(анные)$").unwrap(),
            окончание_строка: "анные",
            вид_окончания: Вид_Окончания::Ан,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+анных$").unwrap(),
            re_замены: Regex::new(r"(?i)(анных)$").unwrap(),
            окончание_строка: "анных",
            вид_окончания: Вид_Окончания::Ан,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_АН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ан",
            "ана",
            "анная",
            "анную",
            "ано",
            "аннее",
            "аны",
            "анном",
            "анному",
            "анно",
            "анного",
            "анное",
            "анной",
            "анность",
            "анностью",
            "анности",
            "анностей",
            "анностям",
            "анностями",
            "анностях",
            "анным",
            "анными",
            "анный",
            "анные",
            "анных",
        ])
    });

// ============================================================
// РАЗДЕЛ_ОКОНЧАНИЙ_АВШ
// ============================================================
pub static РАЗДЕЛ_ОКОНЧАНИЙ_АВШ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ав$").unwrap(),
            re_замены: Regex::new(r"(?i)(ав)$").unwrap(),
            окончание_строка: "ав",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авша$").unwrap(),
            re_замены: Regex::new(r"(?i)(авша)$").unwrap(),
            окончание_строка: "авша",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшая$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшая)$").unwrap(),
            окончание_строка: "авшая",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшее$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшее)$").unwrap(),
            окончание_строка: "авшее",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшую$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшую)$").unwrap(),
            окончание_строка: "авшую",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авше$").unwrap(),
            re_замены: Regex::new(r"(?i)(авше)$").unwrap(),
            окончание_строка: "авше",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшее$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшее)$").unwrap(),
            окончание_строка: "авшее",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авши$").unwrap(),
            re_замены: Regex::new(r"(?i)(авши)$").unwrap(),
            окончание_строка: "авши",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшем$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшем)$").unwrap(),
            окончание_строка: "авшем",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшему$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшему)$").unwrap(),
            окончание_строка: "авшему",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшего$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшего)$").unwrap(),
            окончание_строка: "авшего",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшее$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшее)$").unwrap(),
            окончание_строка: "авшее",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшей$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшей)$").unwrap(),
            окончание_строка: "авшей",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшесть$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшесть)$").unwrap(),
            окончание_строка: "авшесть",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшестью$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестью)$").unwrap(),
            окончание_строка: "авшестью",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшести$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшести)$").unwrap(),
            окончание_строка: "авшести",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшестей$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестей)$").unwrap(),
            окончание_строка: "авшестей",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшестям$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестям)$").unwrap(),
            окончание_строка: "авшестям",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшестями$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестями)$").unwrap(),
            окончание_строка: "авшестями",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшестях$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшестях)$").unwrap(),
            окончание_строка: "авшестях",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшим$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшим)$").unwrap(),
            окончание_строка: "авшим",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшими$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшими)$").unwrap(),
            окончание_строка: "авшими",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авший$").unwrap(),
            re_замены: Regex::new(r"(?i)(авший)$").unwrap(),
            окончание_строка: "авший",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+авшие$").unwrap(),
            re_замены: Regex::new(r"(?i)(авшие)$").unwrap(),
            окончание_строка: "авшие",
            вид_окончания: Вид_Окончания::Авш,
        },
        RE_полное_окончание {
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
});
//раздел ЫТ
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЫТ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ыт$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыт)$").unwrap(),
            окончание_строка: "ыт",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ыте$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыте)$").unwrap(),
            окончание_строка: "ыте",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытее)$").unwrap(),
            окончание_строка: "ытее",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ыта$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыта)$").unwrap(),
            окончание_строка: "ыта",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытая)$").unwrap(),
            окончание_строка: "ытая",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытую)$").unwrap(),
            окончание_строка: "ытую",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ыто$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыто)$").unwrap(),
            окончание_строка: "ыто",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ыты$").unwrap(),
            re_замены: Regex::new(r"(?i)(ыты)$").unwrap(),
            окончание_строка: "ыты",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытом)$").unwrap(),
            окончание_строка: "ытом",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытому)$").unwrap(),
            окончание_строка: "ытому",
            вид_окончания: Вид_Окончания::Ыт,
        },
        // Дубликат "ыто" удалён (уже есть выше)
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытого)$").unwrap(),
            окончание_строка: "ытого",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытое)$").unwrap(),
            окончание_строка: "ытое",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытой)$").unwrap(),
            окончание_строка: "ытой",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытость)$").unwrap(),
            окончание_строка: "ытость",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостью)$").unwrap(),
            окончание_строка: "ытостью",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытости)$").unwrap(),
            окончание_строка: "ытости",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостей)$").unwrap(),
            окончание_строка: "ытостей",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостям)$").unwrap(),
            окончание_строка: "ытостям",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостями)$").unwrap(),
            окончание_строка: "ытостями",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытостях)$").unwrap(),
            окончание_строка: "ытостях",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытым)$").unwrap(),
            окончание_строка: "ытым",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытыми)$").unwrap(),
            окончание_строка: "ытыми",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытый)$").unwrap(),
            окончание_строка: "ытый",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытые)$").unwrap(),
            окончание_строка: "ытые",
            вид_окончания: Вид_Окончания::Ыт,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ытых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ытых)$").unwrap(),
            окончание_строка: "ытых",
            вид_окончания: Вид_Окончания::Ыт,
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
            re_поиска: Regex::new(r"(?i)\w+тым$").unwrap(),
            re_замены: Regex::new(r"(?i)(тым)$").unwrap(),
            окончание_строка: "тым",
            вид_окончания: Вид_Окончания::Ты,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(тыми)$").unwrap(),
            окончание_строка: "тыми",
            вид_окончания: Вид_Окончания::Ты,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тый$").unwrap(),
            re_замены: Regex::new(r"(?i)(тый)$").unwrap(),
            окончание_строка: "тый",
            вид_окончания: Вид_Окончания::Ты,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тые$").unwrap(),
            re_замены: Regex::new(r"(?i)(тые)$").unwrap(),
            окончание_строка: "тые",
            вид_окончания: Вид_Окончания::Ты,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тых$").unwrap(),
            re_замены: Regex::new(r"(?i)(тых)$").unwrap(),
            окончание_строка: "тых",
            вид_окончания: Вид_Окончания::Ты,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ты$").unwrap(),
            re_замены: Regex::new(r"(?i)(ты)$").unwrap(),
            окончание_строка: "ты",
            вид_окончания: Вид_Окончания::Ты,
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ТИ: LazyLock<[RE_полное_окончание; 9]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тий$").unwrap(),
            re_замены: Regex::new(r"(?i)(тий)$").unwrap(),
            окончание_строка: "тий",
            вид_окончания: Вид_Окончания::Ти,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тие$").unwrap(),
            re_замены: Regex::new(r"(?i)(тие)$").unwrap(),
            окончание_строка: "тие",
            вид_окончания: Вид_Окончания::Ти,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тии$").unwrap(),
            re_замены: Regex::new(r"(?i)(тии)$").unwrap(),
            окончание_строка: "тии",
            вид_окончания: Вид_Окончания::Ти,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тий$").unwrap(),
            re_замены: Regex::new(r"(?i)(тий)$").unwrap(),
            окончание_строка: "тий",
            вид_окончания: Вид_Окончания::Ти,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тие$").unwrap(),
            re_замены: Regex::new(r"(?i)(тие)$").unwrap(),
            окончание_строка: "тие",
            вид_окончания: Вид_Окончания::Ти,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тию$").unwrap(),
            re_замены: Regex::new(r"(?i)(тию)$").unwrap(),
            окончание_строка: "тию",
            вид_окончания: Вид_Окончания::Ти,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тиям$").unwrap(),
            re_замены: Regex::new(r"(?i)(тиям)$").unwrap(),
            окончание_строка: "тиям",
            вид_окончания: Вид_Окончания::Ти,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тиями$").unwrap(),
            re_замены: Regex::new(r"(?i)(тиями)$").unwrap(),
            окончание_строка: "тиями",
            вид_окончания: Вид_Окончания::Ти,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тиях$").unwrap(),
            re_замены: Regex::new(r"(?i)(тиях)$").unwrap(),
            окончание_строка: "тиях",
            вид_окончания: Вид_Окончания::Ти,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ТИ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter(["тий", "тие", "тии", "тию", "тиям", "тиями", "тиях"])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ТА_ТО: LazyLock<[RE_полное_окончание; 17]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+та$").unwrap(),
            re_замены: Regex::new(r"(?i)(та)$").unwrap(),
            окончание_строка: "та",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тая$").unwrap(),
            re_замены: Regex::new(r"(?i)(тая)$").unwrap(),
            окончание_строка: "тая",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тую$").unwrap(),
            re_замены: Regex::new(r"(?i)(тую)$").unwrap(),
            окончание_строка: "тую",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+то$").unwrap(),
            re_замены: Regex::new(r"(?i)(то)$").unwrap(),
            окончание_строка: "то",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+той$").unwrap(),
            re_замены: Regex::new(r"(?i)(той)$").unwrap(),
            окончание_строка: "той",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тее$").unwrap(),
            re_замены: Regex::new(r"(?i)(тее)$").unwrap(),
            окончание_строка: "тее",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тое$").unwrap(),
            re_замены: Regex::new(r"(?i)(тое)$").unwrap(),
            окончание_строка: "тое",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тому$").unwrap(),
            re_замены: Regex::new(r"(?i)(тому)$").unwrap(),
            окончание_строка: "тому",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+том$").unwrap(),
            re_замены: Regex::new(r"(?i)(том)$").unwrap(),
            окончание_строка: "том",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+того$").unwrap(),
            re_замены: Regex::new(r"(?i)(того)$").unwrap(),
            окончание_строка: "того",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тость$").unwrap(),
            re_замены: Regex::new(r"(?i)(тость)$").unwrap(),
            окончание_строка: "тость",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостью)$").unwrap(),
            окончание_строка: "тостью",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тости$").unwrap(),
            re_замены: Regex::new(r"(?i)(тости)$").unwrap(),
            окончание_строка: "тости",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостей)$").unwrap(),
            окончание_строка: "тостей",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостям)$").unwrap(),
            окончание_строка: "тостям",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостями)$").unwrap(),
            окончание_строка: "тостями",
            вид_окончания: Вид_Окончания::Та_то,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+тостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(тостях)$").unwrap(),
            окончание_строка: "тостях",
            вид_окончания: Вид_Окончания::Та_то,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ТА_ТО: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
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
    ])
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ВА_ВО: LazyLock<[RE_полное_окончание; 15]> = LazyLock::new(|| {
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
});
//
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СТВ: LazyLock<[RE_полное_окончание; 9]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ства$").unwrap(),
            re_замены: Regex::new(r"(?i)(ства)$").unwrap(),
            окончание_строка: "ства",
            вид_окончания: Вид_Окончания::Ств,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ству$").unwrap(),
            re_замены: Regex::new(r"(?i)(ству)$").unwrap(),
            окончание_строка: "ству",
            вид_окончания: Вид_Окончания::Ств,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+стве$").unwrap(),
            re_замены: Regex::new(r"(?i)(стве)$").unwrap(),
            окончание_строка: "стве",
            вид_окончания: Вид_Окончания::Ств,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ств$").unwrap(),
            re_замены: Regex::new(r"(?i)(ств)$").unwrap(),
            окончание_строка: "ств",
            вид_окончания: Вид_Окончания::Ств,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ствам$").unwrap(),
            re_замены: Regex::new(r"(?i)(ствам)$").unwrap(),
            окончание_строка: "ствам",
            вид_окончания: Вид_Окончания::Ств,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ствами$").unwrap(),
            re_замены: Regex::new(r"(?i)(ствами)$").unwrap(),
            окончание_строка: "ствами",
            вид_окончания: Вид_Окончания::Ств,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ство$").unwrap(),
            re_замены: Regex::new(r"(?i)(ство)$").unwrap(),
            окончание_строка: "ство",
            вид_окончания: Вид_Окончания::Ств,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ством$").unwrap(),
            re_замены: Regex::new(r"(?i)(ством)$").unwrap(),
            окончание_строка: "ством",
            вид_окончания: Вид_Окончания::Ств,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ствах$").unwrap(),
            re_замены: Regex::new(r"(?i)(ствах)$").unwrap(),
            окончание_строка: "ствах",
            вид_окончания: Вид_Окончания::Ств,
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
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_НЯТ: LazyLock<[RE_полное_окончание; 23]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нят$").unwrap(),
            re_замены: Regex::new(r"(?i)(нят)$").unwrap(),
            окончание_строка: "нят",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нята$").unwrap(),
            re_замены: Regex::new(r"(?i)(нята)$").unwrap(),
            окончание_строка: "нята",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятая$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятая)$").unwrap(),
            окончание_строка: "нятая",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятую$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятую)$").unwrap(),
            окончание_строка: "нятую",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятое$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятое)$").unwrap(),
            окончание_строка: "нятое",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+няты$").unwrap(),
            re_замены: Regex::new(r"(?i)(няты)$").unwrap(),
            окончание_строка: "няты",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятом$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятом)$").unwrap(),
            окончание_строка: "нятом",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятому$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятому)$").unwrap(),
            окончание_строка: "нятому",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятого$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятого)$").unwrap(),
            окончание_строка: "нятого",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нято$").unwrap(),
            re_замены: Regex::new(r"(?i)(нято)$").unwrap(),
            окончание_строка: "нято",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятой$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятой)$").unwrap(),
            окончание_строка: "нятой",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятость$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятость)$").unwrap(),
            окончание_строка: "нятость",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостью)$").unwrap(),
            окончание_строка: "нятостью",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятости$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятости)$").unwrap(),
            окончание_строка: "нятости",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостей)$").unwrap(),
            окончание_строка: "нятостей",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостям)$").unwrap(),
            окончание_строка: "нятостям",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостями)$").unwrap(),
            окончание_строка: "нятостями",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятостях)$").unwrap(),
            окончание_строка: "нятостях",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятым$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятым)$").unwrap(),
            окончание_строка: "нятым",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятыми)$").unwrap(),
            окончание_строка: "нятыми",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятый$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятый)$").unwrap(),
            окончание_строка: "нятый",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятые$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятые)$").unwrap(),
            окончание_строка: "нятые",
            вид_окончания: Вид_Окончания::Нят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нятых$").unwrap(),
            re_замены: Regex::new(r"(?i)(нятых)$").unwrap(),
            окончание_строка: "нятых",
            вид_окончания: Вид_Окончания::Нят,
        },
    ]
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_УТ: LazyLock<[RE_полное_окончание; 23]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ут$").unwrap(),
            re_замены: Regex::new(r"(?i)(ут)$").unwrap(),
            окончание_строка: "ут",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ута$").unwrap(),
            re_замены: Regex::new(r"(?i)(ута)$").unwrap(),
            окончание_строка: "ута",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утая$").unwrap(),
            re_замены: Regex::new(r"(?i)(утая)$").unwrap(),
            окончание_строка: "утая",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утую$").unwrap(),
            re_замены: Regex::new(r"(?i)(утую)$").unwrap(),
            окончание_строка: "утую",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утое$").unwrap(),
            re_замены: Regex::new(r"(?i)(утое)$").unwrap(),
            окончание_строка: "утое",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+уты$").unwrap(),
            re_замены: Regex::new(r"(?i)(уты)$").unwrap(),
            окончание_строка: "уты",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утом$").unwrap(),
            re_замены: Regex::new(r"(?i)(утом)$").unwrap(),
            окончание_строка: "утом",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утому$").unwrap(),
            re_замены: Regex::new(r"(?i)(утому)$").unwrap(),
            окончание_строка: "утому",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утого$").unwrap(),
            re_замены: Regex::new(r"(?i)(утого)$").unwrap(),
            окончание_строка: "утого",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+уто$").unwrap(),
            re_замены: Regex::new(r"(?i)(уто)$").unwrap(),
            окончание_строка: "уто",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утой$").unwrap(),
            re_замены: Regex::new(r"(?i)(утой)$").unwrap(),
            окончание_строка: "утой",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утость$").unwrap(),
            re_замены: Regex::new(r"(?i)(утость)$").unwrap(),
            окончание_строка: "утость",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостью)$").unwrap(),
            окончание_строка: "утостью",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утости$").unwrap(),
            re_замены: Regex::new(r"(?i)(утости)$").unwrap(),
            окончание_строка: "утости",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостей)$").unwrap(),
            окончание_строка: "утостей",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостям)$").unwrap(),
            окончание_строка: "утостям",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостями)$").unwrap(),
            окончание_строка: "утостями",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(утостях)$").unwrap(),
            окончание_строка: "утостях",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утым$").unwrap(),
            re_замены: Regex::new(r"(?i)(утым)$").unwrap(),
            окончание_строка: "утым",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(утыми)$").unwrap(),
            окончание_строка: "утыми",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утый$").unwrap(),
            re_замены: Regex::new(r"(?i)(утый)$").unwrap(),
            окончание_строка: "утый",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утые$").unwrap(),
            re_замены: Regex::new(r"(?i)(утые)$").unwrap(),
            окончание_строка: "утые",
            вид_окончания: Вид_Окончания::Ут,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+утых$").unwrap(),
            re_замены: Regex::new(r"(?i)(утых)$").unwrap(),
            окончание_строка: "утых",
            вид_окончания: Вид_Окончания::Ут,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_УТ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ут",
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЮТ: LazyLock<[RE_полное_окончание; 23]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ют$").unwrap(),
            re_замены: Regex::new(r"(?i)(ют)$").unwrap(),
            окончание_строка: "ют",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+юта$").unwrap(),
            re_замены: Regex::new(r"(?i)(юта)$").unwrap(),
            окончание_строка: "юта",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютая)$").unwrap(),
            окончание_строка: "ютая",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютую)$").unwrap(),
            окончание_строка: "ютую",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютое)$").unwrap(),
            окончание_строка: "ютое",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+юты$").unwrap(),
            re_замены: Regex::new(r"(?i)(юты)$").unwrap(),
            окончание_строка: "юты",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютом)$").unwrap(),
            окончание_строка: "ютом",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютому)$").unwrap(),
            окончание_строка: "ютому",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютого)$").unwrap(),
            окончание_строка: "ютого",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+юто$").unwrap(),
            re_замены: Regex::new(r"(?i)(юто)$").unwrap(),
            окончание_строка: "юто",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютой)$").unwrap(),
            окончание_строка: "ютой",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютость)$").unwrap(),
            окончание_строка: "ютость",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостью)$").unwrap(),
            окончание_строка: "ютостью",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютости)$").unwrap(),
            окончание_строка: "ютости",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостей)$").unwrap(),
            окончание_строка: "ютостей",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостям)$").unwrap(),
            окончание_строка: "ютостям",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостями)$").unwrap(),
            окончание_строка: "ютостями",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютостях)$").unwrap(),
            окончание_строка: "ютостях",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютым)$").unwrap(),
            окончание_строка: "ютым",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютыми)$").unwrap(),
            окончание_строка: "ютыми",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютый)$").unwrap(),
            окончание_строка: "ютый",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютые)$").unwrap(),
            окончание_строка: "ютые",
            вид_окончания: Вид_Окончания::Ют,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ютых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ютых)$").unwrap(),
            окончание_строка: "ютых",
            вид_окончания: Вид_Окончания::Ют,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЮТ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ют",
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
pub static КУЧА_ОКОНЧАНИЙ_НЯТ: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "нят",
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
            re_поиска: Regex::new(r"(?i)\w+ъят$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъят)$").unwrap(),
            окончание_строка: "ъят",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъята$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъята)$").unwrap(),
            окончание_строка: "ъята",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятая)$").unwrap(),
            окончание_строка: "ъятая",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятую)$").unwrap(),
            окончание_строка: "ъятую",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятое)$").unwrap(),
            окончание_строка: "ъятое",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъяты$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъяты)$").unwrap(),
            окончание_строка: "ъяты",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятом)$").unwrap(),
            окончание_строка: "ъятом",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятому)$").unwrap(),
            окончание_строка: "ъятому",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятого)$").unwrap(),
            окончание_строка: "ъятого",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъято$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъято)$").unwrap(),
            окончание_строка: "ъято",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятой)$").unwrap(),
            окончание_строка: "ъятой",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятость)$").unwrap(),
            окончание_строка: "ъятость",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостью)$").unwrap(),
            окончание_строка: "ъятостью",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятости)$").unwrap(),
            окончание_строка: "ъятости",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостей)$").unwrap(),
            окончание_строка: "ъятостей",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостям)$").unwrap(),
            окончание_строка: "ъятостям",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостями)$").unwrap(),
            окончание_строка: "ъятостями",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятостях)$").unwrap(),
            окончание_строка: "ъятостях",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятым)$").unwrap(),
            окончание_строка: "ъятым",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятыми)$").unwrap(),
            окончание_строка: "ъятыми",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятый)$").unwrap(),
            окончание_строка: "ъятый",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятые)$").unwrap(),
            окончание_строка: "ъятые",
            вид_окончания: Вид_Окончания::Ъят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ъятых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ъятых)$").unwrap(),
            окончание_строка: "ъятых",
            вид_окончания: Вид_Окончания::Ъят,
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Щ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ща$").unwrap(),
            re_замены: Regex::new(r"(?i)(ща)$").unwrap(),
            окончание_строка: "ща",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ще$").unwrap(),
            re_замены: Regex::new(r"(?i)(ще)$").unwrap(),
            окончание_строка: "ще",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щая$").unwrap(),
            re_замены: Regex::new(r"(?i)(щая)$").unwrap(),
            окончание_строка: "щая",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щую$").unwrap(),
            re_замены: Regex::new(r"(?i)(щую)$").unwrap(),
            окончание_строка: "щую",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щее$").unwrap(),
            re_замены: Regex::new(r"(?i)(щее)$").unwrap(),
            окончание_строка: "щее",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щи$").unwrap(),
            re_замены: Regex::new(r"(?i)(щи)$").unwrap(),
            окончание_строка: "щи",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щем$").unwrap(),
            re_замены: Regex::new(r"(?i)(щем)$").unwrap(),
            окончание_строка: "щем",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щему$").unwrap(),
            re_замены: Regex::new(r"(?i)(щему)$").unwrap(),
            окончание_строка: "щему",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щего$").unwrap(),
            re_замены: Regex::new(r"(?i)(щего)$").unwrap(),
            окончание_строка: "щего",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ще$").unwrap(),
            re_замены: Regex::new(r"(?i)(ще)$").unwrap(),
            окончание_строка: "ще",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щей$").unwrap(),
            re_замены: Regex::new(r"(?i)(щей)$").unwrap(),
            окончание_строка: "щей",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щесть$").unwrap(),
            re_замены: Regex::new(r"(?i)(щесть)$").unwrap(),
            окончание_строка: "щесть",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щестью$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестью)$").unwrap(),
            окончание_строка: "щестью",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щести$").unwrap(),
            re_замены: Regex::new(r"(?i)(щести)$").unwrap(),
            окончание_строка: "щести",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щестей$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестей)$").unwrap(),
            окончание_строка: "щестей",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щестям$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестям)$").unwrap(),
            окончание_строка: "щестям",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щестями$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестями)$").unwrap(),
            окончание_строка: "щестями",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щестях$").unwrap(),
            re_замены: Regex::new(r"(?i)(щестях)$").unwrap(),
            окончание_строка: "щестях",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щи$").unwrap(),
            re_замены: Regex::new(r"(?i)(щи)$").unwrap(),
            окончание_строка: "щи",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щим$").unwrap(),
            re_замены: Regex::new(r"(?i)(щим)$").unwrap(),
            окончание_строка: "щим",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щими$").unwrap(),
            re_замены: Regex::new(r"(?i)(щими)$").unwrap(),
            окончание_строка: "щими",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щий$").unwrap(),
            re_замены: Regex::new(r"(?i)(щий)$").unwrap(),
            окончание_строка: "щий",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щие$").unwrap(),
            re_замены: Regex::new(r"(?i)(щие)$").unwrap(),
            окончание_строка: "щие",
            вид_окончания: Вид_Окончания::_Щ,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+щих$").unwrap(),
            re_замены: Regex::new(r"(?i)(щих)$").unwrap(),
            окончание_строка: "щих",
            вид_окончания: Вид_Окончания::_Щ,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Щ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "ща",
            "ще",
            "щая",
            "щую",
            "щее",
            "щи",
            "щем",
            "щему",
            "щего",
            // "ще" уже было выше — дубликат убран
            "щей",
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
            "щий",
            "щие",
            "щих",
        ])
    });

//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЕЛЬ: LazyLock<[RE_полное_окончание; 7]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ель$").unwrap(),
            re_замены: Regex::new(r"(?i)(ель)$").unwrap(),
            окончание_строка: "ель",
            вид_окончания: Вид_Окончания::Ель,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ели$").unwrap(),
            re_замены: Regex::new(r"(?i)(ели)$").unwrap(),
            окончание_строка: "ели",
            вид_окончания: Вид_Окончания::Ель,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+елью$").unwrap(),
            re_замены: Regex::new(r"(?i)(елью)$").unwrap(),
            окончание_строка: "елью",
            вид_окончания: Вид_Окончания::Ель,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+елям$").unwrap(),
            re_замены: Regex::new(r"(?i)(елям)$").unwrap(),
            окончание_строка: "елям",
            вид_окончания: Вид_Окончания::Ель,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+елей$").unwrap(),
            re_замены: Regex::new(r"(?i)(елей)$").unwrap(),
            окончание_строка: "елей",
            вид_окончания: Вид_Окончания::Ель,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+елями$").unwrap(),
            re_замены: Regex::new(r"(?i)(елями)$").unwrap(),
            окончание_строка: "елями",
            вид_окончания: Вид_Окончания::Ель,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+елях$").unwrap(),
            re_замены: Regex::new(r"(?i)(елях)$").unwrap(),
            окончание_строка: "елях",
            вид_окончания: Вид_Окончания::Ель,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЕЛЬ: LazyLock<
    rapidhash::fast::RapidHashSet<&'static str>,
> = LazyLock::new(|| {
    rapidhash::fast::RapidHashSet::from_iter([
        "ель",
        "елей",
        "ели",
        "елью",
        "елям",
        "елями",
        "елях",
    ])
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Ш: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ша$").unwrap(),
            re_замены: Regex::new(r"(?i)(ша)$").unwrap(),
            окончание_строка: "ша",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ше$").unwrap(),
            re_замены: Regex::new(r"(?i)(ше)$").unwrap(),
            окончание_строка: "ше",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шая$").unwrap(),
            re_замены: Regex::new(r"(?i)(шая)$").unwrap(),
            окончание_строка: "шая",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шую$").unwrap(),
            re_замены: Regex::new(r"(?i)(шую)$").unwrap(),
            окончание_строка: "шую",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шее$").unwrap(),
            re_замены: Regex::new(r"(?i)(шее)$").unwrap(),
            окончание_строка: "шее",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ши$").unwrap(),
            re_замены: Regex::new(r"(?i)(ши)$").unwrap(),
            окончание_строка: "ши",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шем$").unwrap(),
            re_замены: Regex::new(r"(?i)(шем)$").unwrap(),
            окончание_строка: "шем",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шему$").unwrap(),
            re_замены: Regex::new(r"(?i)(шему)$").unwrap(),
            окончание_строка: "шему",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шего$").unwrap(),
            re_замены: Regex::new(r"(?i)(шего)$").unwrap(),
            окончание_строка: "шего",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ше$").unwrap(),
            re_замены: Regex::new(r"(?i)(ше)$").unwrap(),
            окончание_строка: "ше",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шей$").unwrap(),
            re_замены: Regex::new(r"(?i)(шей)$").unwrap(),
            окончание_строка: "шей",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шесть$").unwrap(),
            re_замены: Regex::new(r"(?i)(шесть)$").unwrap(),
            окончание_строка: "шесть",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шестью$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестью)$").unwrap(),
            окончание_строка: "шестью",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шести$").unwrap(),
            re_замены: Regex::new(r"(?i)(шести)$").unwrap(),
            окончание_строка: "шести",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шестей$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестей)$").unwrap(),
            окончание_строка: "шестей",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шестям$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестям)$").unwrap(),
            окончание_строка: "шестям",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шестями$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестями)$").unwrap(),
            окончание_строка: "шестями",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шестях$").unwrap(),
            re_замены: Regex::new(r"(?i)(шестях)$").unwrap(),
            окончание_строка: "шестях",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ши$").unwrap(),
            re_замены: Regex::new(r"(?i)(ши)$").unwrap(),
            окончание_строка: "ши",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шим$").unwrap(),
            re_замены: Regex::new(r"(?i)(шим)$").unwrap(),
            окончание_строка: "шим",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шими$").unwrap(),
            re_замены: Regex::new(r"(?i)(шими)$").unwrap(),
            окончание_строка: "шими",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ший$").unwrap(),
            re_замены: Regex::new(r"(?i)(ший)$").unwrap(),
            окончание_строка: "ший",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+шие$").unwrap(),
            re_замены: Regex::new(r"(?i)(шие)$").unwrap(),
            окончание_строка: "шие",
            вид_окончания: Вид_Окончания::_Ш,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ших$").unwrap(),
            re_замены: Regex::new(r"(?i)(ших)$").unwrap(),
            окончание_строка: "ших",
            вид_окончания: Вид_Окончания::_Ш,
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_НИ: LazyLock<[RE_полное_окончание; 9]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ния$").unwrap(),
            re_замены: Regex::new(r"(?i)(ния)$").unwrap(),
            окончание_строка: "ния",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ниям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ниям$)$").unwrap(),
            окончание_строка: "ниям",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ниями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ниями$)$").unwrap(),
            окончание_строка: "ниями",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ниях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ниях$)$").unwrap(),
            окончание_строка: "ниях",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нию$").unwrap(),
            re_замены: Regex::new(r"(?i)(нию$)$").unwrap(),
            окончание_строка: "нию",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нии$").unwrap(),
            re_замены: Regex::new(r"(?i)(нии)$").unwrap(),
            окончание_строка: "нии",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ний$").unwrap(),
            re_замены: Regex::new(r"(?i)(ний)$").unwrap(),
            окончание_строка: "ний",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ние$").unwrap(),
            re_замены: Regex::new(r"(?i)(ние)$").unwrap(),
            окончание_строка: "ние",
            вид_окончания: Вид_Окончания::Ни,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+нием$").unwrap(),
            re_замены: Regex::new(r"(?i)(нием)$").unwrap(),
            окончание_строка: "нием",
            вид_окончания: Вид_Окончания::Ни,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_НИ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ния",
        "ниям",
        "ниями",
        "ниях",
        "нию",
        "нии",
        "ний",
        "ние",
        "нием",
    ])
});
//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Р: LazyLock<[RE_полное_окончание; 10]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)р$").unwrap(),
            re_замены: Regex::new(r"(?i)(р)$").unwrap(),
            окончание_строка: "р",
            вид_окончания: Вид_Окончания::_Р,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ре$").unwrap(),
            re_замены: Regex::new(r"(?i)(ре)$").unwrap(),
            окончание_строка: "ре",
            вид_окончания: Вид_Окончания::_Р,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)рах$").unwrap(),
            re_замены: Regex::new(r"(?i)(рах)$").unwrap(),
            окончание_строка: "рах",
            вид_окончания: Вид_Окончания::_Р,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ра$").unwrap(),
            re_замены: Regex::new(r"(?i)(ра)$").unwrap(),
            окончание_строка: "ра",
            вид_окончания: Вид_Окончания::_Р,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)рами$").unwrap(),
            re_замены: Regex::new(r"(?i)(рами)$").unwrap(),
            окончание_строка: "рами",
            вид_окончания: Вид_Окончания::_Р,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)рам$").unwrap(),
            re_замены: Regex::new(r"(?i)(рам)$").unwrap(),
            окончание_строка: "рам",
            вид_окончания: Вид_Окончания::_Р,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ру$").unwrap(),
            re_замены: Regex::new(r"(?i)(ру)$").unwrap(),
            окончание_строка: "ру",
            вид_окончания: Вид_Окончания::_Р,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ры$").unwrap(),
            re_замены: Regex::new(r"(?i)(ры)$").unwrap(),
            окончание_строка: "ры",
            вид_окончания: Вид_Окончания::_Р,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ром$").unwrap(),
            re_замены: Regex::new(r"(?i)(ром)$").unwrap(),
            окончание_строка: "ром",
            вид_окончания: Вид_Окончания::_Р,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)ров$").unwrap(),
            re_замены: Regex::new(r"(?i)(ров)$").unwrap(),
            окончание_строка: "ров",
            вид_окончания: Вид_Окончания::_Р,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Р: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "р", "ре", "рах", "ра", "рами", "рам", "ру", "ры", "ром", "ров",
    ])
});

pub static РАЗДЕЛ_ОКОНЧАНИЙ_В: LazyLock<[RE_полное_окончание; 9]> = LazyLock::new(|| {
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

//
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СК: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ск$").unwrap(),
            re_замены: Regex::new(r"(?i)(ск)$").unwrap(),
            окончание_строка: "ск",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скее$").unwrap(),
            re_замены: Regex::new(r"(?i)(скее)$").unwrap(),
            окончание_строка: "скее",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ска$").unwrap(),
            re_замены: Regex::new(r"(?i)(ска)$").unwrap(),
            окончание_строка: "ска",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ская$").unwrap(),
            re_замены: Regex::new(r"(?i)(ская)$").unwrap(),
            окончание_строка: "ская",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скую$").unwrap(),
            re_замены: Regex::new(r"(?i)(скую)$").unwrap(),
            окончание_строка: "скую",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ско$").unwrap(),
            re_замены: Regex::new(r"(?i)(ско)$").unwrap(),
            окончание_строка: "ско",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ски$").unwrap(),
            re_замены: Regex::new(r"(?i)(ски)$").unwrap(),
            окончание_строка: "ски",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ском$").unwrap(),
            re_замены: Regex::new(r"(?i)(ском)$").unwrap(),
            окончание_строка: "ском",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скому$").unwrap(),
            re_замены: Regex::new(r"(?i)(скому)$").unwrap(),
            окончание_строка: "скому",
            вид_окончания: Вид_Окончания::Ск,
        },
        // Дубликат "ско" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ского$").unwrap(),
            re_замены: Regex::new(r"(?i)(ского)$").unwrap(),
            окончание_строка: "ского",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ское$").unwrap(),
            re_замены: Regex::new(r"(?i)(ское)$").unwrap(),
            окончание_строка: "ское",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ской$").unwrap(),
            re_замены: Regex::new(r"(?i)(ской)$").unwrap(),
            окончание_строка: "ской",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скость$").unwrap(),
            re_замены: Regex::new(r"(?i)(скость)$").unwrap(),
            окончание_строка: "скость",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостью)$").unwrap(),
            окончание_строка: "скостью",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скости$").unwrap(),
            re_замены: Regex::new(r"(?i)(скости)$").unwrap(),
            окончание_строка: "скости",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостей)$").unwrap(),
            окончание_строка: "скостей",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостям)$").unwrap(),
            окончание_строка: "скостям",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостями)$").unwrap(),
            окончание_строка: "скостями",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(скостях)$").unwrap(),
            окончание_строка: "скостях",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ским$").unwrap(),
            re_замены: Regex::new(r"(?i)(ским)$").unwrap(),
            окончание_строка: "ским",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+скими$").unwrap(),
            re_замены: Regex::new(r"(?i)(скими)$").unwrap(),
            окончание_строка: "скими",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ский$").unwrap(),
            re_замены: Regex::new(r"(?i)(ский)$").unwrap(),
            окончание_строка: "ский",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ские$").unwrap(),
            re_замены: Regex::new(r"(?i)(ские)$").unwrap(),
            окончание_строка: "ские",
            вид_окончания: Вид_Окончания::Ск,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ских$").unwrap(),
            re_замены: Regex::new(r"(?i)(ских)$").unwrap(),
            окончание_строка: "ских",
            вид_окончания: Вид_Окончания::Ск,
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
            re_поиска: Regex::new(r"(?i)\w+ям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ям)$").unwrap(),
            окончание_строка: "ям",
            вид_окончания: Вид_Окончания::Ям,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ями)$").unwrap(),
            окончание_строка: "ями",
            вид_окончания: Вид_Окончания::Ям,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ях)$").unwrap(),
            окончание_строка: "ях",
            вид_окончания: Вид_Окончания::Ям,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЯМ: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter(["ям", "ями", "ях"]));
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Ц: LazyLock<[RE_полное_окончание; 11]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ец$").unwrap(),
            re_замены: Regex::new(r"(?i)(ец)$").unwrap(),
            окончание_строка: "ец",
            вид_окончания: Вид_Окончания::_Ц,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ца$").unwrap(),
            re_замены: Regex::new(r"(?i)(ца)$").unwrap(),
            окончание_строка: "ца",
            вид_окончания: Вид_Окончания::_Ц,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+цу$").unwrap(),
            re_замены: Regex::new(r"(?i)(цу)$").unwrap(),
            окончание_строка: "цу",
            вид_окончания: Вид_Окончания::_Ц,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+це$").unwrap(),
            re_замены: Regex::new(r"(?i)(це)$").unwrap(),
            окончание_строка: "це",
            вид_окончания: Вид_Окончания::_Ц,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+цом$").unwrap(),
            re_замены: Regex::new(r"(?i)(цом)$").unwrap(),
            окончание_строка: "цом",
            вид_окончания: Вид_Окончания::_Ц,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+цам$").unwrap(),
            re_замены: Regex::new(r"(?i)(цам)$").unwrap(),
            окончание_строка: "цам",
            вид_окончания: Вид_Окончания::_Ц,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+цами$").unwrap(),
            re_замены: Regex::new(r"(?i)(цами)$").unwrap(),
            окончание_строка: "цами",
            вид_окончания: Вид_Окончания::_Ц,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+цах$").unwrap(),
            re_замены: Regex::new(r"(?i)(цах)$").unwrap(),
            окончание_строка: "цах",
            вид_окончания: Вид_Окончания::_Ц,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+цы$").unwrap(),
            re_замены: Regex::new(r"(?i)(цы)$").unwrap(),
            окончание_строка: "цы",
            вид_окончания: Вид_Окончания::_Ц,
        },
        // Дубликат "цо" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+цов$").unwrap(),
            re_замены: Regex::new(r"(?i)(цов)$").unwrap(),
            окончание_строка: "цов",
            вид_окончания: Вид_Окончания::_Ц,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+цах$").unwrap(),
            re_замены: Regex::new(r"(?i)(цах)$").unwrap(),
            окончание_строка: "цах",
            вид_окончания: Вид_Окончания::_Ц,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Ц: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ец", "ца", "цу", "це", "цом", "цам", "цами", "цах", "цы", "цов",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ОВ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ов$").unwrap(),
            re_замены: Regex::new(r"(?i)(ов)$").unwrap(),
            окончание_строка: "ов",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овее$").unwrap(),
            re_замены: Regex::new(r"(?i)(овее)$").unwrap(),
            окончание_строка: "овее",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ова$").unwrap(),
            re_замены: Regex::new(r"(?i)(ова)$").unwrap(),
            окончание_строка: "ова",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овая$").unwrap(),
            re_замены: Regex::new(r"(?i)(овая)$").unwrap(),
            окончание_строка: "овая",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овую$").unwrap(),
            re_замены: Regex::new(r"(?i)(овую)$").unwrap(),
            окончание_строка: "овую",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ово$").unwrap(),
            re_замены: Regex::new(r"(?i)(ово)$").unwrap(),
            окончание_строка: "ово",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овы$").unwrap(),
            re_замены: Regex::new(r"(?i)(овы)$").unwrap(),
            окончание_строка: "овы",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овом$").unwrap(),
            re_замены: Regex::new(r"(?i)(овом)$").unwrap(),
            окончание_строка: "овом",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овому$").unwrap(),
            re_замены: Regex::new(r"(?i)(овому)$").unwrap(),
            окончание_строка: "овому",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ового$").unwrap(),
            re_замены: Regex::new(r"(?i)(ового)$").unwrap(),
            окончание_строка: "ового",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ово$").unwrap(),
            re_замены: Regex::new(r"(?i)(ово)$").unwrap(),
            окончание_строка: "ово",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овое$").unwrap(),
            re_замены: Regex::new(r"(?i)(овое)$").unwrap(),
            окончание_строка: "овое",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овой$").unwrap(),
            re_замены: Regex::new(r"(?i)(овой)$").unwrap(),
            окончание_строка: "овой",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овость$").unwrap(),
            re_замены: Regex::new(r"(?i)(овость)$").unwrap(),
            окончание_строка: "овость",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостью)$").unwrap(),
            окончание_строка: "овостью",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овости$").unwrap(),
            re_замены: Regex::new(r"(?i)(овости)$").unwrap(),
            окончание_строка: "овости",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостей)$").unwrap(),
            окончание_строка: "овостей",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостям)$").unwrap(),
            окончание_строка: "овостям",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостями)$").unwrap(),
            окончание_строка: "овостями",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(овостях)$").unwrap(),
            окончание_строка: "овостях",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овым$").unwrap(),
            re_замены: Regex::new(r"(?i)(овым)$").unwrap(),
            окончание_строка: "овым",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(овыми)$").unwrap(),
            окончание_строка: "овыми",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овый$").unwrap(),
            re_замены: Regex::new(r"(?i)(овый)$").unwrap(),
            окончание_строка: "овый",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овые$").unwrap(),
            re_замены: Regex::new(r"(?i)(овые)$").unwrap(),
            окончание_строка: "овые",
            вид_окончания: Вид_Окончания::Ов,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+овых$").unwrap(),
            re_замены: Regex::new(r"(?i)(овых)$").unwrap(),
            окончание_строка: "овых",
            вид_окончания: Вид_Окончания::Ов,
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЕВ: LazyLock<[RE_полное_окончание; 25]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ев$").unwrap(),
            re_замены: Regex::new(r"(?i)(ев)$").unwrap(),
            окончание_строка: "ев",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евее$").unwrap(),
            re_замены: Regex::new(r"(?i)(евее)$").unwrap(),
            окончание_строка: "евее",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ева$").unwrap(),
            re_замены: Regex::new(r"(?i)(ева)$").unwrap(),
            окончание_строка: "ева",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евая$").unwrap(),
            re_замены: Regex::new(r"(?i)(евая)$").unwrap(),
            окончание_строка: "евая",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евую$").unwrap(),
            re_замены: Regex::new(r"(?i)(евую)$").unwrap(),
            окончание_строка: "евую",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ево$").unwrap(),
            re_замены: Regex::new(r"(?i)(ево)$").unwrap(),
            окончание_строка: "ево",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евы$").unwrap(),
            re_замены: Regex::new(r"(?i)(евы)$").unwrap(),
            окончание_строка: "евы",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евом$").unwrap(),
            re_замены: Regex::new(r"(?i)(евом)$").unwrap(),
            окончание_строка: "евом",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евому$").unwrap(),
            re_замены: Regex::new(r"(?i)(евому)$").unwrap(),
            окончание_строка: "евому",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евого$").unwrap(),
            re_замены: Regex::new(r"(?i)(евого)$").unwrap(),
            окончание_строка: "евого",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ево$").unwrap(),
            re_замены: Regex::new(r"(?i)(ево)$").unwrap(),
            окончание_строка: "ево",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евое$").unwrap(),
            re_замены: Regex::new(r"(?i)(евое)$").unwrap(),
            окончание_строка: "евое",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евой$").unwrap(),
            re_замены: Regex::new(r"(?i)(евой)$").unwrap(),
            окончание_строка: "евой",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евость$").unwrap(),
            re_замены: Regex::new(r"(?i)(евость)$").unwrap(),
            окончание_строка: "евость",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостью)$").unwrap(),
            окончание_строка: "евостью",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евости$").unwrap(),
            re_замены: Regex::new(r"(?i)(евости)$").unwrap(),
            окончание_строка: "евости",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостей)$").unwrap(),
            окончание_строка: "евостей",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостям)$").unwrap(),
            окончание_строка: "евостям",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостями)$").unwrap(),
            окончание_строка: "евостями",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(евостях)$").unwrap(),
            окончание_строка: "евостях",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евым$").unwrap(),
            re_замены: Regex::new(r"(?i)(евым)$").unwrap(),
            окончание_строка: "евым",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(евыми)$").unwrap(),
            окончание_строка: "евыми",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евый$").unwrap(),
            re_замены: Regex::new(r"(?i)(евый)$").unwrap(),
            окончание_строка: "евый",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евые$").unwrap(),
            re_замены: Regex::new(r"(?i)(евые)$").unwrap(),
            окончание_строка: "евые",
            вид_окончания: Вид_Окончания::Ев,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+евых$").unwrap(),
            re_замены: Regex::new(r"(?i)(евых)$").unwrap(),
            окончание_строка: "евых",
            вид_окончания: Вид_Окончания::Ев,
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
            re_поиска: Regex::new(r"(?i)\w+д$").unwrap(),
            re_замены: Regex::new(r"(?i)(д)$").unwrap(),
            окончание_строка: "д",
            вид_окончания: Вид_Окончания::_Д,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+да$").unwrap(),
            re_замены: Regex::new(r"(?i)(да)$").unwrap(),
            окончание_строка: "да",
            вид_окончания: Вид_Окончания::_Д,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ду$").unwrap(),
            re_замены: Regex::new(r"(?i)(ду)$").unwrap(),
            окончание_строка: "ду",
            вид_окончания: Вид_Окончания::_Д,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+де$").unwrap(),
            re_замены: Regex::new(r"(?i)(де)$").unwrap(),
            окончание_строка: "де",
            вид_окончания: Вид_Окончания::_Д,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+дом$").unwrap(),
            re_замены: Regex::new(r"(?i)(дом)$").unwrap(),
            окончание_строка: "дом",
            вид_окончания: Вид_Окончания::_Д,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+дам$").unwrap(),
            re_замены: Regex::new(r"(?i)(дам)$").unwrap(),
            окончание_строка: "дам",
            вид_окончания: Вид_Окончания::_Д,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+дами$").unwrap(),
            re_замены: Regex::new(r"(?i)(дами)$").unwrap(),
            окончание_строка: "дами",
            вид_окончания: Вид_Окончания::_Д,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+дах$").unwrap(),
            re_замены: Regex::new(r"(?i)(дах)$").unwrap(),
            окончание_строка: "дах",
            вид_окончания: Вид_Окончания::_Д,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ды$").unwrap(),
            re_замены: Regex::new(r"(?i)(ды)$").unwrap(),
            окончание_строка: "ды",
            вид_окончания: Вид_Окончания::_Д,
        },
        // Дубликат "до" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+дов$").unwrap(),
            re_замены: Regex::new(r"(?i)(дов)$").unwrap(),
            окончание_строка: "дов",
            вид_окончания: Вид_Окончания::_Д,
        },
    ]
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_С: LazyLock<[RE_полное_окончание; 10]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+с$").unwrap(),
            re_замены: Regex::new(r"(?i)(с)$").unwrap(),
            окончание_строка: "с",
            вид_окончания: Вид_Окончания::_С,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+са$").unwrap(),
            re_замены: Regex::new(r"(?i)(са)$").unwrap(),
            окончание_строка: "са",
            вид_окончания: Вид_Окончания::_С,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+су$").unwrap(),
            re_замены: Regex::new(r"(?i)(су)$").unwrap(),
            окончание_строка: "су",
            вид_окончания: Вид_Окончания::_С,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+се$").unwrap(),
            re_замены: Regex::new(r"(?i)(се)$").unwrap(),
            окончание_строка: "се",
            вид_окончания: Вид_Окончания::_С,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сом$").unwrap(),
            re_замены: Regex::new(r"(?i)(сом)$").unwrap(),
            окончание_строка: "сом",
            вид_окончания: Вид_Окончания::_С,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сам$").unwrap(),
            re_замены: Regex::new(r"(?i)(сам)$").unwrap(),
            окончание_строка: "сам",
            вид_окончания: Вид_Окончания::_С,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сами$").unwrap(),
            re_замены: Regex::new(r"(?i)(сами)$").unwrap(),
            окончание_строка: "сами",
            вид_окончания: Вид_Окончания::_С,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сах$").unwrap(),
            re_замены: Regex::new(r"(?i)(сах)$").unwrap(),
            окончание_строка: "сах",
            вид_окончания: Вид_Окончания::_С,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сы$").unwrap(),
            re_замены: Regex::new(r"(?i)(сы)$").unwrap(),
            окончание_строка: "сы",
            вид_окончания: Вид_Окончания::_С,
        },
        // Дубликат "со" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сов$").unwrap(),
            re_замены: Regex::new(r"(?i)(сов)$").unwrap(),
            окончание_строка: "сов",
            вид_окончания: Вид_Окончания::_С,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_С: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "с", "са", "су", "се", "сом", "сам", "сами", "сах", "сы", "сов",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЛЮ_ЛЯ: LazyLock<[RE_полное_окончание; 9]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ль$").unwrap(),
            re_замены: Regex::new(r"(?i)(ль)$").unwrap(),
            окончание_строка: "ль",
            вид_окончания: Вид_Окончания::Лю_ля,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ля$").unwrap(),
            re_замены: Regex::new(r"(?i)(ля)$").unwrap(),
            окончание_строка: "ля",
            вид_окончания: Вид_Окончания::Лю_ля,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лю$").unwrap(),
            re_замены: Regex::new(r"(?i)(лю)$").unwrap(),
            окончание_строка: "лю",
            вид_окончания: Вид_Окончания::Лю_ля,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ли$").unwrap(),
            re_замены: Regex::new(r"(?i)(ли)$").unwrap(),
            окончание_строка: "ли",
            вид_окончания: Вид_Окончания::Лю_ля,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ле$").unwrap(),
            re_замены: Regex::new(r"(?i)(ле)$").unwrap(),
            окончание_строка: "ле",
            вид_окончания: Вид_Окончания::Лю_ля,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лёй$").unwrap(),
            re_замены: Regex::new(r"(?i)(лёй)$").unwrap(),
            окончание_строка: "лёй",
            вид_окончания: Вид_Окончания::Лю_ля,
        },
        /*RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ль$").unwrap(),
            re_замены: Regex::new(r"(?i)(ль)$").unwrap(),
            окончание_строка: "ель",
            вид_окончания: Вид_Окончания::Лю_ля,
        },*/
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лям$").unwrap(),
            re_замены: Regex::new(r"(?i)(лям)$").unwrap(),
            окончание_строка: "лям",
            вид_окончания: Вид_Окончания::Лю_ля,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лями$").unwrap(),
            re_замены: Regex::new(r"(?i)(лями)$").unwrap(),
            окончание_строка: "лями",
            вид_окончания: Вид_Окончания::Лю_ля,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лях$").unwrap(),
            re_замены: Regex::new(r"(?i)(лях)$").unwrap(),
            окончание_строка: "лях",
            вид_окончания: Вид_Окончания::Лю_ля,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЛЮ_ЛЯ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter(["ля", "лю", "ли", "ле", "лёй", "ль", "лям", "лями", "лях"])
});
pub static КУЧА_ОКОНЧАНИЙ_Д: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "д", "да", "ду", "де", "дом", "дам", "дами", "дах", "ды", "дов",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЧИ: LazyLock<[RE_полное_окончание; 9]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+чия$").unwrap(),
            re_замены: Regex::new(r"(?i)(чия)$").unwrap(),
            окончание_строка: "чия",
            вид_окончания: Вид_Окончания::Чи,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+чие$").unwrap(),
            re_замены: Regex::new(r"(?i)(чие)$").unwrap(),
            окончание_строка: "чие",
            вид_окончания: Вид_Окончания::Чи,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+чии$").unwrap(),
            re_замены: Regex::new(r"(?i)(чии)$").unwrap(),
            окончание_строка: "чии",
            вид_окончания: Вид_Окончания::Чи,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+чий$").unwrap(),
            re_замены: Regex::new(r"(?i)(чий)$").unwrap(),
            окончание_строка: "чий",
            вид_окончания: Вид_Окончания::Чи,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+чие$").unwrap(),
            re_замены: Regex::new(r"(?i)(чие)$").unwrap(),
            окончание_строка: "чие",
            вид_окончания: Вид_Окончания::Чи,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+чию$").unwrap(),
            re_замены: Regex::new(r"(?i)(чию)$").unwrap(),
            окончание_строка: "чию",
            вид_окончания: Вид_Окончания::Чи,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+чиям$").unwrap(),
            re_замены: Regex::new(r"(?i)(чиям)$").unwrap(),
            окончание_строка: "чиям",
            вид_окончания: Вид_Окончания::Чи,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+чиями$").unwrap(),
            re_замены: Regex::new(r"(?i)(чиями)$").unwrap(),
            окончание_строка: "чиями",
            вид_окончания: Вид_Окончания::Чи,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+чиях$").unwrap(),
            re_замены: Regex::new(r"(?i)(чиях)$").unwrap(),
            окончание_строка: "чиях",
            вид_окончания: Вид_Окончания::Чи,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_ЧИ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter(["чий", "чие", "чии", "чия", "чию", "чиям", "чиями", "чиях"])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_ЯТ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ят$").unwrap(),
            re_замены: Regex::new(r"(?i)(ят)$").unwrap(),
            окончание_строка: "ят",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятее$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятее)$").unwrap(),
            окончание_строка: "ятее",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ята$").unwrap(),
            re_замены: Regex::new(r"(?i)(ята)$").unwrap(),
            окончание_строка: "ята",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятая$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятая)$").unwrap(),
            окончание_строка: "ятая",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятую$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятую)$").unwrap(),
            окончание_строка: "ятую",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ято$").unwrap(),
            re_замены: Regex::new(r"(?i)(ято)$").unwrap(),
            окончание_строка: "ято",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+яты$").unwrap(),
            re_замены: Regex::new(r"(?i)(яты)$").unwrap(),
            окончание_строка: "яты",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятом$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятом)$").unwrap(),
            окончание_строка: "ятом",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятому$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятому)$").unwrap(),
            окончание_строка: "ятому",
            вид_окончания: Вид_Окончания::Ят,
        },
        // Дубликат "ято" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятого$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятого)$").unwrap(),
            окончание_строка: "ятого",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятое$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятое)$").unwrap(),
            окончание_строка: "ятое",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятой$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятой)$").unwrap(),
            окончание_строка: "ятой",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятость$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятость)$").unwrap(),
            окончание_строка: "ятость",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостью)$").unwrap(),
            окончание_строка: "ятостью",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятости$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятости)$").unwrap(),
            окончание_строка: "ятости",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостей)$").unwrap(),
            окончание_строка: "ятостей",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостям)$").unwrap(),
            окончание_строка: "ятостям",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостями)$").unwrap(),
            окончание_строка: "ятостями",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятостях)$").unwrap(),
            окончание_строка: "ятостях",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятым$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятым)$").unwrap(),
            окончание_строка: "ятым",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятыми)$").unwrap(),
            окончание_строка: "ятыми",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятый$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятый)$").unwrap(),
            окончание_строка: "ятый",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятые$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятые)$").unwrap(),
            окончание_строка: "ятые",
            вид_окончания: Вид_Окончания::Ят,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ятых$").unwrap(),
            re_замены: Regex::new(r"(?i)(ятых)$").unwrap(),
            окончание_строка: "ятых",
            вид_окончания: Вид_Окончания::Ят,
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_АТ: LazyLock<[RE_полное_окончание; 24]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ат$").unwrap(),
            re_замены: Regex::new(r"(?i)(ат)$").unwrap(),
            окончание_строка: "ат",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атее$").unwrap(),
            re_замены: Regex::new(r"(?i)(атее)$").unwrap(),
            окончание_строка: "атее",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ата$").unwrap(),
            re_замены: Regex::new(r"(?i)(ата)$").unwrap(),
            окончание_строка: "ата",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атая$").unwrap(),
            re_замены: Regex::new(r"(?i)(атая)$").unwrap(),
            окончание_строка: "атая",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атую$").unwrap(),
            re_замены: Regex::new(r"(?i)(атую)$").unwrap(),
            окончание_строка: "атую",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ато$").unwrap(),
            re_замены: Regex::new(r"(?i)(ато)$").unwrap(),
            окончание_строка: "ато",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+аты$").unwrap(),
            re_замены: Regex::new(r"(?i)(аты)$").unwrap(),
            окончание_строка: "аты",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атом$").unwrap(),
            re_замены: Regex::new(r"(?i)(атом)$").unwrap(),
            окончание_строка: "атом",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атому$").unwrap(),
            re_замены: Regex::new(r"(?i)(атому)$").unwrap(),
            окончание_строка: "атому",
            вид_окончания: Вид_Окончания::Ат,
        },
        // Дубликат "ато" удалён
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атого$").unwrap(),
            re_замены: Regex::new(r"(?i)(атого)$").unwrap(),
            окончание_строка: "атого",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атое$").unwrap(),
            re_замены: Regex::new(r"(?i)(атое)$").unwrap(),
            окончание_строка: "атое",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атой$").unwrap(),
            re_замены: Regex::new(r"(?i)(атой)$").unwrap(),
            окончание_строка: "атой",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атость$").unwrap(),
            re_замены: Regex::new(r"(?i)(атость)$").unwrap(),
            окончание_строка: "атость",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостью)$").unwrap(),
            окончание_строка: "атостью",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атости$").unwrap(),
            re_замены: Regex::new(r"(?i)(атости)$").unwrap(),
            окончание_строка: "атости",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостей)$").unwrap(),
            окончание_строка: "атостей",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостям)$").unwrap(),
            окончание_строка: "атостям",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостями)$").unwrap(),
            окончание_строка: "атостями",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(атостях)$").unwrap(),
            окончание_строка: "атостях",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атым$").unwrap(),
            re_замены: Regex::new(r"(?i)(атым)$").unwrap(),
            окончание_строка: "атым",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атыми$").unwrap(),
            re_замены: Regex::new(r"(?i)(атыми)$").unwrap(),
            окончание_строка: "атыми",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атый$").unwrap(),
            re_замены: Regex::new(r"(?i)(атый)$").unwrap(),
            окончание_строка: "атый",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атые$").unwrap(),
            re_замены: Regex::new(r"(?i)(атые)$").unwrap(),
            окончание_строка: "атые",
            вид_окончания: Вид_Окончания::Ат,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+атых$").unwrap(),
            re_замены: Regex::new(r"(?i)(атых)$").unwrap(),
            окончание_строка: "атых",
            вид_окончания: Вид_Окончания::Ат,
        },
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_АТ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "ат",
        "атее",
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
        "атых",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Л: LazyLock<[RE_полное_окончание; 9]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+л$").unwrap(),
            re_замены: Regex::new(r"(?i)(л)$").unwrap(),
            окончание_строка: "л",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ла$").unwrap(),
            re_замены: Regex::new(r"(?i)(ла)$").unwrap(),
            окончание_строка: "ла",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лу$").unwrap(),
            re_замены: Regex::new(r"(?i)(лу)$").unwrap(),
            окончание_строка: "лу",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ле$").unwrap(),
            re_замены: Regex::new(r"(?i)(ле)$").unwrap(),
            окончание_строка: "ле",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лом$").unwrap(),
            re_замены: Regex::new(r"(?i)(лом)$").unwrap(),
            окончание_строка: "лом",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лам$").unwrap(),
            re_замены: Regex::new(r"(?i)(лам)$").unwrap(),
            окончание_строка: "лам",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лами$").unwrap(),
            re_замены: Regex::new(r"(?i)(лами)$").unwrap(),
            окончание_строка: "лами",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лах$").unwrap(),
            re_замены: Regex::new(r"(?i)(лах)$").unwrap(),
            окончание_строка: "лах",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+лы$").unwrap(),
            re_замены: Regex::new(r"(?i)(лы)$").unwrap(),
            окончание_строка: "лы",
            вид_окончания: Вид_Окончания::_Л,
        },
        // Дубликат "ло" удалён
    ]
});
pub static КУЧА_ОКОНЧАНИЙ_Л: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter(["л", "ла", "лу", "ле", "лом", "лам", "лами", "лах", "лы"])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_К: LazyLock<[RE_полное_окончание; 10]> = LazyLock::new(|| {
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
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Г_ПОЛН: LazyLock<[RE_полное_окончание; 23]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+г$").unwrap(),
            re_замены: Regex::new(r"(?i)(г)$").unwrap(),
            окончание_строка: "г",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+га$").unwrap(),
            re_замены: Regex::new(r"(?i)(га)$").unwrap(),
            окончание_строка: "га",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гая$").unwrap(),
            re_замены: Regex::new(r"(?i)(гая)$").unwrap(),
            окончание_строка: "гая",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гую$").unwrap(),
            re_замены: Regex::new(r"(?i)(гую)$").unwrap(),
            окончание_строка: "гую",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гое$").unwrap(),
            re_замены: Regex::new(r"(?i)(гое)$").unwrap(),
            окончание_строка: "гое",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ги$").unwrap(),
            re_замены: Regex::new(r"(?i)(ги)$").unwrap(),
            окончание_строка: "ги",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гом$").unwrap(),
            re_замены: Regex::new(r"(?i)(гом)$").unwrap(),
            окончание_строка: "гом",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гому$").unwrap(),
            re_замены: Regex::new(r"(?i)(гому)$").unwrap(),
            окончание_строка: "гому",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гого$").unwrap(),
            re_замены: Regex::new(r"(?i)(гого)$").unwrap(),
            окончание_строка: "гого",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+го$").unwrap(),
            re_замены: Regex::new(r"(?i)(го)$").unwrap(),
            окончание_строка: "го",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гой$").unwrap(),
            re_замены: Regex::new(r"(?i)(гой)$").unwrap(),
            окончание_строка: "гой",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гость$").unwrap(),
            re_замены: Regex::new(r"(?i)(гость)$").unwrap(),
            окончание_строка: "гость",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гостью$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостью)$").unwrap(),
            окончание_строка: "гостью",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гости$").unwrap(),
            re_замены: Regex::new(r"(?i)(гости)$").unwrap(),
            окончание_строка: "гости",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гостей$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостей)$").unwrap(),
            окончание_строка: "гостей",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гостям$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостям)$").unwrap(),
            окончание_строка: "гостям",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гостями$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостями)$").unwrap(),
            окончание_строка: "гостями",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гостях$").unwrap(),
            re_замены: Regex::new(r"(?i)(гостях)$").unwrap(),
            окончание_строка: "гостях",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гим$").unwrap(),
            re_замены: Regex::new(r"(?i)(гим)$").unwrap(),
            окончание_строка: "гим",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гими$").unwrap(),
            re_замены: Regex::new(r"(?i)(гими)$").unwrap(),
            окончание_строка: "гими",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гий$").unwrap(),
            re_замены: Regex::new(r"(?i)(гий)$").unwrap(),
            окончание_строка: "гий",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гие$").unwrap(),
            re_замены: Regex::new(r"(?i)(гие)$").unwrap(),
            окончание_строка: "гие",
            вид_окончания: Вид_Окончания::Г_полн,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+гих$").unwrap(),
            re_замены: Regex::new(r"(?i)(гих)$").unwrap(),
            окончание_строка: "гих",
            вид_окончания: Вид_Окончания::Г_полн,
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
pub static РАЗДЕЛ_ОКОНЧАНИЙ_СЛ: LazyLock<[RE_полное_окончание; 19]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+есена$").unwrap(),
            re_замены: Regex::new(r"(?i)(есена)$").unwrap(),
            окончание_строка: "есена",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+есен$").unwrap(),
            re_замены: Regex::new(r"(?i)(есен)$").unwrap(),
            окончание_строка: "есен",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+есён$").unwrap(),
            re_замены: Regex::new(r"(?i)(есён)$").unwrap(),
            окончание_строка: "есён",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ес$").unwrap(),
            re_замены: Regex::new(r"(?i)(ес)$").unwrap(),
            окончание_строка: "ес",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+есем$").unwrap(),
            re_замены: Regex::new(r"(?i)(есем)$").unwrap(),
            окончание_строка: "есем",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+есём$").unwrap(),
            re_замены: Regex::new(r"(?i)(есём)$").unwrap(),
            окончание_строка: "есём",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ёс$").unwrap(),
            re_замены: Regex::new(r"(?i)(ёс)$").unwrap(),
            окончание_строка: "ёс",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сла$").unwrap(),
            re_замены: Regex::new(r"(?i)(сла)$").unwrap(),
            окончание_строка: "сла",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+слась$").unwrap(),
            re_замены: Regex::new(r"(?i)(слась)$").unwrap(),
            окончание_строка: "слась",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сло$").unwrap(),
            re_замены: Regex::new(r"(?i)(сло)$").unwrap(),
            окончание_строка: "сло",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+слось$").unwrap(),
            re_замены: Regex::new(r"(?i)(слось)$").unwrap(),
            окончание_строка: "слось",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+слись$").unwrap(),
            re_замены: Regex::new(r"(?i)(слись)$").unwrap(),
            окончание_строка: "слись",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сли$").unwrap(),
            re_замены: Regex::new(r"(?i)(сли)$").unwrap(),
            окончание_строка: "сли",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сет$").unwrap(),
            re_замены: Regex::new(r"(?i)(сет)$").unwrap(),
            окончание_строка: "сет",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сёт$").unwrap(),
            re_замены: Regex::new(r"(?i)(сёт)$").unwrap(),
            окончание_строка: "сёт",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сут$").unwrap(),
            re_замены: Regex::new(r"(?i)(сут)$").unwrap(),
            окончание_строка: "сут",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+сутся$").unwrap(),
            re_замены: Regex::new(r"(?i)(сутся)$").unwrap(),
            окончание_строка: "сутся",
            вид_окончания: Вид_Окончания::Сл,
        },
        RE_полное_окончание {
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
        },
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
        "сти",
        "стись",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_КО_КА: LazyLock<[RE_полное_окончание; 11]> = LazyLock::new(|| {
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
});
pub static КУЧА_ОКОНЧАНИЙ_Ж: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        "жу", "ж", "жи", "же", "жей", "жею", "жю", "жам", "жами", "жах",
    ])
});
pub static РАЗДЕЛ_ОКОНЧАНИЙ_Ж: LazyLock<[RE_полное_окончание; 10]> = LazyLock::new(|| {
    [
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+жу$").unwrap(),
            re_замены: Regex::new(r"(?i)(жу)$").unwrap(),
            окончание_строка: "жу",
            вид_окончания: Вид_Окончания::_Ж,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+ж$").unwrap(),
            re_замены: Regex::new(r"(?i)(ж)$").unwrap(),
            окончание_строка: "ж",
            вид_окончания: Вид_Окончания::_Ж,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+жи$").unwrap(),
            re_замены: Regex::new(r"(?i)(жи)$").unwrap(),
            окончание_строка: "жи",
            вид_окончания: Вид_Окончания::_Ж,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+же$").unwrap(),
            re_замены: Regex::new(r"(?i)(же)$").unwrap(),
            окончание_строка: "же",
            вид_окончания: Вид_Окончания::_Ж,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+жей$").unwrap(),
            re_замены: Regex::new(r"(?i)(жей)$").unwrap(),
            окончание_строка: "жей",
            вид_окончания: Вид_Окончания::_Ж,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+жею$").unwrap(),
            re_замены: Regex::new(r"(?i)(жею)$").unwrap(),
            окончание_строка: "жею",
            вид_окончания: Вид_Окончания::_Ж,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+жю$").unwrap(),
            re_замены: Regex::new(r"(?i)(жю)$").unwrap(),
            окончание_строка: "жю",
            вид_окончания: Вид_Окончания::_Ж,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+жам$").unwrap(),
            re_замены: Regex::new(r"(?i)(жам)$").unwrap(),
            окончание_строка: "жам",
            вид_окончания: Вид_Окончания::_Ж,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+жами$").unwrap(),
            re_замены: Regex::new(r"(?i)(жами)$").unwrap(),
            окончание_строка: "жами",
            вид_окончания: Вид_Окончания::_Ж,
        },
        RE_полное_окончание {
            re_поиска: Regex::new(r"(?i)\w+жах$").unwrap(),
            re_замены: Regex::new(r"(?i)(жах)$").unwrap(),
            окончание_строка: "жах",
            вид_окончания: Вид_Окончания::_Ж,
        },
    ]
});
//------------------------------------------------------
pub static РАЗДЕЛЫ_ОКОНЧАНИЙ_ПОЛНЫЕ: LazyLock<Разделы_окончаний_Полные> =
    LazyLock::new(|| Разделы_окончаний_Полные {
        в_одиночн: Раздел_Окончания {
            куча_окончаний: &*&КУЧА_ОКОНЧАНИЙ_В,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_В,
            вид_окончания: Вид_Окончания::_В,
        },
        г_полн: Раздел_Окончания {
            куча_окончаний: &*&КУЧА_ОКОНЧАНИЙ_Г_ПОЛН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Г_ПОЛН,
            вид_окончания: Вид_Окончания::Г_полн,
        },
        к_одиночн: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_К,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_К,
            вид_окончания: Вид_Окончания::_К,
        },
        ко_ка: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_КО_КА,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_КО_КА,
            вид_окончания: Вид_Окончания::Ко_ка,
        },
        ель: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЕЛЬ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЕЛЬ,
            вид_окончания: Вид_Окончания::Ель,
        },
        ван: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ВАН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ВАН,
            вид_окончания: Вид_Окончания::Ван,
        },
        ыт: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЫТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЫТ,
            вид_окончания: Вид_Окончания::Ыт,
        },
        ым: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЫМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЫМ,
            вид_окончания: Вид_Окончания::Ым,
        },
        ут: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_УТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_УТ,
            вид_окончания: Вид_Окончания::Ут,
        },
        ют: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЮТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЮТ,
            вид_окончания: Вид_Окончания::Ют,
        },
        ит: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ИТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ИТ,
            вид_окончания: Вид_Окончания::Ит,
        },
        ти: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ТИ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ТИ,
            вид_окончания: Вид_Окончания::Ти,
        },
        ят: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЯТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЯТ,
            вид_окончания: Вид_Окончания::Ят,
        },
        с_одиночн: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_С,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_С,
            вид_окончания: Вид_Окончания::_С,
        },
        ат: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_АТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_АТ,
            вид_окончания: Вид_Окончания::Ат,
        },
        чи: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЧИ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЧИ,
            вид_окончания: Вид_Окончания::Чи,
        },
        ив: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ИВ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ИВ,
            вид_окончания: Вид_Окончания::Ив,
        },
        н: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_Н,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Н,
            вид_окончания: Вид_Окончания::_Н,
        },
        ш: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_Ш,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Ш,
            вид_окончания: Вид_Окончания::_Ш,
        },
        щ: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_Щ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Щ,
            вид_окончания: Вид_Окончания::_Щ,
        },
        ъят: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЪЯТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЪЯТ,
            вид_окончания: Вид_Окончания::Ъят,
        },
        нят: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_НЯТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_НЯТ,
            вид_окончания: Вид_Окончания::Нят,
        },
        ст: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_СТ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СТ,
            вид_окончания: Вид_Окончания::Ст,
        },
        ств: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_СТВ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СТВ,
            вид_окончания: Вид_Окончания::Ств,
        },
        ен: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЕН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЕН,
            вид_окончания: Вид_Окончания::Ен,
        },
        лю_ля: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЛЮ_ЛЯ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЛЮ_ЛЯ,
            вид_окончания: Вид_Окончания::Лю_ля,
        },
        ён: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЁН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЁН,
            вид_окончания: Вид_Окончания::Ён,
        },
        ов: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ОВ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ОВ,
            вид_окончания: Вид_Окончания::Ов,
        },
        ев: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЕВ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЕВ,
            вид_окончания: Вид_Окончания::Ев,
        },
        он: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ОН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ОН,
            вид_окончания: Вид_Окончания::Он,
        },
        ем: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЕМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЕМ,
            вид_окончания: Вид_Окончания::Ем,
        },
        ам: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_АМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_АМ,
            вид_окончания: Вид_Окончания::Ам,
        },
        ом: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ОМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ОМ,
            вид_окончания: Вид_Окончания::Ом,
        },
        ан: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_АН,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_АН,
            вид_окончания: Вид_Окончания::Ан,
        },
        ям: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ЯМ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ЯМ,
            вид_окончания: Вид_Окончания::Ям,
        },
        авш: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_АВШ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_АВШ,
            вид_окончания: Вид_Окончания::Авш,
        },
        ты: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ТЫ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ТЫ,
            вид_окончания: Вид_Окончания::Ты,
        },
        та_то: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_ТА_ТО,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ТА_ТО,
            вид_окончания: Вид_Окончания::Та_то,
        },
        ни: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_НИ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_НИ,
            вид_окончания: Вид_Окончания::Ни,
        },
        д_одиночн: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_Д,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Д,
            вид_окончания: Вид_Окончания::_Д,
        },
        р_одиночн: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_Р,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Р,
            вид_окончания: Вид_Окончания::_Р,
        },
        л_одиночн: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_Л,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Л,
            вид_окончания: Вид_Окончания::_Л,
        },
        ж_одиночн: Раздел_Окончания {
            куча_окончаний: &*КУЧА_ОКОНЧАНИЙ_Ж,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Ж,
            вид_окончания: Вид_Окончания::_Ж,
        },
        ва_во: Раздел_Окончания {
            куча_окончаний: &*&КУЧА_ОКОНЧАНИЙ_ВА_ВО,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_ВА_ВО,
            вид_окончания: Вид_Окончания::Ва_Во,
        },
        ц_одиночн: Раздел_Окончания {
            куча_окончаний: &*&КУЧА_ОКОНЧАНИЙ_Ц,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_Ц,
            вид_окончания: Вид_Окончания::_Ц,
        },
        ск: Раздел_Окончания {
            куча_окончаний: &*&КУЧА_ОКОНЧАНИЙ_СК,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СК,
            вид_окончания: Вид_Окончания::Ск,
        },
        сл: Раздел_Окончания {
            куча_окончаний: &*&КУЧА_ОКОНЧАНИЙ_СЛ,
            re_ячейки: &*РАЗДЕЛ_ОКОНЧАНИЙ_СЛ,
            вид_окончания: Вид_Окончания::Сл,
        },
    });

//
#[derive(Debug)]
pub struct Разделы_окончаний_Полные {
    pub в_одиночн: Раздел_Окончания,
    pub ж_одиночн: Раздел_Окончания,
    pub сл: Раздел_Окончания,
    pub к_одиночн: Раздел_Окончания,
    pub ель: Раздел_Окончания,
    pub ым: Раздел_Окончания,
    pub ют: Раздел_Окончания,
    pub ут: Раздел_Окончания,
    pub г_полн: Раздел_Окончания,
    pub ям: Раздел_Окончания,
    pub ван: Раздел_Окончания,
    pub ыт: Раздел_Окончания,
    pub ат: Раздел_Окончания,
    pub лю_ля: Раздел_Окончания,
    pub ят: Раздел_Окончания,
    pub ив: Раздел_Окончания,
    pub ов: Раздел_Окончания,
    pub ев: Раздел_Окончания,
    pub ъят: Раздел_Окончания,
    pub нят: Раздел_Окончания,
    pub ит: Раздел_Окончания,
    pub ти: Раздел_Окончания,
    pub чи: Раздел_Окончания,
    pub ко_ка: Раздел_Окончания,
    pub л_одиночн: Раздел_Окончания,
    pub с_одиночн: Раздел_Окончания,
    pub д_одиночн: Раздел_Окончания,
    pub ц_одиночн: Раздел_Окончания,
    pub р_одиночн: Раздел_Окончания,
    pub н: Раздел_Окончания,
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
    pub авш: Раздел_Окончания,
    pub ни: Раздел_Окончания,
    pub ск: Раздел_Окончания,
    pub ты: Раздел_Окончания,
    pub та_то: Раздел_Окончания,
    pub ва_во: Раздел_Окончания,
}
impl Разделы_окончаний_Полные {
    /// Порядок: ван, ыт, ит, н, ст, ен, ён, он, ем, ом, ан, авш.
    pub fn iter(&self) -> impl Iterator<Item = &Раздел_Окончания> {
        [
            &self.ж_одиночн,
            &self.р_одиночн,
            &self.ель,
            &self.г_полн,
            &self.ван,
            &self.ъят,
            &self.нят,
            &self.ут,
            &self.ют,
            &self.ств,
            &self.ст,
            &self.ен,
            &self.ён,
            &self.он,
            &self.сл,
            &self.ко_ка,
            &self.ям,
            &self.ем,
            &self.ым,
            &self.ев,
            &self.ти,
            &self.чи,
            &self.ов,
            &self.ом,
            &self.ам,
            &self.ят,
            &self.лю_ля,
            &self.ат,
            &self.ан,
            &self.ив,
            &self.ск,
            &self.авш,
            &self.щ,
            &self.ш,
            &self.ни,
            &self.ва_во,
            &self.с_одиночн,
            &self.л_одиночн,
            &self.д_одиночн,
            &self.к_одиночн,
            &self.ц_одиночн,
            &self.в_одиночн,
            //
            &self.н,
            &self.ит,
            &self.ыт,
            &self.ты,
            &self.та_то,
        ]
        .into_iter()
    }
    pub fn проверка_на_наличие_окончаний(&self) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&Вид_Окончания> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        'главный_указатель: for окончание in ВСЕ_ВИДЫ_ОКОНЧАНИЙ.iter()
        {
            for раздел in self.iter() {
                //
                if раздел.вид_окончания == *окончание {
                    continue 'главный_указатель;
                }
                //
            }
            куча_ошибок.insert(окончание);
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
    pub fn проверка_куч_между_собой(&self) -> Result<(), ()> {
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
                for слово_1 in раздел.куча_окончаний.iter() {
                    if раздел_2.куча_окончаний.contains(слово_1) {
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
                style(format!("Пересечение слов в кучах ")).blue(),
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
#[derive(Debug)]
pub struct Раздел_Окончания {
    pub куча_окончаний: &'static rapidhash::fast::RapidHashSet<&'static str>,
    pub re_ячейки: &'static [RE_полное_окончание],
    pub вид_окончания: Вид_Окончания,
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
    pub fn проверка_соответствия_наличия_строки_в_re_заменах(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        'главный_указатель: for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if sz_найти(&ячейка.re_замены.to_string(), ячейка.окончание_строка)
            {
                continue 'главный_указатель;
            } else {
                куча_ошибок.insert(ячейка.окончание_строка);
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

    pub fn проверка_соответствия_re_замены_строке(
        &self,
    ) -> Result<(), ()> {
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //
        'главный_указатель: for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if ячейка.re_замены.is_match(&ячейка.окончание_строка)
            {
                continue 'главный_указатель;
            } else {
                куча_ошибок.insert(ячейка.окончание_строка);
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
            if sz_найти(&ячейка.re_поиска.to_string(), ячейка.окончание_строка)
            {
                continue 'главный_указатель;
            } else {
                куча_ошибок.insert(ячейка.окончание_строка);
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
            куча_ошибок.insert(ячейка.окончание_строка);
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
        'главный_указатель: for слово_из_кучи in self.куча_окончаний.iter()
        {
            for ячейка in self.re_ячейки.iter() {
                //совпадает ли окончание
                if ячейка.окончание_строка == *слово_из_кучи {
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
        let mut куча_ошибок: rapidhash::fast::RapidHashSet<&str> =
            rapidhash::fast::RapidHashSet::default();
        //перебор кучи окончаний
        //перебор раздела
        'главный_указатель: for ячейка in self.re_ячейки.iter() {
            //совпадает ли окончание
            if self.куча_окончаний.contains(&ячейка.окончание_строка)
            {
                continue 'главный_указатель;
            }
            //
            куча_ошибок.insert(ячейка.окончание_строка);
        }
        //
        if куча_ошибок.len() > 0 {
            print!(
                "\r\n{}{}{}",
                style(format!("Ошибка ")).red(),
                style(format!("Нет искомого окончания  ")).blue(),
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
#[derive(Debug)]
pub struct Вид_Окончания_со_строкой {
    pub строка: &'static str,
    pub re_выдер: Regex,
    pub вид_окончания: Вид_Окончания,
}
impl Default for Вид_Окончания_со_строкой {
    fn default() -> Self {
        Self {
            строка: "",
            re_выдер: Regex::new(r"").unwrap(),
            вид_окончания: Вид_Окончания::Не_определено,
        }
    }
}

#[derive(Debug)]
pub struct Окончаний_Торжка {
    pub первая_очередь: Vec<Вид_Окончания_со_строкой>,
    pub вторая_очередь: Vec<Вид_Окончания_со_строкой>,
    pub куча_окончаний: rapidhash::fast::RapidHashSet<&'static str>,
    pub окончания_исключения: rapidhash::fast::RapidHashSet<Вид_Окончания>,
}
impl Окончаний_Торжка {
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
                "\r\n{}{}{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" Окончания  вторичного ряда ")).blue(),
                style(format!(
                    " есть в исключениях , первое окончание для поиска раздела - "
                )),
                style(format!("|{}|", self.вторая_очередь[0].строка)).cyan(),
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
        if self.окончания_исключения.len() == 0 {
            return Ok(());
        }
        //
        'главный_указатель: for (указатель_1, ячейка) in
            self.вторая_очередь.iter().enumerate()
        {
            //сначала с первой очередью
            for (_, ячейка_2) in self
                .вторая_очередь
                .iter()
                .enumerate()
                .filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                if ячейка.строка == ячейка_2.строка {
                    куча_ошибок_строка.insert(&ячейка.строка);
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                }
            }
            //сначала с первой очередью
            for (_, ячейка_2) in self.первая_очередь.iter().enumerate()
            //.filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                if ячейка.строка == ячейка_2.строка {
                    куча_ошибок_строка.insert(&ячейка.строка);
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                }
            }
            //
        }
        //
        'главный_указатель: for (указатель_1, ячейка) in
            self.первая_очередь.iter().enumerate()
        {
            //сначала с первой очередью
            for (_, ячейка_2) in self
                .первая_очередь
                .iter()
                .enumerate()
                .filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            //
            {
                if ячейка.строка == ячейка_2.строка {
                    куча_ошибок_строка.insert(&ячейка.строка);
                }
                if ячейка.re_выдер.as_str() == ячейка_2.re_выдер.as_str() {
                    куча_ошибок_re_выдер.insert(ячейка.re_выдер.to_string());
                }
                if ячейка.вид_окончания == ячейка_2.вид_окончания
                {
                    куча_ошибок_вид_окончания.insert(&ячейка.вид_окончания);
                }
            }
            //сначала с первой очередью
            for (_, ячейка_2) in self.вторая_очередь.iter().enumerate()
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
            ячейка_2: &&Вид_Окончания_со_строкой,
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
                "\r\n{}{}{}{}",
                style(format!("Ошибка")).red(),
                style(format!(" Окончания  первичного ряда ")).blue(),
                style(format!(
                    " есть в исключениях , первое окончание для поиска раздела - "
                )),
                style(format!("|{}|", self.первая_очередь[0].строка)).cyan(),
            );
            for сообщение in куча_ошибок.iter() {
                print!("{}", style(format!("|{}| ", сообщение)).green(),);
            }
            //println!();
            //return Err(());
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
                style(format!(" Нет искомого окончания  вторичного ряда")).cyan(),
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
                style(format!("Ошибка ")).red(),
                style(format!("Нет искомого окончания  первичного ряда ")).blue(),
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
