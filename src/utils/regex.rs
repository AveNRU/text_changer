use crate::utils::stringzilla::*;
use stringzilla::stringzilla::bytesum;
//use clap::error::ErrorKind::Format;
use crate::import::functions::преобразовать_слово_с_чертой_в_начале;
use console::{Emoji, style};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use lib::{
    Словарь_Переносов, Счётчики_Словаря, Ячейка_замены, Ячейка_замены_с_исключением
};
use rand::{Rng, prelude::*};
use rayon::prelude::*;
use std::borrow::Cow;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, AtomicUsize, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};
use std::{cmp::min, fmt::Write};
/*
static PACKAGES: &[&str] = &[
    "fs-events",
    "my-awesome-module",
    "emoji-speaker",
    "wrap-ansi",
    "stream-browserify",
    "acorn-dynamic-import",
];

static COMMANDS: &[&str] = &[
    "cmake .",
    "make",
    "make clean",
    "gcc foo.c -o foo",
    "gcc bar.c -o bar",
    "./helper.sh rebuild-cache",
    "make all-clean",
    "make test",
];
*/
//static LOOKING_GLASS: &str = "🔍";
//если это картинка
use crate::lib::{
    self, Быстрый_Словарь, Полный_Словарь, СЛОВАРЬ_ПЕРЕНОСОВ_ОДНОБУКВЕННЫЕ, Словарь_разделителей,
    Счётчик_замен, Счётчик_разделителей, Ячейка_словаря,
};
use lazy_static::lazy_static;
use rayon::iter::IntoParallelRefIterator;
use regex::{Captures, Match, Regex};

pub fn мусорное_содержимое_архивов(стог_сена: &String) -> bool {
    lazy_static! {
        static ref re_расширения_мусорные: [Regex;4] = [
            Regex::new(r"(?i)\.css$").unwrap(),
              Regex::new(r"(?i)\.rels$").unwrap(),
              Regex::new(r"(?i)\.ttf$").unwrap(),
            //Regex::new(r"(?i)\.xhtml$").unwrap(),
            //целиком имя
             Regex::new(r"(?i)mimetype$").unwrap(),
            //

        ];
    }
    return re_расширения_мусорные
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}

pub fn изображение_расширение_с_точкой(
    стог_сена: &String
) -> bool {
    lazy_static! {
        static ref re_расширения_изображений: [Regex;15] = [
            //
               Regex::new(r"(?i)\.jpe?g$").unwrap(),  // Объединил jpg и jpeg
            Regex::new(r"(?i)\.tiff?$").unwrap(),  // Объединил tif и tiff
            Regex::new(r"(?i)\.bmp$").unwrap(),
            Regex::new(r"(?i)\.gif$").unwrap(),    // Добавил $ в конец
            Regex::new(r"(?i)\.webp$").unwrap(),   // Добавил современные форматы
            Regex::new(r"(?i)\.svg$").unwrap(),
            Regex::new(r"(?i)\.avif$").unwrap(),
            Regex::new(r"(?i)\.jpeg$").unwrap(),
            Regex::new(r"(?i)\.jpg$").unwrap(),
            Regex::new(r"(?i)\.tiff$").unwrap(),
            Regex::new(r"(?i)\.png$").unwrap(),
            Regex::new(r"(?i)\.wmf$").unwrap(),
            Regex::new(r"(?i)\.wpg$").unwrap(),
            Regex::new(r"(?i)\.eps$").unwrap(),
             Regex::new(r"(?i)\.ttf").unwrap(),
        ];
    }
    return re_расширения_изображений
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}

pub fn изображение_расширение_без_точки(
    стог_сена: &String
) -> bool {
    lazy_static! {
        static ref re_расширения_изображений: [Regex;14] = [
            /*Regex::new(r"(?i)\.jpe?g$").unwrap(),  // Объединил jpg и jpeg
            Regex::new(r"(?i)\.tiff?$").unwrap(),  // Объединил tif и tiff
            Regex::new(r"(?i)\.png$").unwrap(),
            Regex::new(r"(?i)\.bmp$").unwrap(),
            Regex::new(r"(?i)\.wmf$").unwrap(),
            Regex::new(r"(?i)\.wpg$").unwrap(),
            Regex::new(r"(?i)\.gif$").unwrap(),    // Добавил $ в конец
            Regex::new(r"(?i)\.webp$").unwrap(),   // Добавил современные форматы
            Regex::new(r"(?i)\.svg$").unwrap(),
            Regex::new(r"(?i)\.avif$").unwrap(),*/
            //
            Regex::new(r"(?i)\.jpe?g$").unwrap(),  // Объединил jpg и jpeg
            Regex::new(r"(?i)\.tiff?$").unwrap(),  // Объединил tif и tiff
            Regex::new(r"(?i)\.bmp$").unwrap(),
            Regex::new(r"(?i)\.gif$").unwrap(),    // Добавил $ в конец
            Regex::new(r"(?i)\.webp$").unwrap(),   // Добавил современные форматы
            Regex::new(r"(?i)\.svg$").unwrap(),
            Regex::new(r"(?i)\.avif$").unwrap(),
            Regex::new(r"(?i)\.jpeg$").unwrap(),
            Regex::new(r"(?i)\.jpg$").unwrap(),
            Regex::new(r"(?i)\.tiff$").unwrap(),
            Regex::new(r"(?i)\.png$").unwrap(),
            Regex::new(r"(?i)\.wmf$").unwrap(),
            Regex::new(r"(?i)\.wpg$").unwrap(),
            Regex::new(r"(?i)\.eps$").unwrap(),
        ];
    }
    return re_расширения_изображений
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}

pub fn не_является_изображением(стог_сена: &String) -> bool {
    lazy_static! {
        static ref re_расширения_изображений: [Regex;14] = [
            Regex::new(r"(?i)jpe?g$").unwrap(),  // Объединил jpg и jpeg
            Regex::new(r"(?i)tiff?$").unwrap(),  // Объединил tif и tiff
            Regex::new(r"(?i)bmp$").unwrap(),
            Regex::new(r"(?i)gif$").unwrap(),    // Добавил $ в конец
            Regex::new(r"(?i)webp$").unwrap(),   // Добавил современные форматы
            Regex::new(r"(?i)svg$").unwrap(),
            Regex::new(r"(?i)avif$").unwrap(),
            Regex::new(r"(?i)jpeg$").unwrap(),
            Regex::new(r"(?i)jpg$").unwrap(),
            Regex::new(r"(?i)tiff$").unwrap(),
            Regex::new(r"(?i)png$").unwrap(),
            Regex::new(r"(?i)wmf$").unwrap(),
            Regex::new(r"(?i)wpg$").unwrap(),
            Regex::new(r"(?i)eps$").unwrap(),
        ];
    }
    return re_расширения_изображений
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}
//если это архивный файл
pub fn fb3_epub(стог_сена: &String) -> bool {
    lazy_static! {
        static ref re_расширения_архивные:[Regex;2] = [
        Regex::new(r"(?i)\.fb3$").unwrap(),
        Regex::new(r"(?i)\.epub$").unwrap(),

        //Regex::new(r"(?i)\.docx$").unwrap(),
        //Regex::new(r"(?i)\.doc$").unwrap(),
     ];
    }
    return re_расширения_архивные
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}
pub fn без_кодировки(стог_сена: &String) -> bool {
    lazy_static! {
        static ref re_расширения:[Regex;1] = [
        Regex::new(r"(?i)\.txt$").unwrap(),

        //Regex::new(r"(?i)\.docx$").unwrap(),
        //Regex::new(r"(?i)\.doc$").unwrap(),
     ];
    }
    return re_расширения
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}
//если это архивный файл
pub fn doc_docx(стог_сена: &String) -> bool {
    lazy_static! {
           static ref re_расширения_word:[Regex;2] = [
        //Regex::new(r"(?i)\.fb3$").unwrap(),
        //Regex::new(r"(?i)\.epub$").unwrap(),
        Regex::new(r"(?i)\.docx$").unwrap(),
        Regex::new(r"(?i)\.doc$").unwrap(),
     ];
    }
    return re_расширения_word
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}
pub fn md_fs_yml(стог_сена: &String) -> bool {
    lazy_static! {
           static ref re_расширения_word:[Regex;3] = [
        //Regex::new(r"(?i)\.fb3$").unwrap(),
        //Regex::new(r"(?i)\.epub$").unwrap(),
        Regex::new(r"(?i)\.md$").unwrap(),
            Regex::new(r"(?i)\.yml$").unwrap(),
            Regex::new(r"(?i)\.fs$").unwrap(),
     ];
    }
    return re_расширения_word
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
    //return false;
}

pub fn htm_html_xhtml(стог_сена: &String) -> bool {
    lazy_static! {
        static ref re_расширения_word: [Regex; 3] = [
            Regex::new(r"(?i)\.htm$").unwrap(),
            Regex::new(r"(?i)\.html$").unwrap(),
            Regex::new(r"(?i)\.xhtml$").unwrap(),
        ];
    }
    return re_расширения_word
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
    //return false;
}
//если это не архивный файл
pub fn fb2_rtf_mht_mhtml(стог_сена: &String) -> bool {
    lazy_static! {
        static ref re_расширения_не_архивные: [Regex; 4] = [
            Regex::new(r"(?i)\.fb2$").unwrap(),
            Regex::new(r"(?i)\.rtf$").unwrap(),
            Regex::new(r"(?i)\.mhtml$").unwrap(),
            Regex::new(r"(?i)\.mht$").unwrap(),
        ];
    }
    return re_расширения_не_архивные
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}
//захват слов
//есть ли маты
pub fn есть_ли_маты(стог_сена: &String) -> bool {
    lazy_static! {
            //маты
     static ref re_матершина_слова:[Regex;1] = [
        Regex::new(r"(?i)\s*([\w]…)\s*").unwrap(),
     ];
    }
    return re_матершина_слова
        .par_iter()
        .any(|образец| образец.is_match(стог_сена));
}

