use crate::utils::stringzilla::*;
use std::sync::LazyLock;
use stringzilla::stringzilla::bytesum;
//use clap::error::ErrorKind::Format;
//use crate::import::functions::преобразовать_слово_с_чертой_в_начале;
use Text_Changer::{
    Словарь_Переносов, Счётчики_Словаря, Ячейка_замены_с_исключением
};
//use console::{Emoji, style};
//use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressState, ProgressStyle};
//use rand::{Rng, prelude::*};
use rayon::prelude::*;
//use std::borrow::Cow;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
//use std::thread;
//use std::time::{Duration, Instant};
//use std::{cmp::min, fmt::Write};
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
static RE_РАСШИРЕНИЯ_ИЗОБРАЖЕНИЙ: LazyLock<[Regex; 14]> = LazyLock::new(|| {
    [
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
        Regex::new(r"(?i)\.jpe?g$").unwrap(), // Объединил jpg и jpeg
        Regex::new(r"(?i)\.tiff?$").unwrap(), // Объединил tif и tiff
        Regex::new(r"(?i)\.bmp$").unwrap(),
        Regex::new(r"(?i)\.gif$").unwrap(),  // Добавил $ в конец
        Regex::new(r"(?i)\.webp$").unwrap(), // Добавил современные форматы
        Regex::new(r"(?i)\.svg$").unwrap(),
        Regex::new(r"(?i)\.avif$").unwrap(),
        Regex::new(r"(?i)\.jpeg$").unwrap(),
        Regex::new(r"(?i)\.jpg$").unwrap(),
        Regex::new(r"(?i)\.tiff$").unwrap(),
        Regex::new(r"(?i)\.png$").unwrap(),
        Regex::new(r"(?i)\.wmf$").unwrap(),
        Regex::new(r"(?i)\.wpg$").unwrap(),
        Regex::new(r"(?i)\.eps$").unwrap(),
    ]
});
//static LOOKING_GLASS: &str = "🔍";
//если это картинка
use Text_Changer::{
    self, Полный_Словарь, Словарь_разделителей, Счётчик_замен, Счётчик_разделителей, Ячейка_словаря,
};

use rayon::iter::IntoParallelRefIterator;
use regex::{Match, Regex};

pub fn нет_разрешения(стог_сена: &String) -> bool {
    static RE_РАСШИРЕНИЯ_ПУСТЫЕ: LazyLock<[Regex; 1]> =
        LazyLock::new(|| [Regex::new(r"(?i)\\([\d\w_-]+)$").unwrap()]);

    return RE_РАСШИРЕНИЯ_ПУСТЫЕ
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}
pub fn мусорное_содержимое_архивов(стог_сена: &String) -> bool {
    static RE_РАСШИРЕНИЯ_МУСОРНЫЕ: LazyLock<[Regex; 4]> = LazyLock::new(|| {
        [
            Regex::new(r"(?i)\.css$").unwrap(),
            Regex::new(r"(?i)\.rels$").unwrap(),
            Regex::new(r"(?i)\.ttf$").unwrap(),
            //Regex::new(r"(?i)\.xhtml$").unwrap(),
            //целиком имя
            Regex::new(r"(?i)mimetype$").unwrap(),
            //
        ]
    });

    return RE_РАСШИРЕНИЯ_МУСОРНЫЕ
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}

pub fn изображение_расширение_с_точкой(
    стог_сена: &String
) -> bool {
    /*static RE_РАСШИРЕНИЯ_ИЗОБРАЖЕНИЙ: LazyLock<[Regex; 15]> = LazyLock::new(|| {
        [
            //
            Regex::new(r"(?i)\.jpe?g$").unwrap(), // Объединил jpg и jpeg
            Regex::new(r"(?i)\.tiff?$").unwrap(), // Объединил tif и tiff
            Regex::new(r"(?i)\.bmp$").unwrap(),
            Regex::new(r"(?i)\.gif$").unwrap(), // Добавил $ в конец
            Regex::new(r"(?i)\.webp$").unwrap(), // Добавил современные форматы
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
        ]
    });*/

    return RE_РАСШИРЕНИЯ_ИЗОБРАЖЕНИЙ
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}

pub fn изображение_расширение_без_точки(
    стог_сена: &String
) -> bool {
    return RE_РАСШИРЕНИЯ_ИЗОБРАЖЕНИЙ
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}

pub fn не_является_изображением(стог_сена: &String) -> bool {
    /*static RE_РАСШИРЕНИЯ_ИЗОБРАЖЕНИЙ: LazyLock<[Regex; 14]> = LazyLock::new(|| {
        [
            Regex::new(r"(?i)jpe?g$").unwrap(), // Объединил jpg и jpeg
            Regex::new(r"(?i)tiff?$").unwrap(), // Объединил tif и tiff
            Regex::new(r"(?i)bmp$").unwrap(),
            Regex::new(r"(?i)gif$").unwrap(),  // Добавил $ в конец
            Regex::new(r"(?i)webp$").unwrap(), // Добавил современные форматы
            Regex::new(r"(?i)svg$").unwrap(),
            Regex::new(r"(?i)avif$").unwrap(),
            Regex::new(r"(?i)jpeg$").unwrap(),
            Regex::new(r"(?i)jpg$").unwrap(),
            Regex::new(r"(?i)tiff$").unwrap(),
            Regex::new(r"(?i)png$").unwrap(),
            Regex::new(r"(?i)wmf$").unwrap(),
            Regex::new(r"(?i)wpg$").unwrap(),
            Regex::new(r"(?i)eps$").unwrap(),
        ]
    });*/

    return RE_РАСШИРЕНИЯ_ИЗОБРАЖЕНИЙ
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}
//если это архивный файл
/*pub fn fb3_epub(стог_сена: &String) -> bool {
    static RE_РАСШИРЕНИЯ_АРХИВНЫЕ: LazyLock<[Regex; 2]> = LazyLock::new(|| {
        [
            Regex::new(r"(?i)\.fb3$").unwrap(),
            Regex::new(r"(?i)\.epub$").unwrap(),
            //Regex::new(r"(?i)\.docx$").unwrap(),
            //Regex::new(r"(?i)\.doc$").unwrap(),
        ]
    });

    return RE_РАСШИРЕНИЯ_АРХИВНЫЕ
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}*/
pub fn без_кодировки(стог_сена: &String) -> bool {
    static RE_РАСШИРЕНИЯ: LazyLock<[Regex; 1]> = LazyLock::new(|| {
        [
            Regex::new(r"(?i)\.txt$").unwrap(),
            //Regex::new(r"(?i)\.docx$").unwrap(),
            //Regex::new(r"(?i)\.doc$").unwrap(),
        ]
    });

    return RE_РАСШИРЕНИЯ
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}
//если это архивный файл
pub fn doc_docx(стог_сена: &String) -> bool {
    static RE_РАСШИРЕНИЯ_WORD: LazyLock<[Regex; 2]> = LazyLock::new(|| {
        [
            //Regex::new(r"(?i)\.fb3$").unwrap(),
            //Regex::new(r"(?i)\.epub$").unwrap(),
            Regex::new(r"(?i)\.docx$").unwrap(),
            Regex::new(r"(?i)\.doc$").unwrap(),
        ]
    });

    return RE_РАСШИРЕНИЯ_WORD
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}
pub fn md_fs_yml(стог_сена: &String) -> bool {
    static RE_РАСШИРЕНИЯ_MD_YML_FS: LazyLock<[Regex; 3]> = LazyLock::new(|| {
        [
            //Regex::new(r"(?i)\.fb3$").unwrap(),
            //Regex::new(r"(?i)\.epub$").unwrap(),
            Regex::new(r"(?i)\.md$").unwrap(),
            Regex::new(r"(?i)\.yml$").unwrap(),
            Regex::new(r"(?i)\.fs$").unwrap(),
        ]
    });

    return RE_РАСШИРЕНИЯ_MD_YML_FS
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
    //return false;
}

/*pub fn htm_html_xhtml(стог_сена: &String) -> bool {
    static RE_РАСШИРЕНИЯ_HTM_XHTM: LazyLock<[Regex; 3]> = LazyLock::new(|| {
        [
            Regex::new(r"(?i)\.htm$").unwrap(),
            Regex::new(r"(?i)\.html$").unwrap(),
            Regex::new(r"(?i)\.xhtml$").unwrap(),
        ]
    });

    return RE_РАСШИРЕНИЯ_HTM_XHTM
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
    //return false;
}*/
//если это не архивный файл
/*pub fn fb2_rtf_mht_mhtml(стог_сена: &String) -> bool {
    static RE_РАСШИРЕНИЯ_НЕ_АРХИВНЫЕ: LazyLock<[Regex; 4]> = LazyLock::new(|| {
        [
            Regex::new(r"(?i)\.fb2$").unwrap(),
            Regex::new(r"(?i)\.rtf$").unwrap(),
            Regex::new(r"(?i)\.mhtml$").unwrap(),
            Regex::new(r"(?i)\.mht$").unwrap(),
        ]
    });

    return RE_РАСШИРЕНИЯ_НЕ_АРХИВНЫЕ
        .par_iter()
        .any(|строка| строка.is_match(стог_сена));
}*/
//захват слов
//есть ли маты
pub fn есть_ли_маты(стог_сена: &String) -> bool {
    //маты
    static RE_МАТЕРШИНА_СЛОВА: LazyLock<[Regex; 1]> =
        LazyLock::new(|| [Regex::new(r"(?i)\s*([\w]…)\s*").unwrap()]);

    return RE_МАТЕРШИНА_СЛОВА
        .par_iter()
        .any(|образец| образец.is_match(стог_сена));
}

