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
    self, Быстрый_Словарь, Полный_Словарь, Словарь_разделителей, Счётчик_замен,
    Счётчик_разделителей, Ячейка_словаря,
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
    let spinner_style = ProgressStyle::with_template("{wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(15));
    pb.set_style(spinner_style.clone());

    //Создаем атомарные счетчики для каждого шаблона
    // let атомарные_счетчики: Vec<AtomicUsize> =
    //   (0..словарь.len()).map(|_| AtomicUsize::new(0)).collect();

    let количество_шагов = словарь.len() * содержимое.len();
    let счетчик_внутренний = ProgressBar::new(количество_шагов as u64);
    let шаг_внутренний = AtomicU64::new(0);
    //выводить или нет
    if условие_вывода_хода(этап) && !вложенный_ли_файл_к_html {
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
    }
    // Обрабатываем каждую строку параллельно
    содержимое
        .par_iter_mut()
        .enumerate()
        .for_each(|(указатель, строка)| {
            if куча_пропусков.contains(&указатель) {
                // Пропускаем строку, но все равно считаем прогресс
                let шаги_для_этой_строки = словарь.len() as u64;
                шаг_внутренний.fetch_add(шаги_для_этой_строки, Ordering::Relaxed);
                счетчик_внутренний.inc(шаги_для_этой_строки);
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

                // Обновляем прогресс
                let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                счетчик_внутренний.set_position(текущий_шаг);
            }
        });
    счетчик_внутренний.finish_and_clear();
    pb.finish_and_clear();
    m.clear().unwrap();

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
    let количество_шагов = общий_счёт * содержимое.len();
    //let счетчик_внутренний = ProgressBar::new(количество_шагов as u64);
    let шаг_внутренний = AtomicU64::new(0);

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
                    // Обновляем прогресс
                    let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
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
                    // Обновляем прогресс
                    let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
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
                    // Обновляем прогресс
                    let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
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
                    // Обновляем прогресс
                    let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
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
                    // Обновляем прогресс
                    let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
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
                    // Обновляем прогресс
                    let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                    //счетчик_внутренний.set_position(текущий_шаг);
                }
            }
        });
    //println!("счётчики замен: {:?}",счётчики_замен.двубуквенные);
}
pub fn создать_словарь_разделителей() -> Словарь_разделителей {
    use crate::dictionary_0::проверка_ряда_regex;
    use crate::lib::Ячейка_замены_с_разделителями;
    let mut ряд_1: Словарь_разделителей = Словарь_разделителей {
        ряд_1: [
            Ячейка_замены_с_разделителями {
                искомое_слово: "тихо".to_string(),
                замена: "тихо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}тихо(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(тихо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "трудо".to_string(),
                замена: "трудо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}трудо(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(трудо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "ясно".to_string(),
                замена: "ясно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}ясно(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(ясно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "мысле".to_string(),
                замена: "мысле-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}мысле(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(мысле)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "светло".to_string(),
                замена: "светло-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}светло(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(светло)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "тёмно".to_string(),
                замена: "тёмно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}тёмно(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(тёмно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "темно".to_string(),
                замена: "темно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}темно(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(темно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "плано".to_string(),
                замена: "плано-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}плано(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(плано)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "платёже".to_string(),
                замена: "платёже-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}платёже(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(платёже)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "платеже".to_string(),
                замена: "платеже-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}платеже(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(платеже)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "гибко".to_string(),
                замена: "гибко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}гибко(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(гибко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "близко".to_string(),
                замена: "близко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}близко(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(близко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "дально".to_string(),
                замена: "дально-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}дально(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(дально)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "чудо".to_string(),
                замена: "чудо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}чудо(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(чудо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "чуже".to_string(),
                замена: "чуже-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}чуже(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(чуже)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "жёстко".to_string(),
                замена: "жёстко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}жёстко(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(жёстко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "жестко".to_string(),
                замена: "жестко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}жестко(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(жестко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "восьми".to_string(),
                замена: "восьми-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}восьми(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(восьми)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "семи".to_string(),
                замена: "семи-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}семи(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(семи)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "шести".to_string(),
                замена: "шести-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}шести(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(шести)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пяти".to_string(),
                замена: "пяти-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}пяти(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(пяти)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "четырёх".to_string(),
                замена: "четырёх-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}четырёх(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(четырёх)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "четырех".to_string(),
                замена: "четырех-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}четырех(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(четырех)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "трёх".to_string(),
                замена: "трёх-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}трёх(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(трёх)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "трех".to_string(),
                замена: "трех-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}трех(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(трех)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "двух".to_string(),
                замена: "двух-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}двух(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(двух)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "грязно".to_string(),
                замена: "грязно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}грязно(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(грязно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "градо".to_string(),
                замена: "градо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}градо(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(градо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "чисто".to_string(),
                замена: "чисто-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}чисто(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(чисто)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "дву".to_string(),
                замена: "дву-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}дву(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(дву)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "одно".to_string(),
                замена: "одно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}одно(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(одно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "недо".to_string(),
                замена: "недо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}недо(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(недо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "девяти".to_string(),
                замена: "девяти-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}девяти(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(девяти)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "десяти".to_string(),
                замена: "десяти-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}десяти(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(десяти)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            //
            Ячейка_замены_с_разделителями {
                искомое_слово: "работо".to_string(),
                замена: "рабоото-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}работо(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(работо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "под".to_string(),
                замена: "под-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}под(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(под)([\w]{4,})")
                    .unwrap(),
                ряд_re_исключений:vec![Regex::new(r"\b{start}подобн").unwrap(),
                                       Regex::new(r"\b{start}подобен").unwrap(),
                ],

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "над".to_string(),
                замена: "над-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}над(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(над)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "широко".to_string(),
                замена: "широко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}широко(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(широко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "нефте".to_string(),
                замена: "нефте-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}нефте(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(нефте)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "газо".to_string(),
                замена: "газо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}газо(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(газо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "металло".to_string(),
                замена: "металло-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}металло(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(металло)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "дерево".to_string(),
                замена: "дерево-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}дерево(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(дерево)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "оптико".to_string(),
                замена: "оптико-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}оптико(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(оптико)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "стекло".to_string(),
                замена: "стекло-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}стекло(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(стекло)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "военно".to_string(),
                замена: "военно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}военно(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(военно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "уравно".to_string(),
                замена: "уравно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}уравно(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(уравно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "равно".to_string(),
                замена: "равно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}равно(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(равно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "просто".to_string(),
                замена: "просто-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}просто(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(просто)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "целе".to_string(),
                замена: "целе-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}целе(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(целе)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "сухо".to_string(),
                замена: "сухо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}сухо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(сухо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "везде".to_string(),
                замена: "везде-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}везде(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(везде)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "земле".to_string(),
                замена: "земле-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}земле(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(земле)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "водо".to_string(),
                замена: "водо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}водо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(водо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пере".to_string(),
                замена: "пере-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}пере(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(пере)([\w]{4,})")
                    .unwrap(),
                ряд_re_исключений: vec![Regex::new(r"\b{start}(передне)").unwrap()],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "передне".to_string(),
                замена: "передне-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}передне(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(передне)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "задне".to_string(),
                замена: "задне-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}задне(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(задне)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "прямо".to_string(),
                замена: "прямо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}прямо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(прямо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "лево".to_string(),
                замена: "лево-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}лево(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(лево)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "право".to_string(),
                замена: "право-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}право(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(право)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "здраво".to_string(),
                замена: "здраво-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}здраво(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(здраво)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "благо".to_string(),
                замена: "благо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}благо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(благо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "жизне".to_string(),
                замена: "жизне-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}жизне(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(жизне)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "законо".to_string(),
                замена: "законо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}законо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(законо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "все".to_string(),
                замена: "все-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}все(\w){4,}").unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(все)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "полно".to_string(),
                замена: "полно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}полно(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(полно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "средне".to_string(),
                замена: "средне-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}средне(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(средне)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "мелко".to_string(),
                замена: "мелко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}мелко(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(мелко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "крупно".to_string(),
                замена: "крупно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}крупно(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(крупно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "осново".to_string(),
                замена: "осново-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}осново(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(осново)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "добро".to_string(),
                замена: "добро-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}добро(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(добро)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "рас".to_string(),
                замена: "рас-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}рас(\w){4,}").unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(рас)([\w]{4,})")
                    .unwrap(),
                ряд_re_исключений:vec![
                  //  Regex::new(r"\b{start}принят").unwrap(),
                ],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "при".to_string(),
                замена: "при-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}при(\w){4,}").unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(при)([\w]{4,})")
                    .unwrap(),
                ряд_re_исключений:vec![
                    Regex::new(r"\b{start}принят").unwrap(),
                ],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "прежде".to_string(),
                замена: "прежде-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}прежде(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(прежде)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пре".to_string(),
                замена: "пре-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}пре(\w){4,}").unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(пре)([\w]{4,})")
                    .unwrap(),
                ряд_исключений: vec!["пред".to_string(),"прежде".to_string()],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "между".to_string(),
                замена: "между-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}между(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(между)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "едино".to_string(),
                замена: "едино-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}едино(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(едино)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пред".to_string(),
                замена: "пред-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}пред(\w){4,}")
                    .unwrap(),

                ряд_исключений: vec!["предат".to_string()],
                re_образец_для_замены: Regex::new(r"\b{start}(пред)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "тепло".to_string(),
                замена: "тепло-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}тепло(\w){4,}")
                    .unwrap(),
                re_образец_для_замены: Regex::new(r"\b{start}(тепло)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "крово".to_string(),
                замена: "крово-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}крово(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(крово)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "кратко".to_string(),
                замена: "кратко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}кратко(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(кратко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "ино".to_string(),
                замена: "ино-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}ино(\w){4,}").unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(ино)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "взрыво".to_string(),
                замена: "взрыво-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}взрыво(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(взрыво)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "мало".to_string(),
                замена: "мало-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}мало(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(мало)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "без".to_string(),
                замена: "без-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}без(\w){4,}").unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(без)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "бес".to_string(),
                замена: "бес-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}бес(\w){4,}").unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(бес)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "громко".to_string(),
                замена: "громко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}громко(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(громко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "скоро".to_string(),
                замена: "скоро-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}скоро(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(скоро)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "быстро".to_string(),
                замена: "быстро-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}быстро(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(быстро)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "долго".to_string(),
                замена: "долго-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}долго(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(долго)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "умо".to_string(),
                замена: "умо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}умо(\w){4,}").unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(умо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "сверх".to_string(),
                замена: "сверх-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}сверх(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(сверх)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "воздухо".to_string(),
                замена: "воздухо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}воздухо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(воздухо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "соот".to_string(),
                замена: "соот-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}соот(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(соот)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "бое".to_string(),
                замена: "бое-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}бое(\w){4,}").unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(бое)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "взаимо".to_string(),
                замена: "взаимо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}взаимо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(взаимо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "само".to_string(),
                замена: "само-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}само(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(само)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "хитро".to_string(),
                замена: "хитро-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}хитро(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(хитро)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "лже".to_string(),
                замена: "лже-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}лже(\w){3,}").unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(лже)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "противо".to_string(),
                замена: "противо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}противо(\w){3,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(противо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "пожаро".to_string(),
                замена: "пожаро-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}пожаро(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(пожаро)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "самолето".to_string(),
                замена: "самолето-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}самолето(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(самолето)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "самолёто".to_string(),
                замена: "самолёто-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}самолёто(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(самолёто)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "тазо".to_string(),
                замена: "тазо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}тазо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(тазо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "прапра".to_string(),
                замена: "прапра-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}прапра(\w){3,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(прапра)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "миро".to_string(),
                замена: "миров".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}миро(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(миро)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "народо".to_string(),
                замена: "народо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}народо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(народо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "верхо".to_string(),
                замена: "верхо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}верхо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(верхо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "ново".to_string(),
                замена: "ново-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}ново(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(ново)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "старо".to_string(),
                замена: "старо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}старо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(старо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "много".to_string(),
                замена: "много-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}много(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(много)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "выше".to_string(),
                замена: "выше-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}выше(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(выше)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "раз".to_string(),
                замена: "раз-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}раз(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(раз)([\w]{4,})")
                    .unwrap(),
                ряд_re_исключений:vec![Regex::new(r"\b{start}разно")
                                           .unwrap(),],
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "разно".to_string(),
                замена: "разно-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}разно(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(разно)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "высоко".to_string(),
                замена: "высоко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}высоко(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(высоко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "низко".to_string(),
                замена: "низко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}низко(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(низко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "родо".to_string(),
                замена: "родо-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}родо(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(родо)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово: "узко".to_string(),
                замена: "узко-".to_string(),
                re_образец_для_поиска: Regex::new(r"\b{start}узко(\w){4,}")
                    .unwrap(),

                re_образец_для_замены: Regex::new(r"\b{start}(узко)([\w]{4,})")
                    .unwrap(),
                ..Default::default()
            },
        ],
    };
    use crate::lib::Возможности_ячейки_замены;

    let однобуквенные: Vec<Regex> = ряд_1
        .ряд_1
        .par_iter()
        .map(|ячейка| ячейка.re_образец_для_поиска.clone())
        .collect();
    //

    //проверка образцов
    проверка_ряда_regex_разделителей(
        &однобуквенные,
        "проверка разделителей",
    );
    //
    for ячейка_замены in ряд_1.ряд_1.iter_mut() {
        ячейка_замены.ряд_re_исключений = ячейка_замены.добавить_re_исключения_изнутри();
    }
    //

    return ряд_1;
}
pub fn создать_словарь_замен() -> Словарь_Переносов {
    use crate::dictionary_0::проверка_ряда_regex;

    let словарь_замен: Словарь_Переносов = Словарь_Переносов {
        исключения: [
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
                re_образец_для_поиска: Regex::new(r"(?i)-ментального\b{end}")
                    .unwrap(),
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
        ],
        однобуквенные: [
            Ячейка_замены {
                искомое_слово: "-о".to_string(),
                замена: "о".to_string(),
                re_образец: Regex::new(r"(?i)\b{end}-о\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-а".to_string(),
                замена: "а".to_string(),
                re_образец: Regex::new(r"(?i)\b{end}-а\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ь".to_string(),
                замена: "ь".to_string(),
                re_образец: Regex::new(r"(?i)\b{end}-ь\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ы".to_string(),
                замена: "ы".to_string(),
                re_образец: Regex::new(r"(?i)\b{end}-ы\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-и".to_string(),
                замена: "и".to_string(),
                re_образец: Regex::new(r"(?i)\b{end}-и\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ъ".to_string(),
                замена: "ъ".to_string(),
                re_образец: Regex::new(r"(?i)\b{end}-ъ\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-у".to_string(),
                замена: "у".to_string(),
                re_образец: Regex::new(r"(?i)\b{end}-у\b{end}").unwrap(),
            },
        ],

        многобуквенные: [
            Ячейка_замены {
                искомое_слово: "-ройства ".to_string(),
                замена: "ройства".to_string(),
                re_образец: Regex::new(r"(?i)-ройства\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вязывающего ".to_string(),
                замена: "вязывающего".to_string(),
                re_образец: Regex::new(r"(?i)-вязывающего\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ближенный ".to_string(),
                замена: "ближенный".to_string(),
                re_образец: Regex::new(r"(?i)-ближенный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стое".to_string(),
                замена: "стое".to_string(),
                re_образец: Regex::new(r"(?i)-стое\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ному".to_string(),
                замена: "ному".to_string(),
                re_образец: Regex::new(r"(?i)-ному\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мыми".to_string(),
                замена: "мыми".to_string(),
                re_образец: Regex::new(r"(?i)-мыми\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-sign".to_string(),
                замена: "sign".to_string(),
                re_образец: Regex::new(r"(?i)-sign\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-utes".to_string(),
                замена: "utes".to_string(),
                re_образец: Regex::new(r"(?i)-utes\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-lete".to_string(),
                замена: "lete".to_string(),
                re_образец: Regex::new(r"(?i)-lete\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-tium".to_string(),
                замена: "tium".to_string(),
                re_образец: Regex::new(r"(?i)-tium\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ющая".to_string(),
                замена: "ющая".to_string(),
                re_образец: Regex::new(r"(?i)-ющая\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нове".to_string(),
                замена: "нове".to_string(),
                re_образец: Regex::new(r"(?i)-нове\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дены".to_string(),
                замена: "дены".to_string(),
                re_образец: Regex::new(r"(?i)-дены\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дить".to_string(),
                замена: "дить".to_string(),
                re_образец: Regex::new(r"(?i)-дить\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лась".to_string(),
                замена: "лась".to_string(),
                re_образец: Regex::new(r"(?i)-лась\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-брос".to_string(),
                замена: "брос".to_string(),
                re_образец: Regex::new(r"(?i)-брос\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-фере".to_string(),
                замена: "фере".to_string(),
                re_образец: Regex::new(r"(?i)-фере\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тоды".to_string(),
                замена: "тоды".to_string(),
                re_образец: Regex::new(r"(?i)-тоды\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стей".to_string(),
                замена: "стей".to_string(),
                re_образец: Regex::new(r"(?i)-стей\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ской".to_string(),
                замена: "ской".to_string(),
                re_образец: Regex::new(r"(?i)-ской\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нием".to_string(),
                замена: "нием".to_string(),
                re_образец: Regex::new(r"(?i)-нием\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ский".to_string(),
                замена: "ский".to_string(),
                re_образец: Regex::new(r"(?i)-ский\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дена".to_string(),
                замена: "дена".to_string(),
                re_образец: Regex::new(r"(?i)-дена\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жима".to_string(),
                замена: "жима".to_string(),
                re_образец: Regex::new(r"(?i)-жима\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рьер".to_string(),
                замена: "рьер".to_string(),
                re_образец: Regex::new(r"(?i)-рьер\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-верх".to_string(),
                замена: "верх".to_string(),
                re_образец: Regex::new(r"(?i)-верх\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стера".to_string(),
                замена: "стера".to_string(),
                re_образец: Regex::new(r"(?i)-стера\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рами".to_string(),
                замена: "рами".to_string(),
                re_образец: Regex::new(r"(?i)-рами\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дела".to_string(),
                замена: "дела".to_string(),
                re_образец: Regex::new(r"(?i)-дела\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ходя".to_string(),
                замена: "ходя".to_string(),
                re_образец: Regex::new(r"(?i)-ходя\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-руте".to_string(),
                замена: "руте".to_string(),
                re_образец: Regex::new(r"(?i)-руте\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ряют".to_string(),
                замена: "ряют".to_string(),
                re_образец: Regex::new(r"(?i)-ряют\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дует".to_string(),
                замена: "дует".to_string(),
                re_образец: Regex::new(r"(?i)-дует\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дачи".to_string(),
                замена: "дачи".to_string(),
                re_образец: Regex::new(r"(?i)-дачи\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-теке".to_string(),
                замена: "теке".to_string(),
                re_образец: Regex::new(r"(?i)-теке\b{end}").unwrap(),
            },
            /*Ячейка_замены {
                искомое_слово: "-либо".to_string(),
                замена: "либо".to_string(),
                re_образец: Regex::new(r"(?i)-либо\b{end}").unwrap(),
            },*/
            Ячейка_замены {
                искомое_слово: "-чить".to_string(),
                замена: "чить".to_string(),
                re_образец: Regex::new(r"(?i)-чить\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-манд".to_string(),
                замена: "манд".to_string(),
                re_образец: Regex::new(r"(?i)-манд\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дать".to_string(),
                замена: "дать".to_string(),
                re_образец: Regex::new(r"(?i)-дать\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иумы".to_string(),
                замена: "иумы".to_string(),
                re_образец: Regex::new(r"(?i)-иумы\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ования".to_string(),
                замена: "ования".to_string(),
                re_образец: Regex::new(r"(?i)-ования\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-овать".to_string(),
                замена: "овать".to_string(),
                re_образец: Regex::new(r"(?i)-овать\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иями".to_string(),
                замена: "иями".to_string(),
                re_образец: Regex::new(r"(?i)-иями\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ующие".to_string(),
                замена: "ующие".to_string(),
                re_образец: Regex::new(r"(?i)-ующие\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ующая".to_string(),
                замена: "ующая".to_string(),
                re_образец: Regex::new(r"(?i)-ующая\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ующий".to_string(),
                замена: "ующий".to_string(),
                re_образец: Regex::new(r"(?i)-ующий\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ующих".to_string(),
                замена: "ующих".to_string(),
                re_образец: Regex::new(r"(?i)-ующих\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уется".to_string(),
                замена: "уется".to_string(),
                re_образец: Regex::new(r"(?i)-уется\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уются".to_string(),
                замена: "уются".to_string(),
                re_образец: Regex::new(r"(?i)-уются\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ичную".to_string(),
                замена: "ичную".to_string(),
                re_образец: Regex::new(r"(?i)-ичную\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ичных".to_string(),
                замена: "ичных".to_string(),
                re_образец: Regex::new(r"(?i)-ичных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ного".to_string(),
                замена: "ного".to_string(),
                re_образец: Regex::new(r"(?i)-ного\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ость".to_string(),
                замена: "ость".to_string(),
                re_образец: Regex::new(r"(?i)-ость\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ости".to_string(),
                замена: "ости".to_string(),
                re_образец: Regex::new(r"(?i)-ости\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-остью".to_string(),
                замена: "остью".to_string(),
                re_образец: Regex::new(r"(?i)-остью\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нные".to_string(),
                замена: "нные".to_string(),
                re_образец: Regex::new(r"(?i)-нные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нного".to_string(),
                замена: "нного".to_string(),
                re_образец: Regex::new(r"(?i)-нного\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нный".to_string(),
                замена: "нный".to_string(),
                re_образец: Regex::new(r"(?i)-нный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нных".to_string(),
                замена: "нных".to_string(),
                re_образец: Regex::new(r"(?i)-нных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уете".to_string(),
                замена: "уете".to_string(),
                re_образец: Regex::new(r"(?i)-уете\b{end}").unwrap(),
            },
        ],
        трехбуквенные: [
            Ячейка_замены {
                искомое_слово: "-ков".to_string(),
                замена: "ков".to_string(),
                re_образец: Regex::new(r"(?i)-ков\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щий".to_string(),
                замена: "щий".to_string(),
                re_образец: Regex::new(r"(?i)-щий\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дят".to_string(),
                замена: "дят".to_string(),
                re_образец: Regex::new(r"(?i)-дят\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ter".to_string(),
                замена: "ter".to_string(),
                re_образец: Regex::new(r"(?i)-ter\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-tus".to_string(),
                замена: "tus".to_string(),
                re_образец: Regex::new(r"(?i)-tus\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-tom".to_string(),
                замена: "tom".to_string(),
                re_образец: Regex::new(r"(?i)-tom\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ции".to_string(),
                замена: "ции".to_string(),
                re_образец: Regex::new(r"(?i)-ции\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кам".to_string(),
                замена: "кам".to_string(),
                re_образец: Regex::new(r"(?i)-кам\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тём".to_string(),
                замена: "тём".to_string(),
                re_образец: Regex::new(r"(?i)-тём\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щью".to_string(),
                замена: "щью".to_string(),
                re_образец: Regex::new(r"(?i)-щью\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лом".to_string(),
                замена: "лом".to_string(),
                re_образец: Regex::new(r"(?i)-лом\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дан".to_string(),
                замена: "дан".to_string(),
                re_образец: Regex::new(r"(?i)-дан\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ста".to_string(),
                замена: "ста".to_string(),
                re_образец: Regex::new(r"(?i)-ста\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тия".to_string(),
                замена: "тия".to_string(),
                re_образец: Regex::new(r"(?i)-тия\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дой".to_string(),
                замена: "дой".to_string(),
                re_образец: Regex::new(r"(?i)-дой\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вая".to_string(),
                замена: "вая".to_string(),
                re_образец: Regex::new(r"(?i)-вая\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ния".to_string(),
                замена: "ния".to_string(),
                re_образец: Regex::new(r"(?i)-ния\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лон".to_string(),
                замена: "лон".to_string(),
                re_образец: Regex::new(r"(?i)-лон\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рых".to_string(),
                замена: "рых".to_string(),
                re_образец: Regex::new(r"(?i)-рых\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рый".to_string(),
                замена: "рый".to_string(),
                re_образец: Regex::new(r"(?i)-рый\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мые".to_string(),
                замена: "мые".to_string(),
                re_образец: Regex::new(r"(?i)-мые\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щем".to_string(),
                замена: "щем".to_string(),
                re_образец: Regex::new(r"(?i)-щем\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ний".to_string(),
                замена: "ний".to_string(),
                re_образец: Regex::new(r"(?i)-ний\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зок".to_string(),
                замена: "зок".to_string(),
                re_образец: Regex::new(r"(?i)-зок\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тем".to_string(),
                замена: "тем".to_string(),
                re_образец: Regex::new(r"(?i)-тем\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ные".to_string(),
                замена: "ные".to_string(),
                re_образец: Regex::new(r"(?i)-ные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нию".to_string(),
                замена: "нию".to_string(),
                re_образец: Regex::new(r"(?i)-нию\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шин".to_string(),
                замена: "шин".to_string(),
                re_образец: Regex::new(r"(?i)-шин\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тый".to_string(),
                замена: "тый".to_string(),
                re_образец: Regex::new(r"(?i)-тый\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нюю".to_string(),
                замена: "нюю".to_string(),
                re_образец: Regex::new(r"(?i)-нюю\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гда".to_string(),
                замена: "гда".to_string(),
                re_образец: Regex::new(r"(?i)-гда\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бой".to_string(),
                замена: "бой".to_string(),
                re_образец: Regex::new(r"(?i)-бой\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вые".to_string(),
                замена: "вые".to_string(),
                re_образец: Regex::new(r"(?i)-вые\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дов".to_string(),
                замена: "дов".to_string(),
                re_образец: Regex::new(r"(?i)-дов\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тов".to_string(),
                замена: "тов".to_string(),
                re_образец: Regex::new(r"(?i)-тов\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пей".to_string(),
                замена: "пей".to_string(),
                re_образец: Regex::new(r"(?i)-пей\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мый".to_string(),
                замена: "мый".to_string(),
                re_образец: Regex::new(r"(?i)-мый\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-nal".to_string(),
                замена: "nal".to_string(),
                re_образец: Regex::new(r"(?i)-nal\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щие".to_string(),
                замена: "щие".to_string(),
                re_образец: Regex::new(r"(?i)-щие\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вой".to_string(),
                замена: "вой".to_string(),
                re_образец: Regex::new(r"(?i)-вой\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ром".to_string(),
                замена: "ром".to_string(),
                re_образец: Regex::new(r"(?i)-ром\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мер".to_string(),
                замена: "мер".to_string(),
                re_образец: Regex::new(r"(?i)-мер\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-них".to_string(),
                замена: "них".to_string(),
                re_образец: Regex::new(r"(?i)-них\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кие".to_string(),
                замена: "кие".to_string(),
                re_образец: Regex::new(r"(?i)-кие\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чет".to_string(),
                замена: "чет".to_string(),
                re_образец: Regex::new(r"(?i)-чет\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ект".to_string(),
                замена: "ект".to_string(),
                re_образец: Regex::new(r"(?i)-ект\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жет".to_string(),
                замена: "жет".to_string(),
                re_образец: Regex::new(r"(?i)-жет\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ком".to_string(),
                замена: "ком".to_string(),
                re_образец: Regex::new(r"(?i)-ком\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вил".to_string(),
                замена: "вил".to_string(),
                re_образец: Regex::new(r"(?i)-вил\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тым".to_string(),
                замена: "тым".to_string(),
                re_образец: Regex::new(r"(?i)-тым\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ких".to_string(),
                замена: "ких".to_string(),
                re_образец: Regex::new(r"(?i)-ких\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вым".to_string(),
                замена: "вым".to_string(),
                re_образец: Regex::new(r"(?i)-вым\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зом".to_string(),
                замена: "зом".to_string(),
                re_образец: Regex::new(r"(?i)-зом\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рой".to_string(),
                замена: "рой".to_string(),
                re_образец: Regex::new(r"(?i)-рой\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чек".to_string(),
                замена: "чек".to_string(),
                re_образец: Regex::new(r"(?i)-чек\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-той".to_string(),
                замена: "той".to_string(),
                re_образец: Regex::new(r"(?i)-той\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гут".to_string(),
                замена: "гут".to_string(),
                re_образец: Regex::new(r"(?i)-гут\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ние".to_string(),
                замена: "ние".to_string(),
                re_образец: Regex::new(r"(?i)-ние\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ных".to_string(),
                замена: "ных".to_string(),
                re_образец: Regex::new(r"(?i)-ных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кой".to_string(),
                замена: "кой".to_string(),
                re_образец: Regex::new(r"(?i)-кой\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ала".to_string(),
                замена: "ала".to_string(),
                re_образец: Regex::new(r"(?i)-ала\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уют".to_string(),
                замена: "уют".to_string(),
                re_образец: Regex::new(r"(?i)-уют\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-еям".to_string(),
                замена: "еям".to_string(),
                re_образец: Regex::new(r"(?i)-еям\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нат".to_string(),
                замена: "нат".to_string(),
                re_образец: Regex::new(r"(?i)-нат\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иев".to_string(),
                замена: "иев".to_string(),
                re_образец: Regex::new(r"(?i)-иев\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иал".to_string(),
                замена: "иал".to_string(),
                re_образец: Regex::new(r"(?i)-иал\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ием".to_string(),
                замена: "ием".to_string(),
                re_образец: Regex::new(r"(?i)-ием\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иум".to_string(),
                замена: "иум".to_string(),
                re_образец: Regex::new(r"(?i)-иум\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ыми".to_string(),
                замена: "ыми".to_string(),
                re_образец: Regex::new(r"(?i)-ыми\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чим".to_string(),
                замена: "чим".to_string(),
                re_образец: Regex::new(r"(?i)-чим\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ика".to_string(),
                замена: "ика".to_string(),
                re_образец: Regex::new(r"(?i)-ика\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ику".to_string(),
                замена: "ику".to_string(),
                re_образец: Regex::new(r"(?i)-ику\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ики".to_string(),
                замена: "ики".to_string(),
                re_образец: Regex::new(r"(?i)-ики\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ать".to_string(),
                замена: "ать".to_string(),
                re_образец: Regex::new(r"(?i)-ать\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ять".to_string(),
                замена: "ять".to_string(),
                re_образец: Regex::new(r"(?i)-ять\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ным".to_string(),
                замена: "ным".to_string(),
                re_образец: Regex::new(r"(?i)-ным\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-еть".to_string(),
                замена: "еть".to_string(),
                re_образец: Regex::new(r"(?i)-еть\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лен".to_string(),
                замена: "лен".to_string(),
                re_образец: Regex::new(r"(?i)-лен\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иям".to_string(),
                замена: "иям".to_string(),
                re_образец: Regex::new(r"(?i)-иям\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дом".to_string(),
                замена: "дом".to_string(),
                re_образец: Regex::new(r"(?i)-дом\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-sor".to_string(),
                замена: "sor".to_string(),
                re_образец: Regex::new(r"(?i)-sor\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уум".to_string(),
                замена: "уум".to_string(),
                re_образец: Regex::new(r"(?i)-уум\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уем".to_string(),
                замена: "уем".to_string(),
                re_образец: Regex::new(r"(?i)-уем\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ким".to_string(),
                замена: "ким".to_string(),
                re_образец: Regex::new(r"(?i)-ким\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ешь".to_string(),
                замена: "ешь".to_string(),
                re_образец: Regex::new(r"(?i)-ешь\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ишь".to_string(),
                замена: "ишь".to_string(),
                re_образец: Regex::new(r"(?i)-ишь\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ток".to_string(),
                замена: "ток".to_string(),
                re_образец: Regex::new(r"(?i)-ток\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ете".to_string(),
                замена: "ете".to_string(),
                re_образец: Regex::new(r"(?i)-ете\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ите".to_string(),
                замена: "ите".to_string(),
                re_образец: Regex::new(r"(?i)-ите\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ует".to_string(),
                замена: "ует".to_string(),
                re_образец: Regex::new(r"(?i)-ует\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-яла".to_string(),
                замена: "яла".to_string(),
                re_образец: Regex::new(r"(?i)-яла\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-али".to_string(),
                замена: "али".to_string(),
                re_образец: Regex::new(r"(?i)-али\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-яли".to_string(),
                замена: "яли".to_string(),
                re_образец: Regex::new(r"(?i)-яли\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ола".to_string(),
                замена: "ола".to_string(),
                re_образец: Regex::new(r"(?i)-ола\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ела".to_string(),
                замена: "ела".to_string(),
                re_образец: Regex::new(r"(?i)-ела\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-оли".to_string(),
                замена: "оли".to_string(),
                re_образец: Regex::new(r"(?i)-оли\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ели".to_string(),
                замена: "ели".to_string(),
                re_образец: Regex::new(r"(?i)-ели\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ула".to_string(),
                замена: "ула".to_string(),
                re_образец: Regex::new(r"(?i)-ула\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ули".to_string(),
                замена: "ули".to_string(),
                re_образец: Regex::new(r"(?i)-ули\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ами".to_string(),
                замена: "ами".to_string(),
                re_образец: Regex::new(r"(?i)-ами\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-еми".to_string(),
                замена: "еми".to_string(),
                re_образец: Regex::new(r"(?i)-еми\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-емя".to_string(),
                замена: "емя".to_string(),
                re_образец: Regex::new(r"(?i)-емя\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ёте".to_string(),
                замена: "ёте".to_string(),
                re_образец: Regex::new(r"(?i)-ёте\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чие".to_string(),
                замена: "чие".to_string(),
                re_образец: Regex::new(r"(?i)-чие\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сте".to_string(),
                замена: "сте".to_string(),
                re_образец: Regex::new(r"(?i)-сте\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ёшь".to_string(),
                замена: "ёшь".to_string(),
                re_образец: Regex::new(r"(?i)-ёшь\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-том".to_string(),
                замена: "том".to_string(),
                re_образец: Regex::new(r"(?i)-том\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ого".to_string(),
                замена: "ого".to_string(),
                re_образец: Regex::new(r"(?i)-ого\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ций".to_string(),
                замена: "ций".to_string(),
                re_образец: Regex::new(r"(?i)-ций\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жен".to_string(),
                замена: "жен".to_string(),
                re_образец: Regex::new(r"(?i)-жен\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ому".to_string(),
                замена: "ому".to_string(),
                re_образец: Regex::new(r"(?i)-ому\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дач".to_string(),
                замена: "дач".to_string(),
                re_образец: Regex::new(r"(?i)-дач\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иях".to_string(),
                замена: "иях".to_string(),
                re_образец: Regex::new(r"(?i)-иях\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ией".to_string(),
                замена: "ией".to_string(),
                re_образец: Regex::new(r"(?i)-ией\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-умя".to_string(),
                замена: "умя".to_string(),
                re_образец: Regex::new(r"(?i)-умя\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ими".to_string(),
                замена: "ими".to_string(),
                re_образец: Regex::new(r"(?i)-ими\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тор".to_string(),
                замена: "тор".to_string(),
                re_образец: Regex::new(r"(?i)-тор\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рые".to_string(),
                замена: "рые".to_string(),
                re_образец: Regex::new(r"(?i)-рые\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сти".to_string(),
                замена: "сти".to_string(),
                re_образец: Regex::new(r"(?i)-сти\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чае".to_string(),
                замена: "чае".to_string(),
                re_образец: Regex::new(r"(?i)-чае\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вод".to_string(),
                замена: "вод".to_string(),
                re_образец: Regex::new(r"(?i)-вод\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лов".to_string(),
                замена: "лов".to_string(),
                re_образец: Regex::new(r"(?i)-лов\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кое".to_string(),
                замена: "кое".to_string(),
                re_образец: Regex::new(r"(?i)-кое\b{end}").unwrap(),
            },
        ],
        двубуквенные: [
            Ячейка_замены {
                искомое_слово: "-ца".to_string(),
                замена: "ца".to_string(),
                re_образец: Regex::new(r"(?i)-ца\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сы".to_string(),
                замена: "сы".to_string(),
                re_образец: Regex::new(r"(?i)-сы\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-er".to_string(),
                замена: "er".to_string(),
                re_образец: Regex::new(r"(?i)-er\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мы".to_string(),
                замена: "мы".to_string(),
                re_образец: Regex::new(r"(?i)-мы\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ры".to_string(),
                замена: "ры".to_string(),
                re_образец: Regex::new(r"(?i)-ры\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ра".to_string(),
                замена: "ра".to_string(),
                re_образец: Regex::new(r"(?i)-ра\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ты".to_string(),
                замена: "ты".to_string(),
                re_образец: Regex::new(r"(?i)-ты\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ка".to_string(),
                замена: "ка".to_string(),
                re_образец: Regex::new(r"(?i)-ка\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ло".to_string(),
                замена: "ло".to_string(),
                re_образец: Regex::new(r"(?i)-ло\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жа".to_string(),
                замена: "жа".to_string(),
                re_образец: Regex::new(r"(?i)-жа\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-та".to_string(),
                замена: "та".to_string(),
                re_образец: Regex::new(r"(?i)-та\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ли".to_string(),
                замена: "ли".to_string(),
                re_образец: Regex::new(r"(?i)-ли\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ея".to_string(),
                замена: "ея".to_string(),
                re_образец: Regex::new(r"(?i)-ея\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-еи".to_string(),
                замена: "еи".to_string(),
                re_образец: Regex::new(r"(?i)-еи\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ях".to_string(),
                замена: "ях".to_string(),
                re_образец: Regex::new(r"(?i)-ях\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ев".to_string(),
                замена: "ев".to_string(),
                re_образец: Regex::new(r"(?i)-ев\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ки".to_string(),
                замена: "ки".to_string(),
                re_образец: Regex::new(r"(?i)-ки\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-да".to_string(),
                замена: "да".to_string(),
                re_образец: Regex::new(r"(?i)-да\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ых".to_string(),
                замена: "ых".to_string(),
                re_образец: Regex::new(r"(?i)-ых\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ям".to_string(),
                замена: "ям".to_string(),
                re_образец: Regex::new(r"(?i)-ям\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ии".to_string(),
                замена: "ии".to_string(),
                re_образец: Regex::new(r"(?i)-ии\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ия".to_string(),
                замена: "ия".to_string(),
                re_образец: Regex::new(r"(?i)-ия\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ся".to_string(),
                замена: "ся".to_string(),
                re_образец: Regex::new(r"(?i)-ся\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ая".to_string(),
                замена: "ая".to_string(),
                re_образец: Regex::new(r"(?i)-ая\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-яя".to_string(),
                замена: "яя".to_string(),
                re_образец: Regex::new(r"(?i)-яя\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ое".to_string(),
                замена: "ое".to_string(),
                re_образец: Regex::new(r"(?i)-ое\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ее".to_string(),
                замена: "ее".to_string(),
                re_образец: Regex::new(r"(?i)-ее\b{end}").unwrap(),
            },
            /* Ячейка_замены {
                искомое_слово: "-ой".to_string(),
                замена: "ой".to_string(),
                re_образец: Regex::new(r"(?i)-ой\b{end}").unwrap(),
            },*/
            Ячейка_замены {
                искомое_слово: "-ые".to_string(),
                замена: "ые".to_string(),
                re_образец: Regex::new(r"(?i)-ые\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ий".to_string(),
                замена: "ий".to_string(),
                re_образец: Regex::new(r"(?i)-ий\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ем".to_string(),
                замена: "ем".to_string(),
                re_образец: Regex::new(r"(?i)-ем\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-им".to_string(),
                замена: "им".to_string(),
                re_образец: Regex::new(r"(?i)-им\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ет".to_string(),
                замена: "ет".to_string(),
                re_образец: Regex::new(r"(?i)-ет\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ит".to_string(),
                замена: "ит".to_string(),
                re_образец: Regex::new(r"(?i)-ит\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ут".to_string(),
                замена: "ут".to_string(),
                re_образец: Regex::new(r"(?i)-ут\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ру".to_string(),
                замена: "ру".to_string(),
                re_образец: Regex::new(r"(?i)-ру\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ют".to_string(),
                замена: "ют".to_string(),
                re_образец: Regex::new(r"(?i)-ют\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ят".to_string(),
                замена: "ят".to_string(),
                re_образец: Regex::new(r"(?i)-ят\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ял".to_string(),
                замена: "ял".to_string(),
                re_образец: Regex::new(r"(?i)-ял\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ол".to_string(),
                замена: "ол".to_string(),
                re_образец: Regex::new(r"(?i)-ол\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ел".to_string(),
                замена: "ел".to_string(),
                re_образец: Regex::new(r"(?i)-ел\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ул".to_string(),
                замена: "ул".to_string(),
                re_образец: Regex::new(r"(?i)-ул\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ам".to_string(),
                замена: "ам".to_string(),
                re_образец: Regex::new(r"(?i)-ам\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ас".to_string(),
                замена: "ас".to_string(),
                re_образец: Regex::new(r"(?i)-ас\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ах".to_string(),
                замена: "ах".to_string(),
                re_образец: Regex::new(r"(?i)-ах\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ко".to_string(),
                замена: "ко".to_string(),
                re_образец: Regex::new(r"(?i)-ко\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-её".to_string(),
                замена: "её".to_string(),
                re_образец: Regex::new(r"(?i)-её\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ей".to_string(),
                замена: "ей".to_string(),
                re_образец: Regex::new(r"(?i)-ей\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ех".to_string(),
                замена: "ех".to_string(),
                re_образец: Regex::new(r"(?i)-ех\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ею".to_string(),
                замена: "ею".to_string(),
                re_образец: Regex::new(r"(?i)-ею\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ёт".to_string(),
                замена: "ёт".to_string(),
                re_образец: Regex::new(r"(?i)-ёт\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ёх".to_string(),
                замена: "ёх".to_string(),
                re_образец: Regex::new(r"(?i)-ёх\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ие".to_string(),
                замена: "ие".to_string(),
                re_образец: Regex::new(r"(?i)-ие\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-их".to_string(),
                замена: "их".to_string(),
                re_образец: Regex::new(r"(?i)-их\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ию".to_string(),
                замена: "ию".to_string(),
                re_образец: Regex::new(r"(?i)-ию\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-но".to_string(),
                замена: "но".to_string(),
                re_образец: Regex::new(r"(?i)-но\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ми".to_string(),
                замена: "ми".to_string(),
                re_образец: Regex::new(r"(?i)-ми\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мя".to_string(),
                замена: "мя".to_string(),
                re_образец: Regex::new(r"(?i)-мя\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ов".to_string(),
                замена: "ов".to_string(),
                re_образец: Regex::new(r"(?i)-ов\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-оё".to_string(),
                замена: "оё".to_string(),
                re_образец: Regex::new(r"(?i)-оё\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-см".to_string(),
                замена: "см".to_string(),
                re_образец: Regex::new(r"(?i)-см\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ум".to_string(),
                замена: "ум".to_string(),
                re_образец: Regex::new(r"(?i)-ум\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уя".to_string(),
                замена: "уя".to_string(),
                re_образец: Regex::new(r"(?i)-уям\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ух".to_string(),
                замена: "ух".to_string(),
                re_образец: Regex::new(r"(?i)-ух\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ую".to_string(),
                замена: "ую".to_string(),
                re_образец: Regex::new(r"(?i)-ую\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шь".to_string(),
                замена: "шь".to_string(),
                re_образец: Regex::new(r"(?i)-шь\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ны".to_string(),
                замена: "ны".to_string(),
                re_образец: Regex::new(r"(?i)-ны\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пи".to_string(),
                замена: "пи".to_string(),
                re_образец: Regex::new(r"(?i)-пи\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-па".to_string(),
                замена: "па".to_string(),
                re_образец: Regex::new(r"(?i)-па\b{end}").unwrap(),
            },
        ],
        целиковые: [
            Ячейка_замены {
                искомое_слово: "-валентных".to_string(),
                замена: "валентных".to_string(),
                re_образец: Regex::new(r"(?i)-валентных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-поминающих".to_string(),
                замена: "поминающих".to_string(),
                re_образец: Regex::new(r"(?i)-поминающих\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зации".to_string(),
                замена: "зации".to_string(),
                re_образец: Regex::new(r"(?i)-зации\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-денции".to_string(),
                замена: "денции".to_string(),
                re_образец: Regex::new(r"(?i)-денции\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-личаются".to_string(),
                замена: "личаются".to_string(),
                re_образец: Regex::new(r"(?i)-личаются\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ровать".to_string(),
                замена: "ровать".to_string(),
                re_образец: Regex::new(r"(?i)-ровать\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тельными".to_string(),
                замена: "тельными".to_string(),
                re_образец: Regex::new(r"(?i)-тельными\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рифмический".to_string(),
                замена: "рифмический".to_string(),
                re_образец: Regex::new(r"(?i)-рифмический\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рительными".to_string(),
                замена: "рительными".to_string(),
                re_образец: Regex::new(r"(?i)-рительными\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лучила".to_string(),
                замена: "лучила".to_string(),
                re_образец: Regex::new(r"(?i)-лучила\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пульсный".to_string(),
                замена: "пульсный".to_string(),
                re_образец: Regex::new(r"(?i)-пульсный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-менными".to_string(),
                замена: "менными".to_string(),
                re_образец: Regex::new(r"(?i)-менными\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-правленный".to_string(),
                замена: "правленный".to_string(),
                re_образец: Regex::new(r"(?i)-правленный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зится".to_string(),
                замена: "зится".to_string(),
                re_образец: Regex::new(r"(?i)-зится\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дификацию".to_string(),
                замена: "дификацию".to_string(),
                re_образец: Regex::new(r"(?i)-дификацию\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ляться".to_string(),
                замена: "ляться".to_string(),
                re_образец: Regex::new(r"(?i)-ляться\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рительной".to_string(),
                замена: "рительной".to_string(),
                re_образец: Regex::new(r"(?i)-рительной\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зических".to_string(),
                замена: "зических".to_string(),
                re_образец: Regex::new(r"(?i)-зических\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вается".to_string(),
                замена: "вается".to_string(),
                re_образец: Regex::new(r"(?i)-вается\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-корректности".to_string(),
                замена: "корректности".to_string(),
                re_образец: Regex::new(r"(?i)-корректности\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-руется".to_string(),
                замена: "руется".to_string(),
                re_образец: Regex::new(r"(?i)-руется\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-совано".to_string(),
                замена: "совано".to_string(),
                re_образец: Regex::new(r"(?i)-совано\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-турой".to_string(),
                замена: "турой".to_string(),
                re_образец: Regex::new(r"(?i)-турой\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пустимого".to_string(),
                замена: "пустимого".to_string(),
                re_образец: Regex::new(r"(?i)-пустимого\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стовый".to_string(),
                замена: "стовый".to_string(),
                re_образец: Regex::new(r"(?i)-стовый\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стояние".to_string(),
                замена: "стояние".to_string(),
                re_образец: Regex::new(r"(?i)-стояние\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ствами".to_string(),
                замена: "ствами".to_string(),
                re_образец: Regex::new(r"(?i)-ствами\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гическую".to_string(),
                замена: "гическую".to_string(),
                re_образец: Regex::new(r"(?i)-гическую\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шинного".to_string(),
                замена: "шинного".to_string(),
                re_образец: Regex::new(r"(?i)-шинного\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-матном".to_string(),
                замена: "матном".to_string(),
                re_образец: Regex::new(r"(?i)-матном\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-значены".to_string(),
                замена: "значены".to_string(),
                re_образец: Regex::new(r"(?i)-значены\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нальные".to_string(),
                замена: "нальные".to_string(),
                re_образец: Regex::new(r"(?i)-нальные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-крепленные".to_string(),
                замена: "крепленные".to_string(),
                re_образец: Regex::new(r"(?i)-крепленные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тимальности".to_string(),
                замена: "тимальности".to_string(),
                re_образец: Regex::new(r"(?i)-тимальности\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гональных".to_string(),
                замена: "гональных".to_string(),
                re_образец: Regex::new(r"(?i)-гональных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чезнут".to_string(),
                замена: "чезнут".to_string(),
                re_образец: Regex::new(r"(?i)-чезнут\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кание".to_string(),
                замена: "кание".to_string(),
                re_образец: Regex::new(r"(?i)-кание\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гаться".to_string(),
                замена: "гаться".to_string(),
                re_образец: Regex::new(r"(?i)-гаться\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зируя".to_string(),
                замена: "зируя".to_string(),
                re_образец: Regex::new(r"(?i)-зируя\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рячими".to_string(),
                замена: "рячими".to_string(),
                re_образец: Regex::new(r"(?i)-рячими\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ливаемое".to_string(),
                замена: "ливаемое".to_string(),
                re_образец: Regex::new(r"(?i)-ливаемое\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лагаемый".to_string(),
                замена: "лагаемый".to_string(),
                re_образец: Regex::new(r"(?i)-лагаемый\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ритета".to_string(),
                замена: "ритета".to_string(),
                re_образец: Regex::new(r"(?i)-ритета\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-почтительный".to_string(),
                замена: "почтительный".to_string(),
                re_образец: Regex::new(r"(?i)-почтительный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ляющее".to_string(),
                замена: "ляющее".to_string(),
                re_образец: Regex::new(r"(?i)-ляющее\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нейкой".to_string(),
                замена: "нейкой".to_string(),
                re_образец: Regex::new(r"(?i)-нейкой\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-хождении".to_string(),
                замена: "хождении".to_string(),
                re_образец: Regex::new(r"(?i)-хождении\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-исходит".to_string(),
                замена: "исходит".to_string(),
                re_образец: Regex::new(r"(?i)-исходит\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-метров".to_string(),
                замена: "метров".to_string(),
                re_образец: Regex::new(r"(?i)-метров\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ства".to_string(),
                замена: "ства".to_string(),
                re_образец: Regex::new(r"(?i)-ства\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ровой".to_string(),
                замена: "ровой".to_string(),
                re_образец: Regex::new(r"(?i)-ровой\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-знаку".to_string(),
                замена: "знаку".to_string(),
                re_образец: Regex::new(r"(?i)-знаку\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-числены".to_string(),
                замена: "числены".to_string(),
                re_образец: Regex::new(r"(?i)-числены\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рованы".to_string(),
                замена: "рованы".to_string(),
                re_образец: Regex::new(r"(?i)-рованы\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-межуточных".to_string(),
                замена: "межуточных".to_string(),
                re_образец: Regex::new(r"(?i)-межуточных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гласование".to_string(),
                замена: "гласование".to_string(),
                re_образец: Regex::new(r"(?i)-гласование\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-обходимое".to_string(),
                замена: "обходимое".to_string(),
                re_образец: Regex::new(r"(?i)-обходимое\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-новления".to_string(),
                замена: "новления".to_string(),
                re_образец: Regex::new(r"(?i)-новления\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ских".to_string(),
                замена: "ских".to_string(),
                re_образец: Regex::new(r"(?i)-ских\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-данса".to_string(),
                замена: "данса".to_string(),
                re_образец: Regex::new(r"(?i)-данса\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-фектов".to_string(),
                замена: "фектов".to_string(),
                re_образец: Regex::new(r"(?i)-фектов\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-редач".to_string(),
                замена: "редач".to_string(),
                re_образец: Regex::new(r"(?i)-редач\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нитные".to_string(),
                замена: "нитные".to_string(),
                re_образец: Regex::new(r"(?i)-нитные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ключается".to_string(),
                замена: "ключается".to_string(),
                re_образец: Regex::new(r"(?i)-ключается\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ментов".to_string(),
                замена: "ментов".to_string(),
                re_образец: Regex::new(r"(?i)-ментов\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-граммный".to_string(),
                замена: "граммный".to_string(),
                re_образец: Regex::new(r"(?i)-граммный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вания".to_string(),
                замена: "вания".to_string(),
                re_образец: Regex::new(r"(?i)-вания\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шений".to_string(),
                замена: "шений".to_string(),
                re_образец: Regex::new(r"(?i)-шений\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-никло".to_string(),
                замена: "никло".to_string(),
                re_образец: Regex::new(r"(?i)-никло\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чиком".to_string(),
                замена: "чиком".to_string(),
                re_образец: Regex::new(r"(?i)-чиком\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чатных".to_string(),
                замена: "чатных".to_string(),
                re_образец: Regex::new(r"(?i)-чатных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-полняются".to_string(),
                замена: "полняются".to_string(),
                re_образец: Regex::new(r"(?i)-полняются\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нелей".to_string(),
                замена: "нелей".to_string(),
                re_образец: Regex::new(r"(?i)-нелей\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-торые".to_string(),
                замена: "торые".to_string(),
                re_образец: Regex::new(r"(?i)-торые\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тально".to_string(),
                замена: "тально".to_string(),
                re_образец: Regex::new(r"(?i)-тально\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-менно".to_string(),
                замена: "менно".to_string(),
                re_образец: Regex::new(r"(?i)-менно\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-торая".to_string(),
                замена: "торая".to_string(),
                re_образец: Regex::new(r"(?i)-торая\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-раммного".to_string(),
                замена: "раммного".to_string(),
                re_образец: Regex::new(r"(?i)-раммного\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мендуется".to_string(),
                замена: "мендуется".to_string(),
                re_образец: Regex::new(r"(?i)-мендуется\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-крытый".to_string(),
                замена: "крытый".to_string(),
                re_образец: Regex::new(r"(?i)-крытый\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тивным".to_string(),
                замена: "тивным".to_string(),
                re_образец: Regex::new(r"(?i)-тивным\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-манды".to_string(),
                замена: "манды".to_string(),
                re_образец: Regex::new(r"(?i)-манды\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец: Regex::new(r"(?i)-тронной\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-численных".to_string(),
                замена: "численных".to_string(),
                re_образец: Regex::new(r"(?i)-численных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ленную".to_string(),
                замена: "ленную".to_string(),
                re_образец: Regex::new(r"(?i)-ленную\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стемный".to_string(),
                замена: "стемный".to_string(),
                re_образец: Regex::new(r"(?i)-стемный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ческих".to_string(),
                замена: "ческих".to_string(),
                re_образец: Regex::new(r"(?i)-ческих\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тура".to_string(),
                замена: "тура".to_string(),
                re_образец: Regex::new(r"(?i)-тура\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ждений".to_string(),
                замена: "ждений".to_string(),
                re_образец: Regex::new(r"(?i)-ждений\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шемся".to_string(),
                замена: "шемся".to_string(),
                re_образец: Regex::new(r"(?i)-шемся\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мента".to_string(),
                замена: "мента".to_string(),
                re_образец: Regex::new(r"(?i)-мента\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мандой".to_string(),
                замена: "мандой".to_string(),
                re_образец: Regex::new(r"(?i)-мандой\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тинные".to_string(),
                замена: "тинные".to_string(),
                re_образец: Regex::new(r"(?i)-тинные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нель".to_string(),
                замена: "нель".to_string(),
                re_образец: Regex::new(r"(?i)-нель\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сутствует".to_string(),
                замена: "сутствует".to_string(),
                re_образец: Regex::new(r"(?i)-сутствует\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-симо".to_string(),
                замена: "симо".to_string(),
                re_образец: Regex::new(r"(?i)-симо\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пени".to_string(),
                замена: "пени".to_string(),
                re_образец: Regex::new(r"(?i)-пени\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тельно".to_string(),
                замена: "тельно".to_string(),
                re_образец: Regex::new(r"(?i)-тельно\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чанию".to_string(),
                замена: "чанию".to_string(),
                re_образец: Regex::new(r"(?i)-чанию\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ческая".to_string(),
                замена: "ческая".to_string(),
                re_образец: Regex::new(r"(?i)-ческая\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бирать".to_string(),
                замена: "бирать".to_string(),
                re_образец: Regex::new(r"(?i)-бирать\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-единитель".to_string(),
                замена: "единитель".to_string(),
                re_образец: Regex::new(r"(?i)-единитель\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зуемся".to_string(),
                замена: "зуемся".to_string(),
                re_образец: Regex::new(r"(?i)-зуемся\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ветствующие".to_string(),
                замена: "ветствующие".to_string(),
                re_образец: Regex::new(r"(?i)-ветствующие\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-матическая".to_string(),
                замена: "матическая".to_string(),
                re_образец: Regex::new(r"(?i)-матическая\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нентов".to_string(),
                замена: "нентов".to_string(),
                re_образец: Regex::new(r"(?i)-нентов\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нала".to_string(),
                замена: "нала".to_string(),
                re_образец: Regex::new(r"(?i)-нала\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тистические".to_string(),
                замена: "тистические".to_string(),
                re_образец: Regex::new(r"(?i)-тистические\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стимо".to_string(),
                замена: "стимо".to_string(),
                re_образец: Regex::new(r"(?i)-стимо\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жителем".to_string(),
                замена: "жителем".to_string(),
                re_образец: Regex::new(r"(?i)-жителем\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-товых".to_string(),
                замена: "товых".to_string(),
                re_образец: Regex::new(r"(?i)-товых\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-цессе".to_string(),
                замена: "цессе".to_string(),
                re_образец: Regex::new(r"(?i)-цессе\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-екта".to_string(),
                замена: "екта".to_string(),
                re_образец: Regex::new(r"(?i)-екта\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-новлены".to_string(),
                замена: "новлены".to_string(),
                re_образец: Regex::new(r"(?i)-новлены\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рования".to_string(),
                замена: "рования".to_string(),
                re_образец: Regex::new(r"(?i)-рования\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-раметры".to_string(),
                замена: "раметры".to_string(),
                re_образец: Regex::new(r"(?i)-раметры\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чески".to_string(),
                замена: "чески".to_string(),
                re_образец: Regex::new(r"(?i)-чески\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-брав".to_string(),
                замена: "брав".to_string(),
                re_образец: Regex::new(r"(?i)-брав\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-реноса".to_string(),
                замена: "реноса".to_string(),
                re_образец: Regex::new(r"(?i)-реноса\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зультаты".to_string(),
                замена: "зультаты".to_string(),
                re_образец: Regex::new(r"(?i)-зультаты\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ходных".to_string(),
                замена: "ходных".to_string(),
                re_образец: Regex::new(r"(?i)-ходных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тырех".to_string(),
                замена: "тырех".to_string(),
                re_образец: Regex::new(r"(?i)-тырех\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кать".to_string(),
                замена: "кать".to_string(),
                re_образец: Regex::new(r"(?i)-кать\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мент".to_string(),
                замена: "мент".to_string(),
                re_образец: Regex::new(r"(?i)-мент\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-штаба".to_string(),
                замена: "штаба".to_string(),
                re_образец: Regex::new(r"(?i)-штаба\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-местно".to_string(),
                замена: "местно".to_string(),
                re_образец: Regex::new(r"(?i)-местно\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ления".to_string(),
                замена: "ления".to_string(),
                re_образец: Regex::new(r"(?i)-ления\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тактные".to_string(),
                замена: "тактные".to_string(),
                re_образец: Regex::new(r"(?i)-тактные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-таллизации".to_string(),
                замена: "таллизации".to_string(),
                re_образец: Regex::new(r"(?i)-таллизации\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нить".to_string(),
                замена: "нить".to_string(),
                re_образец: Regex::new(r"(?i)-нить\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ветствующим".to_string(),
                замена: "ветствующим".to_string(),
                re_образец: Regex::new(r"(?i)-ветствующим\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-единения".to_string(),
                замена: "единения".to_string(),
                re_образец: Regex::new(r"(?i)-единения\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вать".to_string(),
                замена: "вать".to_string(),
                re_образец: Regex::new(r"(?i)-вать\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тически".to_string(),
                замена: "тически".to_string(),
                re_образец: Regex::new(r"(?i)-тически\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дами".to_string(),
                замена: "дами".to_string(),
                re_образец: Regex::new(r"(?i)-дами\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-борочно".to_string(),
                замена: "борочно".to_string(),
                re_образец: Regex::new(r"(?i)-борочно\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-веден".to_string(),
                замена: "веден".to_string(),
                re_образец: Regex::new(r"(?i)-веден\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ражает".to_string(),
                замена: "ражает".to_string(),
                re_образец: Regex::new(r"(?i)-ражает\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ством".to_string(),
                замена: "ством".to_string(),
                re_образец: Regex::new(r"(?i)-ством\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тора".to_string(),
                замена: "тора".to_string(),
                re_образец: Regex::new(r"(?i)-тора\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кусом".to_string(),
                замена: "кусом".to_string(),
                re_образец: Regex::new(r"(?i)-кусом\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лучить".to_string(),
                замена: "лучить".to_string(),
                re_образец: Regex::new(r"(?i)-лучить\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вание".to_string(),
                замена: "вание".to_string(),
                re_образец: Regex::new(r"(?i)-вание\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рантирует".to_string(),
                замена: "рантирует".to_string(),
                re_образец: Regex::new(r"(?i)-рантирует\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-менных".to_string(),
                замена: "менных".to_string(),
                re_образец: Regex::new(r"(?i)-менных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ствующим".to_string(),
                замена: "ствующим".to_string(),
                re_образец: Regex::new(r"(?i)-ствующим\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тронных".to_string(),
                замена: "тронных".to_string(),
                re_образец: Regex::new(r"(?i)-тронных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-логического".to_string(),
                замена: "логического".to_string(),
                re_образец: Regex::new(r"(?i)-логического\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рину".to_string(),
                замена: "рину".to_string(),
                re_образец: Regex::new(r"(?i)-рину\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нент".to_string(),
                замена: "нент".to_string(),
                re_образец: Regex::new(r"(?i)-нент\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тива".to_string(),
                замена: "тива".to_string(),
                re_образец: Regex::new(r"(?i)-тива\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нений".to_string(),
                замена: "нений".to_string(),
                re_образец: Regex::new(r"(?i)-нений\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ченных".to_string(),
                замена: "ченных".to_string(),
                re_образец: Regex::new(r"(?i)-ченных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ченный".to_string(),
                замена: "ченный".to_string(),
                re_образец: Regex::new(r"(?i)-ченный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рации".to_string(),
                замена: "рации".to_string(),
                re_образец: Regex::new(r"(?i)-рации\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-митивов".to_string(),
                замена: "митивов".to_string(),
                re_образец: Regex::new(r"(?i)-торого\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щение".to_string(),
                замена: "щение".to_string(),
                re_образец: Regex::new(r"(?i)-щение\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щего".to_string(),
                замена: "щего".to_string(),
                re_образец: Regex::new(r"(?i)-щего\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-виша".to_string(),
                замена: "виша".to_string(),
                re_образец: Regex::new(r"(?i)-виша\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ление".to_string(),
                замена: "ление".to_string(),
                re_образец: Regex::new(r"(?i)-ление\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рибуты".to_string(),
                замена: "рибуты".to_string(),
                re_образец: Regex::new(r"(?i)-рибуты\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-понент".to_string(),
                замена: "понент".to_string(),
                re_образец: Regex::new(r"(?i)-понент\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-понента".to_string(),
                замена: "понента".to_string(),
                re_образец: Regex::new(r"(?i)-понента\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-норамирования".to_string(),
                замена: "норамирования".to_string(),
                re_образец: Regex::new(r"(?i)-норамирования\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-можно".to_string(),
                замена: "можно".to_string(),
                re_образец: Regex::new(r"(?i)-можно\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стра".to_string(),
                замена: "стра".to_string(),
                re_образец: Regex::new(r"(?i)-стра\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-изведен".to_string(),
                замена: "изведен".to_string(),
                re_образец: Regex::new(r"(?i)-изведен\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бранному".to_string(),
                замена: "бранному".to_string(),
                re_образец: Regex::new(r"(?i)-бранному\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вится".to_string(),
                замена: "вится".to_string(),
                re_образец: Regex::new(r"(?i)-вится\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-скую".to_string(),
                замена: "скую".to_string(),
                re_образец: Regex::new(r"(?i)-скую\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-митивов".to_string(),
                замена: "митивов".to_string(),
                re_образец: Regex::new(r"(?i)-митивов\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-струкция".to_string(),
                замена: "струкция".to_string(),
                re_образец: Regex::new(r"(?i)-струкция\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-торых".to_string(),
                замена: "торых".to_string(),
                re_образец: Regex::new(r"(?i)-торых\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-веденных".to_string(),
                замена: "веденных".to_string(),
                re_образец: Regex::new(r"(?i)-веденных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сколько".to_string(),
                замена: "сколько".to_string(),
                re_образец: Regex::new(r"(?i)-сколько\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ются".to_string(),
                замена: "ются".to_string(),
                re_образец: Regex::new(r"(?i)-ются\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ствуют".to_string(),
                замена: "ствуют".to_string(),
                re_образец: Regex::new(r"(?i)-ствуют\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-павшие".to_string(),
                замена: "павшие".to_string(),
                re_образец: Regex::new(r"(?i)-павшие\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-верстия".to_string(),
                замена: "верстия".to_string(),
                re_образец: Regex::new(r"(?i)-верстия\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ванные".to_string(),
                замена: "ванные".to_string(),
                re_образец: Regex::new(r"(?i)-ванные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-реходных".to_string(),
                замена: "реходных".to_string(),
                re_образец: Regex::new(r"(?i)-реходных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-слойные".to_string(),
                замена: "слойные".to_string(),
                re_образец: Regex::new(r"(?i)-слойные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-водится".to_string(),
                замена: "водится".to_string(),
                re_образец: Regex::new(r"(?i)-водится\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вами".to_string(),
                замена: "вами".to_string(),
                re_образец: Regex::new(r"(?i)-вами\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-митивы".to_string(),
                замена: "митивы".to_string(),
                re_образец: Regex::new(r"(?i)-митивы\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пользуемых".to_string(),
                замена: "пользуемых".to_string(),
                re_образец: Regex::new(r"(?i)-пользуемых\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-няться".to_string(),
                замена: "няться".to_string(),
                re_образец: Regex::new(r"(?i)-няться\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дартов".to_string(),
                замена: "дартов".to_string(),
                re_образец: Regex::new(r"(?i)-дартов\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ность".to_string(),
                замена: "ность".to_string(),
                re_образец: Regex::new(r"(?i)-ность\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ленных".to_string(),
                замена: "ленных".to_string(),
                re_образец: Regex::new(r"(?i)-ленных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пусках".to_string(),
                замена: "пусках".to_string(),
                re_образец: Regex::new(r"(?i)-пусках\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бавления".to_string(),
                замена: "бавления".to_string(),
                re_образец: Regex::new(r"(?i)-бавления\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дактировать".to_string(),
                замена: "дактировать".to_string(),
                re_образец: Regex::new(r"(?i)-дактировать\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тический".to_string(),
                замена: "тический".to_string(),
                re_образец: Regex::new(r"(?i)-тический\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дактор".to_string(),
                замена: "дактор".to_string(),
                re_образец: Regex::new(r"(?i)-дактор\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ретащим".to_string(),
                замена: "ретащим".to_string(),
                re_образец: Regex::new(r"(?i)-ретащим\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зицию".to_string(),
                замена: "зицию".to_string(),
                re_образец: Regex::new(r"(?i)-зицию\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рения".to_string(),
                замена: "рения".to_string(),
                re_образец: Regex::new(r"(?i)-рения\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зателя".to_string(),
                замена: "зателя".to_string(),
                re_образец: Regex::new(r"(?i)-зателя\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-затель".to_string(),
                замена: "затель".to_string(),
                re_образец: Regex::new(r"(?i)-затель\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-водами".to_string(),
                замена: "водами".to_string(),
                re_образец: Regex::new(r"(?i)-водами\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кладка".to_string(),
                замена: "кладка".to_string(),
                re_образец: Regex::new(r"(?i)-кладка\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-деления".to_string(),
                замена: "деления".to_string(),
                re_образец: Regex::new(r"(?i)-деления\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ражения".to_string(),
                замена: "ражения".to_string(),
                re_образец: Regex::new(r"(?i)-ражения\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-телем".to_string(),
                замена: "телем".to_string(),
                re_образец: Regex::new(r"(?i)-телем\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-садочных".to_string(),
                замена: "садочных".to_string(),
                re_образец: Regex::new(r"(?i)-садочных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дактора".to_string(),
                замена: "дактора".to_string(),
                re_образец: Regex::new(r"(?i)-дактора\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ченной".to_string(),
                замена: "ченной".to_string(),
                re_образец: Regex::new(r"(?i)-ченной\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-распознанный".to_string(),
                замена: "распознанный".to_string(),
                re_образец: Regex::new(r"(?i)-распознанный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-моугольный".to_string(),
                замена: "моугольный".to_string(),
                re_образец: Regex::new(r"(?i)-моугольный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-циями".to_string(),
                замена: "циями".to_string(),
                re_образец: Regex::new(r"(?i)-циями\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тированный".to_string(),
                замена: "тированный".to_string(),
                re_образец: Regex::new(r"(?i)-тированный\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-варительно".to_string(),
                замена: "варительно".to_string(),
                re_образец: Regex::new(r"(?i)-варительно\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-емость".to_string(),
                замена: "емость".to_string(),
                re_образец: Regex::new(r"(?i)-емость\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ваться".to_string(),
                замена: "ваться".to_string(),
                re_образец: Regex::new(r"(?i)-ваться\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-когда".to_string(),
                замена: "когда".to_string(),
                re_образец: Regex::new(r"(?i)-когда\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ответствии".to_string(),
                замена: "ответствии".to_string(),
                re_образец: Regex::new(r"(?i)-ответствии\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-этому".to_string(),
                замена: "этому".to_string(),
                re_образец: Regex::new(r"(?i)-этому\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-слеживания".to_string(),
                замена: "слеживания".to_string(),
                re_образец: Regex::new(r"(?i)-слеживания\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рирует".to_string(),
                замена: "рирует".to_string(),
                re_образец: Regex::new(r"(?i)-рирует\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сения".to_string(),
                замена: "сения".to_string(),
                re_образец: Regex::new(r"(?i)-сения\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ниями".to_string(),
                замена: "ниями".to_string(),
                re_образец: Regex::new(r"(?i)-ниями\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-структивными".to_string(),
                замена: "структивными".to_string(),
                re_образец: Regex::new(r"(?i)-структивными\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ствия".to_string(),
                замена: "ствия".to_string(),
                re_образец: Regex::new(r"(?i)-ствия\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-единять".to_string(),
                замена: "единять".to_string(),
                re_образец: Regex::new(r"(?i)-единять\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шения".to_string(),
                замена: "шения".to_string(),
                re_образец: Regex::new(r"(?i)-шения\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-изводить".to_string(),
                замена: "изводить".to_string(),
                re_образец: Regex::new(r"(?i)-изводить\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жимах".to_string(),
                замена: "жимах".to_string(),
                re_образец: Regex::new(r"(?i)-жимах\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чайшее".to_string(),
                замена: "чайшее".to_string(),
                re_образец: Regex::new(r"(?i)-чайшее\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ношении".to_string(),
                замена: "ношении".to_string(),
                re_образец: Regex::new(r"(?i)-ношении\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ровки".to_string(),
                замена: "ровки".to_string(),
                re_образец: Regex::new(r"(?i)-ровки\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-изводиться".to_string(),
                замена: "изводиться".to_string(),
                re_образец: Regex::new(r"(?i)-изводиться\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бирает".to_string(),
                замена: "бирает".to_string(),
                re_образец: Regex::new(r"(?i)-бирает\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ностями".to_string(),
                замена: "ностями".to_string(),
                re_образец: Regex::new(r"(?i)-ностями\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-виши".to_string(),
                замена: "виши".to_string(),
                re_образец: Regex::new(r"(?i)-виши\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тивизации".to_string(),
                замена: "тивизации".to_string(),
                re_образец: Regex::new(r"(?i)-тивизации\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-личных".to_string(),
                замена: "личных".to_string(),
                re_образец: Regex::new(r"(?i)-личных\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ложение".to_string(),
                замена: "ложение".to_string(),
                re_образец: Regex::new(r"(?i)-ложение\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тивной".to_string(),
                замена: "тивной".to_string(),
                re_образец: Regex::new(r"(?i)-тивной\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-логических".to_string(),
                замена: "логических".to_string(),
                re_образец: Regex::new(r"(?i)-логических\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нивает".to_string(),
                замена: "нивает".to_string(),
                re_образец: Regex::new(r"(?i)-нивает\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-слойной".to_string(),
                замена: "слойной".to_string(),
                re_образец: Regex::new(r"(?i)-слойной\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нимается".to_string(),
                замена: "нимается".to_string(),
                re_образец: Regex::new(r"(?i)-нимается\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тельного".to_string(),
                замена: "тельного".to_string(),
                re_образец: Regex::new(r"(?i)-тельного\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вость".to_string(),
                замена: "вость".to_string(),
                re_образец: Regex::new(r"(?i)-вость\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сматриваются".to_string(),
                замена: "сматриваются".to_string(),
                re_образец: Regex::new(r"(?i)-сматриваются\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-суждений".to_string(),
                замена: "суждений".to_string(),
                re_образец: Regex::new(r"(?i)-суждений\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дарственное".to_string(),
                замена: "дарственное".to_string(),
                re_образец: Regex::new(r"(?i)-дарственное\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чайное".to_string(),
                замена: "чайное".to_string(),
                re_образец: Regex::new(r"(?i)-чайное\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ниченные".to_string(),
                замена: "ниченные".to_string(),
                re_образец: Regex::new(r"(?i)-ниченные\b{end}").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ветить".to_string(),
                замена: "ветить".to_string(),
                re_образец: Regex::new(r"(?i)-ветить\b{end}").unwrap(),
            },
            /*
               Ячейка_замены {
                   искомое_слово: "—ментального".to_string(),
                   замена: "ментального".to_string(),
                   re_образец: Regex::new(r"(?i)—ментального\b{end}").unwrap(),
               },
               Ячейка_замены {
                   искомое_слово: "—вание".to_string(),
                   замена: "вание".to_string(),
                   re_образец: Regex::new(r"(?i)—вание\b{end}").unwrap(),
               },

            */
        ],
    };
    //let словарь_второй

    let исключения: Vec<Regex> = словарь_замен
        .исключения
        .par_iter()
        .map(|ячейка| ячейка.re_образец_для_поиска.clone())
        .collect();
    let целиковые: Vec<Regex> = словарь_замен
        .целиковые
        .par_iter()
        .map(|ячейка| ячейка.re_образец.clone())
        .collect();
    let трёхбуквенные: Vec<Regex> = словарь_замен
        .трехбуквенные
        .par_iter()
        .map(|ячейка| ячейка.re_образец.clone())
        .collect();
    let двубуквенные: Vec<Regex> = словарь_замен
        .двубуквенные
        .par_iter()
        .map(|ячейка| ячейка.re_образец.clone())
        .collect();
    let многоуквенные: Vec<Regex> = словарь_замен
        .многобуквенные
        .par_iter()
        .map(|ячейка| ячейка.re_образец.clone())
        .collect();
    let однобуквенные: Vec<Regex> = словарь_замен
        .однобуквенные
        .par_iter()
        .map(|ячейка| ячейка.re_образец.clone())
        .collect();

    //проверка образцов
    проверка_ряда_regex_замен(&трёхбуквенные, "проверка замен трёхбуквенные");
    проверка_ряда_regex_замен(&*двубуквенные, "проверка замен двубуквенные");
    проверка_ряда_regex_замен(&*многоуквенные, "проверка замен многобуквенные");
    проверка_ряда_regex_замен(&*однобуквенные, "проверка замен однобуквенные");
    проверка_ряда_regex_замен(&*целиковые, "проверка замен целиковые");
    проверка_ряда_regex_замен(&*исключения, "проверка замен исключения");
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

pub fn создать_второй_словарь_разделителей(
    mut словарь_изначальный: Словарь_разделителей,
) -> Словарь_разделителей {
    use crate::lib::Возможности_ячейки_замены;
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
    use crate::lib::Возможности_ячейки_замены;
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
    re_ряд: impl AsRef<[Regex]>, сообщение: &str
) {
    let ряд = re_ряд.as_ref();
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
    re_ряд: impl AsRef<[Regex]>,
    сообщение: &str,
) {
    let ряд = re_ряд.as_ref();
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