//выдел строки
pub fn re_получить_имя_файла_без_пути(стог_сена: &String) -> String {
    lazy_static! {
        static ref без_пути: [Regex; 3] = [
            Regex::new(r"(?i)\\(.[^\\]+)$").unwrap(),
            Regex::new(r"(?i)\\([\d\w\s_\-\=\(\)]+)$").unwrap(),
            Regex::new(r"(?i)/([\d\w\s_\-\=\(\)]+)$").unwrap(),
        ];
        static ref первая_палка: Regex = Regex::new(r"(?i)\\").unwrap();
        static ref вторая_палка: Regex = Regex::new(r"(?i)/").unwrap();
    }
    if первая_палка.find_iter(стог_сена).count() == 0
        && вторая_палка.find_iter(стог_сена).count() == 0
    {
        return стог_сена.to_string();
    }
    for указатель in 0..без_пути.len() {
        if let Some(строка) = без_пути[указатель].captures(&стог_сена)
        {
            return строка[1].trim().to_string();
        }
    }

    panic!(
        "ошибка при выдирания имени файла без пути к нему |{}|",
        &стог_сена,
    );
}

//выдел строки
pub fn re_получить_строку_с_описанием(
    стог_сена: &String,
    образец: &Regex,
    ошибка: &str,
) -> Result<String, String> {
    lazy_static! {
        static ref нет_расширения: Regex = Regex::new(r"(?i)(?:\\)+([\d\w&&[^\.]]+)$").unwrap();
    }
    let Some(строка) = образец.captures(&стог_сена) else {
        if let Some(строка) = нет_расширения.captures(&стог_сена) {
            return Err("Пусто".to_string());
        } else {
            //println!("{}", ошибка);
            //  panic!(
            //      "ошибка при выдирания {}, сама строка : {}",
            //        &образец, &стог_сена
            //    );
            return Err(format!(
                "Расширение файла: Ошибка при выдирания {}, сама строка : {}. Ошибка: {}",
                &образец, &стог_сена, ошибка
            ));
        }
    };
    return Ok(строка[1].trim().to_string());
}
//выдел строки
/*
pub fn получить_строку_из_ряда_re_с_описанием(стог_сена: &String, образец: &[Regex;5],ошибка:&str) -> String {
    let Some(строка) = образец.captures(&стог_сена) else {
        println!("{}",ошибка);
        panic!("ошибка при выдирания {}, сама строка : {}", &образец, &стог_сена);
    };
    return строка[1].trim().to_string();
}

 */

pub fn определить_имя_книги(стог_сена: &String) -> String {
    lazy_static! {
        static ref re_пути_до_книг: [Regex; 6] = [
            Regex::new(r"(?i)книги/([\d\w_\-\s\.,]+)\.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i)книги\\([\d\w_\-\s\.,]+)\.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i)книги/([\d\w_\-\s\.,]+)/.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i)книги\\([\d\w_\-\s\.,]+)/.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i).+/(.+)\.").unwrap(),
             Regex::new(r"(?i)([\d\w\-_\s[^\\]]+)$").unwrap(),
            //Regex::new(r"(?i)\\(.[^\\]+)$").unwrap(),
           //  Regex::new(r"(?i)/(.[^\\/]+)$").unwrap(),
        ];
    }

    re_пути_до_книг
        .par_iter()
        .find_map_any(|образец| {
            образец.captures(стог_сена).and_then(|cap| {
                let строка = cap[1].trim().to_string();
                if строка.is_empty() {
                    None
                } else {
                    Some(строка)
                }
            })
        })
        .unwrap_or_else(|| panic!("Не удалось выдрать имя файла: {}", стог_сена))
    /*
    for образец in re_пути_до_книг.iter() {
        if let Some(строка) = образец.captures(&стог_сена) {
            let строка = строка[1].trim().to_string();
            if строка.is_empty() {
                panic!("Не удалось выдрать имя файла: {}", &стог_сена);
            } else {
                //возврат значения
                return строка;
            }
        };
    }
    panic!("ошибка при выдирания сама строка : {}", &стог_сена);

         */
}
/*
pub fn замена_слов_через_regex(
    re_образцы: &[Regex;5],
    содержимое: &mut Vec<String>,
    замены: &Vec<String>,
    счётчик_словаря: &mut Vec<usize>,
    искомое_слово: &Vec<String>,
    сообщение: &str,
    расширение: &String,
    указатель_захода: &mut usize,
    куча_пропусков: &rapidhash::fast::RapidHashSet<usize>,
    //  pb_общий: &mut ProgressBar,
) {
    //let mut итоговый_ряд_строк: Vec<String> = содержимое.clone();
    //провера указателя захода

    //увеление указателя захода
    *указатель_захода += 1;
    //обязательная проверка на входе

    //
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    println!(
        "{} {}Завершено...",
        style(сообщение).bold().dim(),
        LOOKING_GLASS
    );

    //
    let mut downloaded = 0;
    let количество_шагов: u64 = u64::try_from(re_образцы.len() * содержимое.len()).unwrap();
    let счетчик_внутренний = ProgressBar::new(количество_шагов);
    let mut шаг_внутренний: u64 = 0;
    счетчик_внутренний.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );
    //
    for указатель in 0..содержимое.len() {
        //for указатель in 0..2 {

        //проверка формата
        // if проверка_содержимого_в_зависимости_от_расширения_книги(&строка, &расширение) { continue 'перебор_строк }
        if куча_пропусков.contains(&указатель) {
            continue;
            //return;//многопоточка
        }
        //сам перебор
            //содержимое.par_iter().enumerate().for_each(
        //
        for указатель_образца in 0..re_образцы.len() {
            let re_образец: &Regex = &re_образцы[указатель_образца];
            if sz_найти(&содержимое[указатель], &искомое_слово[указатель_образца])
            {
                //regex
                let замененная_строка: std::borrow::Cow<'_, str> = re_образец.replace_all(
                    &содержимое[указатель],     //строка, в которой происходит замена
                    &замены[указатель_образца], //на что заменить
                );
                содержимое[указатель] = замененная_строка.to_string();
                //увеличение счётчика замен
                счётчик_словаря[указатель_образца] += 1;

                //thread::sleep(Duration::from_millis(1));
            }
            шаг_внутренний += 1;
            счетчик_внутренний.set_position(шаг_внутренний);

            // pb_общий.inc(1);
            //thread::sleep(Duration::from_millis(1));
        }
    }
    // if итоговый_ряд_строк==*содержимое { println!("векторы равны :{}",сообщение) }
    // return итоговый_ряд_строк;
}

*/
//многопоточность
/*
pub fn замена_слов_через_regex(
    re_образцы: &[Regex],
    содержимое: &mut [String],
    замены: &[String],
    счётчик_словаря: &mut [usize],
    искомое_слово: &[String],
    сообщение: &str,
    расширение: &str,
    указатель_захода: &mut usize,
    куча_пропусков: &rapidhash::fast::RapidHashSet<usize>,
) {
    *указатель_захода += 1;
    println!(
        "{} {}Завершено...",
        style(сообщение).bold().dim(),
        LOOKING_GLASS
    );

    // Создаем атомарные счетчики для каждого шаблона
    let атомарные_счетчики: Vec<AtomicUsize> =
        (0..re_образцы.len()).map(|_| AtomicUsize::new(0)).collect();

    let количество_шагов = re_образцы.len() * содержимое.len();
    let счетчик_внутренний = ProgressBar::new(количество_шагов as u64);
    let шаг_внутренний = AtomicU64::new(0);

    счетчик_внутренний.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    // Обрабатываем каждую строку параллельно
    содержимое
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, строка)| {
            if куча_пропусков.contains(&указатель) {
                // Пропускаем строку, но все равно считаем прогресс
                let шаги_для_этой_строки = re_образцы.len() as u64;
                шаг_внутренний.fetch_add(шаги_для_этой_строки, Ordering::Relaxed);
                счетчик_внутренний.inc(шаги_для_этой_строки);
                return;
            }

            // Сохраняем оригинальную строку для проверки
            //  let оригинальная_строка = строка.clone();

            for указатель_образца in 0..re_образцы.len() {
                let re_образец = &re_образцы[указатель_образца];

                if sz_найти(&строка, &искомое_слово[указатель_образца])
                {
                    let замененная_строка =
                        re_образец.replace_all(&строка, &замены[указатель_образца]);
                    let замененная_строка = замененная_строка.to_string();
                    if bytesum(&замененная_строка) != bytesum(&строка) {
                        // Увеличиваем атомарный счетчик
                        атомарные_счетчики[указатель_образца].fetch_add(1, Ordering::Relaxed);
                    }
                    // Заменяем строку
                    *строка = замененная_строка;
                }

                // Обновляем прогресс
                let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                счетчик_внутренний.set_position(текущий_шаг);
            }
        });

    // Копируем результаты из атомарных счетчиков
    for (i, атомарный) in атомарные_счетчики.iter().enumerate() {
        счётчик_словаря[i] += атомарный.load(Ordering::Relaxed);
    }

    счетчик_внутренний.finish_and_clear();
}
*/
pub fn замена_слов_через_кучу(
    словарь: &[Ячейка_словаря],
    содержимое: &mut [String],
    счётчик_словаря: &[AtomicUsize],
    сообщение: &str,
    расширение: &str,
    куча_пропусков: &rapidhash::fast::RapidHashSet<usize>,
    словарь_куча: &rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    этап: usize,
    указатель_содержимого: usize,
    количество_вложений: usize,
    вложенный_ли_файл_к_html: bool,
) {
    /*let spinner_style = ProgressStyle::with_template("{wide_msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
        let m = MultiProgress::new();
        let pb = m.add(ProgressBar::new(15));
        pb.set_style(spinner_style.clone());
    */
    //Создаем атомарные счетчики для каждого шаблона
    // let атомарные_счетчики: Vec<AtomicUsize> =
    //   (0..словарь.len()).map(|_| AtomicUsize::new(0)).collect();
    //слшком жрёт дохрена - нахрен
    /* let количество_шагов = словарь.len() * содержимое.len();
    let счетчик_внутренний = ProgressBar::new(количество_шагов as u64);
    let шаг_внутренний = AtomicU64::new(0);*/
    //выводить или нет
    /*if условие_вывода_хода(этап) && !вложенный_ли_файл_к_html {
        счетчик_внутренний.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg:.green}",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#>-"),
        );
        счетчик_внутренний.set_message(format!("{}", сообщение));
    } else {
        счетчик_внутренний.finish_and_clear();
        pb.finish_and_clear();
        m.clear().unwrap();
    }*/
    // Обрабатываем каждую строку параллельно
    содержимое
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, строка)| {
            if куча_пропусков.contains(&указатель) {
                // Пропускаем строку, но все равно считаем прогресс
                let шаги_для_этой_строки = словарь.len() as u64;
                //слшком жрёт дохрена - нахрен
                //шаг_внутренний.fetch_add(шаги_для_этой_строки, Ordering::Relaxed);
                //счетчик_внутренний.inc(шаги_для_этой_строки);
                return;
            }
            for (образец, куча_указателей) in словарь_куча.iter() {
                // let re_образец = &re_образцы[указатель_образца];
                //если образец из кучи есть в строке
                if sz_найти(&строка, &образец) {
                    //перебор укзаталей в куче от самого искомого слова (в котором удалено окончание)
                    for указатель_образца in куча_указателей.iter() {
                        //если больше чем 2 зачений в словаре - то поиск совпадения каждого каждого
                        if куча_указателей.len() > 2 {
                            //поиск уже образца точного в строке
                            if sz_найти(&строка, &словарь[*указатель_образца].искомое_слово)
                            {
                                let замененная_строка = &словарь
                                    [*указатель_образца]
                                    .re_образец
                                    .replace_all(&строка, &словарь[*указатель_образца].замена);
                                //
                                let замененная_строка = замененная_строка.to_string();
                                if замененная_строка.as_str() != строка.as_str()
                                {
                                    счётчик_словаря[*указатель_образца]
                                        .fetch_add(1, Ordering::Relaxed);
                                }
                                // Заменяем строку
                                *строка = замененная_строка;
                            }
                        }
                        //если 1-2 значения в ключе
                        else {
                            let замененная_строка = &словарь[*указатель_образца]
                                .re_образец
                                .replace_all(&строка, &словарь[*указатель_образца].замена);

                            let замененная_строка = замененная_строка.to_string();
                            if замененная_строка.as_str() != строка.as_str() {
                                // Увеличиваем атомарный счетчик
                                счётчик_словаря[*указатель_образца].fetch_add(1, Ordering::Relaxed);
                            }
                            // Заменяем строку
                            *строка = замененная_строка;
                        }
                    }
                }
                //слшком жрёт дохрена - нахрен
                // Обновляем прогресс
                //let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                //счетчик_внутренний.set_position(текущий_шаг);
            }
        });
    /*
    счетчик_внутренний.finish_and_clear();
    pb.finish_and_clear();
    m.clear().unwrap();*/

    // Копируем результаты из атомарных счетчиков
    /* атомарные_счетчики
    .iter()
    .enumerate()
    .for_each(|(указатель, число)| {
        счётчик_словаря[указатель].fetch_add(число.load(Ordering::Relaxed), Ordering::Relaxed); //
    });*/
    fn условие_вывода_хода(этап: usize) -> bool {
        //пока отменил вывод с указанием текущего этапа прохода слов, слишком быстро всё делает и в итоге чисто кроме мусора ничего нет
        if этап == 99 { true } else { false }
    }
}

