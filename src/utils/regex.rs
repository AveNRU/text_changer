use crate::utils::stringzilla::*;
//use clap::error::ErrorKind::Format;
use console::{Emoji, style};
use foldhash::{HashSet, HashSetExt, quality::FixedState};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use rand::{Rng, prelude::*};
use std::thread;
use std::time::{Duration, Instant};
use std::{cmp::min, fmt::Write};
use rayon::prelude::*;
use std::sync::{Mutex, atomic::{AtomicU64, Ordering}};
use std::sync::atomic::AtomicUsize;


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

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

//если это картинка
use lazy_static::lazy_static;
use rayon::iter::IntoParallelRefIterator;
use regex::Regex;
pub fn изображение_расширение(word: &String) -> bool {
    lazy_static! {
        static ref re_расширения_изображений: Vec<Regex> = vec![
            Regex::new(r"(?i)\.jpeg$").unwrap(),
            Regex::new(r"(?i)\.jpg$").unwrap(),
            Regex::new(r"(?i)\.tiff$").unwrap(),
            Regex::new(r"(?i)\.png$").unwrap(),
            Regex::new(r"(?i)\.bmp$").unwrap(),
            Regex::new(r"(?i)\.wmf$").unwrap(),
            Regex::new(r"(?i)\.wpg$").unwrap(),
            Regex::new(r"(?i)\.eps$").unwrap(),
        ];
    }
    for строка in re_расширения_изображений.iter() {
        if строка.is_match(word) {
            return true;
        }
    }
    return false;
}
//если это архивный файл
pub fn fb3_epub(word: &String) -> bool {
    lazy_static! {
        static ref re_расширения_архивные:Vec<Regex> = vec![
        Regex::new(r"(?i)\.fb3$").unwrap(),
        Regex::new(r"(?i)\.epub$").unwrap(),
        //Regex::new(r"(?i)\.docx$").unwrap(),
        //Regex::new(r"(?i)\.doc$").unwrap(),
     ];
    }
    for строка in re_расширения_архивные.iter() {
        if строка.is_match(word) {
            return true;
        }
    }
    return false;
}
//если это архивный файл
pub fn doc_docx(word: &String) -> bool {
    lazy_static! {
           static ref re_расширения_word:Vec<Regex> = vec![
        //Regex::new(r"(?i)\.fb3$").unwrap(),
        //Regex::new(r"(?i)\.epub$").unwrap(),
        Regex::new(r"(?i)\.docx$").unwrap(),
        Regex::new(r"(?i)\.doc$").unwrap(),
     ];
    }
    for строка in re_расширения_word.iter() {
        if строка.is_match(word) {
            return true;
        }
    }
    return false;
}
pub fn md_fs_yml(word: &String) -> bool {
    lazy_static! {
           static ref re_расширения_word:Vec<Regex> = vec![
        //Regex::new(r"(?i)\.fb3$").unwrap(),
        //Regex::new(r"(?i)\.epub$").unwrap(),
        Regex::new(r"(?i)\.md$").unwrap(),
            Regex::new(r"(?i)\.yml$").unwrap(),
            Regex::new(r"(?i)\.fs$").unwrap(),
     ];
    }
    for строка in re_расширения_word.iter() {
        if строка.is_match(word) {
            return true;
        }
    }
    return false;
}
//если это не архивный файл
pub fn fb2_rtf_mhtml(word: &String) -> bool {
    lazy_static! {
        static ref re_расширения_не_архивные: Vec<Regex> = vec![
            Regex::new(r"(?i)\.fb2$").unwrap(),
            Regex::new(r"(?i)\.rtf$").unwrap(),
            Regex::new(r"(?i)\.mhtml$").unwrap(),
        ];
    }
    for строка in re_расширения_не_архивные.iter() {
        if строка.is_match(word) {
            return true;
        }
    }
    return false;
}
//захват слов
//есть ли маты
pub fn есть_ли_маты(hay: &String) -> bool {
    lazy_static! {
            //маты
     static ref re_матершина_слова:Vec<Regex> = vec![
        Regex::new(r"(?i)\s*([\w]…)\s*").unwrap(),
     ];
    }
    for строка in re_матершина_слова.iter() {
        if строка.is_match(hay) {
            return true;
        }
    }
    return false;
}

//выдел строки
pub fn re_получить_строку_с_описанием(
    стог_сена: &String,
    образец: &Regex,
    ошибка: &str,
) -> String {
    lazy_static! {
        static ref нет_расширения: Regex = Regex::new(r"(?i)(?:\\)+([\d\w&&[^\.]]+)$").unwrap();
    }
    let Some(строка) = образец.captures(&стог_сена) else {
        if let Some(строка) = нет_расширения.captures(&стог_сена) {
            return "Пусто".to_string();
        } else {
            println!("{}", ошибка);
            panic!(
                "ошибка при выдирания {}, сама строка : {}",
                &образец, &стог_сена
            );
        }
    };
    return строка[1].trim().to_string();
}

//выдел строки
/*
pub fn получить_строку_из_ряда_re_с_описанием(стог_сена: &String, образец: &Vec<Regex>,ошибка:&str) -> String {
    let Some(строка) = образец.captures(&стог_сена) else {
        println!("{}",ошибка);
        panic!("ошибка при выдирания {}, сама строка : {}", &образец, &стог_сена);
    };
    return строка[1].trim().to_string();
}

 */

pub fn определить_имя_книги(стог_сена: &String) -> String {
    lazy_static! {
        static ref re_пути_до_книг: Vec<Regex> = vec![
            Regex::new(r"(?i)books/([\d\w_\-\s\.,]+)\.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i)books\\([\d\w_\-\s\.,]+)\.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i).+/(.+)\.").unwrap(),
        ];
    }
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
}
/*
pub fn замена_слов_через_regex(
    re_образцы: &Vec<Regex>,
    содержимое: &mut Vec<String>,
    замены: &Vec<String>,
    счётчик_словаря: &mut Vec<usize>,
    искомое_слово: &Vec<String>,
    сообщение: &str,
    расширение: &String,
    указатель_захода: &mut usize,
    куча_пропусков: &HashSet<usize>,
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
pub fn замена_слов_через_regex(
    re_образцы: &[Regex],
    содержимое: &mut [String],
    замены: &[String],
    счётчик_словаря: &mut [usize],
    искомое_слово: &[String],
    сообщение: &str,
    расширение: &str,
    указатель_захода: &mut usize,
    куча_пропусков: &HashSet<usize>,
) {
    *указатель_захода += 1;
    println!("{} {}Завершено...", style(сообщение).bold().dim(), LOOKING_GLASS);

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
    содержимое.par_iter_mut().enumerate().for_each(|(указатель, строка)| {
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

            if sz_найти(&строка, &искомое_слово[указатель_образца]) {
                let замененная_строка = re_образец.replace_all(
                    &строка,
                    &замены[указатель_образца],
                );

                // Заменяем строку
                *строка = замененная_строка.to_string();

                // Увеличиваем атомарный счетчик
                атомарные_счетчики[указатель_образца].fetch_add(1, Ordering::Relaxed);
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