//выдел строки
pub fn re_получить_имя_файла_без_пути(стог_сена: &String) -> String {
    static БЕЗ_ПУТИ: LazyLock<[Regex; 3]> = LazyLock::new(|| {
        [
            Regex::new(r"(?i)\\(.[^\\]+)$").unwrap(),
            Regex::new(r"(?i)\\([\d\w\s_\-\=\(\)]+)$").unwrap(),
            Regex::new(r"(?i)/([\d\w\s_\-\=\(\)]+)$").unwrap(),
        ]
    });
    static ПЕРВАЯ_ПАЛКА: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\\").unwrap());
    static ВТОРАЯ_ПАЛКА: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)/").unwrap());

    if ПЕРВАЯ_ПАЛКА.find_iter(стог_сена).count() == 0
        && ВТОРАЯ_ПАЛКА.find_iter(стог_сена).count() == 0
    {
        return стог_сена.to_string();
    }
    for указатель in 0..БЕЗ_ПУТИ.len() {
        if let Some(строка) = БЕЗ_ПУТИ[указатель].captures(&стог_сена)
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
    static НЕТ_РАСШИРЕНИЯ: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)(?:\\)+([\d\w&&[^\.]]+)$").unwrap());

    let Some(строка) = образец.captures(&стог_сена) else {
        if let Some(_строка) = НЕТ_РАСШИРЕНИЯ.captures(&стог_сена) {
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
pub fn получить_строку_из_ряда_re_с_описанием(стог_сена: &String, образец: &LazyLock<[Regex;5],ошибка:&str) -> String {
    let Some(строка) = образец.captures(&стог_сена) else {
        println!("{}",ошибка);
        panic!("ошибка при выдирания {}, сама строка : {}", &образец, &стог_сена);
    };
    return строка[1].trim().to_string();
}

 */

pub fn определить_имя_книги(стог_сена: &String) -> String {
    static RE_ПУТИ_ДО_КНИГ: LazyLock<[Regex; 6]> = LazyLock::new(|| {
        [
            Regex::new(r"(?i)книги/([\d\w_\-\s\.,]+)\.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i)книги\\([\d\w_\-\s\.,]+)\.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i)книги/([\d\w_\-\s\.,]+)/.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i)книги\\([\d\w_\-\s\.,]+)/.(?:([\d\w]+))$").unwrap(),
            Regex::new(r"(?i).+/(.+)\.").unwrap(),
            Regex::new(r"(?i)([\d\w\-_\s[^\\]]+)$").unwrap(),
            //Regex::new(r"(?i)\\(.[^\\]+)$").unwrap(),
            //  Regex::new(r"(?i)/(.[^\\/]+)$").unwrap(),
        ]
    });

    RE_ПУТИ_ДО_КНИГ
        .iter()
        .find_map(|образец| {
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
    re_образцы: &LazyLock<[Regex;5],
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
    _сообщение: &str,
    _расширение: &str,
    куча_пропусков: &rapidhash::fast::RapidHashSet<usize>,
    словарь_куча: &rapidhash::fast::RapidHashMap<String, rapidhash::fast::RapidHashSet<usize>>,
    _этап: usize,
    _указатель_содержимого: usize,
    _количество_вложений: usize,
    _вложенный_ли_файл_к_html: bool,
    раздел_словаря: Text_Changer::Раздел_Словаря,
) {
    // const СТРОКА_ИСКОМАЯ: &str = "фазовая модуляция формы";

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
            // let mut условие_вывода: bool = false;
            //
            if куча_пропусков.contains(&указатель) {
                // Пропускаем строку, но все равно считаем прогресс
                //let шаги_для_этой_строки = словарь.len() as u64;
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
                                /*if раздел_словаря==Text_Changer::Раздел_Словаря::Составные_важные {
                                //поиск условия
                                if sz_найти(&строка, &СТРОКА_ИСКОМАЯ) {
                                    условие_вывода = true;
                                    //println!("Нашло: {}", строка);
                                }
                                }*/
                                //
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
                                /*if условие_вывода {
                                    println!("До: {}", строка);
                                    println!("После: {}\r\n", замененная_строка);
                                }*/
                                *строка = замененная_строка;
                            }
                        }
                        //если 1-2 значения в ключе
                        else {
                            /*if sz_найти(&строка, &СТРОКА_ИСКОМАЯ) {
                                условие_вывода = true;
                                println!("Нашло: {}", строка);
                            }*/
                            let замененная_строка = &словарь[*указатель_образца]
                                .re_образец
                                .replace_all(&строка, &словарь[*указатель_образца].замена);

                            let замененная_строка = замененная_строка.to_string();
                            if замененная_строка.as_str() != строка.as_str() {
                                // Увеличиваем атомарный счетчик
                                счётчик_словаря[*указатель_образца].fetch_add(1, Ordering::Relaxed);
                            }
                            // Заменяем строку
                            /*if условие_вывода {
                                println!("До: {}", строка);
                                println!("После: {}\r\n", замененная_строка);
                            }*/
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
    /*fn условие_вывода_хода(этап: usize) -> bool {
        //пока отменил вывод с указанием текущего этапа прохода слов, слишком быстро всё делает и в итоге чисто кроме мусора ничего нет
        if этап == 99 { true } else { false }
    }*/
}

//многопоточность
pub fn добавить_разделители(
    словарь_разделителей: &Словарь_разделителей,
    содержимое: &mut [String],
    _сообщение: &str,
    _расширение: &str,
    куча_пропусков: &rapidhash::fast::RapidHashSet<usize>,
    _указатель_захода: &mut usize,

    счётчики_замен: &mut Arc<Счётчик_разделителей>,
    указатель_словаря_переносов: usize,
    словарь_разделителей_полный:&[Text_Changer::Словарь_разделителей;
        Text_Changer::КОЛИЧЕСТВО_УРОВНЕЙ_СЛОВАРЯ_КУЧ],
) {
    //  let mut условие_вывода_1: bool = false;
    // let mut условие_вывода_2: bool = false;
    // Общее количество шагов для прогресса (если нужен)
    // let общий_счёт_шагов = словарь_разделителей.ряд_1.len() * содержимое.len();
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
                словарь_разделителей.содержимое.iter().enumerate()
            {
                let условие_у: bool = if ячейка.искомое_слово.as_str() == "спо" {
                    true
                } else {
                    false
                };
                let mut условие_вывода: bool = false;
                //let mut счётчик_раз: usize = 0;
                // Проверяем наличие искомого слова (можно заменить на другое условие)
                if sz_найти(строка, ячейка.искомое_слово.as_str(),) {
                    //
                    if ячейка.re_образец_для_замены.is_match(&строка) {
                        // Если есть совпадение с основным regex
                        // Проверяем, не попадает ли строка под исключения
                        //
                        let количество_совпадений: usize = ячейка
                            .re_образец_для_замены
                            .find_iter(&строка)
                            .count();

                        let все_совпадения: Vec<Match> =
                            ячейка.re_образец_для_замены.find_iter(&строка).collect();
                        //
                        let mut количество_совпадений_успех: usize =
                            количество_совпадений;
                        //если не нашло - то следующее слово
                        if количество_совпадений == 0 {
                            continue 'круговорот_главный;
                        }
                        if sz_найти(&строка, "спокойный") {
                            условие_вывода = true
                        }
                        /*let главное_условие:bool=if условие_вывода && условие_у {true} else {false};
                         if главное_условие {
                             println!("исход строка |{}|",строка);
                             println!("Исход количество_совпадений |{}| все_совпадения |{:?}| ",количество_совпадений,все_совпадения)
                         }*/
                        //
                        let mut количество_исключений:usize=0;
                        let mut количество_успехов:usize=0;
                        /*let mut начальный_указатель: usize = 0;
                        //
                        'нижний_круговорот: while
                            количество_совпадений_успех <= количество_совпадений*/
                        'нижний_круговорот:for заход_главный in 0..количество_совпадений
                        {
                        let заход:usize=заход_главный-количество_успехов;
                            //умный проход
                            /*for указатель_захода in 0..количество_совпадений
                            {*/
                            //let строка2 = строка.to_string();
                            //
                            let все_совпадения: Vec<Match> =
                                ячейка.re_образец_для_замены.find_iter(&строка).collect();
                            if все_совпадения.len()==0 {continue 'нижний_круговорот}
                            if все_совпадения.len()<заход {
                                println!("строка |{}|",строка);
                                println!(" заход |{}| количество_совпадений изначально |{количество_совпадений}| количество совпадений |{}| количество_исключений |{}|",
                                    заход,все_совпадения.len(),количество_исключений);
                                println!("все_совпадения |{:?}|",все_совпадения);
                                panic!();

                            }
                            //
                            /*if условие_вывода && условие_у {
                                println!("До: начальный_указатель |{}|", начальный_указатель);
                                println!("До: |{:?}|", все_совпадения);
                            }*/
                            //количество_совпадений = все_совпадения.len();
                            //если были изменения - то заного просчитать начало и конец совпадений в строке
                            //счётчик изменяемый - так как ходим им и бывает исключений уже нет
                            //let число_захода: usize = указатель_захода - счётчик_раз;
                            // let число_захода:usize=указатель_захода;
                            //если исключение - следующий заход

                            //поиск совпадения по числу
                            if let Some(найденное_совпадение) = ячейка
                                .re_образец_для_замены
                                .captures(&все_совпадения[заход].as_str())
                            {

                                //
                                let уровень_букв_в_замене:Text_Changer::Правописание_слова= все_ли_заглавные_буквы_в_слове(
                                    &найденное_совпадение[2],
                                );
                                let уровень_букв_в_словаре:Text_Changer::Правописание_слова= match указатель_словаря_переносов {
                                    0=>Text_Changer::Правописание_слова::Все_строчные,
                                    1=>Text_Changer::Правописание_слова::С_Заглавной,
                                    2=>Text_Changer::Правописание_слова::Все_Заглавные,
                                    _=>panic!(),
                                };
                                //
                                let указатель_на_re_исключения =
                                    match уровень_букв_в_замене
                                     {
                                        Text_Changer::Правописание_слова::Все_строчные =>
                                        //
                                        match уровень_букв_в_словаре {
                                           Text_Changer::Правописание_слова::Все_строчные=>  &словарь_разделителей_полный[указатель_словаря_переносов].содержимое
                                               [указатель_ячейки]
                                               .ряд_re_пропуски,
                                           //
                                           Text_Changer::Правописание_слова::С_Заглавной|Text_Changer::Правописание_слова::Все_Заглавные=>&словарь_разделителей_полный[1].содержимое
                                               [указатель_ячейки]
                                               .ряд_re_пропуски,
                                           _=>panic!(),
                                        }
                                        //если в замене заглавные буквы - то всегда все заглавные выставлять
                                         Text_Changer::Правописание_слова::Все_Заглавные =>

                                            &словарь_разделителей_полный[2].содержимое
                                                [указатель_ячейки]
                                                .ряд_re_пропуски,

                                            _=>panic!(),
                                    };
                                //
                                /*if главное_условие {
                                    println!("Нашлось решение до");
                                }*/
                                //
                                if есть_ли_исключение(
                                    //&ячейка.ряд_re_исключений,
                                    указатель_на_re_исключения,
                                    &все_совпадения[заход].as_str(),
                                ) {
                                    //количество_совпадений_успех += 1;
                                    //заход += 1;
                                    количество_исключений+=1;
                                    /*if главное_условие {
                                        println!("Нашлось исключение - выход");
                                        println!("само слово |{}|",все_совпадения[заход].as_str());
                                        println!("re иключения|{:?}|",указатель_на_re_исключения);
                                    }*/
                                    continue 'нижний_круговорот;
                                }
                                /*if главное_условие {
                                    println!("Нашлось решение после");
                                }*/
                                //
                                let замена: String = format!(
                                    "{}-{}",
                                    &найденное_совпадение[1], &найденное_совпадение[2]
                                );
                                //
                                //
                                let mut замененная_строка: String = строка.to_string();
                                // Проверяем, что индексы в пределах строки
                                // Получаем корректные границы символов
                                let начало: usize =
                                    все_совпадения[заход].start();
                                let конец: usize =
                                    все_совпадения[заход].end();
                                // Корректируем индексы до границ символов
                                let начало: usize = замененная_строка.floor_char_boundary(начало);
                                let конец: usize = замененная_строка.floor_char_boundary(конец);
                                //
                                замененная_строка.replace_range(начало..конец, &замена);
                                /*if главное_условие {
                                    //
                                    println!("уровень_букв |{}| искомое_слово |{}| re_образец_для_поиска |{}| замена |{}|\r\n",
                                        уровень_букв_в_замене,
                                        ячейка.искомое_слово,
                                        ячейка.re_образец_для_поиска.as_str(),
                                        ячейка.замена,
                                    );

                                    /*let условие_исклюения = есть_ли_исключение(
                                        &ячейка.ряд_re_исключений,
                                        &все_совпадения[начальный_указатель].as_str(),
                                    );*/

                                    for (указатель_иск, само_иск) in
                                        указатель_на_re_исключения.iter().enumerate()
                                    {

                                        println!("re #|{}| = |{}|", указатель_иск, само_иск);
                                    }
                                    println!(
                                        "\r\nre #|{}| найденные совпадения |{:?}|",
                                        ячейка.re_образец_для_замены, найденное_совпадение
                                    );
                                    /*
                                    for (указатель_иск, само_иск) in
                                        ячейка.ряд_re_исключений.iter().enumerate()
                                    {
                                        println!("re #|{}| = |{}|", указатель_иск, само_иск);
                                    }
                                    for (указатель_иск, само_иск) in
                                        ячейка.ряд_исключений.iter().enumerate()
                                    {
                                        println!(
                                            "исключ #|{}| = |{}|",
                                            указатель_иск, само_иск
                                        );
                                    }*/
                                    //
                                    println!("изначальная строка |{}|", строка);
                                    println!("Все совпадения1: |{:?}|", все_совпадения);
                                    println!("замена |{}|", замена);
                                    println!("количество_совпадений |{}|",количество_совпадений);
                                    println!("начальный_указатель |{}|", заход);
                                    println!("замененная_строка |{}|", замененная_строка);
                                }*/
                                //проверка что есть изменения, тогда заменяем содержимое
                                if замененная_строка != *строка {
                                    количество_успехов+=1;
                                    //
                                    счётчики_замен.подсчёт[указатель_ячейки]
                                        .fetch_add(1, Ordering::Relaxed);
                                    //количество_совпадений_успех += 1;
                                    //

                                    //начальный_указатель+=1;
                                    *строка = замененная_строка;
                                    //continue 'нижний_круговорот;
                                    // += 1;
                                }
                                // Обновляем строку
                            }
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
                    //}
                    //
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
    _сообщение: &str,
    _расширение: &str,
    _указатель_захода: &mut usize,
    счётчики_замен: &mut Arc<Счётчик_замен>,
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
    //use crate::dictionary_0::проверка_ряда_regex;

    //если первый раз заходит - то проверить

    //подсчёт для видимого счётчика в окне
    /*let общий_счёт: usize = словарь_замен.целиковые.len()
    + словарь_замен.многобуквенные.len()
    + словарь_замен.трехбуквенные.len()
    + словарь_замен.двубуквенные.len()
    + словарь_замен.однобуквенные.len()
    + словарь_замен.исключения.len();*/

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
        .for_each(|(_указатель, строка)| {
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
                                // Заменяем строку
                                if строка.as_str() != замененная_строка {
                                    *строка = замененная_строка;
                                }
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
                    if sz_найти(&строка, искомое_слово.as_str()) {
                        let замененная_строка = re_образец.replace_all(&строка, замена.as_str());
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.целиковые[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                            // счётчик_однобуквенные[указатель_образца].fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        // Заменяем строку
                        if строка.as_str() != замененная_строка {
                            *строка = замененная_строка;
                        }
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
                    if sz_найти(&строка, искомое_слово.as_str()) {
                        let замененная_строка = re_образец.replace_all(&строка, замена.as_str());
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.многобуквенные[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        // Заменяем строку
                        if строка.as_str() != замененная_строка {
                            *строка = замененная_строка;
                        }
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
                    if sz_найти(&строка, искомое_слово.as_str()) {
                        // println!("нашло двукбуквенное");
                        let замененная_строка = re_образец.replace_all(&строка, замена.as_str());
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.трехбуквенные[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        // Заменяем строку
                        if строка.as_str() != замененная_строка {
                            *строка = замененная_строка;
                        }
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
                    if sz_найти(&строка, искомое_слово.as_str()) {
                        // println!("нашло двукбуквенное");
                        let замененная_строка = re_образец.replace_all(&строка, замена.as_str());
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.двубуквенные[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        if строка.as_str() != замененная_строка {
                            *строка = замененная_строка;
                        }
                    }
                    //слшком жрёт дохрена - нахрен
                    // Обновляем прогресс
                    //let текущий_шаг = шаг_внутренний.fetch_add(1, Ordering::Relaxed) + 1;
                    //счетчик_внутренний.set_position(текущий_шаг);
                }
                //однобуквенные
                for (указатель_образца, образец) in словарь_замен.однобуквенные.iter().enumerate()
                {
                    //
                    let re_образец = &образец.re_образец;
                    let искомое_слово = &образец.искомое_слово;
                    let замена = &образец.замена;
                    //if re_образец.is_match(&строка)
                    if sz_найти(&строка, искомое_слово.as_str()) {
                        let замененная_строка = re_образец.replace_all(&строка, замена.as_str());
                        let замененная_строка = замененная_строка.to_string();
                        if bytesum(&замененная_строка) != bytesum(&строка) {
                            // Увеличиваем атомарный счетчик
                            счётчики_замен.однобуквенные[указатель_образца]
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        // Заменяем строку
                        if строка.as_str() != замененная_строка {
                            *строка = замененная_строка;
                        }
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
pub fn создать_словарь_разделителей() -> Result<Словарь_разделителей, String> {
    use Text_Changer::Умные_Строки_Ряд;
    //use std::default::Default;
    let mut счётчик_i: usize = 0;
    println!("создать словарь разделителей вход |{}|", счётчик_i);
    счётчик_i += 1;
    //use crate::dictionary_0::проверка_ряда_regex;
    use Text_Changer::Ячейка_замены_с_разделителями;
    let mut итоговый_словарь: Словарь_разделителей = Словарь_разделителей {
        содержимое: vec![
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "обще".to_string(),
                    ),
                ряд_пропусков: vec!["обществ".to_string(), "общен".to_string()]
                    .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "крае".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "книго".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "рабо".to_string(),
                    ),
                ряд_пропусков: vec!["работ".to_string(), "рабоч".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мрако".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "паро".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "казно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сбое".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "неравно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "тихо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "трудо".to_string(),
                    ),
                ряд_пропусков: vec![
                    "трудово".to_string(),
                    "трудовы".to_string(),
                    "трудова".to_string(),
                    "трудову".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "трупо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "ясно".to_string(),
                    ),
                ряд_пропусков: vec!["ясност".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мысле".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "светло".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "тёмно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "темно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "плано".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "платёже".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "платеже".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "лето".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "клейко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "золото".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "злато".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "гибко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "древне".to_string(),
                    ),
                ряд_пропусков: vec!["древней".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "близко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "дально".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "далеко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "чудо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "чуже".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "жёстко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "жестко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сельско".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сельхоз".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "восьми".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "товаро".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "семено".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "быче".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "тонко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "живо".to_string(),
                    ),
                ряд_пропусков: vec![
                    "животно".to_string(),
                    "животны".to_string(),
                    "животен".to_string(),
                    "животну".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "животно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "еже".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "семи".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "шести".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пяти".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "четырёх".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "четырех".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "трёх".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "трех".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "двух".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "грязно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "смехо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "глубоко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "градо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "во".to_string(),
                    ),
                ряд_пропусков: vec![
                    "востр".to_string(),
                    "восток".to_string(),
                    "восточ".to_string(),
                    "вось".to_string(),
                    "вослав".to_string(),
                    "вором".to_string(),
                    "воров".to_string(),
                    "воруе".to_string(),
                    "ворую".to_string(),
                    "волч".to_string(),
                    "волк".to_string(),
                    "вороб".to_string(),
                    "волх".to_string(),
                    "воль".to_string(),
                    "водят".to_string(),
                    "водит".to_string(),
                    "волн".to_string(),
                    "волш".to_string(),
                    "воз".to_string(),
                    "водо".to_string(),
                    "волос".to_string(),
                    "воен".to_string(),
                    "воин".to_string(),
                    "вой".to_string(),
                    "вож".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "чисто".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "древле".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "правдо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "ранне".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "дву".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "изо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "из".to_string(),
                    ),
                ряд_пропусков: vec!["изо".to_string(), "изящ".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "по".to_string(),
                    ),
                ряд_пропусков: vec![
                    "пото".to_string(),
                    "посту".to_string(),
                    "покой".to_string(),
                    "покоя".to_string(),
                    "покои".to_string(),
                    "покою".to_string(),
                    "помн".to_string(),
                    "порог".to_string(),
                    "порч".to_string(),
                    "порт".to_string(),
                    "почв".to_string(),
                    "понт".to_string(),
                    "пояс".to_string(),
                    "под".to_string(),
                    "поздн".to_string(),
                    "пол".to_string(),
                    "потер".to_string(),
                    "после".to_string(),
                    "почерко".to_string(),
                    "позже".to_string(),
                    "постов".to_string(),
                    "постом".to_string(),
                    "постой".to_string(),
                    "посты".to_string(),
                    "постам".to_string(),
                    "потоп".to_string(),
                    "поощр".to_string(),
                    "пой".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пол".to_string(),
                    ),
                ряд_пропусков: vec![
                    "полаком".to_string(),
                    "полз".to_string(),
                    "полев".to_string(),
                    "полиц".to_string(),
                    "полит".to_string(),
                    "полаг".to_string(),
                    "полн".to_string(),
                    "полу".to_string(),
                    "поло".to_string(),
                    "поль".to_string(),
                    "полюб".to_string(),
                    "полезн".to_string(),
                    "полезе".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пост".to_string(),
                    ),
                ряд_пропусков: vec![
                    "постыд".to_string(),
                    "постеп".to_string(),
                    "постор".to_string(),
                    "посту".to_string(),
                    "постоя".to_string(),
                    "постар".to_string(),
                    "постав".to_string(),
                    "постел".to_string(),
                    "постро".to_string(),
                    "постанов".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пра".to_string(),
                    ),
                ряд_пропусков: vec!["прах".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "грузо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "черно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "красно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "псевдо".to_string(),
                    ),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "время".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "грехо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "свое".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "законо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "за".to_string(),
                    ),
                ряд_пропусков: vec![
                    "замок".to_string(),
                    "замком".to_string(),
                    "замков".to_string(),
                    "замки".to_string(),
                    "замку".to_string(),
                    "замкам".to_string(),
                    "заяц".to_string(),
                    "зайц".to_string(),
                    "заяч".to_string(),
                    "завтр".to_string(),
                    "задн".to_string(),
                    "законо".to_string(),
                    "зарплат".to_string(),
                    "забот".to_string(),
                    "запад".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "одно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "черто".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "чёрто".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "счёто".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "дело".to_string(),
                    ),
                ряд_пропусков: vec!["делов".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "судо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "недо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "девяти".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "десяти".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "работо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "чадо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "ис".to_string(),
                    ),
                ряд_пропусков: vec![
                    "исток".to_string(),
                    "источ".to_string(),
                    "испан".to_string(),
                    "иск".to_string(),
                    "истин".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "плодо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "тысяче".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "строе".to_string(),
                    ),
                ряд_пропусков: vec!["строен".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "судьбо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "чело".to_string(),
                    ),
                ряд_пропусков: vec!["человеко".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "собако".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "напере".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "заподно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "у".to_string(),
                    ),
                ряд_пропусков: vec![
                    "ужин".to_string(),
                    "утюг".to_string(),
                    "утюж".to_string(),
                    "утр".to_string(),
                    "ужас".to_string(),
                    "утк".to_string(),
                    "утин".to_string(),
                    "уточк".to_string(),
                    "успок".to_string(),
                    "убав".to_string(),
                    "угл".to_string(),
                    "уль".to_string(),
                    "угол".to_string(),
                    "упл".to_string(),
                    "урк".to_string(),
                    "успеш".to_string(),
                    "успех".to_string(),
                    "уза".to_string(),
                    "узы".to_string(),
                    "убежд".to_string(),
                    "удоб".to_string(),
                    "узилищ".to_string(),
                    "ум".to_string(),
                    "уровн".to_string(),
                    "узко".to_string(),
                    "уравно".to_string(),
                    "уч".to_string(),
                    "удоч".to_string(),
                    "уровен".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "под".to_string(),
                    ),
                ряд_пропусков: vec![
                    "подняв".to_string(),
                    "поду".to_string(),
                    "подру".to_string(),
                    "подер".to_string(),
                    "пода".to_string(),
                    "подрал".to_string(),
                    "подрат".to_string(),
                    "подробн".to_string(),
                    "подел".to_string(),
                    "подл".to_string(),
                    "подо".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "подо".to_string(),
                    ),
                ряд_пропусков: vec![
                    "подолг".to_string(),
                    "подор".to_string(),
                    "подоб".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "подор".to_string(),
                    ),
                ряд_пропусков: vec!["подорож".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "со".to_string(),
                    ),
                ряд_пропусков: vec![
                    "собак".to_string(),
                    "собач".to_string(),
                    "соль".to_string(),
                    "соля".to_string(),
                    "соле".to_string(),
                    "собия".to_string(),
                    "собие".to_string(),
                    "собии".to_string(),
                    "собий".to_string(),
                    "соты".to_string(),
                    "собст".to_string(),
                    "сорт".to_string(),
                    "сокол".to_string(),
                    "собла".to_string(),
                    "сокр".to_string(),
                    "сопш".to_string(),
                    "соот".to_string(),
                    "создан".to_string(),
                    "солн".to_string(),
                    "сотов".to_string(),
                    "сотни".to_string(),
                    "сотне".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "над".to_string(),
                    ),
                ряд_пропусков: vec![
                    "над".to_string(),
                    "надумат".to_string(),
                    "надуман".to_string(),
                    "надей".to_string(),
                    "надеят".to_string(),
                    "надеюс".to_string(),
                    "надзор".to_string(),
                    "надее".to_string(),
                    "надоум".to_string(),
                    "надея".to_string(),
                    "надел".to_string(),
                    "надеж".to_string(),
                    "надёж".to_string(),
                    "наден".to_string(),
                    "надпис".to_string(),
                    "надув".to_string(),
                    "надоб".to_string(),
                    "надое".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "об".to_string(),
                    ),
                ряд_пропусков: vec![
                    "оберег".to_string(),
                    "обереж".to_string(),
                    "обил".to_string(),
                    "обо".to_string(),
                    "обал".to_string(),
                    "обыв".to_string(),
                    "обозн".to_string(),
                    "обу".to_string(),
                    "обеи".to_string(),
                    "обог".to_string(),
                    "обор".to_string(),
                    "обит".to_string(),
                    "обет".to_string(),
                    "обещ".to_string(),
                    "обаят".to_string(),
                    "обаян".to_string(),
                    "обыч".to_string(),
                    "обез".to_string(),
                    "общ".to_string(),
                    "обесп".to_string(),
                    "обяз".to_string(),
                    "обиж".to_string(),
                    "обид".to_string(),
                    "обыд".to_string(),
                    "обин".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "варено".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пото".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "толчко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "варёно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сердце".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "обез".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "млеко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "моче".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "папоротнико".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "боле".to_string(),
                    ),
                ряд_пропусков: vec![
                    "болеющ".to_string(),
                    "болезн".to_string(),
                    "болево".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сног".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "бородавко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "естество".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мозго".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "широко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "трудно".to_string(),
                    ),
                ряд_пропусков: vec!["трудност".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "шумо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сине".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "голубо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "волно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "место".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сильно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "микро".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пуско".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "нефте".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "газо".to_string(),
                    ),
                ряд_пропусков: vec![
                    "газово".to_string(),
                    "газовые".to_string(),
                    "газовых".to_string(),
                    "газовым".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "металло".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "дерево".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "оптико".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "стекло".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "военно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "уравно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мягко".to_string(),
                    ),
                ряд_пропусков: vec!["мягкос".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "равно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "просто".to_string(),
                    ),
                ряд_пропусков: vec!["просторн".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "целе".to_string(),
                    ),
                ряд_пропусков: vec![
                    "целения".to_string(),
                    "целений".to_string(),
                    "целением".to_string(),
                    "целению".to_string(),
                    "целении".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "перво".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сухо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "второ".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "третье".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "внутри".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сума".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "не".to_string(),
                    ),
                ряд_пропусков: vec![
                    "нести".to_string(),
                    "неко".to_string(),
                    "немец".to_string(),
                    "неку".to_string(),
                    "неки".to_string(),
                    "нерв".to_string(),
                    "нефт".to_string(),
                    "нейро".to_string(),
                    "недель".to_string(),
                    "неделю".to_string(),
                    "неделя".to_string(),
                    "недовер".to_string(),
                    "нель".to_string(),
                    "недели".to_string(),
                    "неделе".to_string(),
                    "недо".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "внешне".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "вне".to_string(),
                    ),
                ряд_пропусков: vec!["внешн".to_string(), "внезап".to_string()]
                    .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "душевно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "вновь".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "велико".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мета".to_string(),
                    ),
                ряд_пропусков: vec![
                    "метал".to_string(),
                    "метат".to_string(),
                    "метани".to_string(),
                    "метающ".to_string(),
                    "метаем".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "всплеско".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "полу".to_string(),
                    ),
                ряд_пропусков: vec![
                    "получит".to_string(),
                    "получк".to_string(),
                    "получаем".to_string(),
                    "получаю".to_string(),
                    "получал".to_string(),
                    "получат".to_string(),
                    "получен".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "скачко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "громо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пышно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "коротко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "везде".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "земле".to_string(),
                    ),
                ряд_пропусков: vec!["землен".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "водо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "водно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пере".to_string(),
                    ),
                ряд_пропусков: vec![
                    "переч".to_string(),
                    "передне".to_string(),
                    "передни".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "передне".to_string(),
                    ),
                ряд_пропусков: vec!["передн".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "задне".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "прямо".to_string(),
                    ),
                ряд_пропусков: vec!["прямост".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "любо".to_string(),
                    ),
                ряд_пропусков: vec!["любов".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "лево".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "легко".to_string(),
                    ),
                ряд_пропусков: vec!["легкост".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "право".to_string(),
                    ),
                ряд_пропусков: vec!["правово".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "здраво".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "благо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "людо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "жизне".to_string(),
                    ),
                ряд_пропусков: vec!["жизнен".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "все".to_string(),
                    ),
                ряд_пропусков: vec!["всегд".to_string(), "всерь".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "поло".to_string(),
                    ),
                ряд_пропусков: vec![
                    "полост".to_string(),
                    "полом".to_string(),
                    "полож".to_string(),
                    "полов".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "винто".to_string(),
                    ),
                ряд_пропусков: vec!["винтовоч".to_string(), "винтовк".to_string()]
                    .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "выгодо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "полно".to_string(),
                    ),
                ряд_пропусков: vec!["полност".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "слабо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "средне".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "путе".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мелко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "драго".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "крупно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "строй".to_string(),
                    ),
                ряд_пропусков: vec!["стройн".to_string(), "стройст".to_string()]
                    .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "осново".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "добро".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "солнце".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "кругло".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "рас".to_string(),
                    ),
                ряд_пропусков: vec![
                    "растал".to_string(),
                    "растан".to_string(),
                    "расам".to_string(),
                    "расах".to_string(),
                    "расов".to_string(),
                    "растущ".to_string(),
                    "расте".to_string(),
                    "растит".to_string(),
                    "растят".to_string(),
                    "распр".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "природо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "при".to_string(),
                    ),
                ряд_пропусков: vec![
                    "природо".to_string(),
                    "прием".to_string(),
                    "приём".to_string(),
                    "приня".to_string(),
                    "прият".to_string(),
                    "принц".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "прежде".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "рыбо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пре".to_string(),
                    ),
                ряд_пропусков: vec![
                    "прежде".to_string(),
                    "прещ".to_string(),
                    "прет".to_string(),
                    "предмет".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "между".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "едино".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "море".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пуле".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "веро".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пусто".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "букво".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "три".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пред".to_string(),
                    ),
                ряд_пропусков: vec![
                    "предават".to_string(),
                    "предавал".to_string(),
                    "предал".to_string(),
                    "предан".to_string(),
                    "предат".to_string(),
                    "предел".to_string(),
                    "предм".to_string(),
                    "преж".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "друже".to_string(),
                    ),
                ряд_пропусков: vec!["дружес".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "вос".to_string(),
                    ),
                ряд_пропусков: vec![
                    "востр".to_string(),
                    "восток".to_string(),
                    "восточ".to_string(),
                    "вось".to_string(),
                    "вослав".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "суе".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "криво".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "небо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "южно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "северо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "западо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "востоко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "тепло".to_string(),
                    ),
                ряд_пропусков: vec![
                    "теплову".to_string(),
                    "тепловым".to_string(),
                    "тепловых".to_string(),
                    "тепловые".to_string(),
                    "теплово".to_string(),
                    "теплова".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "свет".to_string(),
                    ),
                ряд_пропусков: vec![
                    "светск".to_string(),
                    "светло".to_string(),
                    "свето".to_string(),
                    "свети".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "вы".to_string(),
                    ),
                ряд_пропусков: vec![
                    "выкл".to_string(),
                    "выка".to_string(),
                    "выч".to_string(),
                    "выкш".to_string(),
                    "выкн".to_string(),
                    "высит".to_string(),
                    "высок".to_string(),
                    "высш".to_string(),
                    "выбр".to_string(),
                    "вымп".to_string(),
                    "выше".to_string(),
                    "выбор".to_string(),
                    "выбир".to_string(),
                    "выша".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "крово".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "обо".to_string(),
                    ),
                ряд_пропусков: vec!["обобщ".to_string(), "обог".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "кратко".to_string(),
                    ),
                ряд_пропусков: vec!["краткост".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "ино".to_string(),
                    ),
                ряд_пропусков: vec!["иногд".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "взрыво".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "власть".to_string(),
                    ),
                ряд_пропусков: vec!["властьм".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мало".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "быдло".to_string(),
                    ),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "крове".to_string(),
                    ),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "материало".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "машино".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мульти".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "зубо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "радио".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "без".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "бес".to_string(),
                    ),
                ряд_пропусков: vec![
                    "бесн".to_string(),
                    "бесед".to_string(),
                    "беси".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "громко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "скоро".to_string(),
                    ),
                ряд_пропусков: vec!["скорост".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "быстро".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "зло".to_string(),
                    ),
                //ряд_пропусков: vec!["злов".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "домо".to_string(),
                    ),
                ряд_пропусков: vec!["домолв".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "семя".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "долго".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "до".to_string(),
                    ),
                ряд_пропусков: vec![
                    "доск".to_string(),
                    "довой".to_string(),
                    "довл".to_string(),
                    "доник".to_string(),
                    "досад".to_string(),
                    "дол".to_string(),
                    "дом".to_string(),
                    "дой".to_string(),
                    "дор".to_string(),
                    "достат".to_string(),
                    "досуг".to_string(),
                    "доч".to_string(),
                    "добр".to_string(),
                    "досто".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "умо".to_string(),
                    ),
                ряд_пропусков: vec!["умолч".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "досто".to_string(),
                    ),
                ряд_пропусков: vec!["достойн".to_string(), "достоин".to_string()]
                    .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "сверх".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "воздухо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "соот".to_string(),
                    ),
                ряд_пропусков: vec!["соотеч".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "бое".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "взаимо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "взаимно".to_string(),
                    ),
                ряд_пропусков: vec!["взаимност".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "само".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "хитро".to_string(),
                    ),
                ряд_пропусков: vec!["хитрост".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "лже".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "противо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "лице".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "круго".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "про".to_string(),
                    ),
                ряд_пропусков: vec![
                    "проч".to_string(),
                    "проса".to_string(),
                    "просо".to_string(),
                    "прост".to_string(),
                    "прось".to_string(),
                    "проси".to_string(),
                    "прощ".to_string(),
                    "против".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пожаро".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "свеже".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "стихо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "чрез".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "самолето".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "ума".to_string(),
                    ),
                ряд_пропусков: vec![
                    "умалял".to_string(),
                    "умаляе".to_string(),
                    "умалит".to_string(),
                    "умаляю".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "священно".to_string(),
                    ),
                ряд_пропусков: vec!["священност".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "на".to_string(),
                    ),
                ряд_пропусков: vec![
                    "надет".to_string(),
                    "надев".to_string(),
                    "нагл".to_string(),
                    "наил".to_string(),
                    "нажая".to_string(),
                    "наруж".to_string(),
                    "наук".to_string(),
                    "най".to_string(),
                    "нач".to_string(),
                    "нарк".to_string(),
                    "наш".to_string(),
                    "начал".to_string(),
                    "нано".to_string(),
                    "народо".to_string(),
                    "над".to_string(),
                    "напере".to_string(),
                    "наиб".to_string(),
                    "наим".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "наи".to_string(),
                    ),
                ряд_пропусков: vec!["наимено".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "крыше".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "косно".to_string(),
                    ),
                ряд_пропусков: vec!["косновен".to_string(), "коснос".to_string()]
                    .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "нраво".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "нано".to_string(),
                    ),
                ряд_пропусков: vec!["наносит".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "пико".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "будо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "милли".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "килло".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "твердо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мега".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "самолёто".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "тазо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "прапра".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "языко".to_string(),
                    ),
                ряд_пропусков: vec![
                    "языково".to_string(),
                    "языкова".to_string(),
                    "языкову".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "миро".to_string(),
                    ),
                ряд_пропусков: vec![
                    "мировой".to_string(),
                    "мирова".to_string(),
                    "мировы".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "народо".to_string(),
                    ),
                ряд_пропусков: vec!["народова".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "верхо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "верхне".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "нижне".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "ново".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "с".to_string(),
                    ),
                ряд_пропусков: vec![
                    "стают".to_string(),
                    "стачу".to_string(),
                    "стача".to_string(),
                    "стачей".to_string(),
                    "стач".to_string(),
                    "с".to_string(),
                    "сапог".to_string(),
                    "сапож".to_string(),
                    "смел".to_string(),
                    "субб".to_string(),
                    "стекол".to_string(),
                    "стёкл".to_string(),
                    "стёкол".to_string(),
                    "слёз".to_string(),
                    "слезы".to_string(),
                    "слезам".to_string(),
                    "слезливо".to_string(),
                    "слезах".to_string(),
                    "слуг".to_string(),
                    "сёстр".to_string(),
                    "стир".to_string(),
                    "скам".to_string(),
                    "сидят".to_string(),
                    "сидел".to_string(),
                    "сидет".to_string(),
                    "сидев".to_string(),
                    "сидит".to_string(),
                    "сложн".to_string(),
                    "стаив".to_string(),
                    "супруг".to_string(),
                    "супруж".to_string(),
                    "сраз".to_string(),
                    "сух".to_string(),
                    "счаст".to_string(),
                    "скром".to_string(),
                    "стакан".to_string(),
                    "стекл".to_string(),
                    "сиде".to_string(),
                    "сидя".to_string(),
                    "стоя".to_string(),
                    "стою".to_string(),
                    "страив".to_string(),
                    "стрем".to_string(),
                    "слажд".to_string(),
                    "сият".to_string(),
                    "сияет".to_string(),
                    "сиял".to_string(),
                    "сияю".to_string(),
                    "сияем".to_string(),
                    "строил".to_string(),
                    "строящ".to_string(),
                    "строит".to_string(),
                    "строющ".to_string(),
                    "сеи".to_string(),
                    "сея".to_string(),
                    "слыш".to_string(),
                    "слуш".to_string(),
                    "строи".to_string(),
                    "сей".to_string(),
                    "свою".to_string(),
                    "стро".to_string(),
                    "сюд".to_string(),
                    "слав".to_string(),
                    "серж".to_string(),
                    "серд".to_string(),
                    "стаё".to_string(),
                    "скук".to_string(),
                    "стае".to_string(),
                    "срок".to_string(),
                    "сроч".to_string(),
                    "скуч".to_string(),
                    "сумоч".to_string(),
                    "сумк".to_string(),
                    "свобод".to_string(),
                    "спел".to_string(),
                    "сест".to_string(),
                    "стар".to_string(),
                    "скор".to_string(),
                    "слоя".to_string(),
                    "сужд".to_string(),
                    "сим".to_string(),
                    "стич".to_string(),
                    "серед".to_string(),
                    "сыр".to_string(),
                    "слои".to_string(),
                    "слоё".to_string(),
                    "слой".to_string(),
                    "слое".to_string(),
                    "сков".to_string(),
                    "степ".to_string(),
                    "сед".to_string(),
                    "стрек".to_string(),
                    "скот".to_string(),
                    "сперм".to_string(),
                    "спал".to_string(),
                    "спас".to_string(),
                    "сып".to_string(),
                    "сон".to_string(),
                    "сад".to_string(),
                    "стиг".to_string(),
                    "стяг".to_string(),
                    "слад".to_string(),
                    "стран".to_string(),
                    "страх".to_string(),
                    "страш".to_string(),
                    "сыл".to_string(),
                    "сещ".to_string(),
                    "сеч".to_string(),
                    "суч".to_string(),
                    "сук".to_string(),
                    "сеив".to_string(),
                    "сын".to_string(),
                    "сал".to_string(),
                    "смат".to_string(),
                    "стат".to_string(),
                    "сиг".to_string(),
                    "сказ".to_string(),
                    "стал".to_string(),
                    "сел".to_string(),
                    "сут".to_string(),
                    "сек".to_string(),
                    "сколь".to_string(),
                    "сет".to_string(),
                    "стен".to_string(),
                    "сбор".to_string(),
                    "ссыл".to_string(),
                    "стру".to_string(),
                    "стальн".to_string(),
                    "сил".to_string(),
                    "сах".to_string(),
                    "сег".to_string(),
                    "сем".to_string(),
                    "скач".to_string(),
                    "строг".to_string(),
                    "скаль".to_string(),
                    "сис".to_string(),
                    "сто".to_string(),
                    "сним".to_string(),
                    "служ".to_string(),
                    "смотр".to_string(),
                    "сред".to_string(),
                    "суд".to_string(),
                    "стоп".to_string(),
                    "случ".to_string(),
                    "себ".to_string(),
                    "свои".to_string(),
                    "свой".to_string(),
                    "сваи".to_string(),
                    "слаб".to_string(),
                    "стрел".to_string(),
                    "снов".to_string(),
                    "став".to_string(),
                    "сам".to_string(),
                    "счёт".to_string(),
                    "счет".to_string(),
                    "счит".to_string(),
                    "спин".to_string(),
                    "стан".to_string(),
                    "ступ".to_string(),
                    "стой".to_string(),
                    "слов".to_string(),
                    "след".to_string(),
                    "сущ".to_string(),
                    "свя".to_string(),
                    "сбое".to_string(),
                    "светло".to_string(),
                    "сельско".to_string(),
                    "сельхоз".to_string(),
                    "семено".to_string(),
                    "семи".to_string(),
                    "смехо".to_string(),
                    "свое".to_string(),
                    "своё".to_string(),
                    "судо".to_string(),
                    "строе".to_string(),
                    "судьбо".to_string(),
                    "со".to_string(),
                    "сног".to_string(),
                    "сине".to_string(),
                    "сильно".to_string(),
                    "стекло".to_string(),
                    "сухо".to_string(),
                    "сума".to_string(),
                    "скачко".to_string(),
                    "слабо".to_string(),
                    "средне".to_string(),
                    "строй".to_string(),
                    "солнце".to_string(),
                    "свет".to_string(),
                    "скоро".to_string(),
                    "сверх".to_string(),
                    "соот".to_string(),
                    "само".to_string(),
                    "старо".to_string(),
                    "спо".to_string(),
                    "слово".to_string(),
                    "счето".to_string(),
                    "свето".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "старо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "бого".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "много".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "выше".to_string(),
                    ),
                ряд_пропусков: vec!["вышен".to_string(), "вышения".to_string()]
                    .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "клетко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "русско".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "четверо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "корабле".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "удобо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "спо".to_string(),
                    ),
                ряд_пропусков: vec![
                    "спок".to_string(),
                    "способ".to_string(),
                    "спор".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "верно".to_string(),
                    ),
                ряд_пропусков: vec!["верност".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "разо".to_string(),
                    ),
                ряд_пропусков: vec!["разова".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "раз".to_string(),
                    ),
                ряд_пропусков: vec![
                    "разивш".to_string(),
                    "разит".to_string(),
                    "разую".to_string(),
                    "разует".to_string(),
                    "разуют".to_string(),
                    "разц".to_string(),
                    "разов".to_string(),
                    "разны".to_string(),
                    "разно".to_string(),
                    "разыгр".to_string(),
                    "разор".to_string(),
                    "разни".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "разно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "высоко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "низко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "впереди".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "родо".to_string(),
                    ),
                ряд_пропусков: vec!["родован".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "родно".to_string(),
                    ),
                ряд_пропусков: vec!["родност".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "руко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "дето".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "редко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "узко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "овоще".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "муже".to_string(),
                    ),
                ряд_пропусков: vec!["мужест".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "жено".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "жертво".to_string(),
                    ),
                ряд_пропусков: vec!["жертвов".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "чино".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "видо".to_string(),
                    ),
                ряд_пропусков: vec!["видов".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "лизо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "вино".to_string(),
                    ),
                ряд_пропусков: vec!["винов".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "члено".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "черво".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "чрево".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "человеко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "огне".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "звуко".to_string(),
                    ),
                ряд_пропусков: vec!["звуков".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "камне".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "слово".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "после".to_string(),
                    ),
                ряд_пропусков: vec!["послед".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "идоло".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "счето".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "духо".to_string(),
                    ),
                ряд_пропусков: vec!["духов".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "отказо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "тяго".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "о".to_string(),
                    ),
                ряд_пропусков: vec![
                    "окон".to_string(),
                    "окна".to_string(),
                    "окном".to_string(),
                    "озер".to_string(),
                    "озёр".to_string(),
                    "олен".to_string(),
                    "охот".to_string(),
                    "овраг".to_string(),
                    "оврал".to_string(),
                    "овраж".to_string(),
                    "орёт".to_string(),
                    "орет".to_string(),
                    "орал".to_string(),
                    "орат".to_string(),
                    "окая".to_string(),
                    "окие".to_string(),
                    "окий".to_string(),
                    "оким".to_string(),
                    "окость".to_string(),
                    "оких".to_string(),
                    "опас".to_string(),
                    "онов".to_string(),
                    "одн".to_string(),
                    "октяб".to_string(),
                    "оч".to_string(),
                    "однаж".to_string(),
                    "опер".to_string(),
                    "опор".to_string(),
                    "очум".to_string(),
                    "очен".to_string(),
                    "одни".to_string(),
                    "основ".to_string(),
                    "особ".to_string(),
                    "орг".to_string(),
                    "от".to_string(),
                    "один".to_string(),
                    "об".to_string(),
                    "обще".to_string(),
                    "одно".to_string(),
                    "обез".to_string(),
                    "оптико".to_string(),
                    "осново".to_string(),
                    "овоще".to_string(),
                    "огне".to_string(),
                    "отказ".to_string(),
                    "около".to_string(),
                    "обороно".to_string(),
                    "остро".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "от".to_string(),
                    ),
                ряд_пропусков: vec![
                    "отрав".to_string(),
                    "отяго".to_string(),
                    "отказо".to_string(),
                    "отец".to_string(),
                    "отцо".to_string(),
                    "отч".to_string(),
                    "отдых".to_string(),
                    "отдох".to_string(),
                    "отуп".to_string(),
                ]
                .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "меж".to_string(),
                    ),
                ряд_пропусков: vec!["межут".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "мертво".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "двадцати".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "дурно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "садо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "лихо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "тяжело".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "душе".to_string(),
                    ),
                ряд_пропусков: vec!["душевн".to_string(), "душевен".to_string()]
                    .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "около".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "волко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "обороно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "фрукто".to_string(),
                    ),
                ряд_пропусков: vec!["фруктов".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "ветхо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "тело".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "дорого".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "городо".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "госте".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "хладно".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "электро".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "воз".to_string(),
                    ),
                ряд_пропусков: vec!["возчик".to_string()].в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "голово".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "почерко".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "брако".to_string(),
                    ),

                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "остро".to_string(),
                    ),
                ряд_пропусков: vec!["остров".to_string(), "острота".to_string()]
                    .в_умные(),
                ..Default::default()
            },
            Ячейка_замены_с_разделителями {
                искомое_слово:
                    Text_Changer::Умная_Строка::создать_значение(
                        "свето".to_string(),
                    ),

                ..Default::default()
            },
        ],
    };
    use Text_Changer::Возможности_ячейки_замены_с_разделителями;
    use Text_Changer::Умная_Строка;
    //
    //
    for ячейка_замены in итоговый_словарь.содержимое.iter() {
        let mut куча_проверочная: rapidhash::fast::RapidHashSet<String> =
            rapidhash::fast::RapidHashSet::default();
        //заполнение оставшихся полей
        for образец in ячейка_замены.ряд_пропусков.iter() {
            //проверка на полное схождение
            if !куча_проверочная.contains(&образец.получить_значение())
            {
                let значение = образец.получить_значение();
                куча_проверочная.insert(значение);
            } else {
                println!(
                    "Разделители (полное схождение исключений) |{}| содержит повторно исключение  |{}|",
                    ячейка_замены.искомое_слово, образец
                );
            }
        }
        //собрать ряд RE исключений
    }
    //
    //
    //
    for ячейка_замены in итоговый_словарь.содержимое.iter() {
        //заполнение оставшихся полей
        for (указатель_1, образец_1) in ячейка_замены.ряд_пропусков.iter().enumerate()
        {
            for (указатель_2, образец_2) in ячейка_замены
                .ряд_пропусков
                .iter()
                .enumerate()
                .filter(|(указатель_2, _)| *указатель_2 != указатель_1)
            {
                if sz_найти_в_умной_строке_себя(образец_2, образец_1)
                {
                    println!(
                        "Разделители (частичное схождение исключений) |{}| содержит частичное исключение #{указатель_1} |{}| в слове  |{}| #{указатель_2}",
                        ячейка_замены.искомое_слово,
                        образец_1,
                        образец_2,
                        //ячейка_замены.ряд_исключений
                    );
                }
            }
        }
        //собрать ряд RE исключений
    }
    //исключения с заглавной буквы
    //
    //
    //
    for ячейка_замены in итоговый_словарь.содержимое.iter_mut()
    {
        //заполнение оставшихся полей
        ячейка_замены.добавить_оставшиеся_поля();
        ячейка_замены.ряд_пропусков = ячейка_замены
            .ряд_пропусков
            .iter()
            .map(|ячейка| {
                Text_Changer::Умная_Строка::создать_значение(
                    ячейка.as_str().to_case(Case::Lower),
                )
            })
            .collect();
        //собрать ряд RE исключений
        ячейка_замены.ряд_re_пропуски = ячейка_замены.добавить_re_пропуски_изнутри();
        //
        ячейка_замены.ряд_обязательств = ячейка_замены
            .ряд_обязательств
            .iter()
            .map(|ячейка| {
                Text_Changer::Умная_Строка::создать_значение(
                    ячейка.получить_значение().to_case(Case::Lower),
                )
            })
            .collect();
        //собрать ряд RE исключений
        ячейка_замены.ряд_re_обязательства = ячейка_замены.добавить_re_обязательства_изнутри();
    }
    //
    //
    let образцы_поиска_re_для_проверки: Vec<&Regex> = итоговый_словарь
        .содержимое
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
    return Ok(итоговый_словарь);
}
pub fn создать_словарь_замен() -> Словарь_Переносов {
    //use crate::dictionary_0::проверка_ряда_regex;
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
        подсчёт: (0..словарь_замен.содержимое.len())
            .map(|_| AtomicUsize::new(0))
            .collect(),
        /*с_заглавной: (0..словарь_замен.ряд_1.len())
        .map(|_| AtomicUsize::new(0))
        .collect(),*/
    });
}
use crate::import::functions::{
    преобразовать_слово_с_чертой_в_конце, преобразовать_слово_с_чертой_в_конце_в_умную_строку,
};
use crate::xlsx::import_xlsx::{
    найти_особые_знаки, найти_особые_знаки_в_умной_строке, обратно_убрать_особые_знаки,
    обратно_убрать_особые_знаки_умная_строка,
};
use convert_case::{Case, Casing};
//use xml::Encoding::Default;

pub fn создать_второй_словарь_разделителей(
    mut словарь_изначальный: Словарь_разделителей,
) -> Словарь_разделителей {
    use Text_Changer::Возможности_ячейки_замены_с_разделителями;
    use Text_Changer::КОЛИЧЕСТВО_БУКВ_ПОСЛЕ_РАЗДЕЛИТЕЛЯ;
    словарь_изначальный
        .содержимое
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово =
                Text_Changer::Умная_Строка::создать_значение(
                    ячейка
                        .искомое_слово
                        .получить_значение()
                        .to_case(Case::Sentence),
                );
            let новый_образец: String = format!(r#"\b{{start}}{}\w"#, ячейка.искомое_слово);
            ячейка.re_образец_для_поиска = Regex::new(&новый_образец).unwrap();
            //
            let временный_составной_ряд: (Text_Changer::Умная_Строка, Vec<char>) =
                match найти_особые_знаки_в_умной_строке(&ячейка.замена)
                {
                    Ok(успех) => успех,
                    Err(ошибка) => panic!(),
                };
            //
            //ячейка.re_образец_для_замены=
            let новый_образец: String = format!(
                r#"\b{{start}}({})([\w]{{{КОЛИЧЕСТВО_БУКВ_ПОСЛЕ_РАЗДЕЛИТЕЛЯ},}})"#,
                ячейка.искомое_слово
            );
            ячейка.re_образец_для_замены = Regex::new(&новый_образец).unwrap();
            ячейка.замена = обратно_убрать_особые_знаки_умная_строка(
                преобразовать_слово_с_чертой_в_конце_в_умную_строку(
                    временный_составной_ряд.0.to_case(Case::Sentence),
                ),
            );
            //исключения с заглавной буквы
            ячейка.ряд_пропусков = ячейка
                .ряд_пропусков
                .iter()
                .map(|ячейка| ячейка.to_case(Case::Sentence))
                .collect();
            //собрать ряд RE исключений
            ячейка.ряд_re_пропуски = ячейка.добавить_re_пропуски_изнутри();
            //исключения с заглавной буквы
            ячейка.ряд_обязательств = ячейка
                .ряд_обязательств
                .iter()
                .map(|ячейка| ячейка.to_case(Case::Sentence))
                .collect();
            //собрать ряд RE исключений
            ячейка.ряд_re_обязательства = ячейка.добавить_re_обязательства_изнутри();
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
    use Text_Changer::Возможности_ячейки_замены_с_разделителями;
    use Text_Changer::КОЛИЧЕСТВО_БУКВ_ПОСЛЕ_РАЗДЕЛИТЕЛЯ;
    словарь_изначальный
        .содержимое
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.to_case(Case::Upper);
            let новый_образец: String = format!(r#"\b{{start}}{}\w"#, ячейка.искомое_слово);
            ячейка.re_образец_для_поиска = Regex::new(&новый_образец).unwrap();
            //
            let временный_составной_ряд: (Text_Changer::Умная_Строка, Vec<char>) =
                найти_особые_знаки_в_умной_строке(&ячейка.замена).unwrap();
            //
            //ячейка.re_образец_для_замены=
            let новый_образец: String = format!(
                r#"\b{{start}}({})([\w]{{{КОЛИЧЕСТВО_БУКВ_ПОСЛЕ_РАЗДЕЛИТЕЛЯ},}})"#,
                ячейка.искомое_слово
            );
            ячейка.re_образец_для_замены = Regex::new(&новый_образец).unwrap();
            ячейка.замена =
                обратно_убрать_особые_знаки_умная_строка(
                    преобразовать_слово_с_чертой_в_конце_в_умную_строку(
                        временный_составной_ряд.0.to_case(Case::Upper),
                    ),
                );
            //исключения с заглавной буквы
            ячейка.ряд_пропусков = ячейка
                .ряд_пропусков
                .iter()
                .map(|ячейка| ячейка.to_case(Case::Upper))
                .collect();
            //собрать ряд RE исключений
            ячейка.ряд_re_пропуски = ячейка.добавить_re_пропуски_изнутри();
            //
            ячейка.ряд_обязательств = ячейка
                .ряд_обязательств
                .iter()
                .map(|ячейка| ячейка.to_case(Case::Upper))
                .collect();
            //собрать ряд RE исключений
            ячейка.ряд_re_обязательства = ячейка.добавить_re_обязательства_изнутри();
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
    // let замена_тире: LazyLock<Regex> = LazyLock::new(|| Regex::new("-").unwrap();
    словарь_переносов
        .однобуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", "—");
        });
    словарь_переносов
        .многобуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", "—");
        });
    словарь_переносов
        .исключения
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", "—");
            let новый_образец = ячейка.re_образец_для_поиска.as_str().replace("-", "—");
            ячейка.re_образец_для_поиска = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", "—");
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
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", "—");
        });
    словарь_переносов
        .трехбуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", "—");
        });
    словарь_переносов
        .целиковые
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", "—");
            let новый_образец = ячейка.re_образец.as_str().replace("-", "—");
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", "—");
        });

    словарь_переносов
}
pub fn создать_третий_словарь_переносов(
    mut словарь_переносов: Словарь_Переносов,
) -> Словарь_Переносов {
    //let замена_тире: LazyLock<Regex> = LazyLock::new(|| Regex::new("-").unwrap();
    let на_что_заменять: String = " - ".to_string();
    словарь_переносов
        .однобуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", &на_что_заменять);
        });
    словарь_переносов
        .многобуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", &на_что_заменять);
        });
    словарь_переносов
        .исключения
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", &на_что_заменять);
            let новый_образец = ячейка
                .re_образец_для_поиска
                .as_str()
                .replace("-", &на_что_заменять);
            ячейка.re_образец_для_поиска = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", &на_что_заменять);
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
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", &на_что_заменять);
        });
    словарь_переносов
        .трехбуквенные
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", &на_что_заменять);
        });
    словарь_переносов
        .целиковые
        .par_iter_mut()
        .enumerate()
        .for_each(|(_указатель, ячейка)| {
            ячейка.искомое_слово = ячейка.искомое_слово.replace("-", &на_что_заменять);
            let новый_образец = ячейка.re_образец.as_str().replace("-", &на_что_заменять);
            ячейка.re_образец = Regex::new(&новый_образец).unwrap();
            ячейка.замена = ячейка.замена.replace("-", &на_что_заменять);
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
/*/
pub fn добавить_слова_с_окончаниями() {
    use std::default::Default;

    pub struct Окончания {
        pub щ: [String; 17],
    }
    impl Default for Окончания {
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
}*/

pub fn есть_ли_исключение(шаблоны: &[Regex], текст: &str) -> bool {
    шаблоны.par_iter().any(|шаблон| шаблон.is_match(текст))
}
/*pub fn есть_ли_исключение(
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
    //
    return false;
}*/

fn создать_разделы_словаря_переносов() -> Словарь_Переносов {
    const ОДНОБУКВЕННЫЕ_РЯД: [&'static str;
        Text_Changer::СЛОВАРЬ_ПЕРЕНОСОВ_ОДНОБУКВЕННЫЕ] = ["-о", "-а", "-ь", "-ы", "-и", "-ъ", "-у"];

    const МНОГОБУКВЕННЫЕ_РЯД: [&'static str;
        Text_Changer::СЛОВАРЬ_ПЕРЕНОСОВ_МНОГОБУКВЕННЫЕ] = [
        "-ройства ",
        "-вязывающего ",
        "-ближенный ",
        "-стое",
        "-ному",
        "-мыми",
        "-sign",
        "-utes",
        "-lete",
        "-tium",
        "-ющая",
        "-нове",
        "-дены",
        "-дить",
        "-лась",
        "-брос",
        "-фере",
        "-тоды",
        "-стей",
        "-ской",
        "-нием",
        "-ский",
        "-дена",
        "-жима",
        "-рьер",
        "-верх",
        "-стера",
        "-рами",
        "-дела",
        "-ходя",
        "-руте",
        "-ряют",
        "-дует",
        "-дачи",
        "-теке",
        /*
            "-либо",
        */
        "-чить",
        "-манд",
        "-дать",
        "-иумы",
        "-ования",
        "-овать",
        "-иями",
        "-ующие",
        "-ующая",
        "-ующий",
        "-ующих",
        "-уется",
        "-уются",
        "-ичную",
        "-ичных",
        "-ного",
        "-ость",
        "-ости",
        "-остью",
        "-нные",
        "-нного",
        "-нный",
        "-нных",
        "-уете",
    ];
    const ТРЕХБУКВЕННЫЕ_РЯД: [&'static str;
        Text_Changer::СЛОВАРЬ_ПЕРЕНОСОВ_ТРЕХБУКВЕННЫЕ] = [
        "-ков", "-щий", "-дят", "-ter", "-tus", "-tom", "-ции", "-кам", "-тём", "-щью", "-лом",
        "-дан", "-ста", "-тия", "-дой", "-вая", "-ния", "-лон", "-рых", "-рый", "-мые", "-щем",
        "-ний", "-зок", "-тем", "-ные", "-нию", "-шин", "-тый", "-нюю", "-гда", "-бой", "-вые",
        "-дов", "-тов", "-пей", "-мый", "-nal", "-щие", "-вой", "-ром", "-мер", "-них", "-кие",
        "-чет", "-ект", "-жет", "-ком", "-вил", "-тым", "-ких", "-вым", "-зом", "-рой", "-чек",
        "-той", "-гут", "-ние", "-ных", "-кой", "-ала", "-уют", "-еям", "-нат", "-иев", "-иал",
        "-ием", "-иум", "-ыми", "-чим", "-ика", "-ику", "-ики", "-ать", "-ять", "-ным", "-еть",
        "-лен", "-иям", "-дом", "-sor", "-уум", "-уем", "-ким", "-ешь", "-ишь", "-ток", "-ете",
        "-ите", "-ует", "-яла", "-али", "-яли", "-ола", "-ела", "-оли", "-ели", "-ула", "-ули",
        "-ами", "-еми", "-емя", "-ёте", "-чие", "-сте", "-ёшь", "-том", "-ого", "-ций", "-жен",
        "-ому", "-дач", "-иях", "-ией", "-умя", "-ими", "-тор", "-рые", "-сти", "-чае", "-вод",
        "-лов", "-кое",
    ];
    const ДВУБУКВЕННЫЕ_РЯД: [&'static str;
        Text_Changer::СЛОВАРЬ_ПЕРЕНОСОВ_ДВУБУКВЕННЫЕ] = [
        "-ца", "-сы", "-er", "-мы", "-ры", "-ра", "-ты", "-ка", "-ло", "-жа", "-та", "-ли", "-ея",
        "-еи", "-ях", "-ев", "-ки", "-да", "-ых", "-ям", "-ии", "-ия", "-ся", "-ая", "-яя", "-ое",
        "-ее", /*
                   "-ой",
               */
        "-ые", "-ий", "-ем", "-им", "-ет", "-ит", "-ут", "-ру", "-ют", "-ят", "-ял", "-ол", "-ел",
        "-ул", "-ам", "-ас", "-ах", "-ко", "-её", "-ей", "-ех", "-ею", "-ёт", "-ёх", "-ие", "-их",
        "-ию", "-но", "-ми", "-мя", "-ов", "-оё", "-см", "-ум", "-уя", "-ух", "-ую", "-шь", "-ны",
        "-пи", "-па",
    ];
    const ЦЕЛИКОВЫЕ_РЯД: [&'static str;
        Text_Changer::СЛОВАРЬ_ПЕРЕНОСОВ_ЦЕЛИКОВЫЕ] = [
        "-валентных",
        "-поминающих",
        "-зации",
        "-денции",
        "-личаются",
        "-ровать",
        "-тельными",
        "-рифмический",
        "-рительными",
        "-лучила",
        "-пульсный",
        "-менными",
        "-правленный",
        "-зится",
        "-дификацию",
        "-ляться",
        "-рительной",
        "-зических",
        "-вается",
        "-корректности",
        "-руется",
        "-совано",
        "-турой",
        "-пустимого",
        "-стовый",
        "-стояние",
        "-ствами",
        "-гическую",
        "-шинного",
        "-матном",
        "-значены",
        "-нальные",
        "-крепленные",
        "-тимальности",
        "-гональных",
        "-чезнут",
        "-кание",
        "-гаться",
        "-зируя",
        "-рячими",
        "-ливаемое",
        "-лагаемый",
        "-ритета",
        "-почтительный",
        "-ляющее",
        "-нейкой",
        "-хождении",
        "-исходит",
        "-метров",
        "-ства",
        "-ровой",
        "-знаку",
        "-числены",
        "-рованы",
        "-межуточных",
        "-гласование",
        "-обходимое",
        "-новления",
        "-ских",
        "-данса",
        "-фектов",
        "-редач",
        "-нитные",
        "-ключается",
        "-ментов",
        "-граммный",
        "-вания",
        "-шений",
        "-никло",
        "-чиком",
        "-чатных",
        "-полняются",
        "-нелей",
        "-торые",
        "-тально",
        "-менно",
        "-торая",
        "-раммного",
        "-мендуется",
        "-крытый",
        "-тивным",
        "-манды",
        "-тронной",
        "-численных",
        "-ленную",
        "-стемный",
        "-ческих",
        "-тура",
        "-ждений",
        "-шемся",
        "-мента",
        "-мандой",
        "-тинные",
        "-нель",
        "-сутствует",
        "-симо",
        "-пени",
        "-тельно",
        "-чанию",
        "-ческая",
        "-бирать",
        "-единитель",
        "-зуемся",
        "-ветствующие",
        "-матическая",
        "-нентов",
        "-нала",
        "-тистические",
        "-стимо",
        "-жителем",
        "-товых",
        "-цессе",
        "-екта",
        "-новлены",
        "-рования",
        "-раметры",
        "-чески",
        "-брав",
        "-реноса",
        "-зультаты",
        "-ходных",
        "-тырех",
        "-кать",
        "-мент",
        "-штаба",
        "-местно",
        "-ления",
        "-тактные",
        "-таллизации",
        "-нить",
        "-ветствующим",
        "-единения",
        "-вать",
        "-тически",
        "-дами",
        "-борочно",
        "-веден",
        "-ражает",
        "-ством",
        "-тора",
        "-кусом",
        "-лучить",
        "-вание",
        "-рантирует",
        "-менных",
        "-ствующим",
        "-тронных",
        "-логического",
        "-рину",
        "-нент",
        "-тива",
        "-нений",
        "-ченных",
        "-ченный",
        "-рации",
        "-митивов",
        "-щение",
        "-щего",
        "-виша",
        "-ление",
        "-рибуты",
        "-понент",
        "-понента",
        "-норамирования",
        "-можно",
        "-стра",
        "-изведен",
        "-бранному",
        "-вится",
        "-скую",
        "-струкция",
        "-торых",
        "-веденных",
        "-сколько",
        "-ются",
        "-ствуют",
        "-павшие",
        "-верстия",
        "-ванные",
        "-реходных",
        "-слойные",
        "-водится",
        "-вами",
        "-митивы",
        "-пользуемых",
        "-няться",
        "-дартов",
        "-ность",
        "-ленных",
        "-пусках",
        "-бавления",
        "-дактировать",
        "-тический",
        "-дактор",
        "-ретащим",
        "-зицию",
        "-рения",
        "-зателя",
        "-затель",
        "-водами",
        "-кладка",
        "-деления",
        "-ражения",
        "-телем",
        "-садочных",
        "-дактора",
        "-ченной",
        "-распознанный",
        "-моугольный",
        "-циями",
        "-тированный",
        "-варительно",
        "-емость",
        "-ваться",
        "-когда",
        "-ответствии",
        "-этому",
        "-слеживания",
        "-рирует",
        "-сения",
        "-ниями",
        "-структивными",
        "-ствия",
        "-единять",
        "-шения",
        "-изводить",
        "-жимах",
        "-чайшее",
        "-ношении",
        "-ровки",
        "-изводиться",
        "-бирает",
        "-ностями",
        "-виши",
        "-тивизации",
        "-личных",
        "-ложение",
        "-тивной",
        "-логических",
        "-нивает",
        "-слойной",
        "-нимается",
        "-тельного",
        "-вость",
        "-сматриваются",
        "-суждений",
        "-дарственное",
        "-чайное",
        "-ниченные",
        "-ветить",
    ];
    //
    let исключения: [Ячейка_замены_с_исключением;
        Text_Changer::СЛОВАРЬ_ПЕРЕНОСОВ_ИСКЛЮЧЕНИЯ] = [
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
        привести_ряд_строк_словаря_переносов_в_стопку_строгую(
                ОДНОБУКВЕННЫЕ_РЯД,
            ),
        многобуквенные:
        привести_ряд_строк_словаря_переносов_в_стопку_строгую(
                МНОГОБУКВЕННЫЕ_РЯД,
            ),
        трехбуквенные:
        привести_ряд_строк_словаря_переносов_в_стопку_строгую(
                ТРЕХБУКВЕННЫЕ_РЯД,
            ),
        двубуквенные:
        привести_ряд_строк_словаря_переносов_в_стопку_строгую(
                ДВУБУКВЕННЫЕ_РЯД,
            ),
        целиковые:
        привести_ряд_строк_словаря_переносов_в_стопку_строгую(
                ЦЕЛИКОВЫЕ_РЯД,
            ),
        исключения: исключения,
    };
}
//
pub fn привести_ряд_сло_словаря_переносов_в_стопку_строгую<
    const N: usize,
>(
    ряд: [String; N],
) -> [Text_Changer::Ячейка_замены; N] {
    use std::default::Default;
    //Default
    let mut ряд_итоговый: [Text_Changer::Ячейка_замены; N] =
        std::array::from_fn(|_| Default::default());
    //
    for (указатель, слово) in ряд.into_iter().enumerate() {
        //
        let ряд_знаков: Vec<char> = слово.chars().collect();
        let замена: Text_Changer::Умная_Строка =
            Text_Changer::Умная_Строка::создать_значение(
                ряд_знаков[1..].iter().collect::<String>(),
            );
        //
        ряд_итоговый[указатель].re_образец =
            Regex::new(&format!(r##"(?i)\b{{end}}{}\b{{end}}"##, слово)).unwrap();
        ряд_итоговый[указатель].замена = замена;
        ряд_итоговый[указатель].искомое_слово =
            Text_Changer::Умная_Строка::создать_значение(
                слово.to_string(),
            );
    }
    //
    return ряд_итоговый;
}
pub fn привести_ряд_строк_словаря_переносов_в_стопку_строгую<
    const N: usize,
>(
    ряд: [&'static str; N],
) -> [Text_Changer::Ячейка_замены; N] {
    use std::default::Default;
    //Default
    let mut ряд_итоговый: [Text_Changer::Ячейка_замены; N] =
        std::array::from_fn(|_| Default::default());
    //
    for (указатель, слово) in ряд.into_iter().enumerate() {
        //
        let ряд_знаков: Vec<char> = слово.chars().collect();
        let замена: Text_Changer::Умная_Строка =
            Text_Changer::Умная_Строка::создать_значение(
                ряд_знаков[1..].iter().collect::<String>(),
            );
        //
        ряд_итоговый[указатель].re_образец =
            Regex::new(&format!(r##"(?i)\b{{end}}{}\b{{end}}"##, слово)).unwrap();
        ряд_итоговый[указатель].замена = замена;
        ряд_итоговый[указатель].искомое_слово =
            Text_Changer::Умная_Строка::создать_значение(
                слово.to_string(),
            );
    }
    //
    return ряд_итоговый;
}
fn поиск_повторов_re_словаря_замен(
    словарь_замен: &Text_Changer::Словарь_Переносов,
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
pub fn все_ли_заглавные_буквы_в_слове(
    слово: &str,
) -> Text_Changer::Правописание_слова {
    let mut счётчик_заглавных_букв: usize = 0;
    let mut длина_слова: usize = 0;
    // Все ли буквы заглавные?
    for знак in слово.chars() {
        длина_слова += 1;
        if знак.is_uppercase() {
            счётчик_заглавных_букв += 1;
        }
    }
    let пополам =
        безопасное_деление_на_2_не_цело_численное(длина_слова);
    //
    if счётчик_заглавных_букв >= пополам {
        return Text_Changer::Правописание_слова::Все_Заглавные;
    } else {
        return Text_Changer::Правописание_слова::Все_строчные;
    }

    /*if слово.chars().all(|c| c.is_uppercase()) {
        println!("Слово '{}' полностью ЗАГЛАВНОЕ", слово);
    }*/
}
fn безопасное_деление_на_2_цело_численное(
    число: usize,
) -> Option<usize> {
    if число % 2 == 0 {
        Some(число / 2)
    } else {
        None // Нечётное число — деление без остатка невозможно
    }
}
fn безопасное_деление_на_2_не_цело_численное(
    число: usize,
) -> usize {
    match число {
        0 => 0, // 0 / 2 = 0, остаток 0
        n => n / 2,
    }
}