//многопоточность
pub fn добавить_разделители(
    словарь_разделителей: &Словарь_разделителей,
    содержимое: &mut [String],
    сообщение: &str,
    расширение: &str,
    куча_пропусков: &rapidhash::fast::RapidHashSet<usize>,
    указатель_захода: &mut usize,

    счётчики_замен: &mut Arc<Счётчик_разделителей>,
    указатель_словаря_переносов: usize,
) {
    let mut условие_вывода_1: bool = false;
    let mut условие_вывода_2: bool = false;
    // Общее количество шагов для прогресса (если нужен)
    let общий_счёт_шагов = словарь_разделителей.ряд_1.len() * содержимое.len();
    let шаг_внутренний = AtomicUsize::new(0); // для отслеживания прогресса (опционально)

    // Параллельная обработка каждой строки
    содержимое
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель_строки, строка)| {
            if куча_пропусков.contains(&указатель_строки) {
                // Пропускаем строку, но все равно считаем прогресс
                // let шаги_для_этой_строки = словарь.len() as u64;
                // шаг_внутренний.fetch_add(шаги_для_этой_строки, Ordering::Relaxed);
                // счетчик_внутренний.inc(шаги_для_этой_строки);
                return;
            }
            // Перебираем все ячейки словаря последовательно (для текущей строки)
            'круговорот_главный: for (указатель_ячейки, ячейка) in
                словарь_разделителей.ряд_1.iter().enumerate()
            {
                let mut счётчик_раз: usize = 0;
                //
                /* if указатель_ячейки == 331 {
                    условие_вывода_2 = true;
                };*/
                // Проверяем наличие искомого слова (можно заменить на другое условие)
                //if sz_найти(строка, &ячейка.искомое_слово) {
                if ячейка.re_образец_для_замены.is_match(&строка) {
                    // Если есть совпадение с основным regex
                    // Проверяем, не попадает ли строка под исключения
                    //
                    let количество_совпадений: usize = ячейка
                        .re_образец_для_замены
                        .find_iter(&строка.clone())
                        .count();
                    //если не нашло - то следующее слово
                    if количество_совпадений == 0 {
                        continue 'круговорот_главный;
                    }
                    //
                    /*if количество_совпадений > 1 {
                        условие_вывода_1 = true;
                    }*/
                    //

                    //количество совпадений в стркое

                    //умный проход
                    for указатель_захода in 0..количество_совпадений
                    {
                        let строка2 = строка.to_string();
                        let все_совпадения: Vec<Match> =
                            ячейка.re_образец_для_замены.find_iter(&строка2).collect();
                        //если были изменения - то заного просчитать начало и конец совпадений в строке
                        /*if счётчик_раз>0{

                            все_совпадения =
                                ячейка.re_образец_для_замены.find_iter(&строка2).collect();

                            if   (ячейка.искомое_слово=="бое".to_string()) && указатель_строки==30 {
                                println!("счётчик раз больще 0");
                            }
                        }*/
                        //счётчик изменяемый - так как ходим им и бывает исключений уже нет
                        let число_захода: usize = указатель_захода - счётчик_раз;
                        // let число_захода:usize=указатель_захода;
                        //если исключение - следующий заход
                        if есть_ли_исключение(
                            &ячейка.ряд_re_исключений,
                            &все_совпадения[число_захода].as_str(),
                        ) {
                            continue;
                        }
                        //поиск совпадения по числу
                        if let Some(найденное_совпадение) = ячейка
                            .re_образец_для_замены
                            .captures(&все_совпадения[число_захода].as_str())
                        {
                            //
                            let замена: String = format!(
                                "{}-{}",
                                &найденное_совпадение[1], &найденное_совпадение[2]
                            );
                            //
                            let mut замененная_строка: String = строка.to_string();
                            // Проверяем, что индексы в пределах строки
                            // Получаем корректные границы символов
                            let начало: usize = все_совпадения[число_захода].start();
                            let конец: usize = все_совпадения[число_захода].end();
                            // Корректируем индексы до границ символов
                            let начало: usize = замененная_строка.floor_char_boundary(начало);
                            let конец: usize = замененная_строка.floor_char_boundary(конец);
                            //
                            замененная_строка.replace_range(начало..конец, &замена);
                            /* if (ячейка.искомое_слово=="бое".to_string()) && указатель_строки==30{
                                println!("_указатель_совпадения_re:|{}| _само_совпадение_re: {:?} ",число_захода,все_совпадения[число_захода]);
                                println!("нашло2: |{}|",строка);
                                println!("Счётчик раз[2]: {счётчик_раз}");
                            }*/
                            //
                            if замененная_строка != *строка {
                                счётчики_замен.подсчёт[указатель_ячейки]
                                    .fetch_add(1, Ordering::Relaxed);
                                счётчик_раз += 1;
                            }
                            // Обновляем строку
                            *строка = замененная_строка;
                        }
                    }
                    //

                    //
                    /*
                    //
                      let все_совпадения: Vec<Match> =
                          ячейка.re_образец_для_замены.find_iter(&строка).collect();
                      //все совпадения в виде слов в ряд
                      let mut все_совпадения_в_ряд: Vec<String> = все_совпадения
                          .iter()
                          .map(|совпадение_re| совпадение_re.as_str().to_string())
                          .collect();
                      //      перебор количества совпадений
                      'круговорот_совпадений: for (
                          совпадение_числитель,
                          совпадение_слово,
                      ) in
                          все_совпадения_в_ряд.iter().enumerate()
                      {
                          for исключение_ряда in ячейка.ряд_re_исключений.iter()
                          {
                              //если исключение - то следующее совпадение этого образца
                              //проверка
                              if исключение_ряда
                                  .is_match(&все_совпадения[совпадение_числитель].as_str())
                              {
                                  continue 'круговорот_совпадений;
                              }
                          }
                          // Выполняем замену
                          //количество совпадений найденного образца в строке
                          let все_совпадения: Vec<Match> =
                              ячейка.re_образец_для_замены.find_iter(строка.as_str()).collect();
                          //перебор всех совпадений
                          for (_указатель_совпадения_re, _само_совпадение_re) in
                              все_совпадения.iter().enumerate()
                          {
                              if _само_совпадение_re.as_str() != совпадение_слово.as_str()
                              {
                                  continue;
                              }

                              //вынимаем
                              let найденное_совпадение: Captures = ячейка
                                  .re_образец_для_замены
                                  .captures(&_само_совпадение_re.as_str())
                                  .unwrap();
                              //
                              let замена: String = format!("{}-{}", &найденное_совпадение[1], &найденное_совпадение[2]);
                              //
                              if sz_найти(&строка,"времябое-вогоо")&& ячейка.искомое_слово=="бое".to_string() {
                                  println!("_указатель_совпадения_re:|{}| _само_совпадение_re: {:?} ",_указатель_совпадения_re,_само_совпадение_re);
                                  println!("нашло2: |{}|",строка)
                              }
                              let mut замененная_строка: String = строка.clone();
                              // Проверяем, что индексы в пределах строки
                              // Получаем корректные границы символов
                              let начало: usize = _само_совпадение_re.start();
                              let конец: usize = _само_совпадение_re.end();
                              // Корректируем индексы до границ символов
                              let начало: usize = замененная_строка.floor_char_boundary(начало);
                              let конец: usize = замененная_строка.floor_char_boundary(конец);
                              //
                              замененная_строка.replace_range(начало..конец, &замена);
                              //
                              if  ячейка.искомое_слово=="бое".to_string()&& указатель_строки==30 {
                                  println!();
                                  println!("нашло # строки: {}: |{}|",указатель_строки,замененная_строка)
                              }
                              //sz_найти(&строка,"времябое-вогоо")&&
                              // Если строка действительно изменилась, увеличиваем счетчик
                              if замененная_строка != *строка {
                                  счётчики_замен.подсчёт[указатель_ячейки]
                                      .fetch_add(1, Ordering::Relaxed);
                                  счётчик_раз+=1;
                              }
                              // Обновляем строку
                              *строка = замененная_строка;
                          }
                          //
                          /*let замененная_строка: Cow<str> =ячейка.re_образец_для_замены.replace(строка,|caps: &Captures| {
                              format!("{}-{}", &caps[1],&caps[2])
                              //$1-$2
                          });*/
                          //
                          //
                          // let замененная_строка:String = замененная_строка.to_string();
                      }*/

                    // Обновляем прогресс (если нужно)
                    шаг_внутренний.fetch_add(1, Ordering::Relaxed);
                    // Например, можно вызывать внешний прогресс-бар: счетчик_внутренний.inc(1);
                }
            }
        });
}

