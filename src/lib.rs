#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//use foldhash::{rapidhash::fast::RapidHashMap,  fast::RandomState,*};
//use rapidhash::*;
use regex::Regex;
use rust_xlsxwriter::{ColNum, Format, IntoExcelData, RowNum, Worksheet, XlsxError};
use std::fmt::{self};
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;
use std::sync::atomic::AtomicUsize;
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
    Ttf,
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
    Pdf,
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
            Вид_Изображения::Gif => write!(f, "gif"),
            Вид_Изображения::Tif => write!(f, "tif"),
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
        }
    }
}

// Вид_Word
impl fmt::Display for Вид_Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Вид_Word::Doc => write!(f, "doc"),
            Вид_Word::Docx => write!(f, "docx"),
            Вид_Word::Rtf => write!(f, "rtf"),
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
        }
    }
}

// Вид_Разметки_Паутины
impl fmt::Display for Вид_Разметки_Паутины {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            Вид_Разметки_Паутины::Не_определено => {
                write!(f, "не_определено")
            }
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
            Вид_одичноной_книги::Pdf => write!(f, "pdf"),
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
            Основной_Вид_Расширения::Мусорные_Разметка(
                вид,
            ) => write!(f, "мусорная_разметка({})", вид),
            Основной_Вид_Расширения::Word(вид) => {
                write!(f, "word({})", вид)
            }
            Основной_Вид_Расширения::Excel(вид) => {
                write!(f, "excel({})", вид)
            }
            Основной_Вид_Расширения::XML => write!(f, "xml"),
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
        static ОБРАЗЦЫ_RE: LazyLock<[Regex; 1]> =
            LazyLock::new(|| [Regex::new("(?i)Пусто$").unwrap()]);
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
        static ОБРАЗЦЫ_RE: LazyLock<[Regex; 1]> =
            LazyLock::new(|| [Regex::new("(?i)Пусто$").unwrap()]);
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
        return Умная_Строка::Значение(строка.to_string());
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
impl Основной_Вид_Расширения {
    pub fn пусто(&self) -> bool {
        match self {
            Основной_Вид_Расширения::Пусто => true,
            _ => false,
        }
    }
    pub fn не_архивная(&self) -> bool {
        self.fb2_mht() || self.md_yml_fs() || self.htm_html_xhtml()
    }
    //
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
                Вид_Разметки_Паутины::Htm
                | Вид_Разметки_Паутины::Htm
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
    pub расширение: String,
    pub расширение_подробно: Основной_Вид_Расширения,
    //pub изображение: Vec<u8>, //если это картинки, нельзя в utf8 переводить
}

impl Default for Вложения {
    fn default() -> Self {
        Self {
            содержимое: Vec::new(),
            расширение: "".to_string(),
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

//use std::fmt;
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
pub const РАЗМЕР_РАЗДЕЛИТЕЛЕЙ: usize = 352;

//
//
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
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
#[derive(Debug, Clone)]
pub struct Ячейка_замены_Epub {
    pub искомое_слово: Умная_Строка,
    pub re_образец: Regex,
    // pub счёчтки:usize,
}
impl Default for Ячейка_замены_Epub {
    fn default() -> Self {
        Self {
            искомое_слово: Умная_Строка::default(),
            re_образец: Regex::new(r"(?i)").unwrap(),
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
pub struct Ячейка_замены_с_разделителями {
    pub искомое_слово: Умная_Строка,
    // #[serde(skip)]
    pub ряд_re_пропуски: Vec<Regex>,
    //  #[serde(skip)]
    pub re_образец_для_поиска: Regex,
    pub замена: Умная_Строка,
    pub ряд_пропусков: Vec<Умная_Строка>,
    //
    pub ряд_обязательств: Vec<Умная_Строка>,
    //  #[serde(skip)]
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
