use crate::utils::stringzilla::*;
use stringzilla::stringzilla::bytesum;
//use clap::error::ErrorKind::Format;
use console::{Emoji, style};
use foldhash::fast::RandomState;
use foldhash::{HashMap, HashSet, HashSetExt};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use lib::{Словарь_Переносов, Ячейка_замены};
use rand::{Rng, prelude::*};
use rayon::prelude::*;
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
static LOOKING_GLASS: &str = "🔍";
//если это картинка
use crate::lib::{self, Счётчик_замен, Ячейка_словаря};
use lazy_static::lazy_static;
use rayon::iter::IntoParallelRefIterator;
use regex::Regex;

pub fn мусорное_содержимое_архивов(стог_сена: &String) -> bool {
    lazy_static! {
        static ref re_расширения_мусорные: [Regex;5] = [
            Regex::new(r"(?i)\.css$").unwrap(),
              Regex::new(r"(?i)\.rels$").unwrap(),
              Regex::new(r"(?i)\.ttf$").unwrap(),
            Regex::new(r"(?i)\.xhtml$").unwrap(),
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
        static ref re_расширения_изображений: [Regex;10] = [
            Regex::new(r"(?i)\.jpe?g$").unwrap(),  // Объединил jpg и jpeg
            Regex::new(r"(?i)\.tiff?$").unwrap(),  // Объединил tif и tiff
            Regex::new(r"(?i)\.png$").unwrap(),
            Regex::new(r"(?i)\.bmp$").unwrap(),
            Regex::new(r"(?i)\.wmf$").unwrap(),
            Regex::new(r"(?i)\.wpg$").unwrap(),
            Regex::new(r"(?i)\.gif$").unwrap(),    // Добавил $ в конец
            Regex::new(r"(?i)\.webp$").unwrap(),   // Добавил современные форматы
            Regex::new(r"(?i)\.svg$").unwrap(),
            Regex::new(r"(?i)\.eps$").unwrap(),
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
        static ref re_расширения_изображений: [Regex;10] = [
            Regex::new(r"(?i)\.jpe?g$").unwrap(),  // Объединил jpg и jpeg
            Regex::new(r"(?i)\.tiff?$").unwrap(),  // Объединил tif и tiff
            Regex::new(r"(?i)\.png$").unwrap(),
            Regex::new(r"(?i)\.bmp$").unwrap(),
            Regex::new(r"(?i)\.wmf$").unwrap(),
            Regex::new(r"(?i)\.wpg$").unwrap(),
            Regex::new(r"(?i)\.gif$").unwrap(),    // Добавил $ в конец
            Regex::new(r"(?i)\.webp$").unwrap(),   // Добавил современные форматы
            Regex::new(r"(?i)\.svg$").unwrap(),
            Regex::new(r"(?i)\.avif$").unwrap(),
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

pub fn htm_html(стог_сена: &String) -> bool {
    lazy_static! {
        static ref re_расширения_word: [Regex; 2] = [
            Regex::new(r"(?i)\.htm$").unwrap(),
            Regex::new(r"(?i)\.html$").unwrap(),
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
        static ref без_пути:[Regex;1] = [
            Regex::new(r"(?i)\\(.[^\\]+)$").unwrap(),
        //     Regex::new(r"(?i)(.[^\\]+)$").unwrap(),
        ];
        static ref первая_палка:Regex= Regex::new(r"(?i)\\").unwrap();
        static ref вторая_палка:Regex= Regex::new(r"(?i)/").unwrap();
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
        static ref re_пути_до_книг: [Regex; 3] = [
            Regex::new(r"(?i)books/([\d\w_\-\s\.,]+)\.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i)books\\([\d\w_\-\s\.,]+)\.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i).+/(.+)\.").unwrap(),
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
//многопоточность
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

pub fn замена_слов_через_кучу(
    словарь: &[Ячейка_словаря],
    содержимое: &mut [String],
    счётчик_словаря: &mut Vec<Arc<AtomicUsize>>,
    сообщение: &str,
    расширение: &str,
    куча_пропусков: &HashSet<usize>,
    словарь_куча: &HashMap<String, HashSet<usize>>,
) {
    let spinner_style = ProgressStyle::with_template("{wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(15));
    pb.set_style(spinner_style.clone());

    //Создаем атомарные счетчики для каждого шаблона
    let атомарные_счетчики: Vec<AtomicUsize> =
        (0..словарь.len()).map(|_| AtomicUsize::new(0)).collect();

    let количество_шагов = словарь.len() * содержимое.len();
    let счетчик_внутренний = ProgressBar::new(количество_шагов as u64);
    let шаг_внутренний = AtomicU64::new(0);

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
                                    атомарные_счетчики[*указатель_образца]
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
                                атомарные_счетчики[*указатель_образца]
                                    .fetch_add(1, Ordering::Relaxed);
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
    атомарные_счетчики
        .iter()
        .enumerate()
        .for_each(|(указатель, число)| {
            счётчик_словаря[указатель].fetch_add(число.load(Ordering::Relaxed), Ordering::Relaxed); //
        });
}

//многопоточность

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
    //куча_пропусков: &HashSet<usize>,
) {
    use crate::dictionary_0::проверка_ряда_regex;

    //если первый раз заходит - то проверить

    //подсчёт для видимого счётчика в окне
    let общий_счёт: usize = словарь_замен.целиковые.len()
        + словарь_замен.многобуквенные.len()
        + словарь_замен.трехбуквенные.len()
        + словарь_замен.двубуквенные.len()
        + словарь_замен.однобуквенные.len();

    //общий счёт
    let количество_шагов = общий_счёт * содержимое.len();
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
            /* if куча_пропусков.contains(&указатель) {
                // Пропускаем строку, но все равно считаем прогресс
                let шаги_для_этой_строки = словарь_замен.len() as u64;
                шаг_внутренний.fetch_add(шаги_для_этой_строки, Ordering::Relaxed);
                счетчик_внутренний.inc(шаги_для_этой_строки);
                return;
            }*/

            // Сохраняем оригинальную строку для проверки
            //  let оригинальная_строка = строка.clone();
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
                        счётчики_замен.целиковые[указатель_образца].fetch_add(1, Ordering::Relaxed);
                        // счётчик_однобуквенные[указатель_образца].fetch_add(1, Ordering::Relaxed);
                    }
                    // Заменяем строку
                    *строка = замененная_строка;
                }
                // Обновляем прогресс
                let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                счетчик_внутренний.set_position(текущий_шаг);
            }
            //многобуквенные
            for указатель_образца in 0..словарь_замен.многобуквенные.len()
            {
                let re_образец = &словарь_замен.многобуквенные[указатель_образца].re_образец;
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
                счетчик_внутренний.set_position(текущий_шаг);
            }
            //трехбуквенные
            for указатель_образца in 0..словарь_замен.трехбуквенные.len()
            {
                let re_образец = &словарь_замен.трехбуквенные[указатель_образца].re_образец;
                // println!("образец №{указатель_образца}: {}",re_образец);
                let замена = &словарь_замен.трехбуквенные[указатель_образца].замена;
                let искомое_слово = &словарь_замен.трехбуквенные[указатель_образца].искомое_слово;
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
                счетчик_внутренний.set_position(текущий_шаг);
            }
            //двубуквенные
            for указатель_образца in 0..словарь_замен.двубуквенные.len()
            {
                let re_образец = &словарь_замен.двубуквенные[указатель_образца].re_образец;
                // println!("образец №{указатель_образца}: {}",re_образец);
                let замена = &словарь_замен.двубуквенные[указатель_образца].замена;
                let искомое_слово = &словарь_замен.двубуквенные[указатель_образца].искомое_слово;
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
                счетчик_внутренний.set_position(текущий_шаг);
            }
            //однобуквенные
            for указатель_образца in 0..словарь_замен.однобуквенные.len()
            {
                let re_образец = &словарь_замен.однобуквенные[указатель_образца].re_образец;
                let искомое_слово = &словарь_замен.однобуквенные[указатель_образца].искомое_слово;
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
                счетчик_внутренний.set_position(текущий_шаг);
            }
        });
    //println!("счётчики замен: {:?}",счётчики_замен.двубуквенные);
}

pub fn создать_словарь_замен() -> (Arc<Счётчик_замен>, Словарь_Переносов) {
    use crate::dictionary_0::проверка_ряда_regex;
    let словарь_замен: Словарь_Переносов = Словарь_Переносов {
        однобуквенные: vec![
            Ячейка_замены {
                искомое_слово: "-о".to_string(),
                замена: "о".to_string(),
                re_образец: Regex::new(r"(?i)-о\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-а".to_string(),
                замена: "а".to_string(),
                re_образец: Regex::new(r"(?i)-а\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-я".to_string(),
                замена: "я".to_string(),
                re_образец: Regex::new(r"(?i)-я\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-е".to_string(),
                замена: "е".to_string(),
                re_образец: Regex::new(r"(?i)-е\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ь".to_string(),
                замена: "ь".to_string(),
                re_образец: Regex::new(r"(?i)-ь\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ы".to_string(),
                замена: "ы".to_string(),
                re_образец: Regex::new(r"(?i)-ы\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-и".to_string(),
                замена: "и".to_string(),
                re_образец: Regex::new(r"(?i)-и\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ъ".to_string(),
                замена: "ъ".to_string(),
                re_образец: Regex::new(r"(?i)-ъ\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-у".to_string(),
                замена: "у".to_string(),
                re_образец: Regex::new(r"(?i)-у\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ю".to_string(),
                замена: "ю".to_string(),
                re_образец: Regex::new(r"(?i)-ю\>").unwrap(),
            },
        ],

        многобуквенные: vec![
            Ячейка_замены {
                искомое_слово: "-sign".to_string(),
                замена: "sign".to_string(),
                re_образец: Regex::new(r"(?i)-sign\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-line".to_string(),
                замена: "line".to_string(),
                re_образец: Regex::new(r"(?i)-line\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-utes".to_string(),
                замена: "utes".to_string(),
                re_образец: Regex::new(r"(?i)-utes\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-lete".to_string(),
                замена: "lete".to_string(),
                re_образец: Regex::new(r"(?i)-lete\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-tium".to_string(),
                замена: "tium".to_string(),
                re_образец: Regex::new(r"(?i)-tium\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ющая".to_string(),
                замена: "ющая".to_string(),
                re_образец: Regex::new(r"(?i)-ющая\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нове".to_string(),
                замена: "нове".to_string(),
                re_образец: Regex::new(r"(?i)-нове\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дены".to_string(),
                замена: "дены".to_string(),
                re_образец: Regex::new(r"(?i)-дены\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дить".to_string(),
                замена: "дить".to_string(),
                re_образец: Regex::new(r"(?i)-дить\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лась".to_string(),
                замена: "лась".to_string(),
                re_образец: Regex::new(r"(?i)-лась\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-брос".to_string(),
                замена: "брос".to_string(),
                re_образец: Regex::new(r"(?i)-брос\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-фере".to_string(),
                замена: "фере".to_string(),
                re_образец: Regex::new(r"(?i)-фере\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тоды".to_string(),
                замена: "тоды".to_string(),
                re_образец: Regex::new(r"(?i)-тоды\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стей".to_string(),
                замена: "стей".to_string(),
                re_образец: Regex::new(r"(?i)-стей\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ской".to_string(),
                замена: "ской".to_string(),
                re_образец: Regex::new(r"(?i)-ской\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нием".to_string(),
                замена: "нием".to_string(),
                re_образец: Regex::new(r"(?i)-нием\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ский".to_string(),
                замена: "ский".to_string(),
                re_образец: Regex::new(r"(?i)-ский\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дена".to_string(),
                замена: "дена".to_string(),
                re_образец: Regex::new(r"(?i)-дена\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жима".to_string(),
                замена: "жима".to_string(),
                re_образец: Regex::new(r"(?i)-жима\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рьер".to_string(),
                замена: "рьер".to_string(),
                re_образец: Regex::new(r"(?i)-рьер\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-верх".to_string(),
                замена: "верх".to_string(),
                re_образец: Regex::new(r"(?i)-верх\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стера".to_string(),
                замена: "стера".to_string(),
                re_образец: Regex::new(r"(?i)-стера\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рами".to_string(),
                замена: "рами".to_string(),
                re_образец: Regex::new(r"(?i)-рами\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дела".to_string(),
                замена: "дела".to_string(),
                re_образец: Regex::new(r"(?i)-дела\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ходя".to_string(),
                замена: "ходя".to_string(),
                re_образец: Regex::new(r"(?i)-ходя\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-руте".to_string(),
                замена: "руте".to_string(),
                re_образец: Regex::new(r"(?i)-руте\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ряют".to_string(),
                замена: "ряют".to_string(),
                re_образец: Regex::new(r"(?i)-ряют\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дует".to_string(),
                замена: "дует".to_string(),
                re_образец: Regex::new(r"(?i)-дует\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дачи".to_string(),
                замена: "дачи".to_string(),
                re_образец: Regex::new(r"(?i)-дачи\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-теке".to_string(),
                замена: "теке".to_string(),
                re_образец: Regex::new(r"(?i)-теке\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-либо".to_string(),
                замена: "либо".to_string(),
                re_образец: Regex::new(r"(?i)-либо\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чить".to_string(),
                замена: "чить".to_string(),
                re_образец: Regex::new(r"(?i)-чить\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-манд".to_string(),
                замена: "манд".to_string(),
                re_образец: Regex::new(r"(?i)-манд\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дать".to_string(),
                замена: "дать".to_string(),
                re_образец: Regex::new(r"(?i)-дать\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иумы".to_string(),
                замена: "иумы".to_string(),
                re_образец: Regex::new(r"(?i)-иумы\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ования".to_string(),
                замена: "ования".to_string(),
                re_образец: Regex::new(r"(?i)-ования\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-овать".to_string(),
                замена: "овать".to_string(),
                re_образец: Regex::new(r"(?i)-овать\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иями".to_string(),
                замена: "иями".to_string(),
                re_образец: Regex::new(r"(?i)-иями\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ующие".to_string(),
                замена: "ующие".to_string(),
                re_образец: Regex::new(r"(?i)-ующие\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ующая".to_string(),
                замена: "ующая".to_string(),
                re_образец: Regex::new(r"(?i)-ующая\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ующий".to_string(),
                замена: "ующий".to_string(),
                re_образец: Regex::new(r"(?i)-ующий\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ующих".to_string(),
                замена: "ующих".to_string(),
                re_образец: Regex::new(r"(?i)-ующих\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уется".to_string(),
                замена: "уется".to_string(),
                re_образец: Regex::new(r"(?i)-уется\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уются".to_string(),
                замена: "уются".to_string(),
                re_образец: Regex::new(r"(?i)-уются\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ичную".to_string(),
                замена: "ичную".to_string(),
                re_образец: Regex::new(r"(?i)-ичную\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ичных".to_string(),
                замена: "ичных".to_string(),
                re_образец: Regex::new(r"(?i)-ичных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ного".to_string(),
                замена: "ного".to_string(),
                re_образец: Regex::new(r"(?i)-ного\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ость".to_string(),
                замена: "ость".to_string(),
                re_образец: Regex::new(r"(?i)-ость\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ости".to_string(),
                замена: "ости".to_string(),
                re_образец: Regex::new(r"(?i)-ости\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-остью".to_string(),
                замена: "остью".to_string(),
                re_образец: Regex::new(r"(?i)-остью\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нные".to_string(),
                замена: "нные".to_string(),
                re_образец: Regex::new(r"(?i)-нные\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нного".to_string(),
                замена: "нного".to_string(),
                re_образец: Regex::new(r"(?i)-нного\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нный".to_string(),
                замена: "нный".to_string(),
                re_образец: Regex::new(r"(?i)-нный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нных".to_string(),
                замена: "нных".to_string(),
                re_образец: Regex::new(r"(?i)-нных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уете".to_string(),
                замена: "уете".to_string(),
                re_образец: Regex::new(r"(?i)-уете\>").unwrap(),
            },
        ],
        трехбуквенные: vec![
            Ячейка_замены {
                искомое_слово: "-ter".to_string(),
                замена: "ter".to_string(),
                re_образец: Regex::new(r"(?i)-ter\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-tus".to_string(),
                замена: "tus".to_string(),
                re_образец: Regex::new(r"(?i)-tus\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-tom".to_string(),
                замена: "tom".to_string(),
                re_образец: Regex::new(r"(?i)-tom\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ции".to_string(),
                замена: "ции".to_string(),
                re_образец: Regex::new(r"(?i)-ции\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кам".to_string(),
                замена: "кам".to_string(),
                re_образец: Regex::new(r"(?i)-кам\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тём".to_string(),
                замена: "тём".to_string(),
                re_образец: Regex::new(r"(?i)-тём\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щью".to_string(),
                замена: "щью".to_string(),
                re_образец: Regex::new(r"(?i)-щью\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лом".to_string(),
                замена: "лом".to_string(),
                re_образец: Regex::new(r"(?i)-лом\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дан".to_string(),
                замена: "дан".to_string(),
                re_образец: Regex::new(r"(?i)-дан\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ста".to_string(),
                замена: "ста".to_string(),
                re_образец: Regex::new(r"(?i)-ста\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тия".to_string(),
                замена: "тия".to_string(),
                re_образец: Regex::new(r"(?i)-тия\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дой".to_string(),
                замена: "дой".to_string(),
                re_образец: Regex::new(r"(?i)-дой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вая".to_string(),
                замена: "вая".to_string(),
                re_образец: Regex::new(r"(?i)-вая\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ния".to_string(),
                замена: "ния".to_string(),
                re_образец: Regex::new(r"(?i)-ния\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лон".to_string(),
                замена: "лон".to_string(),
                re_образец: Regex::new(r"(?i)-лон\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рых".to_string(),
                замена: "рых".to_string(),
                re_образец: Regex::new(r"(?i)-рых\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ное".to_string(),
                замена: "ное".to_string(),
                re_образец: Regex::new(r"(?i)-ное\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рый".to_string(),
                замена: "рый".to_string(),
                re_образец: Regex::new(r"(?i)-рый\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мые".to_string(),
                замена: "мые".to_string(),
                re_образец: Regex::new(r"(?i)-мые\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щем".to_string(),
                замена: "щем".to_string(),
                re_образец: Regex::new(r"(?i)-щем\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ной".to_string(),
                замена: "ной".to_string(),
                re_образец: Regex::new(r"(?i)-ной\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ний".to_string(),
                замена: "ний".to_string(),
                re_образец: Regex::new(r"(?i)-ний\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зок".to_string(),
                замена: "зок".to_string(),
                re_образец: Regex::new(r"(?i)-зок\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тем".to_string(),
                замена: "тем".to_string(),
                re_образец: Regex::new(r"(?i)-тем\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ные".to_string(),
                замена: "ные".to_string(),
                re_образец: Regex::new(r"(?i)-ные\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нию".to_string(),
                замена: "нию".to_string(),
                re_образец: Regex::new(r"(?i)-нию\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шин".to_string(),
                замена: "шин".to_string(),
                re_образец: Regex::new(r"(?i)-шин\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тый".to_string(),
                замена: "тый".to_string(),
                re_образец: Regex::new(r"(?i)-тый\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нюю".to_string(),
                замена: "нюю".to_string(),
                re_образец: Regex::new(r"(?i)-нюю\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гда".to_string(),
                замена: "гда".to_string(),
                re_образец: Regex::new(r"(?i)-гда\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бой".to_string(),
                замена: "бой".to_string(),
                re_образец: Regex::new(r"(?i)-бой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вые".to_string(),
                замена: "вые".to_string(),
                re_образец: Regex::new(r"(?i)-вые\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дов".to_string(),
                замена: "дов".to_string(),
                re_образец: Regex::new(r"(?i)-дов\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тов".to_string(),
                замена: "тов".to_string(),
                re_образец: Regex::new(r"(?i)-тов\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пей".to_string(),
                замена: "пей".to_string(),
                re_образец: Regex::new(r"(?i)-пей\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мый".to_string(),
                замена: "мый".to_string(),
                re_образец: Regex::new(r"(?i)-мый\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-nal".to_string(),
                замена: "nal".to_string(),
                re_образец: Regex::new(r"(?i)-nal\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щие".to_string(),
                замена: "щие".to_string(),
                re_образец: Regex::new(r"(?i)-щие\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вой".to_string(),
                замена: "вой".to_string(),
                re_образец: Regex::new(r"(?i)-вой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ром".to_string(),
                замена: "ром".to_string(),
                re_образец: Regex::new(r"(?i)-ром\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мер".to_string(),
                замена: "мер".to_string(),
                re_образец: Regex::new(r"(?i)-мер\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-них".to_string(),
                замена: "них".to_string(),
                re_образец: Regex::new(r"(?i)-них\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кие".to_string(),
                замена: "кие".to_string(),
                re_образец: Regex::new(r"(?i)-кие\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чет".to_string(),
                замена: "чет".to_string(),
                re_образец: Regex::new(r"(?i)-чет\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ект".to_string(),
                замена: "ект".to_string(),
                re_образец: Regex::new(r"(?i)-ект\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жет".to_string(),
                замена: "жет".to_string(),
                re_образец: Regex::new(r"(?i)-жет\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ную".to_string(),
                замена: "ную".to_string(),
                re_образец: Regex::new(r"(?i)-ную\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ком".to_string(),
                замена: "ком".to_string(),
                re_образец: Regex::new(r"(?i)-ком\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вил".to_string(),
                замена: "вил".to_string(),
                re_образец: Regex::new(r"(?i)-вил\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тым".to_string(),
                замена: "тым".to_string(),
                re_образец: Regex::new(r"(?i)-тым\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ких".to_string(),
                замена: "ких".to_string(),
                re_образец: Regex::new(r"(?i)-ких\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вым".to_string(),
                замена: "вым".to_string(),
                re_образец: Regex::new(r"(?i)-вым\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зом".to_string(),
                замена: "зом".to_string(),
                re_образец: Regex::new(r"(?i)-зом\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рой".to_string(),
                замена: "рой".to_string(),
                re_образец: Regex::new(r"(?i)-рой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чек".to_string(),
                замена: "чек".to_string(),
                re_образец: Regex::new(r"(?i)-чек\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-той".to_string(),
                замена: "той".to_string(),
                re_образец: Regex::new(r"(?i)-той\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гут".to_string(),
                замена: "гут".to_string(),
                re_образец: Regex::new(r"(?i)-гут\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ние".to_string(),
                замена: "ние".to_string(),
                re_образец: Regex::new(r"(?i)-ние\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ный".to_string(),
                замена: "ный".to_string(),
                re_образец: Regex::new(r"(?i)-ный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ных".to_string(),
                замена: "ных".to_string(),
                re_образец: Regex::new(r"(?i)-ных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кой".to_string(),
                замена: "кой".to_string(),
                re_образец: Regex::new(r"(?i)-кой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ала".to_string(),
                замена: "ала".to_string(),
                re_образец: Regex::new(r"(?i)-ала\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уют".to_string(),
                замена: "уют".to_string(),
                re_образец: Regex::new(r"(?i)-уют\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-еям".to_string(),
                замена: "еям".to_string(),
                re_образец: Regex::new(r"(?i)-еям\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нат".to_string(),
                замена: "нат".to_string(),
                re_образец: Regex::new(r"(?i)-нат\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иев".to_string(),
                замена: "иев".to_string(),
                re_образец: Regex::new(r"(?i)-иев\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иал".to_string(),
                замена: "иал".to_string(),
                re_образец: Regex::new(r"(?i)-иал\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ием".to_string(),
                замена: "ием".to_string(),
                re_образец: Regex::new(r"(?i)-ием\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иум".to_string(),
                замена: "иум".to_string(),
                re_образец: Regex::new(r"(?i)-иум\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ыми".to_string(),
                замена: "ыми".to_string(),
                re_образец: Regex::new(r"(?i)-ыми\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чим".to_string(),
                замена: "чим".to_string(),
                re_образец: Regex::new(r"(?i)-чим\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ика".to_string(),
                замена: "ика".to_string(),
                re_образец: Regex::new(r"(?i)-ика\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ику".to_string(),
                замена: "ику".to_string(),
                re_образец: Regex::new(r"(?i)-ику\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ики".to_string(),
                замена: "ики".to_string(),
                re_образец: Regex::new(r"(?i)-ики\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ать".to_string(),
                замена: "ать".to_string(),
                re_образец: Regex::new(r"(?i)-ать\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ять".to_string(),
                замена: "ять".to_string(),
                re_образец: Regex::new(r"(?i)-ять\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ным".to_string(),
                замена: "ным".to_string(),
                re_образец: Regex::new(r"(?i)-ным\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-еть".to_string(),
                замена: "еть".to_string(),
                re_образец: Regex::new(r"(?i)-еть\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ная".to_string(),
                замена: "ная".to_string(),
                re_образец: Regex::new(r"(?i)-ная\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лен".to_string(),
                замена: "лен".to_string(),
                re_образец: Regex::new(r"(?i)-лен\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иям".to_string(),
                замена: "иям".to_string(),
                re_образец: Regex::new(r"(?i)-иям\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дом".to_string(),
                замена: "дом".to_string(),
                re_образец: Regex::new(r"(?i)-дом\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-sor".to_string(),
                замена: "sor".to_string(),
                re_образец: Regex::new(r"(?i)-sor\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уум".to_string(),
                замена: "уум".to_string(),
                re_образец: Regex::new(r"(?i)-уум\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уем".to_string(),
                замена: "уем".to_string(),
                re_образец: Regex::new(r"(?i)-уем\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ким".to_string(),
                замена: "ким".to_string(),
                re_образец: Regex::new(r"(?i)-ким\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ешь".to_string(),
                замена: "ешь".to_string(),
                re_образец: Regex::new(r"(?i)-ешь\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ишь".to_string(),
                замена: "ишь".to_string(),
                re_образец: Regex::new(r"(?i)-ишь\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ток".to_string(),
                замена: "ток".to_string(),
                re_образец: Regex::new(r"(?i)-ток\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ете".to_string(),
                замена: "ете".to_string(),
                re_образец: Regex::new(r"(?i)-ете\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ите".to_string(),
                замена: "ите".to_string(),
                re_образец: Regex::new(r"(?i)-ите\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ует".to_string(),
                замена: "ует".to_string(),
                re_образец: Regex::new(r"(?i)-ует\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-яла".to_string(),
                замена: "яла".to_string(),
                re_образец: Regex::new(r"(?i)-яла\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-али".to_string(),
                замена: "али".to_string(),
                re_образец: Regex::new(r"(?i)-али\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-яли".to_string(),
                замена: "яли".to_string(),
                re_образец: Regex::new(r"(?i)-яли\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ола".to_string(),
                замена: "ола".to_string(),
                re_образец: Regex::new(r"(?i)-ола\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ела".to_string(),
                замена: "ела".to_string(),
                re_образец: Regex::new(r"(?i)-ела\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-оли".to_string(),
                замена: "оли".to_string(),
                re_образец: Regex::new(r"(?i)-оли\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ели".to_string(),
                замена: "ели".to_string(),
                re_образец: Regex::new(r"(?i)-ели\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ула".to_string(),
                замена: "ула".to_string(),
                re_образец: Regex::new(r"(?i)-ула\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ули".to_string(),
                замена: "ули".to_string(),
                re_образец: Regex::new(r"(?i)-ули\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ами".to_string(),
                замена: "ами".to_string(),
                re_образец: Regex::new(r"(?i)-ами\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-еми".to_string(),
                замена: "еми".to_string(),
                re_образец: Regex::new(r"(?i)-еми\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-емя".to_string(),
                замена: "емя".to_string(),
                re_образец: Regex::new(r"(?i)-емя\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ёте".to_string(),
                замена: "ёте".to_string(),
                re_образец: Regex::new(r"(?i)-ёте\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чие".to_string(),
                замена: "чие".to_string(),
                re_образец: Regex::new(r"(?i)-чие\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сте".to_string(),
                замена: "сте".to_string(),
                re_образец: Regex::new(r"(?i)-сте\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ёшь".to_string(),
                замена: "ёшь".to_string(),
                re_образец: Regex::new(r"(?i)-ёшь\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-том".to_string(),
                замена: "том".to_string(),
                re_образец: Regex::new(r"(?i)-том\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ого".to_string(),
                замена: "ого".to_string(),
                re_образец: Regex::new(r"(?i)-ого\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ций".to_string(),
                замена: "ций".to_string(),
                re_образец: Regex::new(r"(?i)-ций\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жен".to_string(),
                замена: "жен".to_string(),
                re_образец: Regex::new(r"(?i)-жен\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ому".to_string(),
                замена: "ому".to_string(),
                re_образец: Regex::new(r"(?i)-ому\>").unwrap(),
            },

            Ячейка_замены {
                искомое_слово: "-дач".to_string(),
                замена: "дач".to_string(),
                re_образец: Regex::new(r"(?i)-дач\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-иях".to_string(),
                замена: "иях".to_string(),
                re_образец: Regex::new(r"(?i)-иях\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ией".to_string(),
                замена: "ией".to_string(),
                re_образец: Regex::new(r"(?i)-ией\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-умя".to_string(),
                замена: "умя".to_string(),
                re_образец: Regex::new(r"(?i)-умя\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ими".to_string(),
                замена: "ими".to_string(),
                re_образец: Regex::new(r"(?i)-ими\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тор".to_string(),
                замена: "тор".to_string(),
                re_образец: Regex::new(r"(?i)-тор\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рые".to_string(),
                замена: "рые".to_string(),
                re_образец: Regex::new(r"(?i)-рые\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сти".to_string(),
                замена: "сти".to_string(),
                re_образец: Regex::new(r"(?i)-сти\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чае".to_string(),
                замена: "чае".to_string(),
                re_образец: Regex::new(r"(?i)-чае\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вод".to_string(),
                замена: "вод".to_string(),
                re_образец: Regex::new(r"(?i)-вод\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лов".to_string(),
                замена: "лов".to_string(),
                re_образец: Regex::new(r"(?i)-лов\>").unwrap(),
            },
        ],
        двубуквенные: vec![
            Ячейка_замены {
                искомое_слово: "-er".to_string(),
                замена: "er".to_string(),
                re_образец: Regex::new(r"(?i)-er\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-го".to_string(),
                замена: "го".to_string(),
                re_образец: Regex::new(r"(?i)-го\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-на".to_string(),
                замена: "на".to_string(),
                re_образец: Regex::new(r"(?i)-на\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мы".to_string(),
                замена: "мы".to_string(),
                re_образец: Regex::new(r"(?i)-мы\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-му".to_string(),
                замена: "му".to_string(),
                re_образец: Regex::new(r"(?i)-му\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ры".to_string(),
                замена: "ры".to_string(),
                re_образец: Regex::new(r"(?i)-ры\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ра".to_string(),
                замена: "ра".to_string(),
                re_образец: Regex::new(r"(?i)-ра\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ты".to_string(),
                замена: "ты".to_string(),
                re_образец: Regex::new(r"(?i)-ты\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ка".to_string(),
                замена: "ка".to_string(),
                re_образец: Regex::new(r"(?i)-ка\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ло".to_string(),
                замена: "ло".to_string(),
                re_образец: Regex::new(r"(?i)-ло\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жа".to_string(),
                замена: "жа".to_string(),
                re_образец: Regex::new(r"(?i)-жа\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-та".to_string(),
                замена: "та".to_string(),
                re_образец: Regex::new(r"(?i)-та\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ли".to_string(),
                замена: "ли".to_string(),
                re_образец: Regex::new(r"(?i)-ли\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ея".to_string(),
                замена: "ея".to_string(),
                re_образец: Regex::new(r"(?i)-ея\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-еи".to_string(),
                замена: "еи".to_string(),
                re_образец: Regex::new(r"(?i)-еи\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ях".to_string(),
                замена: "ях".to_string(),
                re_образец: Regex::new(r"(?i)-ях\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ев".to_string(),
                замена: "ев".to_string(),
                re_образец: Regex::new(r"(?i)-ев\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ки".to_string(),
                замена: "ки".to_string(),
                re_образец: Regex::new(r"(?i)-ки\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ым".to_string(),
                замена: "ым".to_string(),
                re_образец: Regex::new(r"(?i)-ым\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-да".to_string(),
                замена: "да".to_string(),
                re_образец: Regex::new(r"(?i)-да\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ых".to_string(),
                замена: "ых".to_string(),
                re_образец: Regex::new(r"(?i)-ых\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ям".to_string(),
                замена: "ям".to_string(),
                re_образец: Regex::new(r"(?i)-ям\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ии".to_string(),
                замена: "ии".to_string(),
                re_образец: Regex::new(r"(?i)-ии\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ия".to_string(),
                замена: "ия".to_string(),
                re_образец: Regex::new(r"(?i)-ия\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ся".to_string(),
                замена: "ся".to_string(),
                re_образец: Regex::new(r"(?i)-ся\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ая".to_string(),
                замена: "ая".to_string(),
                re_образец: Regex::new(r"(?i)-ая\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-яя".to_string(),
                замена: "яя".to_string(),
                re_образец: Regex::new(r"(?i)-яя\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ое".to_string(),
                замена: "ое".to_string(),
                re_образец: Regex::new(r"(?i)-ое\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ее".to_string(),
                замена: "ее".to_string(),
                re_образец: Regex::new(r"(?i)-ее\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ой".to_string(),
                замена: "ой".to_string(),
                re_образец: Regex::new(r"(?i)-ой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ые".to_string(),
                замена: "ые".to_string(),
                re_образец: Regex::new(r"(?i)-ые\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ый".to_string(),
                замена: "ый".to_string(),
                re_образец: Regex::new(r"(?i)-ый\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ий".to_string(),
                замена: "ий".to_string(),
                re_образец: Regex::new(r"(?i)-ий\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ем".to_string(),
                замена: "ем".to_string(),
                re_образец: Regex::new(r"(?i)-ем\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-им".to_string(),
                замена: "им".to_string(),
                re_образец: Regex::new(r"(?i)-им\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ет".to_string(),
                замена: "ет".to_string(),
                re_образец: Regex::new(r"(?i)-ет\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ит".to_string(),
                замена: "ит".to_string(),
                re_образец: Regex::new(r"(?i)-ит\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ут".to_string(),
                замена: "ут".to_string(),
                re_образец: Regex::new(r"(?i)-ут\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ру".to_string(),
                замена: "ру".to_string(),
                re_образец: Regex::new(r"(?i)-ру\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ют".to_string(),
                замена: "ют".to_string(),
                re_образец: Regex::new(r"(?i)-ют\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ят".to_string(),
                замена: "ят".to_string(),
                re_образец: Regex::new(r"(?i)-ят\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ял".to_string(),
                замена: "ял".to_string(),
                re_образец: Regex::new(r"(?i)-ял\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ол".to_string(),
                замена: "ол".to_string(),
                re_образец: Regex::new(r"(?i)-ол\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ел".to_string(),
                замена: "ел".to_string(),
                re_образец: Regex::new(r"(?i)-ел\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ул".to_string(),
                замена: "ул".to_string(),
                re_образец: Regex::new(r"(?i)-ул\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ам".to_string(),
                замена: "ам".to_string(),
                re_образец: Regex::new(r"(?i)-ам\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ас".to_string(),
                замена: "ас".to_string(),
                re_образец: Regex::new(r"(?i)-ас\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ах".to_string(),
                замена: "ах".to_string(),
                re_образец: Regex::new(r"(?i)-ах\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ко".to_string(),
                замена: "ко".to_string(),
                re_образец: Regex::new(r"(?i)-ко\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-её".to_string(),
                замена: "её".to_string(),
                re_образец: Regex::new(r"(?i)-её\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ей".to_string(),
                замена: "ей".to_string(),
                re_образец: Regex::new(r"(?i)-ей\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ех".to_string(),
                замена: "ех".to_string(),
                re_образец: Regex::new(r"(?i)-ех\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ею".to_string(),
                замена: "ею".to_string(),
                re_образец: Regex::new(r"(?i)-ею\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ёт".to_string(),
                замена: "ёт".to_string(),
                re_образец: Regex::new(r"(?i)-ёт\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ёх".to_string(),
                замена: "ёх".to_string(),
                re_образец: Regex::new(r"(?i)-ёх\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ие".to_string(),
                замена: "ие".to_string(),
                re_образец: Regex::new(r"(?i)-ие\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-их".to_string(),
                замена: "их".to_string(),
                re_образец: Regex::new(r"(?i)-их\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ию".to_string(),
                замена: "ию".to_string(),
                re_образец: Regex::new(r"(?i)-ию\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-но".to_string(),
                замена: "но".to_string(),
                re_образец: Regex::new(r"(?i)-но\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ми".to_string(),
                замена: "ми".to_string(),
                re_образец: Regex::new(r"(?i)-ми\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мя".to_string(),
                замена: "мя".to_string(),
                re_образец: Regex::new(r"(?i)-мя\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ов".to_string(),
                замена: "ов".to_string(),
                re_образец: Regex::new(r"(?i)-ов\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-оё".to_string(),
                замена: "оё".to_string(),
                re_образец: Regex::new(r"(?i)-оё\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ом".to_string(),
                замена: "ом".to_string(),
                re_образец: Regex::new(r"(?i)-ом\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-см".to_string(),
                замена: "см".to_string(),
                re_образец: Regex::new(r"(?i)-см\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ум".to_string(),
                замена: "ум".to_string(),
                re_образец: Regex::new(r"(?i)-ум\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-уя".to_string(),
                замена: "уя".to_string(),
                re_образец: Regex::new(r"(?i)-уям\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ух".to_string(),
                замена: "ух".to_string(),
                re_образец: Regex::new(r"(?i)-ух\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ую".to_string(),
                замена: "ую".to_string(),
                re_образец: Regex::new(r"(?i)-ую\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шь".to_string(),
                замена: "шь".to_string(),
                re_образец: Regex::new(r"(?i)-шь\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ны".to_string(),
                замена: "ны".to_string(),
                re_образец: Regex::new(r"(?i)-ны\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пи".to_string(),
                замена: "пи".to_string(),
                re_образец: Regex::new(r"(?i)-пи\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-па".to_string(),
                замена: "па".to_string(),
                re_образец: Regex::new(r"(?i)-па\>").unwrap(),
            },
        ],
        целиковые: vec![
            Ячейка_замены {
                искомое_слово: "-валентных".to_string(),
                замена: "валентных".to_string(),
                re_образец: Regex::new(r"(?i)-валентных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-поминающих".to_string(),
                замена: "поминающих".to_string(),
                re_образец: Regex::new(r"(?i)-поминающих\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зации".to_string(),
                замена: "зации".to_string(),
                re_образец: Regex::new(r"(?i)-зации\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-денции".to_string(),
                замена: "денции".to_string(),
                re_образец: Regex::new(r"(?i)-денции\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-личаются".to_string(),
                замена: "личаются".to_string(),
                re_образец: Regex::new(r"(?i)-личаются\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ровать".to_string(),
                замена: "ровать".to_string(),
                re_образец: Regex::new(r"(?i)-ровать\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тельными".to_string(),
                замена: "тельными".to_string(),
                re_образец: Regex::new(r"(?i)-тельными\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рифмический".to_string(),
                замена: "рифмический".to_string(),
                re_образец: Regex::new(r"(?i)-рифмический\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рительными".to_string(),
                замена: "рительными".to_string(),
                re_образец: Regex::new(r"(?i)-рительными\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лучила".to_string(),
                замена: "лучила".to_string(),
                re_образец: Regex::new(r"(?i)-лучила\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пульсный".to_string(),
                замена: "пульсный".to_string(),
                re_образец: Regex::new(r"(?i)-пульсный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-менными".to_string(),
                замена: "менными".to_string(),
                re_образец: Regex::new(r"(?i)-менными\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-правленный".to_string(),
                замена: "правленный".to_string(),
                re_образец: Regex::new(r"(?i)-правленный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зится".to_string(),
                замена: "зится".to_string(),
                re_образец: Regex::new(r"(?i)-зится\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дификацию".to_string(),
                замена: "дификацию".to_string(),
                re_образец: Regex::new(r"(?i)-дификацию\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ляться".to_string(),
                замена: "ляться".to_string(),
                re_образец: Regex::new(r"(?i)-ляться\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рительной".to_string(),
                замена: "рительной".to_string(),
                re_образец: Regex::new(r"(?i)-рительной\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зических".to_string(),
                замена: "зических".to_string(),
                re_образец: Regex::new(r"(?i)-зических\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вается".to_string(),
                замена: "вается".to_string(),
                re_образец: Regex::new(r"(?i)-вается\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-корректности".to_string(),
                замена: "корректности".to_string(),
                re_образец: Regex::new(r"(?i)-корректности\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-руется".to_string(),
                замена: "руется".to_string(),
                re_образец: Regex::new(r"(?i)-руется\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-совано".to_string(),
                замена: "совано".to_string(),
                re_образец: Regex::new(r"(?i)-совано\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-турой".to_string(),
                замена: "турой".to_string(),
                re_образец: Regex::new(r"(?i)-турой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пустимого".to_string(),
                замена: "пустимого".to_string(),
                re_образец: Regex::new(r"(?i)-пустимого\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стовый".to_string(),
                замена: "стовый".to_string(),
                re_образец: Regex::new(r"(?i)-стовый\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стояние".to_string(),
                замена: "стояние".to_string(),
                re_образец: Regex::new(r"(?i)-стояние\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ствами".to_string(),
                замена: "ствами".to_string(),
                re_образец: Regex::new(r"(?i)-ствами\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гическую".to_string(),
                замена: "гическую".to_string(),
                re_образец: Regex::new(r"(?i)-гическую\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шинного".to_string(),
                замена: "шинного".to_string(),
                re_образец: Regex::new(r"(?i)-шинного\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-матном".to_string(),
                замена: "матном".to_string(),
                re_образец: Regex::new(r"(?i)-матном\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-значены".to_string(),
                замена: "значены".to_string(),
                re_образец: Regex::new(r"(?i)-значены\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нальные".to_string(),
                замена: "нальные".to_string(),
                re_образец: Regex::new(r"(?i)-нальные\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-крепленные".to_string(),
                замена: "крепленные".to_string(),
                re_образец: Regex::new(r"(?i)-крепленные\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тимальности".to_string(),
                замена: "тимальности".to_string(),
                re_образец: Regex::new(r"(?i)-тимальности\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гональных".to_string(),
                замена: "гональных".to_string(),
                re_образец: Regex::new(r"(?i)-гональных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чезнут".to_string(),
                замена: "чезнут".to_string(),
                re_образец: Regex::new(r"(?i)-чезнут\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кание".to_string(),
                замена: "кание".to_string(),
                re_образец: Regex::new(r"(?i)-кание\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гаться".to_string(),
                замена: "гаться".to_string(),
                re_образец: Regex::new(r"(?i)-гаться\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зируя".to_string(),
                замена: "зируя".to_string(),
                re_образец: Regex::new(r"(?i)-зируя\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рячими".to_string(),
                замена: "рячими".to_string(),
                re_образец: Regex::new(r"(?i)-рячими\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ливаемое".to_string(),
                замена: "ливаемое".to_string(),
                re_образец: Regex::new(r"(?i)-ливаемое\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лагаемый".to_string(),
                замена: "лагаемый".to_string(),
                re_образец: Regex::new(r"(?i)-лагаемый\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ритета".to_string(),
                замена: "ритета".to_string(),
                re_образец: Regex::new(r"(?i)-ритета\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-почтительный".to_string(),
                замена: "почтительный".to_string(),
                re_образец: Regex::new(r"(?i)-почтительный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ляющее".to_string(),
                замена: "ляющее".to_string(),
                re_образец: Regex::new(r"(?i)-ляющее\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нейкой".to_string(),
                замена: "нейкой".to_string(),
                re_образец: Regex::new(r"(?i)-нейкой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-хождении".to_string(),
                замена: "хождении".to_string(),
                re_образец: Regex::new(r"(?i)-хождении\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-исходит".to_string(),
                замена: "исходит".to_string(),
                re_образец: Regex::new(r"(?i)-исходит\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-метров".to_string(),
                замена: "метров".to_string(),
                re_образец: Regex::new(r"(?i)-метров\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ства".to_string(),
                замена: "ства".to_string(),
                re_образец: Regex::new(r"(?i)-ства\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ровой".to_string(),
                замена: "ровой".to_string(),
                re_образец: Regex::new(r"(?i)-ровой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-знаку".to_string(),
                замена: "знаку".to_string(),
                re_образец: Regex::new(r"(?i)-знаку\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-числены".to_string(),
                замена: "числены".to_string(),
                re_образец: Regex::new(r"(?i)-числены\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рованы".to_string(),
                замена: "рованы".to_string(),
                re_образец: Regex::new(r"(?i)-рованы\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-межуточных".to_string(),
                замена: "межуточных".to_string(),
                re_образец: Regex::new(r"(?i)-межуточных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-гласование".to_string(),
                замена: "гласование".to_string(),
                re_образец: Regex::new(r"(?i)-гласование\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-обходимое".to_string(),
                замена: "обходимое".to_string(),
                re_образец: Regex::new(r"(?i)-обходимое\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-новления".to_string(),
                замена: "новления".to_string(),
                re_образец: Regex::new(r"(?i)-новления\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ских".to_string(),
                замена: "ских".to_string(),
                re_образец: Regex::new(r"(?i)-ских\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-данса".to_string(),
                замена: "данса".to_string(),
                re_образец: Regex::new(r"(?i)-данса\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-фектов".to_string(),
                замена: "фектов".to_string(),
                re_образец: Regex::new(r"(?i)-фектов\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-редач".to_string(),
                замена: "редач".to_string(),
                re_образец: Regex::new(r"(?i)-редач\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нитные".to_string(),
                замена: "нитные".to_string(),
                re_образец: Regex::new(r"(?i)-нитные\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ключается".to_string(),
                замена: "ключается".to_string(),
                re_образец: Regex::new(r"(?i)-ключается\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ментов".to_string(),
                замена: "ментов".to_string(),
                re_образец: Regex::new(r"(?i)-ментов\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-граммный".to_string(),
                замена: "граммный".to_string(),
                re_образец: Regex::new(r"(?i)-граммный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вания".to_string(),
                замена: "вания".to_string(),
                re_образец: Regex::new(r"(?i)-вания\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шений".to_string(),
                замена: "шений".to_string(),
                re_образец: Regex::new(r"(?i)-шений\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-никло".to_string(),
                замена: "никло".to_string(),
                re_образец: Regex::new(r"(?i)-никло\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чиком".to_string(),
                замена: "чиком".to_string(),
                re_образец: Regex::new(r"(?i)-чиком\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чатных".to_string(),
                замена: "чатных".to_string(),
                re_образец: Regex::new(r"(?i)-чатных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-полняются".to_string(),
                замена: "полняются".to_string(),
                re_образец: Regex::new(r"(?i)-полняются\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нелей".to_string(),
                замена: "нелей".to_string(),
                re_образец: Regex::new(r"(?i)-нелей\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-торые".to_string(),
                замена: "торые".to_string(),
                re_образец: Regex::new(r"(?i)-торые\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тально".to_string(),
                замена: "тально".to_string(),
                re_образец: Regex::new(r"(?i)-тально\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-менно".to_string(),
                замена: "менно".to_string(),
                re_образец: Regex::new(r"(?i)-менно\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-торая".to_string(),
                замена: "торая".to_string(),
                re_образец: Regex::new(r"(?i)-торая\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-раммного".to_string(),
                замена: "раммного".to_string(),
                re_образец: Regex::new(r"(?i)-раммного\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мендуется".to_string(),
                замена: "мендуется".to_string(),
                re_образец: Regex::new(r"(?i)-мендуется\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-крытый".to_string(),
                замена: "крытый".to_string(),
                re_образец: Regex::new(r"(?i)-крытый\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тивным".to_string(),
                замена: "тивным".to_string(),
                re_образец: Regex::new(r"(?i)-тивным\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-манды".to_string(),
                замена: "манды".to_string(),
                re_образец: Regex::new(r"(?i)-манды\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец: Regex::new(r"(?i)-тронной\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-численных".to_string(),
                замена: "численных".to_string(),
                re_образец: Regex::new(r"(?i)-численных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ленную".to_string(),
                замена: "ленную".to_string(),
                re_образец: Regex::new(r"(?i)-ленную\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стемный".to_string(),
                замена: "стемный".to_string(),
                re_образец: Regex::new(r"(?i)-стемный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ческих".to_string(),
                замена: "ческих".to_string(),
                re_образец: Regex::new(r"(?i)-ческих\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тура".to_string(),
                замена: "тура".to_string(),
                re_образец: Regex::new(r"(?i)-тура\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ждений".to_string(),
                замена: "ждений".to_string(),
                re_образец: Regex::new(r"(?i)-ждений\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шемся".to_string(),
                замена: "шемся".to_string(),
                re_образец: Regex::new(r"(?i)-шемся\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мента".to_string(),
                замена: "мента".to_string(),
                re_образец: Regex::new(r"(?i)-мента\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мандой".to_string(),
                замена: "мандой".to_string(),
                re_образец: Regex::new(r"(?i)-мандой\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тинные".to_string(),
                замена: "тинные".to_string(),
                re_образец: Regex::new(r"(?i)-тинные\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нель".to_string(),
                замена: "нель".to_string(),
                re_образец: Regex::new(r"(?i)-нель\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сутствует".to_string(),
                замена: "сутствует".to_string(),
                re_образец: Regex::new(r"(?i)-сутствует\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-симо".to_string(),
                замена: "симо".to_string(),
                re_образец: Regex::new(r"(?i)-симо\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пени".to_string(),
                замена: "пени".to_string(),
                re_образец: Regex::new(r"(?i)-пени\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тельно".to_string(),
                замена: "тельно".to_string(),
                re_образец: Regex::new(r"(?i)-тельно\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чанию".to_string(),
                замена: "чанию".to_string(),
                re_образец: Regex::new(r"(?i)-чанию\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ческая".to_string(),
                замена: "ческая".to_string(),
                re_образец: Regex::new(r"(?i)-ческая\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бирать".to_string(),
                замена: "бирать".to_string(),
                re_образец: Regex::new(r"(?i)-бирать\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-единитель".to_string(),
                замена: "единитель".to_string(),
                re_образец: Regex::new(r"(?i)-единитель\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ному".to_string(),
                замена: "ному".to_string(),
                re_образец: Regex::new(r"(?i)-ному\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зуемся".to_string(),
                замена: "зуемся".to_string(),
                re_образец: Regex::new(r"(?i)-зуемся\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ветствующие".to_string(),
                замена: "ветствующие".to_string(),
                re_образец: Regex::new(r"(?i)-ветствующие\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-матическая".to_string(),
                замена: "матическая".to_string(),
                re_образец: Regex::new(r"(?i)-матическая\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нентов".to_string(),
                замена: "нентов".to_string(),
                re_образец: Regex::new(r"(?i)-нентов\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нала".to_string(),
                замена: "нала".to_string(),
                re_образец: Regex::new(r"(?i)-нала\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тистические".to_string(),
                замена: "тистические".to_string(),
                re_образец: Regex::new(r"(?i)-тистические\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стимо".to_string(),
                замена: "стимо".to_string(),
                re_образец: Regex::new(r"(?i)-стимо\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жителем".to_string(),
                замена: "жителем".to_string(),
                re_образец: Regex::new(r"(?i)-жителем\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-товых".to_string(),
                замена: "товых".to_string(),
                re_образец: Regex::new(r"(?i)-товых\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-цессе".to_string(),
                замена: "цессе".to_string(),
                re_образец: Regex::new(r"(?i)-цессе\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-екта".to_string(),
                замена: "екта".to_string(),
                re_образец: Regex::new(r"(?i)-екта\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-новлены".to_string(),
                замена: "новлены".to_string(),
                re_образец: Regex::new(r"(?i)-новлены\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рования".to_string(),
                замена: "рования".to_string(),
                re_образец: Regex::new(r"(?i)-рования\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-раметры".to_string(),
                замена: "раметры".to_string(),
                re_образец: Regex::new(r"(?i)-раметры\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чески".to_string(),
                замена: "чески".to_string(),
                re_образец: Regex::new(r"(?i)-чески\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-брав".to_string(),
                замена: "брав".to_string(),
                re_образец: Regex::new(r"(?i)-брав\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-реноса".to_string(),
                замена: "реноса".to_string(),
                re_образец: Regex::new(r"(?i)-реноса\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зультаты".to_string(),
                замена: "зультаты".to_string(),
                re_образец: Regex::new(r"(?i)-зультаты\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ходных".to_string(),
                замена: "ходных".to_string(),
                re_образец: Regex::new(r"(?i)-ходных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тырех".to_string(),
                замена: "тырех".to_string(),
                re_образец: Regex::new(r"(?i)-тырех\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кать".to_string(),
                замена: "кать".to_string(),
                re_образец: Regex::new(r"(?i)-кать\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-мент".to_string(),
                замена: "мент".to_string(),
                re_образец: Regex::new(r"(?i)-мент\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-штаба".to_string(),
                замена: "штаба".to_string(),
                re_образец: Regex::new(r"(?i)-штаба\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-местно".to_string(),
                замена: "местно".to_string(),
                re_образец: Regex::new(r"(?i)-местно\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ления".to_string(),
                замена: "ления".to_string(),
                re_образец: Regex::new(r"(?i)-ления\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тактные".to_string(),
                замена: "тактные".to_string(),
                re_образец: Regex::new(r"(?i)-тактные\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-таллизации".to_string(),
                замена: "таллизации".to_string(),
                re_образец: Regex::new(r"(?i)-таллизации\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нить".to_string(),
                замена: "нить".to_string(),
                re_образец: Regex::new(r"(?i)-нить\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ветствующим".to_string(),
                замена: "ветствующим".to_string(),
                re_образец: Regex::new(r"(?i)-ветствующим\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-единения".to_string(),
                замена: "единения".to_string(),
                re_образец: Regex::new(r"(?i)-единения\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вать".to_string(),
                замена: "вать".to_string(),
                re_образец: Regex::new(r"(?i)-вать\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тически".to_string(),
                замена: "тически".to_string(),
                re_образец: Regex::new(r"(?i)-тически\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дами".to_string(),
                замена: "дами".to_string(),
                re_образец: Regex::new(r"(?i)-дами\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-борочно".to_string(),
                замена: "борочно".to_string(),
                re_образец: Regex::new(r"(?i)-борочно\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-веден".to_string(),
                замена: "веден".to_string(),
                re_образец: Regex::new(r"(?i)-веден\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ражает".to_string(),
                замена: "ражает".to_string(),
                re_образец: Regex::new(r"(?i)-ражает\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ством".to_string(),
                замена: "ством".to_string(),
                re_образец: Regex::new(r"(?i)-ством\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тора".to_string(),
                замена: "тора".to_string(),
                re_образец: Regex::new(r"(?i)-тора\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кусом".to_string(),
                замена: "кусом".to_string(),
                re_образец: Regex::new(r"(?i)-кусом\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-лучить".to_string(),
                замена: "лучить".to_string(),
                re_образец: Regex::new(r"(?i)-лучить\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вание".to_string(),
                замена: "вание".to_string(),
                re_образец: Regex::new(r"(?i)-вание\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рантирует".to_string(),
                замена: "рантирует".to_string(),
                re_образец: Regex::new(r"(?i)-рантирует\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-менных".to_string(),
                замена: "менных".to_string(),
                re_образец: Regex::new(r"(?i)-менных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ствующим".to_string(),
                замена: "ствующим".to_string(),
                re_образец: Regex::new(r"(?i)-ствующим\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тронных".to_string(),
                замена: "тронных".to_string(),
                re_образец: Regex::new(r"(?i)-тронных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-логического".to_string(),
                замена: "логического".to_string(),
                re_образец: Regex::new(r"(?i)-логического\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рину".to_string(),
                замена: "рину".to_string(),
                re_образец: Regex::new(r"(?i)-рину\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нент".to_string(),
                замена: "нент".to_string(),
                re_образец: Regex::new(r"(?i)-нент\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тива".to_string(),
                замена: "тива".to_string(),
                re_образец: Regex::new(r"(?i)-тива\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нений".to_string(),
                замена: "нений".to_string(),
                re_образец: Regex::new(r"(?i)-нений\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ченных".to_string(),
                замена: "ченных".to_string(),
                re_образец: Regex::new(r"(?i)-ченных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ченный".to_string(),
                замена: "ченный".to_string(),
                re_образец: Regex::new(r"(?i)-ченный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рации".to_string(),
                замена: "рации".to_string(),
                re_образец: Regex::new(r"(?i)-рации\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-митивов".to_string(),
                замена: "митивов".to_string(),
                re_образец: Regex::new(r"(?i)-торого\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щение".to_string(),
                замена: "щение".to_string(),
                re_образец: Regex::new(r"(?i)-щение\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-щего".to_string(),
                замена: "щего".to_string(),
                re_образец: Regex::new(r"(?i)-щего\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-виша".to_string(),
                замена: "виша".to_string(),
                re_образец: Regex::new(r"(?i)-виша\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ление".to_string(),
                замена: "ление".to_string(),
                re_образец: Regex::new(r"(?i)-ление\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рибуты".to_string(),
                замена: "рибуты".to_string(),
                re_образец: Regex::new(r"(?i)-рибуты\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-понент".to_string(),
                замена: "понент".to_string(),
                re_образец: Regex::new(r"(?i)-понент\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-понента".to_string(),
                замена: "понента".to_string(),
                re_образец: Regex::new(r"(?i)-понента\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-норамирования".to_string(),
                замена: "норамирования".to_string(),
                re_образец: Regex::new(r"(?i)-норамирования\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-можно".to_string(),
                замена: "можно".to_string(),
                re_образец: Regex::new(r"(?i)-можно\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-стра".to_string(),
                замена: "стра".to_string(),
                re_образец: Regex::new(r"(?i)-стра\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-изведен".to_string(),
                замена: "изведен".to_string(),
                re_образец: Regex::new(r"(?i)-изведен\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бранному".to_string(),
                замена: "бранному".to_string(),
                re_образец: Regex::new(r"(?i)-бранному\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вится".to_string(),
                замена: "вится".to_string(),
                re_образец: Regex::new(r"(?i)-вится\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-скую".to_string(),
                замена: "скую".to_string(),
                re_образец: Regex::new(r"(?i)-скую\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-митивов".to_string(),
                замена: "митивов".to_string(),
                re_образец: Regex::new(r"(?i)-митивов\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-струкция".to_string(),
                замена: "струкция".to_string(),
                re_образец: Regex::new(r"(?i)-струкция\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-торых".to_string(),
                замена: "торых".to_string(),
                re_образец: Regex::new(r"(?i)-торых\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-веденных".to_string(),
                замена: "веденных".to_string(),
                re_образец: Regex::new(r"(?i)-веденных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сколько".to_string(),
                замена: "сколько".to_string(),
                re_образец: Regex::new(r"(?i)-сколько\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ются".to_string(),
                замена: "ются".to_string(),
                re_образец: Regex::new(r"(?i)-ются\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ствуют".to_string(),
                замена: "ствуют".to_string(),
                re_образец: Regex::new(r"(?i)-ствуют\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-павшие".to_string(),
                замена: "павшие".to_string(),
                re_образец: Regex::new(r"(?i)-павшие\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-верстия".to_string(),
                замена: "верстия".to_string(),
                re_образец: Regex::new(r"(?i)-верстия\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ванные".to_string(),
                замена: "ванные".to_string(),
                re_образец: Regex::new(r"(?i)-ванные\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-реходных".to_string(),
                замена: "реходных".to_string(),
                re_образец: Regex::new(r"(?i)-реходных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-слойные".to_string(),
                замена: "слойные".to_string(),
                re_образец: Regex::new(r"(?i)-слойные\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-водится".to_string(),
                замена: "водится".to_string(),
                re_образец: Regex::new(r"(?i)-водится\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-вами".to_string(),
                замена: "вами".to_string(),
                re_образец: Regex::new(r"(?i)-вами\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-митивы".to_string(),
                замена: "митивы".to_string(),
                re_образец: Regex::new(r"(?i)-митивы\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пользуемых".to_string(),
                замена: "пользуемых".to_string(),
                re_образец: Regex::new(r"(?i)-пользуемых\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-няться".to_string(),
                замена: "няться".to_string(),
                re_образец: Regex::new(r"(?i)-няться\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дартов".to_string(),
                замена: "дартов".to_string(),
                re_образец: Regex::new(r"(?i)-дартов\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ность".to_string(),
                замена: "ность".to_string(),
                re_образец: Regex::new(r"(?i)-ность\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ленных".to_string(),
                замена: "ленных".to_string(),
                re_образец: Regex::new(r"(?i)-ленных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-пусках".to_string(),
                замена: "пусках".to_string(),
                re_образец: Regex::new(r"(?i)-пусках\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бавления".to_string(),
                замена: "бавления".to_string(),
                re_образец: Regex::new(r"(?i)-бавления\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дактировать".to_string(),
                замена: "дактировать".to_string(),
                re_образец: Regex::new(r"(?i)-дактировать\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тический".to_string(),
                замена: "тический".to_string(),
                re_образец: Regex::new(r"(?i)-тический\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дактор".to_string(),
                замена: "дактор".to_string(),
                re_образец: Regex::new(r"(?i)-дактор\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ретащим".to_string(),
                замена: "ретащим".to_string(),
                re_образец: Regex::new(r"(?i)-ретащим\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зицию".to_string(),
                замена: "зицию".to_string(),
                re_образец: Regex::new(r"(?i)-зицию\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рения".to_string(),
                замена: "рения".to_string(),
                re_образец: Regex::new(r"(?i)-рения\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-зателя".to_string(),
                замена: "зателя".to_string(),
                re_образец: Regex::new(r"(?i)-зателя\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-затель".to_string(),
                замена: "затель".to_string(),
                re_образец: Regex::new(r"(?i)-затель\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-водами".to_string(),
                замена: "водами".to_string(),
                re_образец: Regex::new(r"(?i)-водами\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-кладка".to_string(),
                замена: "кладка".to_string(),
                re_образец: Regex::new(r"(?i)-кладка\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-деления".to_string(),
                замена: "деления".to_string(),
                re_образец: Regex::new(r"(?i)-деления\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ражения".to_string(),
                замена: "ражения".to_string(),
                re_образец: Regex::new(r"(?i)-ражения\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-телем".to_string(),
                замена: "телем".to_string(),
                re_образец: Regex::new(r"(?i)-телем\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-садочных".to_string(),
                замена: "садочных".to_string(),
                re_образец: Regex::new(r"(?i)-садочных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-дактора".to_string(),
                замена: "дактора".to_string(),
                re_образец: Regex::new(r"(?i)-дактора\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ченной".to_string(),
                замена: "ченной".to_string(),
                re_образец: Regex::new(r"(?i)-ченной\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-распознанный".to_string(),
                замена: "распознанный".to_string(),
                re_образец: Regex::new(r"(?i)-распознанный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-моугольный".to_string(),
                замена: "моугольный".to_string(),
                re_образец: Regex::new(r"(?i)-моугольный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-циями".to_string(),
                замена: "циями".to_string(),
                re_образец: Regex::new(r"(?i)-циями\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тированный".to_string(),
                замена: "тированный".to_string(),
                re_образец: Regex::new(r"(?i)-тированный\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-варительно".to_string(),
                замена: "варительно".to_string(),
                re_образец: Regex::new(r"(?i)-варительно\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-емость".to_string(),
                замена: "емость".to_string(),
                re_образец: Regex::new(r"(?i)-емость\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ваться".to_string(),
                замена: "ваться".to_string(),
                re_образец: Regex::new(r"(?i)-ваться\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-когда".to_string(),
                замена: "когда".to_string(),
                re_образец: Regex::new(r"(?i)-когда\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ответствии".to_string(),
                замена: "ответствии".to_string(),
                re_образец: Regex::new(r"(?i)-ответствии\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-этому".to_string(),
                замена: "этому".to_string(),
                re_образец: Regex::new(r"(?i)-этому\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-слеживания".to_string(),
                замена: "слеживания".to_string(),
                re_образец: Regex::new(r"(?i)-слеживания\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-рирует".to_string(),
                замена: "рирует".to_string(),
                re_образец: Regex::new(r"(?i)-рирует\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-сения".to_string(),
                замена: "сения".to_string(),
                re_образец: Regex::new(r"(?i)-сения\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ниями".to_string(),
                замена: "ниями".to_string(),
                re_образец: Regex::new(r"(?i)-ниями\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-структивными".to_string(),
                замена: "структивными".to_string(),
                re_образец: Regex::new(r"(?i)-структивными\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ствия".to_string(),
                замена: "ствия".to_string(),
                re_образец: Regex::new(r"(?i)-ствия\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-единять".to_string(),
                замена: "единять".to_string(),
                re_образец: Regex::new(r"(?i)-единять\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-шения".to_string(),
                замена: "шения".to_string(),
                re_образец: Regex::new(r"(?i)-шения\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-изводить".to_string(),
                замена: "изводить".to_string(),
                re_образец: Regex::new(r"(?i)-изводить\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-жимах".to_string(),
                замена: "жимах".to_string(),
                re_образец: Regex::new(r"(?i)-жимах\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-чайшее".to_string(),
                замена: "чайшее".to_string(),
                re_образец: Regex::new(r"(?i)-чайшее\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ношении".to_string(),
                замена: "ношении".to_string(),
                re_образец: Regex::new(r"(?i)-ношении\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ровки".to_string(),
                замена: "ровки".to_string(),
                re_образец: Regex::new(r"(?i)-ровки\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-изводиться".to_string(),
                замена: "изводиться".to_string(),
                re_образец: Regex::new(r"(?i)-изводиться\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-бирает".to_string(),
                замена: "бирает".to_string(),
                re_образец: Regex::new(r"(?i)-бирает\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ностями".to_string(),
                замена: "ностями".to_string(),
                re_образец: Regex::new(r"(?i)-ностями\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-виши".to_string(),
                замена: "виши".to_string(),
                re_образец: Regex::new(r"(?i)-виши\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тивизации".to_string(),
                замена: "тивизации".to_string(),
                re_образец: Regex::new(r"(?i)-тивизации\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-личных".to_string(),
                замена: "личных".to_string(),
                re_образец: Regex::new(r"(?i)-личных\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-ложение".to_string(),
                замена: "ложение".to_string(),
                re_образец: Regex::new(r"(?i)-ложение\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тивной".to_string(),
                замена: "тивной".to_string(),
                re_образец: Regex::new(r"(?i)-тивной\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-логических".to_string(),
                замена: "логических".to_string(),
                re_образец: Regex::new(r"(?i)-логических\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нивает".to_string(),
                замена: "нивает".to_string(),
                re_образец: Regex::new(r"(?i)-нивает\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-слойной".to_string(),
                замена: "слойной".to_string(),
                re_образец: Regex::new(r"(?i)-слойной\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-нимается".to_string(),
                замена: "нимается".to_string(),
                re_образец: Regex::new(r"(?i)-нимается\>").unwrap(),
            },
            Ячейка_замены {
                искомое_слово: "-тельного".to_string(),
                замена: "тельного".to_string(),
                re_образец: Regex::new(r"(?i)-тельного\>").unwrap(),
            },
        ],
    };
    let mut счётчики_замен: Arc<Счётчик_замен> = Arc::new(Счётчик_замен {
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
    проверка_ряда_regex_замен(&трёхбуквенные, "провера замен трёхбуквенные");
    проверка_ряда_regex_замен(&*двубуквенные, "провера замен двубуквенные");
    проверка_ряда_regex_замен(&*многоуквенные, "провера замен многобуквенные");
    проверка_ряда_regex_замен(&*однобуквенные, "провера замен однобуквенные");
    проверка_ряда_regex_замен(&*целиковые, "проверка замен целиковые");
    return (счётчики_замен, словарь_замен);
}

/*
pub fn проверка_ряда_regex_замен2(re_ряд: impl AsRef<[Regex]>, сообщение: &str) {
    let ряд = re_ряд.as_ref();
    let куча: HashSet<String> = (0..ряд.len())
        .into_par_iter()
        .flat_map(|i| {
            let mut куча_2: HashSet<String>=HashSet::with_hasher(RandomState::default());
            if !sz_найти(&ряд[i].to_string(),"$") {
                //куча_2.insert(format!("Regex нет знака окончания слова $: {}", ряд[i]))
                куча_2.insert(format!("Regex нет знака окончания слова $: {}", ряд[i]));
            }
            let повторы:HashSet<String>=((i + 1)..ряд.len()).into_par_iter().filter_map(move |j| {
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
    let куча: HashSet<String> = (0..ряд.len())
        .into_par_iter()
        .flat_map(|i| {
            let mut куча_2: std::collections::HashSet<String, RandomState> = HashSet::default();

            // Проверка на отсутствие $
            // if !ряд[i].as_str().contains('$') {
            if !sz_найти(&ряд[i].to_string(), r"\>") {
                куча_2.insert(format!("Regex нет знака окончания слова $: {}", ряд[i]));
            }

            // Проверка на дубликаты
            let повторы: HashSet<String> = ((i + 1)..ряд.len())
                .into_par_iter()
                .filter_map(|j| {
                    if ряд[i].as_str() == ряд[j].as_str() {
                        Some(format!("есть совпадение Regex: {}", ряд[i]))
                    } else {
                        None
                    }
                })
                .collect();

            куча_2.extend(повторы);
            куча_2.into_iter().collect::<HashSet<String>>()
        })
        .collect();

    if !куча.is_empty() {
        println!("длина кучи: {}", куча.len());
        for слово in &куча {
            println!("{} : {}", сообщение, слово);
        }
    }
}