//
pub fn убрать_переносы(
    //словарь: &[Ячейка_словаря],
    словарь_замен: &Словарь_Переносов,
    содержимое: &mut [String],
    //re_образцы: &[Regex],
    //содержимое: &mut [String],
    //замены: &[String],
    //счётчик_словаря: &mut [usize],
    //искомое_слово: &[String],
    сообщение: &str,
    расширение: &str,
    указатель_захода: &mut usize,
    mut счётчики_замен: &mut Arc<Счётчик_замен>,
    //куча_пропусков: &rapidhash::fast::RapidHashSet<usize>,
    указатель_словаря_переносов: usize,
) {
    //искомый знак переноса
    let знак_переноса: String = match указатель_словаря_переносов {
        0 => "-".to_string(),
        1 => "—".to_string(),
        2 => " - ".to_string(),
        _ => panic!(),
    };
    use crate::dictionary_0::проверка_ряда_regex;

    //если первый раз заходит - то проверить

    //подсчёт для видимого счётчика в окне
    let общий_счёт: usize = словарь_замен.целиковые.len()
        + словарь_замен.многобуквенные.len()
        + словарь_замен.трехбуквенные.len()
        + словарь_замен.двубуквенные.len()
        + словарь_замен.однобуквенные.len()
        + словарь_замен.исключения.len();

    //общий счёт
    //let количество_шагов = общий_счёт * содержимое.len();
    //let счетчик_внутренний = ProgressBar::new(количество_шагов as u64);
    //let шаг_внутренний = AtomicU64::new(0);

    /*счетчик_внутренний.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );*/

    // Обрабатываем каждую строку параллельно
    содержимое
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, строка)| {
            /* if куча_пропусков.contains(&указатель) {
                // Пропускаем строку, но все равно считаем прогресс
                let шаги_для_этой_строки = словарь_замен.len() as u64;
                шаг_внутренний.fetch_add(шаги_для_этой_строки, Ordering::Relaxed);
                счетчик_внутренний.inc(шаги_для_этой_строки);
                return;
            }*/

            // Сохраняем оригинальную строку для проверки
            //  let оригинальная_строка = строка.clone();
            //исключения
            if sz_найти(&строка, &знак_переноса) {
                for указатель_образца in 0..словарь_замен.исключения.len()
                {
                    let re_исключение =
                        &словарь_замен.исключения[указатель_образца].re_исключение;
                    let re_образец =
                        &словарь_замен.исключения[указатель_образца].re_образец_для_поиска;
                    let искомое_слово =
                        &словарь_замен.исключения[указатель_образца].искомое_слово;
                    let замена = &словарь_замен.исключения[указатель_образца].замена;
                    //if re_образец.is_match(&строка) /if sz_найти(&строка, &искомое_слово[указатель_образца])

                    if sz_найти(&строка, искомое_слово) {
                        //если есть буква перед переносом - тогда менять, перед числом нет
                        for re_само_исключение in re_исключение.iter() {
                            if !re_само_исключение.is_match(&строка) {
                                //  println!("изначальная строка: {строка}");
                                let замененная_строка = re_образец.replace_all(&строка, замена);
                                let замененная_строка = замененная_строка.to_string();
                                //    println!("заменённая строка: {замененная_строка}");
                                if bytesum(&замененная_строка) != bytesum(&строка)
                                {
                                    // Увеличиваем атомарный счетчик
                                    //println!("");
                                    // println!("исключение искомое слово найдено: {искомое_слово}");
                                    // println!("Строка до: {строка}");
                                    //  println!("");
                                    //  println!("Строка после: {замененная_строка}");
                                    //  println!("");
                                    счётчики_замен.исключения[указатель_образца]
                                        .fetch_add(1, Ordering::Relaxed);
                                    // счётчик_однобуквенные[указатель_образца].fetch_add(1, Ordering::Relaxed);
                                }
                                // Заменяем строку
                                *строка = замененная_строка;
                            }
                        }
                    }
                    //слшком жрёт дохрена - нахрен
                    // Обновляем прогресс
                    //let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                    //счетчик_внутренний.set_position(текущий_шаг);
                }
                //целиковые
                for указатель_образца in 0..словарь_замен.целиковые.len()
                {
                    let re_образец = &словарь_замен.целиковые[указатель_образца].re_образец;
                    let искомое_слово = &словарь_замен.целиковые[указатель_образца].искомое_слово;
                    let замена = &словарь_замен.целиковые[указатель_образца].замена;
                    //if re_образец.is_match(&строка) /if sz_найти(&строка, &искомое_слово[указатель_образца])
                    if sz_найти(&строка, искомое_слово) {
                        let замененная_строка = re_образец.replace_all(&строка, замена);
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.целиковые[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                            // счётчик_однобуквенные[указатель_образца].fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        *строка = замененная_строка;
                    }
                    //слшком жрёт дохрена - нахрен
                    // Обновляем прогресс
                    //let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                    //счетчик_внутренний.set_position(текущий_шаг);
                }
                //многобуквенные
                for указатель_образца in 0..словарь_замен.многобуквенные.len()
                {
                    let re_образец =
                        &словарь_замен.многобуквенные[указатель_образца].re_образец;
                    let искомое_слово =
                        &словарь_замен.многобуквенные[указатель_образца].искомое_слово;
                    let замена = &словарь_замен.многобуквенные[указатель_образца].замена;
                    //if re_образец.is_match(&строка)
                    if sz_найти(&строка, &искомое_слово) {
                        let замененная_строка = re_образец.replace_all(&строка, замена);
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.многобуквенные[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        *строка = замененная_строка;
                    }
                    //слшком жрёт дохрена - нахрен
                    // Обновляем прогресс
                    // let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                    // счетчик_внутренний.set_position(текущий_шаг);
                }
                //трехбуквенные
                for указатель_образца in 0..словарь_замен.трехбуквенные.len()
                {
                    let re_образец =
                        &словарь_замен.трехбуквенные[указатель_образца].re_образец;
                    // println!("образец №{указатель_образца}: {}",re_образец);
                    let замена = &словарь_замен.трехбуквенные[указатель_образца].замена;
                    let искомое_слово =
                        &словарь_замен.трехбуквенные[указатель_образца].искомое_слово;
                    //if re_образец.is_match(&строка)
                    if sz_найти(&строка, &искомое_слово) {
                        // println!("нашло двукбуквенное");
                        let замененная_строка = re_образец.replace_all(&строка, замена);
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.трехбуквенные[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        *строка = замененная_строка;
                    }
                    //слшком жрёт дохрена - нахрен
                    // Обновляем прогресс
                    //let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                    //счетчик_внутренний.set_position(текущий_шаг);
                }
                //двубуквенные
                for указатель_образца in 0..словарь_замен.двубуквенные.len()
                {
                    let re_образец = &словарь_замен.двубуквенные[указатель_образца].re_образец;
                    // println!("образец №{указатель_образца}: {}",re_образец);
                    let замена = &словарь_замен.двубуквенные[указатель_образца].замена;
                    let искомое_слово =
                        &словарь_замен.двубуквенные[указатель_образца].искомое_слово;
                    //if re_образец.is_match(&строка)
                    if sz_найти(&строка, &искомое_слово) {
                        // println!("нашло двукбуквенное");
                        let замененная_строка = re_образец.replace_all(&строка, замена);
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.двубуквенные[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        *строка = замененная_строка;
                    }
                    //слшком жрёт дохрена - нахрен
                    // Обновляем прогресс
                    //let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                    //счетчик_внутренний.set_position(текущий_шаг);
                }
                //однобуквенные
                for указатель_образца in 0..словарь_замен.однобуквенные.len()
                {
                    let re_образец =
                        &словарь_замен.однобуквенные[указатель_образца].re_образец;
                    let искомое_слово =
                        &словарь_замен.однобуквенные[указатель_образца].искомое_слово;
                    let замена = &словарь_замен.однобуквенные[указатель_образца].замена;
                    //if re_образец.is_match(&строка)
                    if sz_найти(&строка, &искомое_слово) {
                        let замененная_строка = re_образец.replace_all(&строка, замена);
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.однобуквенные[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        *строка = замененная_строка;
                    }
                    //слшком жрёт дохрена - нахрен
                    // Обновляем прогресс
                    //let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                    //счетчик_внутренний.set_position(текущий_шаг);
                }
            }
        });
    //println!("счётчики замен: {:?}",счётчики_замен.двубуквенные);
}
pub fn создать_словарь_разделителей() -> Словарь_разделителей {
    use std::default::Default;

    use crate::dictionary_0::проверка_ряда_regex;
    use crate::lib::Ячейка_замены_с_разделителями;
    let mut ряд_1: Словарь_разделителей = Словарь_разделителей {
        ряд_1: [
            Ячейка_замены_с_разделителями {
                искомое_слово: "мрако".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "казно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "сбое".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "тихо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "трудо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "трупо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "ясно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "мысле".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "светло".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "тёмно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "темно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "плано".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "платёже".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "платеже".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "гибко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "близко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "дально".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "чудо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "чуже".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "жёстко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "жестко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "восьми".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "семи".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "шести".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пяти".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "четырёх".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "четырех".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "трёх".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "трех".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "двух".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "грязно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "градо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "чисто".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "дву".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "одно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "недо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "девяти".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "десяти".to_string(),
                ..Default::default()
            },
            //
            Ячейка_замены_с_разделителями {
                искомое_слово: "работо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "под".to_string(),
                ряд_исключений: vec![
                    "подобн".to_string(),
                    "подобен".to_string(),
                    "подорв".to_string(),
                ],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "подор".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "над".to_string(),
                ряд_исключений: vec!["надежд".to_string()],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "широко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "нефте".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "газо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "металло".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "дерево".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "оптико".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "стекло".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "военно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "уравно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "равно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "просто".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "целе".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "сухо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "везде".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "земле".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "водо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пере".to_string(),
                ряд_исключений: vec!["передне".to_string()],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "передне".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "задне".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "прямо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "лево".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "право".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "здраво".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "благо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "жизне".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "законо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "все".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "полно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "средне".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "мелко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "крупно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "осново".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "добро".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "рас".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "при".to_string(),
                ряд_исключений: vec!["принят".to_string()],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "прежде".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пре".to_string(),
                ряд_исключений: vec!["пред".to_string(), "прежде".to_string()],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "между".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "едино".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пред".to_string(),
                ряд_исключений: vec!["предат".to_string()],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "тепло".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "крово".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "кратко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "ино".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "взрыво".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "мало".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "без".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "бес".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "громко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "скоро".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "быстро".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "долго".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "умо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "сверх".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "воздухо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "соот".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "бое".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "взаимо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "само".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "хитро".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "лже".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "противо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пожаро".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "самолето".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "самолёто".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "тазо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "прапра".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "миро".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "народо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "верхо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "ново".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "старо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "много".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "выше".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "раз".to_string(),
                ряд_исключений: vec!["разно".to_string()],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "разно".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "высоко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "низко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "родо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "узко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "муже".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "жено".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "чино".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "лизо".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "вино".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "члено".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "человеко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "огне".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "звуко".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "камне".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "после".to_string(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "идоло".to_string(),
                ..Default::default()
            },
        ],
    };
    use crate::lib::Возможности_ячейки_замены_с_разделителями;

    //
    //исключения с заглавной буквы
    //
    for ячейка_замены in ряд_1.ряд_1.iter_mut() {
        //заполнение оставшихся полей
        ячейка_замены.добавить_оставшиеся_поля();
        ячейка_замены.ряд_исключений = ячейка_замены
            .ряд_исключений
            .iter()
            .map(|ячейка| ячейка.to_case(Case::Lower))
            .collect();
        //собрать ряд RE исключений
        ячейка_замены.ряд_re_исключений = ячейка_замены.добавить_re_исключения_изнутри();
    }
    //
    let образцы_поиска_re_для_проверки: Vec<&Regex> = ряд_1
        .ряд_1
        .par_iter()
        .map(|ячейка| &ячейка.re_образец_для_поиска)
        .collect();
    //
    //
    //проверка образцов
    проверка_ряда_regex_разделителей(
        образцы_поиска_re_для_проверки,
        "проверка разделителей",
    );
    //
    return ряд_1;
}
pub fn создать_словарь_замен() -> Словарь_Переносов {
    use crate::dictionary_0::проверка_ряда_regex;
    let словарь_замен: Словарь_Переносов =
        создать_разделы_словаря_переносов();
    //
    //let словарь_второй
    поиск_повторов_re_словаря_замен(&словарь_замен);
    return словарь_замен;
}

pub fn создать_счётчик_словаря_замен(
    словарь_замен: &Словарь_Переносов,
) -> Arc<Счётчик_замен> {
    return Arc::new(Счётчик_замен {
        исключения: (0..словарь_замен.исключения.len())
            .map(|_| AtomicUsize::new(0))
            .collect(),
        однобуквенные: (0..словарь_замен.однобуквенные.len())
            .map(|_| AtomicUsize::new(0))
            .collect(),
        двубуквенные: (0..словарь_замен.двубуквенные.len())
            .map(|_| AtomicUsize::new(0))
            .collect(),
        трехбуквенные: (0..словарь_замен.трехбуквенные.len())
            .map(|_| AtomicUsize::new(0))
            .collect(),
        многобуквенные: (0..словарь_замен.многобуквенные.len())
            .map(|_| AtomicUsize::new(0))
            .collect(),
        целиковые: (0..словарь_замен.целиковые.len())
            .map(|_| AtomicUsize::new(0))
            .collect(),
    });
}

pub fn создать_счётчик_словаря_разделителей(
    словарь_замен: &Словарь_разделителей,
) -> Arc<Счётчик_разделителей> {
    return Arc::new(Счётчик_разделителей {
        подсчёт: (0..словарь_замен.ряд_1.len())
            .map(|_| AtomicUsize::new(0))
            .collect(),
        /*с_заглавной: (0..словарь_замен.ряд_1.len())
        .map(|_| AtomicUsize::new(0))
        .collect(),*/
    });
}
use crate::import::functions::преобразовать_слово_с_чертой_в_конце;
use crate::xlsx::import_xlsx::{
    найти_особые_знаки, обратно_убрать_особые_знаки
};
use convert_case::{Case, Casing};
use xml::Encoding::Default;

pub fn создать_второй_словарь_разделителей(
    mut словарь_изначальный: Словарь_разделителей,
) -> Словарь_разделителей {
    use crate::lib::Возможности_ячейки_замены_с_разделителями;
    словарь_изначальный
        .ряд_1
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.to_case(Case::Sentence);
            let новый_образец: String = format!(r#"\b{{start}}{}\w"#, ячейка.искомое_слово);
            ячейка.re_образец_для_поиска = Regex::new(&новый_образец).unwrap();
            //
            let временный_составной_ряд: (String, Vec<char>) =
                найти_особые_знаки(&ячейка.замена);
            //
            //ячейка.re_образец_для_замены=
            let новый_образец: String =
                format!(r#"\b{{start}}({})([\w]{{4,}})"#, ячейка.искомое_слово);
            ячейка.re_образец_для_замены = Regex::new(&новый_образец).unwrap();
            ячейка.замена = обратно_убрать_особые_знаки(
                преобразовать_слово_с_чертой_в_конце(
                    временный_составной_ряд.0.to_case(Case::Sentence),
                ),
            );
            //исключения с заглавной буквы
            ячейка.ряд_исключений = ячейка
                .ряд_исключений
                .iter()
                .map(|ячейка| ячейка.to_case(Case::Sentence))
                .collect();
            //собрать ряд RE исключений
            ячейка.ряд_re_исключений = ячейка.добавить_re_исключения_изнутри();
        });

    return словарь_изначальный;

    /*fn capitalize_first(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }*/
}
// все заглавные

pub fn создать_третий_словарь_разделителей(
    mut словарь_изначальный: Словарь_разделителей,
) -> Словарь_разделителей {
    use crate::lib::Возможности_ячейки_замены_с_разделителями;
    словарь_изначальный
        .ряд_1
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.to_case(Case::Upper);
            let новый_образец: String = format!(r#"\b{{start}}{}\w"#, ячейка.искомое_слово);
            ячейка.re_образец_для_поиска = Regex::new(&новый_образец).unwrap();
            //
            let временный_составной_ряд: (String, Vec<char>) =
                найти_особые_знаки(&ячейка.замена);
            //
            //ячейка.re_образец_для_замены=
            let новый_образец: String =
                format!(r#"\b{{start}}({})([\w]{{4,}})"#, ячейка.искомое_слово);
            ячейка.re_образец_для_замены = Regex::new(&новый_образец).unwrap();
            ячейка.замена = обратно_убрать_особые_знаки(
                преобразовать_слово_с_чертой_в_конце(
                    временный_составной_ряд.0.to_case(Case::Upper),
                ),
            );
            //исключения с заглавной буквы
            ячейка.ряд_исключений = ячейка
                .ряд_исключений
                .iter()
                .map(|ячейка| ячейка.to_case(Case::Upper))
                .collect();
            //собрать ряд RE исключений
            ячейка.ряд_re_исключений = ячейка.добавить_re_исключения_изнутри();
        });

    return словарь_изначальный;

    /*fn capitalize_first(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }*/
}

pub fn создать_второй_словарь_переносов(
    mut словарь_переносов: Словарь_Переносов,
) -> Словарь_Переносов {
    let замена_тире: Regex = Regex::new("-").unwrap();
    словарь_переносов
        .однобуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', "—");
        });
    словарь_переносов
        .многобуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', "—");
        });
    словарь_переносов
        .исключения
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', "—");
            let новый_образец = ячейка.re_образец_для_поиска.as_str().replace("-", "—");
            ячейка.re_образец_для_поиска = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', "—");
            ячейка.re_исключение = ячейка
                .re_исключение
                .iter()
                .map(|строка| Regex::new(&строка.replace("-", "—")).unwrap())
                .collect();
        });
    словарь_переносов
        .двубуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', "—");
        });
    словарь_переносов
        .трехбуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', "—");
        });
    словарь_переносов
        .целиковые
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', "—");
        });

    словарь_переносов
}
pub fn создать_третий_словарь_переносов(
    mut словарь_переносов: Словарь_Переносов,
) -> Словарь_Переносов {
    let замена_тире: Regex = Regex::new("-").unwrap();
    let на_что_заменять: String = " - ".to_string();
    словарь_переносов
        .однобуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', &на_что_заменять);
        });
    словарь_переносов
        .многобуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', &на_что_заменять);
        });
    словарь_переносов
        .исключения
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', &на_что_заменять);
            let новый_образец = ячейка
                .re_образец_для_поиска
                .as_str()
                .replace("-", &на_что_заменять);
            ячейка.re_образец_для_поиска = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', &на_что_заменять);
            ячейка.re_исключение = ячейка
                .re_исключение
                .iter()
                .map(|строка| Regex::new(&строка.replace("-", &на_что_заменять)).unwrap())
                .collect();
        });
    словарь_переносов
        .двубуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', &на_что_заменять);
        });
    словарь_переносов
        .трехбуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', &на_что_заменять);
        });
    словарь_переносов
        .целиковые
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace('-', &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace('-', &на_что_заменять);
        });

    словарь_переносов
}
pub fn создать_счётчики_словаря(
    полный_словарь: &Полный_Словарь,
) -> Arc<Счётчики_Словаря> {
    return Arc::new(Счётчики_Словаря {
        простое: (0..полный_словарь.простое.len())
            .into_par_iter()
            .map(|_| AtomicUsize::new(0))
            .collect(),
        составное: (0..полный_словарь.составное.len())
            .into_par_iter()
            .map(|_| AtomicUsize::new(0))
            .collect(),
        составное_важное: (0..полный_словарь.составное_важное.len())
            .into_par_iter()
            .map(|_| AtomicUsize::new(0))
            .collect(),
        вездесущее: (0..полный_словарь.вездесущее.len())
            .into_par_iter()
            .map(|_| AtomicUsize::new(0))
            .collect(),
        //неизменное
        неизменное: (0..полный_словарь.неизменное.len())
            .into_par_iter()
            .map(|_| AtomicUsize::new(0))
            .collect(),
        огласовки: (0..полный_словарь.огласовки.len())
            .into_par_iter()
            .map(|_| AtomicUsize::new(0))
            .collect(),
        //неизменное
        неизменное_длинное: (0..полный_словарь.неизменное_длинное.len())
            .into_par_iter()
            .map(|_| AtomicUsize::new(0))
            .collect(),
        //неизменное
        неизменное_короткое: (0..полный_словарь.неизменное_короткое.len())
            .into_par_iter()
            .map(|_| AtomicUsize::new(0))
            .collect(),
        //
        запятые: (0..полный_словарь.запятые.len())
            .into_par_iter()
            .map(|_| AtomicUsize::new(0))
            .collect(),
    });
}

/*
pub fn проверка_ряда_regex_замен2(re_ряд: impl AsRef<[Regex]>, сообщение: &str) {
    let ряд = re_ряд.as_ref();
    let куча: rapidhash::fast::RapidHashSet<String> = (0..ряд.len())
        .into_par_iter()
        .flat_map(|i| {
            let mut куча_2: rapidhash::fast::RapidHashSet<String>=rapidhash::fast::RapidHashSet::with_hasher(RandomState::default());
            if !sz_найти(&ряд[i].to_string(),"$") {
                //куча_2.insert(format!("Regex нет знака окончания слова $: {}", ряд[i]))
                куча_2.insert(format!("Regex нет знака окончания слова $: {}", ряд[i]));
            }
            let повторы:rapidhash::fast::RapidHashSet<String>=((i + 1)..ряд.len()).into_par_iter().filter_map(move |j| {
                if ряд[i].as_str() == ряд[j].as_str() {
                    Some(format!("есть совпадение Regex: {}", ряд[i]))
                } else {
                    None
                }
            }).collect();
            куча_2.extend(повторы);
            куча_2
        })
        .collect();
    for слово in куча.iter() {
        println!("длина кучи: {}", куча.len());
        println!("{} : {}", сообщение, слово)
    }
}

 */

pub fn проверка_ряда_regex_замен(
    ряд: Vec<&Regex>,
    //re_ряд: impl AsRef<[Regex]>,
    сообщение: &str,
) {
    //let ряд = re_ряд.as_ref();
    let куча: rapidhash::fast::RapidHashSet<String> = (0..ряд.len())
        .into_par_iter()
        .flat_map(|i| {
            let mut куча_2: rapidhash::fast::RapidHashSet<String> =
                rapidhash::fast::RapidHashSet::default();

            // Проверка на отсутствие $
            // if !ряд[i].as_str().contains('$') {
            if !sz_найти(&ряд[i].to_string(), r"\b{end}") {
                куча_2.insert(format!("Regex нет знака окончания слова $: {}", ряд[i]));
            }

            // Проверка на дубликаты
            let повторы: rapidhash::fast::RapidHashSet<String> = (0..ряд.len())
                .into_par_iter()
                .filter(|j| *j != i)
                .filter_map(|j| {
                    if ряд[i].as_str() == ряд[j].as_str() {
                        Some(format!("есть совпадение Regex (Замены) : {}", ряд[i]))
                    } else {
                        None
                    }
                })
                .collect();

            куча_2.extend(повторы);
            куча_2
                .into_iter()
                .collect::<rapidhash::fast::RapidHashSet<String>>()
        })
        .collect();

    if !куча.is_empty() {
        println!("длина кучи: {}", куча.len());
        for слово in &куча {
            println!("{} : {}", сообщение, слово);
        }
    }
}

pub fn проверка_ряда_regex_разделителей(
    ряд: Vec<&Regex>, сообщение: &str
) {
    //let ряд = re_ряд.as_ref();
    let куча: rapidhash::fast::RapidHashSet<String> = (0..ряд.len())
        .into_par_iter()
        .flat_map(|i| {
            let mut куча_2: rapidhash::fast::RapidHashSet<String> =
                rapidhash::fast::RapidHashSet::default();

            // Проверка на отсутствие $
            // if !ряд[i].as_str().contains('$') {
            if !sz_найти(&ряд[i].to_string(), r"\b{start}") {
                куча_2.insert(format!(
                    r##"Regex нет знака начала слова \b{{start}}: {}"##,
                    ряд[i]
                ));
            }

            // Проверка на дубликаты
            let повторы: rapidhash::fast::RapidHashSet<String> = (0..ряд.len())
                .into_par_iter()
                .filter(|j| *j != i)
                .filter_map(|j| {
                    if ряд[i].as_str() == ряд[j].as_str() {
                        Some(format!("есть совпадение Regex: {}", ряд[i]))
                    } else {
                        None
                    }
                })
                .collect();

            куча_2.extend(повторы);
            куча_2
                .into_iter()
                .collect::<rapidhash::fast::RapidHashSet<String>>()
        })
        .collect();

    if !куча.is_empty() {
        println!("длина кучи: {}", куча.len());
        for слово in &куча {
            println!("{} : {}", сообщение, слово);
        }
    }
}

pub fn добавить_слова_с_окончаниями() {
    use std::default::Default;

    pub struct окончания {
        pub щ: [String; 17],
    }
    impl Default for окончания {
        fn default() -> Self {
            Self {
                щ: [
                    "щ",
                    "ща",
                    "щая",
                    "щую",
                    "ще",
                    "щем",
                    "щему",
                    "щего",
                    "щее",
                    "щей",
                    "щесть",
                    "щестью",
                    "щести",
                    "щестью",
                    "щесстям",
                    "щесстями",
                    "щесстях",
                ]
                .map(String::from),
            }
        }
    }
}

pub fn есть_ли_исключение(
    ряд_re_исключений: &Vec<Regex>,
    совпадение_re_в_строке: &str,
) -> bool {
    for исключение_ряда in ряд_re_исключений.iter() {
        //если исключение - то следующее совпадение этого образца
        //проверка
        if исключение_ряда.is_match(&совпадение_re_в_строке) {
            return true;
        }
    }
    return false;
}

fn создать_разделы_словаря_переносов() -> Словарь_Переносов {
    let однобуквенные_ряд: [String;
        lib::СЛОВАРЬ_ПЕРЕНОСОВ_ОДНОБУКВЕННЫЕ] = [
        "-о".to_string(),
        "-а".to_string(),
        "-ь".to_string(),
        "-ы".to_string(),
        "-и".to_string(),
        "-ъ".to_string(),
        "-у".to_string(),
    ];

    let многобуквенные_ряд: [String;
        lib::СЛОВАРЬ_ПЕРЕНОСОВ_МНОГОБУКВЕННЫЕ] = [
        "-ройства ".to_string(),
        "-вязывающего ".to_string(),
        "-ближенный ".to_string(),
        "-стое".to_string(),
        "-ному".to_string(),
        "-мыми".to_string(),
        "-sign".to_string(),
        "-utes".to_string(),
        "-lete".to_string(),
        "-tium".to_string(),
        "-ющая".to_string(),
        "-нове".to_string(),
        "-дены".to_string(),
        "-дить".to_string(),
        "-лась".to_string(),
        "-брос".to_string(),
        "-фере".to_string(),
        "-тоды".to_string(),
        "-стей".to_string(),
        "-ской".to_string(),
        "-нием".to_string(),
        "-ский".to_string(),
        "-дена".to_string(),
        "-жима".to_string(),
        "-рьер".to_string(),
        "-верх".to_string(),
        "-стера".to_string(),
        "-рами".to_string(),
        "-дела".to_string(),
        "-ходя".to_string(),
        "-руте".to_string(),
        "-ряют".to_string(),
        "-дует".to_string(),
        "-дачи".to_string(),
        "-теке".to_string(),
        /*
            "-либо".to_string(),
        */
        "-чить".to_string(),
        "-манд".to_string(),
        "-дать".to_string(),
        "-иумы".to_string(),
        "-ования".to_string(),
        "-овать".to_string(),
        "-иями".to_string(),
        "-ующие".to_string(),
        "-ующая".to_string(),
        "-ующий".to_string(),
        "-ующих".to_string(),
        "-уется".to_string(),
        "-уются".to_string(),
        "-ичную".to_string(),
        "-ичных".to_string(),
        "-ного".to_string(),
        "-ость".to_string(),
        "-ости".to_string(),
        "-остью".to_string(),
        "-нные".to_string(),
        "-нного".to_string(),
        "-нный".to_string(),
        "-нных".to_string(),
        "-уете".to_string(),
    ];
    let трехбуквенные_ряд: [String;
        lib::СЛОВАРЬ_ПЕРЕНОСОВ_ТРЕХБУКВЕННЫЕ] = [
        "-ков".to_string(),
        "-щий".to_string(),
        "-дят".to_string(),
        "-ter".to_string(),
        "-tus".to_string(),
        "-tom".to_string(),
        "-ции".to_string(),
        "-кам".to_string(),
        "-тём".to_string(),
        "-щью".to_string(),
        "-лом".to_string(),
        "-дан".to_string(),
        "-ста".to_string(),
        "-тия".to_string(),
        "-дой".to_string(),
        "-вая".to_string(),
        "-ния".to_string(),
        "-лон".to_string(),
        "-рых".to_string(),
        "-рый".to_string(),
        "-мые".to_string(),
        "-щем".to_string(),
        "-ний".to_string(),
        "-зок".to_string(),
        "-тем".to_string(),
        "-ные".to_string(),
        "-нию".to_string(),
        "-шин".to_string(),
        "-тый".to_string(),
        "-нюю".to_string(),
        "-гда".to_string(),
        "-бой".to_string(),
        "-вые".to_string(),
        "-дов".to_string(),
        "-тов".to_string(),
        "-пей".to_string(),
        "-мый".to_string(),
        "-nal".to_string(),
        "-щие".to_string(),
        "-вой".to_string(),
        "-ром".to_string(),
        "-мер".to_string(),
        "-них".to_string(),
        "-кие".to_string(),
        "-чет".to_string(),
        "-ект".to_string(),
        "-жет".to_string(),
        "-ком".to_string(),
        "-вил".to_string(),
        "-тым".to_string(),
        "-ких".to_string(),
        "-вым".to_string(),
        "-зом".to_string(),
        "-рой".to_string(),
        "-чек".to_string(),
        "-той".to_string(),
        "-гут".to_string(),
        "-ние".to_string(),
        "-ных".to_string(),
        "-кой".to_string(),
        "-ала".to_string(),
        "-уют".to_string(),
        "-еям".to_string(),
        "-нат".to_string(),
        "-иев".to_string(),
        "-иал".to_string(),
        "-ием".to_string(),
        "-иум".to_string(),
        "-ыми".to_string(),
        "-чим".to_string(),
        "-ика".to_string(),
        "-ику".to_string(),
        "-ики".to_string(),
        "-ать".to_string(),
        "-ять".to_string(),
        "-ным".to_string(),
        "-еть".to_string(),
        "-лен".to_string(),
        "-иям".to_string(),
        "-дом".to_string(),
        "-sor".to_string(),
        "-уум".to_string(),
        "-уем".to_string(),
        "-ким".to_string(),
        "-ешь".to_string(),
        "-ишь".to_string(),
        "-ток".to_string(),
        "-ете".to_string(),
        "-ите".to_string(),
        "-ует".to_string(),
        "-яла".to_string(),
        "-али".to_string(),
        "-яли".to_string(),
        "-ола".to_string(),
        "-ела".to_string(),
        "-оли".to_string(),
        "-ели".to_string(),
        "-ула".to_string(),
        "-ули".to_string(),
        "-ами".to_string(),
        "-еми".to_string(),
        "-емя".to_string(),
        "-ёте".to_string(),
        "-чие".to_string(),
        "-сте".to_string(),
        "-ёшь".to_string(),
        "-том".to_string(),
        "-ого".to_string(),
        "-ций".to_string(),
        "-жен".to_string(),
        "-ому".to_string(),
        "-дач".to_string(),
        "-иях".to_string(),
        "-ией".to_string(),
        "-умя".to_string(),
        "-ими".to_string(),
        "-тор".to_string(),
        "-рые".to_string(),
        "-сти".to_string(),
        "-чае".to_string(),
        "-вод".to_string(),
        "-лов".to_string(),
        "-кое".to_string(),
    ];
    let двубуквенные_ряд: [String;
        lib::СЛОВАРЬ_ПЕРЕНОСОВ_ДВУБУКВЕННЫЕ] = [
        "-ца".to_string(),
        "-сы".to_string(),
        "-er".to_string(),
        "-мы".to_string(),
        "-ры".to_string(),
        "-ра".to_string(),
        "-ты".to_string(),
        "-ка".to_string(),
        "-ло".to_string(),
        "-жа".to_string(),
        "-та".to_string(),
        "-ли".to_string(),
        "-ея".to_string(),
        "-еи".to_string(),
        "-ях".to_string(),
        "-ев".to_string(),
        "-ки".to_string(),
        "-да".to_string(),
        "-ых".to_string(),
        "-ям".to_string(),
        "-ии".to_string(),
        "-ия".to_string(),
        "-ся".to_string(),
        "-ая".to_string(),
        "-яя".to_string(),
        "-ое".to_string(),
        "-ее".to_string(),
        /*
            "-ой".to_string(),
        */
        "-ые".to_string(),
        "-ий".to_string(),
        "-ем".to_string(),
        "-им".to_string(),
        "-ет".to_string(),
        "-ит".to_string(),
        "-ут".to_string(),
        "-ру".to_string(),
        "-ют".to_string(),
        "-ят".to_string(),
        "-ял".to_string(),
        "-ол".to_string(),
        "-ел".to_string(),
        "-ул".to_string(),
        "-ам".to_string(),
        "-ас".to_string(),
        "-ах".to_string(),
        "-ко".to_string(),
        "-её".to_string(),
        "-ей".to_string(),
        "-ех".to_string(),
        "-ею".to_string(),
        "-ёт".to_string(),
        "-ёх".to_string(),
        "-ие".to_string(),
        "-их".to_string(),
        "-ию".to_string(),
        "-но".to_string(),
        "-ми".to_string(),
        "-мя".to_string(),
        "-ов".to_string(),
        "-оё".to_string(),
        "-см".to_string(),
        "-ум".to_string(),
        "-уя".to_string(),
        "-ух".to_string(),
        "-ую".to_string(),
        "-шь".to_string(),
        "-ны".to_string(),
        "-пи".to_string(),
        "-па".to_string(),
    ];
    let целиковые_ряд: [String; lib::СЛОВАРЬ_ПЕРЕНОСОВ_ЦЕЛИКОВЫЕ] = [
        "-валентных".to_string(),
        "-поминающих".to_string(),
        "-зации".to_string(),
        "-денции".to_string(),
        "-личаются".to_string(),
        "-ровать".to_string(),
        "-тельными".to_string(),
        "-рифмический".to_string(),
        "-рительными".to_string(),
        "-лучила".to_string(),
        "-пульсный".to_string(),
        "-менными".to_string(),
        "-правленный".to_string(),
        "-зится".to_string(),
        "-дификацию".to_string(),
        "-ляться".to_string(),
        "-рительной".to_string(),
        "-зических".to_string(),
        "-вается".to_string(),
        "-корректности".to_string(),
        "-руется".to_string(),
        "-совано".to_string(),
        "-турой".to_string(),
        "-пустимого".to_string(),
        "-стовый".to_string(),
        "-стояние".to_string(),
        "-ствами".to_string(),
        "-гическую".to_string(),
        "-шинного".to_string(),
        "-матном".to_string(),
        "-значены".to_string(),
        "-нальные".to_string(),
        "-крепленные".to_string(),
        "-тимальности".to_string(),
        "-гональных".to_string(),
        "-чезнут".to_string(),
        "-кание".to_string(),
        "-гаться".to_string(),
        "-зируя".to_string(),
        "-рячими".to_string(),
        "-ливаемое".to_string(),
        "-лагаемый".to_string(),
        "-ритета".to_string(),
        "-почтительный".to_string(),
        "-ляющее".to_string(),
        "-нейкой".to_string(),
        "-хождении".to_string(),
        "-исходит".to_string(),
        "-метров".to_string(),
        "-ства".to_string(),
        "-ровой".to_string(),
        "-знаку".to_string(),
        "-числены".to_string(),
        "-рованы".to_string(),
        "-межуточных".to_string(),
        "-гласование".to_string(),
        "-обходимое".to_string(),
        "-новления".to_string(),
        "-ских".to_string(),
        "-данса".to_string(),
        "-фектов".to_string(),
        "-редач".to_string(),
        "-нитные".to_string(),
        "-ключается".to_string(),
        "-ментов".to_string(),
        "-граммный".to_string(),
        "-вания".to_string(),
        "-шений".to_string(),
        "-никло".to_string(),
        "-чиком".to_string(),
        "-чатных".to_string(),
        "-полняются".to_string(),
        "-нелей".to_string(),
        "-торые".to_string(),
        "-тально".to_string(),
        "-менно".to_string(),
        "-торая".to_string(),
        "-раммного".to_string(),
        "-мендуется".to_string(),
        "-крытый".to_string(),
        "-тивным".to_string(),
        "-манды".to_string(),
        "-тронной".to_string(),
        "-численных".to_string(),
        "-ленную".to_string(),
        "-стемный".to_string(),
        "-ческих".to_string(),
        "-тура".to_string(),
        "-ждений".to_string(),
        "-шемся".to_string(),
        "-мента".to_string(),
        "-мандой".to_string(),
        "-тинные".to_string(),
        "-нель".to_string(),
        "-сутствует".to_string(),
        "-симо".to_string(),
        "-пени".to_string(),
        "-тельно".to_string(),
        "-чанию".to_string(),
        "-ческая".to_string(),
        "-бирать".to_string(),
        "-единитель".to_string(),
        "-зуемся".to_string(),
        "-ветствующие".to_string(),
        "-матическая".to_string(),
        "-нентов".to_string(),
        "-нала".to_string(),
        "-тистические".to_string(),
        "-стимо".to_string(),
        "-жителем".to_string(),
        "-товых".to_string(),
        "-цессе".to_string(),
        "-екта".to_string(),
        "-новлены".to_string(),
        "-рования".to_string(),
        "-раметры".to_string(),
        "-чески".to_string(),
        "-брав".to_string(),
        "-реноса".to_string(),
        "-зультаты".to_string(),
        "-ходных".to_string(),
        "-тырех".to_string(),
        "-кать".to_string(),
        "-мент".to_string(),
        "-штаба".to_string(),
        "-местно".to_string(),
        "-ления".to_string(),
        "-тактные".to_string(),
        "-таллизации".to_string(),
        "-нить".to_string(),
        "-ветствующим".to_string(),
        "-единения".to_string(),
        "-вать".to_string(),
        "-тически".to_string(),
        "-дами".to_string(),
        "-борочно".to_string(),
        "-веден".to_string(),
        "-ражает".to_string(),
        "-ством".to_string(),
        "-тора".to_string(),
        "-кусом".to_string(),
        "-лучить".to_string(),
        "-вание".to_string(),
        "-рантирует".to_string(),
        "-менных".to_string(),
        "-ствующим".to_string(),
        "-тронных".to_string(),
        "-логического".to_string(),
        "-рину".to_string(),
        "-нент".to_string(),
        "-тива".to_string(),
        "-нений".to_string(),
        "-ченных".to_string(),
        "-ченный".to_string(),
        "-рации".to_string(),
        "-митивов".to_string(),
        "-щение".to_string(),
        "-щего".to_string(),
        "-виша".to_string(),
        "-ление".to_string(),
        "-рибуты".to_string(),
        "-понент".to_string(),
        "-понента".to_string(),
        "-норамирования".to_string(),
        "-можно".to_string(),
        "-стра".to_string(),
        "-изведен".to_string(),
        "-бранному".to_string(),
        "-вится".to_string(),
        "-скую".to_string(),
        "-струкция".to_string(),
        "-торых".to_string(),
        "-веденных".to_string(),
        "-сколько".to_string(),
        "-ются".to_string(),
        "-ствуют".to_string(),
        "-павшие".to_string(),
        "-верстия".to_string(),
        "-ванные".to_string(),
        "-реходных".to_string(),
        "-слойные".to_string(),
        "-водится".to_string(),
        "-вами".to_string(),
        "-митивы".to_string(),
        "-пользуемых".to_string(),
        "-няться".to_string(),
        "-дартов".to_string(),
        "-ность".to_string(),
        "-ленных".to_string(),
        "-пусках".to_string(),
        "-бавления".to_string(),
        "-дактировать".to_string(),
        "-тический".to_string(),
        "-дактор".to_string(),
        "-ретащим".to_string(),
        "-зицию".to_string(),
        "-рения".to_string(),
        "-зателя".to_string(),
        "-затель".to_string(),
        "-водами".to_string(),
        "-кладка".to_string(),
        "-деления".to_string(),
        "-ражения".to_string(),
        "-телем".to_string(),
        "-садочных".to_string(),
        "-дактора".to_string(),
        "-ченной".to_string(),
        "-распознанный".to_string(),
        "-моугольный".to_string(),
        "-циями".to_string(),
        "-тированный".to_string(),
        "-варительно".to_string(),
        "-емость".to_string(),
        "-ваться".to_string(),
        "-когда".to_string(),
        "-ответствии".to_string(),
        "-этому".to_string(),
        "-слеживания".to_string(),
        "-рирует".to_string(),
        "-сения".to_string(),
        "-ниями".to_string(),
        "-структивными".to_string(),
        "-ствия".to_string(),
        "-единять".to_string(),
        "-шения".to_string(),
        "-изводить".to_string(),
        "-жимах".to_string(),
        "-чайшее".to_string(),
        "-ношении".to_string(),
        "-ровки".to_string(),
        "-изводиться".to_string(),
        "-бирает".to_string(),
        "-ностями".to_string(),
        "-виши".to_string(),
        "-тивизации".to_string(),
        "-личных".to_string(),
        "-ложение".to_string(),
        "-тивной".to_string(),
        "-логических".to_string(),
        "-нивает".to_string(),
        "-слойной".to_string(),
        "-нимается".to_string(),
        "-тельного".to_string(),
        "-вость".to_string(),
        "-сматриваются".to_string(),
        "-суждений".to_string(),
        "-дарственное".to_string(),
        "-чайное".to_string(),
        "-ниченные".to_string(),
        "-ветить".to_string(),
    ];
    //
    let исключения: [Ячейка_замены_с_исключением;
        lib::СЛОВАРЬ_ПЕРЕНОСОВ_ИСКЛЮЧЕНИЯ] = [
        Ячейка_замены_с_исключением {
            искомое_слово: "-я".to_string(),
            замена: "я".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-я\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)\d-я\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-го".to_string(),
            замена: "го".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-го\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)\d-го\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-е".to_string(),
            замена: "е".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-е\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)\d-е\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ю".to_string(),
            замена: "ю".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ю\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)\d-ю\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-на".to_string(),
            замена: "на".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-на\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)-на-").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-му".to_string(),
            замена: "му".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-му\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)([\d,%])-му\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ой".to_string(),
            замена: "ой".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ой\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)([\d%])-ой\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ное".to_string(),
            замена: "ное".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ное\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)%-ное\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ной".to_string(),
            замена: "ной".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ной\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)%-ной\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ную".to_string(),
            замена: "ную".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ную\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)%-ную\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-line".to_string(),
            замена: "line".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-line\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)empty-line\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ментального".to_string(),
            замена: "ментального".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ментального\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)-ментального\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ная".to_string(),
            замена: "ная".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ная\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)%-ная\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ный".to_string(),
            замена: "ный".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ный\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)%-ный\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ый".to_string(),
            замена: "ый".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ый\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)\d-ый\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ым".to_string(),
            замена: "ым".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ым\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)\d-ым\b{end}").unwrap()],
        },
        Ячейка_замены_с_исключением {
            искомое_слово: "-ом".to_string(),
            замена: "ом".to_string(),
            re_образец_для_поиска: Regex::new(r"(?i)-ом\b{end}").unwrap(),
            re_исключение: vec![Regex::new(r"(?i)\d-ом\b{end}").unwrap()],
        },
    ];
    //

    return Словарь_Переносов {
        однобуквенные:
        привести_ряд_сло_словаря_переносов_в_стопку_строгую(
                однобуквенные_ряд,
            ),
        многобуквенные:
        привести_ряд_сло_словаря_переносов_в_стопку_строгую(
                многобуквенные_ряд,
            ),
        трехбуквенные:
        привести_ряд_сло_словаря_переносов_в_стопку_строгую(
                трехбуквенные_ряд,
            ),
        двубуквенные:
        привести_ряд_сло_словаря_переносов_в_стопку_строгую(
                двубуквенные_ряд,
            ),
        целиковые:
        привести_ряд_сло_словаря_переносов_в_стопку_строгую(
                целиковые_ряд,
            ),
        исключения: исключения,
    };
}
//
pub fn привести_ряд_сло_словаря_переносов_в_стопку_строгую<const N: usize,>
(
    ряд: [String;N],
) -> [lib::Ячейка_замены; N] {
    use std::default::Default;
    //Default
    let mut ряд_итоговый: [lib::Ячейка_замены; N] = std::array::from_fn(|_| Default::default());
    //
    for (указатель, слово) in ряд.into_iter().enumerate() {
        //
        let ряд_знаков: Vec<char> = слово.chars().collect();
        let замена: String = ряд_знаков[1..].iter().collect::<String>();
        //
        ряд_итоговый[указатель].re_образец =
            Regex::new(&format!(r##"(?i)\b{{end}}{}\b{{end}}"##, слово)).unwrap();
        ряд_итоговый[указатель].замена = замена;
        ряд_итоговый[указатель].искомое_слово = слово;
    }
    //
    return ряд_итоговый;
}
fn поиск_повторов_re_словаря_замен(
    словарь_замен: &lib::Словарь_Переносов,
) {
    let исключения: Vec<&Regex> = словарь_замен
        .исключения
        .par_iter()
        .map(|ячейка| &ячейка.re_образец_для_поиска)
        .collect();
    let целиковые: Vec<&Regex> = словарь_замен
        .целиковые
        .par_iter()
        .map(|ячейка| &ячейка.re_образец)
        .collect();
    let трёхбуквенные: Vec<&Regex> = словарь_замен
        .трехбуквенные
        .par_iter()
        .map(|ячейка| &ячейка.re_образец)
        .collect();
    let двубуквенные: Vec<&Regex> = словарь_замен
        .двубуквенные
        .par_iter()
        .map(|ячейка| &ячейка.re_образец)
        .collect();
    let многоуквенные: Vec<&Regex> = словарь_замен
        .многобуквенные
        .par_iter()
        .map(|ячейка| &ячейка.re_образец)
        .collect();
    let однобуквенные: Vec<&Regex> = словарь_замен
        .однобуквенные
        .par_iter()
        .map(|ячейка| &ячейка.re_образец)
        .collect();

    //проверка образцов
    проверка_ряда_regex_замен(трёхбуквенные, "проверка замен трёхбуквенные");
    проверка_ряда_regex_замен(двубуквенные, "проверка замен двубуквенные");
    проверка_ряда_regex_замен(многоуквенные, "проверка замен многобуквенные");
    проверка_ряда_regex_замен(однобуквенные, "проверка замен однобуквенные");
    проверка_ряда_regex_замен(целиковые, "проверка замен целиковые");
    проверка_ряда_regex_замен(исключения, "проверка замен исключения");
}
