//use std::default;
use crate::lib::{
    self, Полный_Словарь, Словарь, Сообщения_для_книги
};
use lazy_static::lazy_static;
use std::thread;

use crate::output::write;
use crate::output::write::вывод_содержимого_в_txt;
use regex::Regex;
//use crate::import::{VirtualFs};
use std::time::{
    //Duration,
    Instant,
};
extern crate rayon;
use crate::utils;
use crate::utils::functions::*;
use crate::utils::functions_txt::*;
use crate::utils::hash::есть_ли_кириллица;
use crate::utils::stringzilla::{sz_найти, sz_упорядочить_ряд_строк};
use crate::xlsx::import_xlsx::поиск_уже_добавленных_слов;
use console::{Emoji, style};
use foldhash::{
    HashMap, HashSet, HashSetExt,
    fast::{FixedState, RandomState},
};
use indicatif::ProgressBar;
use indicatif::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use stringzilla::sz;

#[derive(Debug, Default, Clone)]
pub struct Исключения_для_кучи {
    pub указатель: usize,
    pub исключения: foldhash::HashSet<String>,
}
//изменение слов в книге
pub fn заменить_слова_в_книге(
    полный_словарь: &mut lib::Полный_Словарь, //вектор словарей
    mut книги: Vec<lib::Книги>,               //книги для изменения
    сообщения: &mut lib::Сообщения,
) -> Vec<lib::Книги> {
    use crate::utils::regex::*;
    use crate::utils::stringzilla::sz_найти;
    //шкала
    let mut временные_сообщения: Arc<Mutex<lib::Сообщения>> =
        Arc::new(Mutex::new(сообщения.clone()));
    //
    let точка_отсчёта_по_времени: Instant = Instant::now();
    let пути_общие: lib::Пути_Общие = Default::default();
    //случаи замены слов
    //создание словаря regex
    //быстрый словарь
    let куча_словарь: lib::Куча_Словарь =
        получить_кучи_из_словарей(&полный_словарь);
    //начало замены слов
    let pb = ProgressBar::new(0);
    // Настраиваем стиль прогресс-бара
    pb.set_style(
        ProgressStyle::default_bar()
            //.template("{spinner:.green} [{wide_bar:.cyan/blue}] {pos:>2}/{len:2} {msg}")
            .template("{msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    let LOOKING_GLASS = format!("🔍");
    // Обернем счетчики в Arc для безопасного разделения между потоками
    let mut счётчик_составное_важное: Vec<Arc<AtomicUsize>> =
        (0..полный_словарь.счётчик_составное_важное.len())
            .into_par_iter()
            .map(|_| Arc::new(AtomicUsize::new(0)))
            .collect();
    let mut счётчик_составное: Vec<Arc<AtomicUsize>> = (0..полный_словарь.счётчик_составное.len())
        .into_par_iter()
        .map(|_| Arc::new(AtomicUsize::new(0)))
        .collect();
    let mut счётчик_простое: Vec<Arc<AtomicUsize>> = (0..полный_словарь.счётчик_простое.len())
        .into_par_iter()
        .map(|_| Arc::new(AtomicUsize::new(0)))
        .collect();
    let mut счётчик_вездесущее: Vec<Arc<AtomicUsize>> =
        (0..полный_словарь.счётчик_вездесущее.len())
            .into_par_iter()
            .map(|_| Arc::new(AtomicUsize::new(0)))
            .collect();
    let mut счётчик_неизменное: Vec<Arc<AtomicUsize>> =
        (0..полный_словарь.счётчик_неизменное.len())
            .into_par_iter()
            .map(|_| Arc::new(AtomicUsize::new(0)))
            .collect();

    //перебор
    let количество_книг = книги.len();

    книги.par_iter_mut().enumerate()
         .filter(|(главный_указатель, книга_взятая)|
                 !sz_найти(&книга_взятая.расширение,"doc") //если это doc, то ничего не делать
         )
         .for_each(|(главный_указатель, книга_взятая)| {

             // Создаем Arc-копии счетчиков для этого потока
             let mut счётчик_неизменное_лок = счётчик_неизменное.clone();
             let mut счётчик_составное_важное_лок = счётчик_составное_важное.clone();
             let mut счётчик_вездесущее_лок = счётчик_вездесущее.clone();
             let mut счётчик_составное_лок = счётчик_составное.clone();
             let mut счётчик_простое_лок = счётчик_простое.clone();
         //проверка допустимых расширений
        //остальные расширения
            //временная переменная для хранения всех строк для их сравнения в конце
            let mut вложения_изначальные: Vec<lib::Вложения> = книга_взятая.вложения.clone();
             //Вывод имени книги
             let текущий_шаг_всех_книг:String = format!("[{}/{}]", главный_указатель + 1, количество_книг);
             println!(
                 "{}: {} {}",
                 style(текущий_шаг_всех_книг).strikethrough(),
                 style(&format!("{}.{}",
                     книга_взятая.название_книги,
                     книга_взятая.расширение,
                 )).cyan(),
                 LOOKING_GLASS
             );
             //счётчик файлов всех
             let счётчик_количества_вложенных_файлов:usize=книга_взятая.вложения.par_iter()
                 .filter(|вложение|
                             не_изображение_или_мусор (&вложение.имя)
             ).count();
            //перебор всего содержимого книги
            //перебор каждого файла во вложении (в том числе zip)
             //для указания на вложение
             let шаг_внутренний = AtomicU64::new(0);
            книга_взятая.вложения.par_iter_mut().enumerate()
                .filter(|(указатель, вложения)|
                    не_изображение_или_мусор(&вложения.имя)
                )
                .for_each(|(указатель, вложения)| {
                    let mut счётчик_неизменное_лок = счётчик_неизменное.clone();
                    let mut счётчик_составное_важное_лок = счётчик_составное_важное.clone();
                    let mut счётчик_вездесущее_лок = счётчик_вездесущее.clone();
                    let mut счётчик_составное_лок = счётчик_составное.clone();
                    let mut счётчик_простое_лок = счётчик_простое.clone();
                    
                    let текущий_шаг_всех_книг:String = format!("[{}/{}]", главный_указатель + 1, количество_книг);
                    let шаг_вложенных_книг = format!("[{}/{}]", шаг_внутренний.load(Ordering::Relaxed) + 1, счётчик_количества_вложенных_файлов);
                    шаг_внутренний.fetch_add(1, Ordering::Relaxed);
                    //вывод названия вложенного файла\
                // получение значений шагов всего для шкалы отсчёта
                    let к1= вложения.содержимое.len();
                let общее_количество =
                    полный_словарь.вездесущее.len()*к1+полный_словарь.простое.len()*к1
                +полный_словарь.составное.len()*к1+ полный_словарь.составное_важное.len()*к1;
                //получение указаталей на попуски
                let куча_пропусков: HashSet<usize> = utils::hash::получить_пропуски_для_содержимого(
                    &вложения.содержимое,
                    &вложения.имя,
                    &книга_взятая.расширение);
                //создание пропщенных строк
                let mut пропущенные_строки: Vec<String> = Vec::new();
                for указатель in куча_пропусков.iter() {
                    пропущенные_строки
                        .push(вложения.содержимое[*указатель].clone());
                }
                пропущенные_строки =
                    crate::utils::stringzilla::sz_упорядочить_ряд_строк(
                        пропущенные_строки,
                    );
                crate::output::dir::создать_папку_книги(
                    &книга_взятая.название_книги,
                    &книга_взятая.расширение,
                );
                let mut путь_вывода_пропусков = format!(
                    "{}{}/{}_{}.txt",
                    &пути_общие.вывод_книги_пропуски, &книга_взятая.название_книги,&книга_взятая.расширение,
                    вложения.имя_без_пути
                );
                if sz_найти(&книга_взятая.название_книги, "index")
                {
                    путь_вывода_пропусков = format!(
                        "{}{}/{}_{указатель}.txt",
                        &пути_общие.вывод_книги_пропуски, &книга_взятая.название_книги,&книга_взятая.расширение,
                    );
                }
                //вывод пропущенных строк
                вывод_содержимого_в_txt(
                    &пропущенные_строки,
                    &путь_вывода_пропусков,
                    &mut временные_сообщения.lock().unwrap().общие,
                    false,
                )
                    .unwrap();
                    let сообщение_текущее_вложение=format!("{}: Книга: {}.{} - {} содержимое {} {}",
                                                           style(текущий_шаг_всех_книг).strikethrough(),
                                                           style(&книга_взятая.название_книги).green(),
                                                           style(&книга_взятая.расширение).green(),
                                                           style(шаг_вложенных_книг).strikethrough(),
                                                           style(&вложения.имя).yellow(),
                                                           LOOKING_GLASS
                    );
                    //убрать все переносы
                    //сначала меняются 1)составные (в 1 очередь), 2)вездесущие; 3)сложные слова 4)простые
                    //составные важные
                    замена_слов_через_кучу(
                        &полный_словарь.неизменное,
                        &mut вложения.содержимое,
                        &mut счётчик_неизменное_лок,
                        //"[0/4] Составные важные слова",
                        &format!("{} | [1/5] Неизменные слова",сообщение_текущее_вложение),
                        &книга_взятая.расширение,
                        &куча_пропусков,
                        &куча_словарь.неизменное,
                    );

                //сначала меняются 1)составные (в 1 очередь), 2)вездесущие; 3)сложные слова 4)простые
                //составные важные
                замена_слов_через_кучу(
                    &полный_словарь.составное_важное,
                    &mut вложения.содержимое,
                    &mut счётчик_составное_важное_лок,
                    //"[1/4] Составные важные слова",
                    &format!("{} | [2/5] Составные важные слова",сообщение_текущее_вложение),
                    &книга_взятая.расширение,
                    &куча_пропусков,
                    &куча_словарь.составное_важное,
                );
                //вездесущие
                //замена слов
                замена_слов_через_кучу(
                    &полный_словарь.вездесущее,
                    &mut вложения.содержимое,
                    &mut счётчик_вездесущее_лок,
                    //"[2/4] Вездесущие слова",
                    &format!("{} | [3/5] Вездесущие слова",сообщение_текущее_вложение),
                    &книга_взятая.расширение,
                    &куча_пропусков,
                    &куча_словарь.вездесущее,
                );
                //составные
                //замена слов
                замена_слов_через_кучу(
                    &полный_словарь.составное,
                    &mut вложения.содержимое,
                    &mut счётчик_составное_лок,
                    //"[3/4] Составные  слова",
                    &format!("{} | [4/5] Составные  слова",сообщение_текущее_вложение),
                    &книга_взятая.расширение,
                    &куча_пропусков,
                    &куча_словарь.составное,
                );
                    
                //println!("ВЛожение: {}",вложения.имя);
                //замена слов
                замена_слов_через_кучу(
                    &полный_словарь.простое,
                    &mut вложения.содержимое,
                    &mut счётчик_простое_лок,
                    //"[4/4] Простые слова",
                    &format!("{} | [5/5] Простые слова",сообщение_текущее_вложение),
                    &книга_взятая.расширение,
                    &куча_пропусков,
                    &куча_словарь.простое,
                );


                    pb.finish_and_clear();
            });
            // счётчик_проверочный.fetch_add(1, Ordering::Relaxed);
             //println!("общий заход: {}", счётчик_проверочный.load(Ordering::Relaxed));
             // println!("{}",временный_ряд_книг[0].содержимое[1]);
             let сообщения_проверки_изменений:Vec<String>= проверка_есть_ли_изменения(
                 &вложения_изначальные,
                 &книга_взятая.вложения,
                 &книга_взятая.название_книги,
                 false,//выводить на экран
             );
             //вложение
             временные_сообщения.lock().unwrap().проверка_после_замен[главный_указатель]=lib::Сообщения_для_книги{
                 имя_книги:format!("{}.{}",книга_взятая.название_книги,книга_взятая.расширение),
                 сообщения:сообщения_проверки_изменений,
             };

             pb.finish_and_clear();
    });
    //вывод словаря
    счётчик_составное_важное
        .iter()
        .enumerate()
        .for_each(|(указатель, число)| {
            полный_словарь.счётчик_составное_важное[указатель] = число.load(Ordering::Relaxed)
        });
    счётчик_составное
        .iter()
        .enumerate()
        .for_each(|(указатель, число)| {
            полный_словарь.счётчик_составное[указатель] = число.load(Ordering::Relaxed)
        });
    счётчик_простое
        .iter()
        .enumerate()
        .for_each(|(указатель, число)| {
            полный_словарь.счётчик_простое[указатель] = число.load(Ordering::Relaxed)
        });
    счётчик_вездесущее
        .iter()
        .enumerate()
        .for_each(|(указатель, число)| {
            полный_словарь.счётчик_вездесущее[указатель] = число.load(Ordering::Relaxed)
        });
    счётчик_неизменное
        .iter()
        .enumerate()
        .for_each(|(указатель, число)| {
            полный_словарь.счётчик_неизменное[указатель] = число.load(Ordering::Relaxed)
        });
    write::вывод_всех_словарей_в_xls(&полный_словарь, &куча_словарь).unwrap();
    println!(
        "Время занятое на замену слов: {:.2?}",
        точка_отсчёта_по_времени.elapsed()
    );
    println!();
    *сообщения = Arc::try_unwrap(временные_сообщения)
        .unwrap()
        .into_inner()
        .unwrap();
    return книги;

    fn проверка_есть_ли_изменения(
        содержимое_изначальное: &Vec<lib::Вложения>,
        содержимое_изменённое: &Vec<lib::Вложения>,
        имя_книги: &String,
        условие: bool, //выводить на экран или нет
    ) -> Vec<String> {
        use rayon::prelude::*;

        let сообщения: Vec<String> = содержимое_изначальное
            .par_iter()
            .enumerate()
            .filter(|(указатель, вложение)| {
                не_изображение_или_мусор(
                    &содержимое_изначальное[*указатель].имя,
                )
            })
            .filter_map(|(указатель, вложение)| {
                // шаг_внутренний.fetch_add(1, Ordering::Relaxed);
                //   println!("{}", шаг_внутренний.load(Ordering::Relaxed));
                if сравнение_двух_рядов_построчно(
                    &содержимое_изначальное[указатель].содержимое,
                    &содержимое_изменённое[указатель].содержимое,
                    &вложение.имя,
                ) {
                    let сообщение = format!(
                        "Книга: {}|[{}/{}]| Файл: {}  замены не были произведены",
                        имя_книги,
                        указатель + 1,
                        содержимое_изначальное.len(),
                        содержимое_изначальное[указатель].имя
                    );
                    if условие {
                        println!("{}", сообщение);
                        return Some(сообщение);
                    } else {
                        return Some(сообщение);
                    }
                } else {
                    let сообщение = format!(
                        "Книга: {}|[{}/{}]| Файл: {}  были совершены замены",
                        имя_книги,
                        указатель + 1,
                        содержимое_изначальное.len(),
                        содержимое_изначальное[указатель].имя
                    );
                    if условие {
                        println!("{}", сообщение);
                        return Some(сообщение);
                    } else {
                        return Some(сообщение);
                    }
                }
            })
            .collect();
        //let сообщения:Vec<String>=сообщения.retain(|строка| !строка.is_empty());
        return сообщения;
    }
}
//создание словаря regex
pub fn добавить_все_слова_в_словарь(
    mut ряд_словарей: Vec<Словарь>, //вектор словарей
) -> Полный_Словарь {
    use crate::xlsx::import_xlsx::поиск_уже_добавленных_слов_в_полном_словаре;
    //итоговый словарь
    //let mut полный_словарь: Mutex<Полный_Словарь> = Mutex::new(Default::default());
    //перебор словаря
    let полный_словарь = ряд_словарей
        .into_par_iter()
        .fold_with(
            lib::Полный_Словарь::default(),
            |mut накопитель, ячейка| {
                накопитель.простое.extend(ячейка.простое);
                накопитель.вездесущее.extend(ячейка.вездесущее);
                накопитель.составное.extend(ячейка.составное);
                накопитель.составное_важное.extend(ячейка.составное_важное);
                накопитель.неизменное.extend(ячейка.неизменное);
                накопитель
            },
        )
        .reduce(
            || lib::Полный_Словарь::default(),
            |mut a, b| {
                a.простое.extend(b.простое);
                a.вездесущее.extend(b.вездесущее);
                a.составное.extend(b.составное);
                a.составное_важное.extend(b.составное_важное);
                a.неизменное.extend(b.неизменное);
                a
            },
        );
    //проверка пересечений составных, составных важных и неизменных слов
    поиск_уже_добавленных_слов_в_полном_словаре(
        &полный_словарь,
    ); //номер страницы
    //поиск уже добавленных слов
    return полный_словарь;
}

pub fn создать_быстрый_словарь(
    слова_из_словаря: &Vec<String>,
    вид_слов: &str,
) -> HashMap<String, HashSet<usize>> {
    use crate::utils::stringzilla::sz_упорядочить_кучу;
    let ряд_вывод = Arc::new(Mutex::new(Vec::new()));
    let словарь_куча: HashMap<String, HashSet<usize>> =
        выделить_кучу_из_ряда_для_словаря(&слова_из_словаря);

    let ряд_временный: HashSet<String> = словарь_куча
        .par_iter()
        .filter_map(|(ключ, значения)| {
            let строка = format!("ключ: |{ключ}| Значения ({}):", значения.len());

            let полная_строка = значения
                .par_iter()
                .fold(
                    || String::new(),
                    |mut acc, значение| {
                        if !acc.is_empty() {
                            acc.push(',');
                        }
                        acc.push_str(&слова_из_словаря[*значение].to_string());
                        acc
                    },
                )
                .reduce(
                    || String::new(),
                    |mut a, b| {
                        if !a.is_empty() && !b.is_empty() {
                            a.push(',');
                        }
                        a.push_str(&b);
                        a
                    },
                );

            let итог = format!("{}{}", строка, полная_строка);
            ряд_вывод.lock().unwrap().push(итог);
            Some(ключ.to_string())
        })
        .collect::<HashSet<String>>();
    let ряд_временный = sz_упорядочить_кучу(ряд_временный);
    //
    let пути_общие: lib::Пути_Общие = Default::default();
    let пути_вывода: lib::Пути_Вывода = Default::default();
    let mut пустой_ряд: Vec<String> = Vec::new();
    let путь_простой: String = format!("{}{}.txt", &пути_вывода.вывод_кучи_словаря, вид_слов,);
    let путь_ключи: String = format!("{}{}.txt", &пути_вывода.вывод_кучи_словаря_ключи, вид_слов,);
    let ряд_вывод = Arc::try_unwrap(ряд_вывод).unwrap().into_inner().unwrap();
    вывод_содержимого_в_txt(&ряд_вывод, &путь_простой, &mut пустой_ряд, false).unwrap();
    вывод_содержимого_в_txt(&ряд_временный, &путь_ключи, &mut пустой_ряд, false).unwrap();
    return словарь_куча;
}
/*
pub fn создать_быстрый_словарь2(
    слова_из_словаря: &Vec<String>,
    вид_слов: &str,
) -> HashMap<String, HashSet<usize>> {
    use crate::utils::stringzilla::sz_упорядочить_кучу;
    //let куча_пропусков:HashMap<String,Vec<usize>>=HashMap::with_hasher(foldhash::fast::RandomState::default());
    //let mut куча_простая=куча_пропусков.clone();
    let mut ряд_вывод: Arc<Mutex<Vec<String>> >= Arc::new(Mutex::new(Vec::new()));
    let словарь_куча: HashMap<String, HashSet<usize>> =
        выделить_кучу_из_ряда_для_словаря(&слова_из_словаря);
    let ряд_временный: Mutex<HashSet<String>> =
        Mutex::new(HashSet::with_hasher(foldhash::fast::RandomState::default()));
    //
    словарь_куча.par_iter().for_each(|(ключ, значения)| {
        ряд_временный.lock().unwrap().insert(ключ.to_string());
        let mut строка: Mutex<String> =
            Mutex::new(format!("ключ: |{ключ}| Значения ({}):", значения.len()));
        значения.par_iter().for_each(|значение| {
            строка
                .lock()
                .unwrap()
                .push_str(&format!("{значение},"));
            // строка = format!("{}|{значение}-{}|", строка.lock().unwrap().clone(),значение).into();
        });
        ряд_вывод.lock().unwrap().push(строка.into_inner().unwrap());
    });
    /*for (ключ, значения) in словарь_куча.iter() {
        ряд_временный.insert(ключ.to_string());
        let mut строка = String::new();
        строка = format!("ключ: |{ключ}| Значения ({}):", значения.len());
        for значение in значения.iter() {
            строка = format!("{строка}|{значение}-{}|", слова_из_словаря[*значение]);
        }
        ряд_вывод.push(строка);
    }*/
    let ряд_временный = ряд_временный.into_inner().unwrap();
    let ряд_временный = sz_упорядочить_кучу(ряд_временный);
    //
    let пути_общие: lib::Пути_Общие = Default::default();
    let пути_вывода: lib::Пути_Вывода = Default::default();
    let mut пустой_ряд: Vec<String> = Vec::new();
    let путь_простой: String = format!("{}{}.txt", &пути_вывода.вывод_кучи_словаря, вид_слов,);
    let путь_ключи: String = format!("{}{}.txt", &пути_вывода.вывод_кучи_словаря_ключи, вид_слов,);
    let ряд_вывод = Arc::try_unwrap(ряд_вывод).unwrap().into_inner().unwrap();
    вывод_содержимого_в_txt(&ряд_вывод, &путь_простой, &mut пустой_ряд, false).unwrap();
    вывод_содержимого_в_txt(&ряд_временный, &путь_ключи, &mut пустой_ряд, false).unwrap();
    return словарь_куча;
}

pub fn выделить_кучу_из_ряда_для_словаря3(
    ряд_слов: &Vec<String>,
) -> HashMap<String, HashSet<usize>> {
    let куча:HashMap<String, HashSet<usize>>=ряд_слов.par_iter()
        .enumerate()
        .fold(
            || HashMap::default(),
            |mut acc, (i, строка)| {
                let слово = выделить_окончание_из_слова(строка);
                acc.entry(слово)
                    .or_insert_with(HashSet::new)
                    .insert(i);
                acc
            }
        )
        .reduce(
            || HashMap::default(),
            |mut acc1, acc2| {
                for (ключ, значения) in acc2 {
                    acc1.entry(ключ)
                        .or_insert_with(HashSet::new)
                        .extend(значения);
                }
                acc1
            }
        );
    return куча
}

pub fn выделить_кучу_из_ряда_для_словаря1(
    ряд_слов: &Vec<String>,
) -> HashMap<String, HashSet<usize>> {
    let mut куча_пропусков: HashMap<String, HashSet<usize>> =
        HashMap::with_hasher(foldhash::fast::RandomState::default());
    //перебор ряда слов
    for i in 0..ряд_слов.len() {
        let слово: String = выделить_окончание_из_слова(&ряд_слов[i]);
        //создание пустой кучи
             //проверка есть ли в куче
        if !куча_пропусков.contains_key(&слово) {
            куча_пропусков.insert(слово,   HashSet::from_iter([i]));
        }
        //если содержит куча ключ
        else {
            if let Some(значения) = куча_пропусков.get_mut(&слово) {
                значения.insert(i);
            };
        }
    }
    return куча_пропусков;
}
*/
pub fn выделить_кучу_из_ряда_для_словаря(
    ряд_слов: &[String],
) -> HashMap<String, HashSet<usize>> {
    let mut куча_пропусков: HashMap<String, HashSet<usize>> = HashMap::default();

    for (указатель, строка) in ряд_слов.iter().enumerate() {
        let слово = выделить_окончание_из_слова(строка);

        куча_пропусков
            .entry(слово)
            .or_insert_with(HashSet::new)
            .insert(указатель);
    }

    куча_пропусков
}

pub fn foldhash_пример(слова: &Vec<usize>, значение: usize) {
    let my_set: HashSet<usize> = (0..слова.len())
        .map(|_| if значение == 0 { 1 } else { 2 })
        .collect::<HashSet<usize>>();
    let слова: Vec<String> = Vec::new();
    let пропуски: HashSet<usize> = слова
        .par_iter()
        .enumerate()
        .filter_map(|(указатель, строка)| {
            if !есть_ли_кириллица(&строка) {
                return Some(указатель);
            } else {
                None
            }
        })
        .collect::<HashSet<usize>>();

    use std::hash::BuildHasher;
    //let my_set: HashSet<usize> = 1.into();
    let my_set = HashSet::from_iter([1, 2, 3, 4, 5]);
    let random_state = RandomState::default();
    let hash = random_state.hash_one("hello world");
    let hash: HashSet<usize> = HashSet::from_iter([1, 2])
        .into_iter()
        .collect::<HashSet<usize>>();
    //et my_set:HashSet<usize> = HashSet::from( 1);

    let my_set: HashSet<usize> = [1, 2, 3, 4, 5].into_iter().collect::<HashSet<usize>>();
}

pub fn выделить_окончание_из_слова(слово: &String) -> String {
    let куча_исключений_знак: HashSet<char> =
        HashSet::from_iter(['ы', 'и', 'а', 'я', 'у', 'е', 'ю'])
            .into_iter()
            .collect::<HashSet<char>>();
    //куча_исключений_знак.insert('ь');
    //куча_исключений_знак.insert('ъ');

    lazy_static! {
        static ref re_однобуквенные: [Regex;10] = [
            Regex::new(r"(?i)о$").unwrap(),
            Regex::new(r"(?i)а$").unwrap(),
            Regex::new(r"(?i)я$").unwrap(),
            Regex::new(r"(?i)е$").unwrap(),
            Regex::new(r"(?i)ь$").unwrap(),
            Regex::new(r"(?i)ы$").unwrap(),
            Regex::new(r"(?i)и$").unwrap(),
           Regex::new(r"(?i)ъ$").unwrap(),
            //глаголы
               Regex::new(r"(?i)у$").unwrap(),
             Regex::new(r"(?i)ю$").unwrap(),
                      //Русские флексийные морфы по алфавиту
                   // Regex::new(r"(?i)а$").unwrap(),
            // Regex::new(r"(?i)е$").unwrap(),
              // Regex::new(r"(?i)и$").unwrap(),
            //Regex::new(r"(?i)о$").unwrap(),
            //     Regex::new(r"(?i)у$").unwrap(),

        ];
        static ref re_многобуквенные_с_исключениями_замены: [Regex;11] = [
                          Regex::new(r"(?i)ал$").unwrap(),//0
                                   Regex::new(r"(?i)ала$").unwrap(),//1
           Regex::new(r"(?i)ные$").unwrap(),//2 убрать
              Regex::new(r"(?i)ного$").unwrap(),//3

             Regex::new(r"(?i)ный$").unwrap(),//5
             Regex::new(r"(?i)ных$").unwrap(),//6
                        Regex::new(r"(?i)ких$").unwrap(),//7
            Regex::new(r"(?i)кой$").unwrap(),//8
            Regex::new(r"(?i)ость$").unwrap(),//9
                   Regex::new(r"(?i)ости$").unwrap(),//10
              Regex::new(r"(?i)остью$").unwrap(),//11
        ];
       static ref re_многобуквенные_с_исключениями_образцы: [Regex;11] = [
                        //исключения
            Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})(ал)$").unwrap(),//0
            Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})(ала)$").unwrap(),//1
           Regex::new(r"(?i)нные$").unwrap(),//2 убрать
             Regex::new(r"(?i)нного$").unwrap(),//3

             Regex::new(r"(?i)нный$").unwrap(),//5
                Regex::new(r"(?i)нных$").unwrap(),//6
                        Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})ких$").unwrap(),//7
            Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})кой$").unwrap(),//8
               Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})ость$").unwrap(),//9
                          Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})ости$").unwrap(),//10
                          Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})остью$").unwrap(),//11
       ];
                 static ref re_многобуквенные: [Regex;13] =[
            Regex::new(r"(?i)иумы$").unwrap(),
               Regex::new(r"(?i)ования$").unwrap(),
                Regex::new(r"(?i)овать$").unwrap(),
            //
               Regex::new(r"(?i)иями$").unwrap(),
             Regex::new(r"(?i)ующие$").unwrap(),
             Regex::new(r"(?i)ующая$").unwrap(),
              Regex::new(r"(?i)ующий$").unwrap(),
            Regex::new(r"(?i)ующих$").unwrap(),
             Regex::new(r"(?i)уется$").unwrap(),
             Regex::new(r"(?i)уются$").unwrap(),
   Regex::new(r"(?i)уете$").unwrap(),
             Regex::new(r"(?i)ичную$").unwrap(),
             Regex::new(r"(?i)ичных$").unwrap(),

           ];
        static ref re_трехбуквенные: [Regex;42] =[
             Regex::new(r"(?i)\w+уют$").unwrap(),
            Regex::new(r"(?i)еям$").unwrap(),
    Regex::new(r"(?i)иев$").unwrap(),
            Regex::new(r"(?i)иал$").unwrap(),
              Regex::new(r"(?i)ием$").unwrap(),
           Regex::new(r"(?i)иум$").unwrap(),

            Regex::new(r"(?i)ыми$").unwrap(),
            Regex::new(r"(?i)ика$").unwrap(),
            Regex::new(r"(?i)ику$").unwrap(),
            Regex::new(r"(?i)ики$").unwrap(),
                Regex::new(r"(?i)ать$").unwrap(),
            Regex::new(r"(?i)ять$").unwrap(),
            Regex::new(r"(?i)оть$").unwrap(),
            Regex::new(r"(?i)еть$").unwrap(),
             Regex::new(r"(?i)иям$").unwrap(),
                 Regex::new(r"(?i)уум$").unwrap(),

            Regex::new(r"(?i)уем$").unwrap(),
             Regex::new(r"(?i)ешь$").unwrap(),
               Regex::new(r"(?i)ишь$").unwrap(),
               Regex::new(r"(?i)ете$").unwrap(),
               Regex::new(r"(?i)ите$").unwrap(),
             Regex::new(r"(?i)ует$").unwrap(),
           Regex::new(r"(?i)яла$").unwrap(),
                    Regex::new(r"(?i)али$").unwrap(),
              Regex::new(r"(?i)яли$").unwrap(),
                Regex::new(r"(?i)ола$").unwrap(),
             Regex::new(r"(?i)ела$").unwrap(),
             Regex::new(r"(?i)оли$").unwrap(),
             Regex::new(r"(?i)ели$").unwrap(),
                         Regex::new(r"(?i)\w{2,}ула$").unwrap(),
                 Regex::new(r"(?i)ули$").unwrap(),
                          Regex::new(r"(?i)ами$").unwrap(),
                Regex::new(r"(?i)еми$").unwrap(),
                    Regex::new(r"(?i)емя$").unwrap(),
                 Regex::new(r"(?i)ёте$").unwrap(),
              Regex::new(r"(?i)ёшь$").unwrap(),

                            Regex::new(r"(?i)ого$").unwrap(),
                        Regex::new(r"(?i)ому$").unwrap(),
                Regex::new(r"(?i)иях$").unwrap(),
              Regex::new(r"(?i)ией$").unwrap(),
              Regex::new(r"(?i)умя$").unwrap(),
             Regex::new(r"(?i)ими$").unwrap(),
       ];
            static ref re_двубуквенные: [Regex;53] =[
            //в первую очередь
          // Regex::new(r"(?i)ные$").unwrap(),
            //двукбуквенные
            // гласные ([иаяуюыоеэё]+)
            //остальные
                       Regex::new(r"(?i)ея$").unwrap(),
             Regex::new(r"(?i)еи$").unwrap(),
                   Regex::new(r"(?i)ях$").unwrap(),
            Regex::new(r"(?i)ев$").unwrap(),
             Regex::new(r"(?i)ки$").unwrap(),
                  Regex::new(r"(?i)ым$").unwrap(),
                        Regex::new(r"(?i)ых$").unwrap(),
            Regex::new(r"(?i)ям$").unwrap(),
            Regex::new(r"(?i)ии$").unwrap(),
            Regex::new(r"(?i)ия$").unwrap(),
                Regex::new(r"(?i)ся$").unwrap(),
            Regex::new(r"(?i)ая$").unwrap(),
             Regex::new(r"(?i)яя$").unwrap(),
              Regex::new(r"(?i)ое$").unwrap(),
              Regex::new(r"(?i)ее$").unwrap(),
            Regex::new(r"(?i)ой$").unwrap(),
            Regex::new(r"(?i)ые$").unwrap(),
            Regex::new(r"(?i)ый$").unwrap(),
            Regex::new(r"(?i)ий$").unwrap(),
            //глаголы

               Regex::new(r"(?i)ем$").unwrap(),
               Regex::new(r"(?i)им$").unwrap(),

               Regex::new(r"(?i)ет$").unwrap(),
               Regex::new(r"(?i)ит$").unwrap(),
               Regex::new(r"(?i)ут$").unwrap(),
               Regex::new(r"(?i)ют$").unwrap(),
               Regex::new(r"(?i)ят$").unwrap(),

                 Regex::new(r"(?i)ял$").unwrap(),

             Regex::new(r"(?i)ол$").unwrap(),
             Regex::new(r"(?i)ел$").unwrap(),

            Regex::new(r"(?i)w{2,}ул$").unwrap(),

            //Русские флексийные морфы по алфавиту
                    Regex::new(r"(?i)ам$").unwrap(),

              Regex::new(r"(?i)ас$").unwrap(),
              Regex::new(r"(?i)ах$").unwrap(),
             // Regex::new(r"(?i)ая$").unwrap(),
                 Regex::new(r"(?i)её$").unwrap(),
                 Regex::new(r"(?i)ей$").unwrap(),
               //   Regex::new(r"(?i)ем$").unwrap(),

                    Regex::new(r"(?i)ех$").unwrap(),
                    Regex::new(r"(?i)ею$").unwrap(),
              Regex::new(r"(?i)ёт$").unwrap(),

            Regex::new(r"(?i)ёх$").unwrap(),

                 Regex::new(r"(?i)ие$").unwrap(),
              //Regex::new(r"(?i)ий$").unwrap(),
              // Regex::new(r"(?i)им$").unwrap(),

             //  Regex::new(r"(?i)ите$").unwrap(),
                      //  Regex::new(r"(?i)ит$").unwrap(),
                       Regex::new(r"(?i)их$").unwrap(),
                     //   Regex::new(r"(?i)ишь$").unwrap(),
                       Regex::new(r"(?i)ию$").unwrap(),
           //  Regex::new(r"(?i)м$").unwrap(),
                      Regex::new(r"(?i)ми$").unwrap(),
                         Regex::new(r"(?i)мя$").unwrap(),
                        Regex::new(r"(?i)ов$").unwrap(),

                //  Regex::new(r"(?i)ое$").unwrap(),
            Regex::new(r"(?i)оё$").unwrap(),
          //  Regex::new(r"(?i)ой$").unwrap(),
            Regex::new(r"(?i)ом$").unwrap(),

            Regex::new(r"(?i)см$").unwrap(),
            Regex::new(r"(?i)ум$").unwrap(),
              Regex::new(r"(?i)уя$").unwrap(),

             //  Regex::new(r"(?i)ут$").unwrap(),
                         Regex::new(r"(?i)ух$").unwrap(),
                         Regex::new(r"(?i)ую$").unwrap(),
                         Regex::new(r"(?i)шь$").unwrap(),
        ];
    }
    //проверка на повторы
    проверка_ряда_regex(&*re_трехбуквенные,"Выделения окончаний из слова:трёхбуквенные");
    проверка_ряда_regex(&*re_двубуквенные,"Выделения окончаний из слова:двубуквенные");
    проверка_ряда_regex(&*re_многобуквенные,"Выделения окончаний из слова:многобуквенные");
    проверка_ряда_regex(&*re_однобуквенные,"Выделения окончаний из слова:однобуквенные");
    проверка_ряда_regex(&*re_многобуквенные_с_исключениями_образцы,"Выделения окончаний из слова: многобуквенные_с_исключениями_образцы");
    проверка_ряда_regex(&*re_многобуквенные_с_исключениями_замены,"Выделения окончаний из слова: многобуквенные_с_исключениями_замены");

    //проверка
    //прогон двубуквенного ряда
    match прогон_и_замена_в_слове_через_ряд_re_c_исключениями(
        &слово,
        &*re_многобуквенные_с_исключениями_образцы,
        &*re_многобуквенные_с_исключениями_замены,
    ) {
        Ok(итог) => return итог,
        //перебор в однобуквенном ряде
        Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
            &слово,
            &*re_многобуквенные,
        ) {
            Ok(итог) => return итог,
            //перебор в однобуквенном ряде
            Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
                &слово,
                &*re_трехбуквенные,
            ) {
                Ok(итог) => return итог,
                Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
                    &слово,
                    &*re_двубуквенные,
                ) {
                    Ok(итог) => return итог,
                    Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
                        &слово,
                        &*re_однобуквенные,
                    ) {
                        Ok(итог) => return итог,
                        Err(()) => return слово.to_string(),
                    },
                },
            },
        },
    }
}

pub fn прогон_и_замена_в_слове_через_ряд_re(
    слово: &String,
    re_ряд: impl AsRef<[Regex]>,
) -> Result<String, ()> {
    let re_ряд = re_ряд.as_ref();
    //for re_образец in re_ряд.iter() {
    return re_ряд
        .par_iter()
        .enumerate()
        .find_map_any(|(указатель, re_образец)| {
            if re_образец.is_match(&слово) {
                //regex
                let замененная_строка: std::borrow::Cow<'_, str> = re_образец.replace(
                    &слово, //строка, в которой происходит замена
                    "",     //на что заменить
                );
                Some(замененная_строка.to_string())
            } else {
                None
            }
        })
        .ok_or(());
}

pub fn прогон_и_замена_в_слове_через_ряд_re_c_исключениями(
    слово: &String,
    re_ряд: impl AsRef<[Regex]>,
    исключения: impl AsRef<[Regex]>,
) -> Result<String, ()> {
    let re_ряд = re_ряд.as_ref();
    let исключения = исключения.as_ref();
    re_ряд
        .par_iter()
        .enumerate()
        .find_map_any(|(указатель, re_образец)| {
            if re_образец.is_match(&слово) {
                //regex
                let замененная_строка: std::borrow::Cow<'_, str> = исключения[указатель].replace(
                    &слово, //строка, в которой происходит замена
                    "",     //на что заменить
                );
                Some(замененная_строка.to_string())
            } else {
                None
            }
        })
        .map(Ok)
        .unwrap_or(Err(()))
}
pub fn проверка_ряда_regex(re_ряд: impl AsRef<[Regex]>,сообщение:&str) {
    let ряд = re_ряд.as_ref();
    let куча: HashSet<String> = (0..ряд.len())
        .into_par_iter()
        .flat_map(|i| {
            ((i + 1)..ряд.len()).into_par_iter()
                .filter_map(move |j| {
                if ряд[i].as_str() == ряд[j].as_str() {
                    Some(format!("есть совпадение Regex: {}", ряд[i]))
                } else {
                    None
                }
            })
        })
        .collect();
    for слово in куча.iter() {
        println!("длина кучи: {}",куча.len());
        println!("{} : {}",сообщение, слово)
    }

}

fn получить_кучи_из_словарей(
    полный_словарь: &Полный_Словарь,
) -> lib::Куча_Словарь {
    let простое: HashMap<String, HashSet<usize>> = создать_быстрый_словарь(
        &полный_словарь
            .простое
            .par_iter()
            .map(|ячейка| ячейка.искомое_слово.to_string())
            .collect(),
        "простые",
    );
    let составное: HashMap<String, HashSet<usize>> = создать_быстрый_словарь(
        &полный_словарь
            .составное
            .par_iter()
            .map(|ячейка| ячейка.искомое_слово.to_string())
            .collect(),
        "составные",
    );
    let составное_важное: HashMap<String, HashSet<usize>> =
        создать_быстрый_словарь(
            &полный_словарь
                .составное_важное
                .par_iter()
                .map(|ячейка| ячейка.искомое_слово.to_string())
                .collect(),
            "составные_важные",
        );
    let вездесущее: HashMap<String, HashSet<usize>> = создать_быстрый_словарь(
        &полный_словарь
            .вездесущее
            .par_iter()
            .map(|ячейка| ячейка.искомое_слово.to_string())
            .collect(),
        "вездесущие",
    );
    let неизменное: HashMap<String, HashSet<usize>> = создать_быстрый_словарь(
        &полный_словарь
            .неизменное
            .par_iter()
            .map(|ячейка| ячейка.искомое_слово.to_string())
            .collect(),
        "неизменные",
    );

    return lib::Куча_Словарь {
        простое: простое,
        составное: составное,
        составное_важное: составное_важное,
        вездесущее: вездесущее,
        неизменное: неизменное,
    };
}

pub fn удалить_окончание_из_слова(слово: &String) -> String {
    #[derive(Debug, Clone)]
    pub struct Замена {
        pub однобуквенные: [Ячейка_замены; 10], //одиночные слова
        pub двубуквенные: [Ячейка_замены; 54], //одиночные слова
        pub трехбуквенные: [Ячейка_замены; 48], //одиночные слова
        pub многобуквенные: [Ячейка_замены; 24], //одиночные слова
        pub целиковые: [Ячейка_замены; 100], //одиночные слова
    }

    //словарь
    #[derive(Debug, Clone)]
    pub struct Ячейка_замены {
        pub искомое_слово: String,
        pub re_образец: Regex,
        pub замена: String,
        // pub счёчтки:usize,
    }
    lazy_static! {
        static ref словарь_замен: Замена = Замена {
            однобуквенные: [Ячейка_замены {
                искомое_слово: "-о".to_string(),
                замена: "о".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-о$").unwrap()
            },
             Ячейка_замены {
                искомое_слово: "-а".to_string(),
                замена: "а".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-а$").unwrap()
            },
           Ячейка_замены {
                искомое_слово: "-я".to_string(),
                замена: "я".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-я$").unwrap()
            },
              Ячейка_замены {
                искомое_слово: "-е".to_string(),
                замена: "е".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-е$").unwrap()
            },
              Ячейка_замены {
                искомое_слово: "-ь".to_string(),
                замена: "ь".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ь$").unwrap()
            },
              Ячейка_замены {
                искомое_слово: "-ы".to_string(),
                замена: "ы".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ы$").unwrap()
            },
                 Ячейка_замены {
                искомое_слово: "-и".to_string(),
                замена: "и".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-и$").unwrap()
            },
                 Ячейка_замены {
                искомое_слово: "-ъ".to_string(),
                замена: "ъ".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ъ$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-у".to_string(),
                замена: "у".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-у$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ю".to_string(),
                замена: "ю".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ю$").unwrap()
            },],

             многобуквенные: [Ячейка_замены {
                искомое_слово: "-иумы".to_string(),
                замена: "иумы".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-иумы$").unwrap()
            },
                Ячейка_замены {
                искомое_слово: "-ования".to_string(),
                замена: "ования".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ования$").unwrap()
            },
                Ячейка_замены {
                искомое_слово: "-овать".to_string(),
                замена: "овать".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-овать").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-овать".to_string(),
                замена: "овать".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-овать").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-иями".to_string(),
                замена: "иями".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-иями$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-ующие".to_string(),
                замена: "ующие".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ующие$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-ующая".to_string(),
                замена: "ующая".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ующая$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-ующий".to_string(),
                замена: "ующий".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ующий$").unwrap()
            },
                            Ячейка_замены {
                искомое_слово: "-ующих".to_string(),
                замена: "ующих".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ующих$").unwrap()
            },
                            Ячейка_замены {
                искомое_слово: "-уется".to_string(),
                замена: "уется".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-уется$").unwrap()
            },
                            Ячейка_замены {
                искомое_слово: "-уются".to_string(),
                замена: "уются".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-уются$").unwrap()
            },
                            Ячейка_замены {
                искомое_слово: "-ичную".to_string(),
                замена: "ичную".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ичную$").unwrap()
            },            Ячейка_замены {
                искомое_слово: "-ичных".to_string(),
                замена: "ичных".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ичных$").unwrap()
            },  Ячейка_замены {
                     искомое_слово: "-ного".to_string(),
                замена: "ного".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ного$").unwrap()
            },  Ячейка_замены {
                      искомое_слово: "-ость".to_string(),
                замена: "ость".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ость$").unwrap()
            },
                 Ячейка_замены {
                      искомое_слово: "-ости".to_string(),
                замена: "ости".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ости$").unwrap()
            },
                     Ячейка_замены {
                      искомое_слово: "-остью".to_string(),
                замена: "остью".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-остью$").unwrap()
            },
                    Ячейка_замены {
                      искомое_слово: "-нные".to_string(),
                замена: "нные".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нные$").unwrap()
            },
                  Ячейка_замены {
                      искомое_слово: "-нные".to_string(),
                замена: "нные".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нные$").unwrap()
            },
                  Ячейка_замены {
                      искомое_слово: "-нного".to_string(),
                замена: "нного".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нного$").unwrap()
            },
                 Ячейка_замены {
                      искомое_слово: "-нные".to_string(),
                замена: "нные".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нные$").unwrap()
            },
                Ячейка_замены {
                      искомое_слово: "-нный".to_string(),
                замена: "нный".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нный$").unwrap()
            },
                Ячейка_замены {
                      искомое_слово: "-нных".to_string(),
                замена: "нных".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нных$").unwrap()
            },
                    Ячейка_замены {
                      искомое_слово: "-уете".to_string(),
                замена: "уете".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-уете$").unwrap()
            },

            ],
               трехбуквенные: [
                Ячейка_замены {
                искомое_слово: "-ный".to_string(),
                замена: "ный".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ный$").unwrap()
            },
                 Ячейка_замены {
                искомое_слово: "-ных".to_string(),
                замена: "ных".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ных$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-ких".to_string(),
                замена: "ких".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ких$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-ных".to_string(),
                замена: "ных".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ных$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-кой".to_string(),
                замена: "кой".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-кой$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-ала".to_string(),
                замена: "ала".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ала$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-уют".to_string(),
                замена: "уют".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-уют$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-еям".to_string(),
                замена: "еям".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-еям$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-иев".to_string(),
                замена: "иев".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-иев$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-иал".to_string(),
                замена: "иал".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-иал$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-ием".to_string(),
                замена: "ием".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ием$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-иум".to_string(),
                замена: "иум".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-иум$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-ыми".to_string(),
                замена: "ыми".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ыми$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-ика".to_string(),
                замена: "ика".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ика$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-ику".to_string(),
                замена: "ику".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ику$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-ики".to_string(),
                замена: "ики".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ики$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-ать".to_string(),
                замена: "ать".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ать$").unwrap()
            },  Ячейка_замены {
                искомое_слово: "-ять".to_string(),
                замена: "ять".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ять$").unwrap()
            },
                  Ячейка_замены {
                искомое_слово: "-еть".to_string(),
                замена: "еть".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-еть$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-иям".to_string(),
                замена: "иям".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-иям$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-уум".to_string(),
                замена: "уум".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-уум$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-уем".to_string(),
                замена: "уем".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-уем$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ешь".to_string(),
                замена: "ешь".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ешь$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ишь".to_string(),
                замена: "ишь".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ишь$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ете".to_string(),
                замена: "ете".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ете$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ите".to_string(),
                замена: "ите".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ите$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ует".to_string(),
                замена: "ует".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ует$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-яла".to_string(),
                замена: "яла".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-яла$").unwrap()
            },

                    Ячейка_замены {
                искомое_слово: "-али".to_string(),
                замена: "али".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-али$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-яли".to_string(),
                замена: "яли".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-яли$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ола".to_string(),
                замена: "ола".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ола$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ела".to_string(),
                замена: "ела".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ела$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-оли".to_string(),
                замена: "оли".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-оли$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ели".to_string(),
                замена: "ели".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ели$").unwrap()
            },

                    Ячейка_замены {
                искомое_слово: "-ула".to_string(),
                замена: "ула".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ула$").unwrap()
            },
                    Ячейка_замены {
                искомое_слово: "-ули".to_string(),
                замена: "ули".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ули$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-ами".to_string(),
                замена: "ами".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ами$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-еми".to_string(),
                замена: "еми".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-еми$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-ули".to_string(),
                замена: "ули".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ули$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-емя".to_string(),
                замена: "емя".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-емя$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-ёте".to_string(),
                замена: "ёте".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ёте$").unwrap()
            },

                       Ячейка_замены {
                искомое_слово: "-ёшь".to_string(),
                замена: "ёшь".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ёшь$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-ого".to_string(),
                замена: "ого".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ого$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-ому".to_string(),
                замена: "ому".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ому$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-иях".to_string(),
                замена: "иях".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-иях$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-ией".to_string(),
                замена: "ией".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ией$").unwrap()
            },
                       Ячейка_замены {
                искомое_слово: "-умя".to_string(),
                замена: "умя".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-умя$").unwrap()
            },

                       Ячейка_замены {
                искомое_слово: "-ими".to_string(),
                замена: "ими".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ими$").unwrap()
            },
            ],
            двубуквенные: [
                Ячейка_замены {
                искомое_слово: "-ея".to_string(),
                замена: "ея".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ея$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-еи".to_string(),
                замена: "еи".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-еи$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ях".to_string(),
                замена: "ях".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ях$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ев".to_string(),
                замена: "ев".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ев$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ки".to_string(),
                замена: "ки".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ки$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ым".to_string(),
                замена: "ым".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ым$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ых".to_string(),
                замена: "ых".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ых$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ям".to_string(),
                замена: "ям".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ям$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ии".to_string(),
                замена: "ии".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ии$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ия".to_string(),
                замена: "ия".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ия$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ся".to_string(),
                замена: "ся".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ся$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ая".to_string(),
                замена: "ая".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ая$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-яя".to_string(),
                замена: "яя".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-яя$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ое".to_string(),
                замена: "ое".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ое$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ее".to_string(),
                замена: "ее".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ее$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ой".to_string(),
                замена: "ой".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ой$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ые".to_string(),
                замена: "ые".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ые$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ый".to_string(),
                замена: "ый".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ый$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ий".to_string(),
                замена: "ий".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ий$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ем".to_string(),
                замена: "ем".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ем$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-им".to_string(),
                замена: "им".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-им$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ет".to_string(),
                замена: "ет".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ет$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ит".to_string(),
                замена: "ит".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ит$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ут".to_string(),
                замена: "ут".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ут$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ют".to_string(),
                замена: "ют".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ют$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ят".to_string(),
                замена: "ят".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ят$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ял".to_string(),
                замена: "ял".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ял$").unwrap()
            },
                   Ячейка_замены {
                искомое_слово: "-ол".to_string(),
                замена: "ол".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ол$").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ел".to_string(),
                замена: "ел".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ел$").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ол".to_string(),
                замена: "ол".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ол$").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ул".to_string(),
                замена: "ул".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ул").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ам".to_string(),
                замена: "ам".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ам$").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ас".to_string(),
                замена: "ас".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ас$").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ах".to_string(),
                замена: "ах".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ах$").unwrap()
            },

                      Ячейка_замены {
                искомое_слово: "-её".to_string(),
                замена: "её".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-её$").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ей".to_string(),
                замена: "ей".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ей$").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ех".to_string(),
                замена: "ех".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ех$").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ею".to_string(),
                замена: "ею".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ею$").unwrap()
            },
                      Ячейка_замены {
                искомое_слово: "-ёт".to_string(),
                замена: "ёт".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ёт$").unwrap()
            },
                         Ячейка_замены {
                искомое_слово: "-ёх".to_string(),
                замена: "ёх".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ёх$").unwrap()
            },
                         Ячейка_замены {
                искомое_слово: "-ие".to_string(),
                замена: "ие".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ие$").unwrap()
            },
                         Ячейка_замены {
                искомое_слово: "-их".to_string(),
                замена: "их".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-их$").unwrap()
            },
                         Ячейка_замены {
                искомое_слово: "-ию".to_string(),
                замена: "ию".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ию$").unwrap()
            },

                         Ячейка_замены {
                искомое_слово: "-ми".to_string(),
                замена: "ми".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ми$").unwrap()
            },
                         Ячейка_замены {
                искомое_слово: "-мя".to_string(),
                замена: "мя".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-мя$").unwrap()
            },
                         Ячейка_замены {
                искомое_слово: "-ов".to_string(),
                замена: "ов".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ов$").unwrap()
            },
                         Ячейка_замены {
                искомое_слово: "-оё".to_string(),
                замена: "оё".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-оё$").unwrap()
            },
                         Ячейка_замены {
                искомое_слово: "-ом".to_string(),
                замена: "ом".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ом$").unwrap()
            },
                              Ячейка_замены {
                искомое_слово: "-см".to_string(),
                замена: "см".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-см$").unwrap()
            },
                              Ячейка_замены {
                искомое_слово: "-ум".to_string(),
                замена: "ум".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ум$").unwrap()
            },
                              Ячейка_замены {
                искомое_слово: "-уя".to_string(),
                замена: "уя".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-уям$").unwrap()
            },
                              Ячейка_замены {
                искомое_слово: "-ух".to_string(),
                замена: "ух".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ух$").unwrap()
            },
                              Ячейка_замены {
                искомое_слово: "-ую".to_string(),
                замена: "ую".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ую$").unwrap()
            },                 Ячейка_замены {
                искомое_слово: "-шь".to_string(),
                замена: "шь".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-шь$").unwrap()
            },

            ]
             целиковые: [
                Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-ния".to_string(),
                замена: "ния".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ния$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-рых".to_string(),
                замена: "рых".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-рых$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-метров".to_string(),
                замена: "метров".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-метров$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-ства".to_string(),
                замена: "ства".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ства$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-ровой".to_string(),
                замена: "ровой".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ровой$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-но".to_string(),
                замена: "но".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-но$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-мые".to_string(),
                замена: "мые".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-мые$").unwrap()
            },

                     Ячейка_замены {
                искомое_слово: "-межуточных".to_string(),
                замена: "межуточных".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-межуточных$").unwrap()
            },
                     Ячейка_замены {
                искомое_слово: "-гласование".to_string(),
                замена: "гласование".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-гласование$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-обходимое".to_string(),
                замена: "обходимое".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-обходимое$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ления".to_string(),
                замена: "ления".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ления$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-новления".to_string(),
                замена: "новления".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-новления$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-щем".to_string(),
                замена: "щем".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-щем$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ских".to_string(),
                замена: "ских".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ских$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-данса".to_string(),
                замена: "данса".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-данса$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-фектов".to_string(),
                замена: "фектов".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-фектов$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-редач".to_string(),
                замена: "редач".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-редач$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-нитные".to_string(),
                замена: "нитные".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нитные$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ключается".to_string(),
                замена: "ключается".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ключается$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ной".to_string(),
                замена: "ной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ной$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ментов".to_string(),
                замена: "ментов".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ментов$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-граммный".to_string(),
                замена: "граммный".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-граммный$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-вания".to_string(),
                замена: "вания".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-вания$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ний".to_string(),
                замена: "ний".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ний$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-шений".to_string(),
                замена: "шений".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-шений$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-никло".to_string(),
                замена: "никло".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-никло$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-зок".to_string(),
                замена: "зок".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-зок$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-чиком".to_string(),
                замена: "чиком".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-чиком$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-чатных".to_string(),
                замена: "чатных".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-чатных$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ные".to_string(),
                замена: "ные".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ные$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ства".to_string(),
                замена: "ства".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ства$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-нию".to_string(),
                замена: "нию".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нию").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-полняются".to_string(),
                замена: "полняются".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-полняются$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-го".to_string(),
                замена: "го".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-го$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-нелей".to_string(),
                замена: "нелей".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нелей$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-торые".to_string(),
                замена: "торые".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-торые$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-тально".to_string(),
                замена: "тально".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тально$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-менно".to_string(),
                замена: "менно".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-менно$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-торая".to_string(),
                замена: "торая".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-торая$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-раммного".to_string(),
                замена: "раммного".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-раммного$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-гда".to_string(),
                замена: "гда".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-гда$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-бой".to_string(),
                замена: "бой".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-бой$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-мендуется".to_string(),
                замена: "мендуется".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-мендуется$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-на".to_string(),
                замена: "на".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-на$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-крытый".to_string(),
                замена: "крытый".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-крытый$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-тивным".to_string(),
                замена: "тивным".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тивным$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-дов".to_string(),
                замена: "дов".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-дов$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-численных".to_string(),
                замена: "численных".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-численных$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-мы".to_string(),
                замена: "мы".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-мы$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ленную".to_string(),
                замена: "ленную".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ленную$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-му".to_string(),
                замена: "му".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-му$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-тов".to_string(),
                замена: "тов".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тов$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ры".to_string(),
                замена: "ры".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ры$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-стемный".to_string(),
                замена: "стемный".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-стемный$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-щие".to_string(),
                замена: "щие".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-щие$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-вой".to_string(),
                замена: "вой".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-вой$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ческих".to_string(),
                замена: "ческих".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ческих$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-тура".to_string(),
                замена: "тура".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тура$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ждений".to_string(),
                замена: "ждений".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ждений$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-шемся".to_string(),
                замена: "шемся".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-шемся$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-мента".to_string(),
                замена: "мента".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-мента$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-мандой".to_string(),
                замена: "мандой".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-мандой$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-тинные".to_string(),
                замена: "тинные".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тинные$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-нель".to_string(),
                замена: "нель".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нель$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ром".to_string(),
                замена: "ром".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ром$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-сутствует".to_string(),
                замена: "сутствует".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-сутствует$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-симо".to_string(),
                замена: "симо".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-симо$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-пени".to_string(),
                замена: "пени".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-пени$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-тельно".to_string(),
                замена: "тельно".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тельно$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-мер".to_string(),
                замена: "мер".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-мер$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-чанию".to_string(),
                замена: "чанию".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-чанию$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ческая".to_string(),
                замена: "ческая".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ческая$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-бирать".to_string(),
                замена: "бирать".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-бирать$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-единитель".to_string(),
                замена: "единитель".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-единитель$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ному".to_string(),
                замена: "ному".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ному$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-зуемся".to_string(),
                замена: "зуемся".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-зуемся$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ветствующие".to_string(),
                замена: "ветствующие".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ветствующие$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-матическая".to_string(),
                замена: "матическая".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-матическая$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-нентов".to_string(),
                замена: "нентов".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нентов$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-них".to_string(),
                замена: "них".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-них$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-кие".to_string(),
                замена: "кие".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-кие$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ра".to_string(),
                замена: "ра".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ра$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-чет".to_string(),
                замена: "чет".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-чет$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ект".to_string(),
                замена: "ект".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ект$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-нала".to_string(),
                замена: "нала".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нала$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-жет".to_string(),
                замена: "жет".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-жет$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-ную".to_string(),
                замена: "ную".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ную$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-тистические".to_string(),
                замена: "тистические".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тистические$").unwrap()
            },     Ячейка_замены {
                искомое_слово: "-стимо".to_string(),
                замена: "стимо".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-стимо$").unwrap()
            },
                Ячейка_замены {
                искомое_слово: "-жителем".to_string(),
                замена: "жителем".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-жителем$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-ком".to_string(),
                замена: "ком".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ком$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-товых".to_string(),
                замена: "товых".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-товых$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-цессе".to_string(),
                замена: "цессе".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-цессе$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тым".to_string(),
                замена: "тым".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тым$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-екта".to_string(),
                замена: "екта".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-екта$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-ких".to_string(),
                замена: "ких".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ких$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-новлены".to_string(),
                замена: "новлены".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-новлены$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-рования".to_string(),
                замена: "рования".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-рования$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-вым".to_string(),
                замена: "вым".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-вым$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-зом".to_string(),
                замена: "зом".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-зом$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-рой".to_string(),
                замена: "рой".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-рой$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-раметры".to_string(),
                замена: "раметры".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-раметры$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-чески".to_string(),
                замена: "чески".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-чески$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-ты".to_string(),
                замена: "ты".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ты$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-брав".to_string(),
                замена: "брав".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-брав$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-реноса".to_string(),
                замена: "реноса".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-реноса$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-зультаты".to_string(),
                замена: "зультаты".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-зультаты$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-ходных".to_string(),
                замена: "ходных".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ходных$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-жа".to_string(),
                замена: "жа".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-жа$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тырех".to_string(),
                замена: "тырех".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тырех$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-кать".to_string(),
                замена: "кать".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-кать$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-мент".to_string(),
                замена: "мент".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-мент$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-штаба".to_string(),
                замена: "штаба".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-штаба$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-местно".to_string(),
                замена: "местно".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-местно$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-ления".to_string(),
                замена: "ления".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ления$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тактные".to_string(),
                замена: "тактные".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тактные$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-таллизации".to_string(),
                замена: "таллизации".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-таллизации$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-та".to_string(),
                замена: "та".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-та$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-чек".to_string(),
                замена: "чек".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-чек$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-рые".to_string(),
                замена: "рые".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-рыей$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-нить".to_string(),
                замена: "нить".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-нить$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-ветствующим".to_string(),
                замена: "ветствующим".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ветствующим$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-рый".to_string(),
                замена: "рый".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-рый$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-единения".to_string(),
                замена: "единения".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-единения$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-вать".to_string(),
                замена: "вать".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-вать$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тически".to_string(),
                замена: "тически".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тически$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-ное".to_string(),
                замена: "ное".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ное$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-дами".to_string(),
                замена: "дами".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-дами$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-борочно".to_string(),
                замена: "борочно".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-борочно$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-веден".to_string(),
                замена: "веден".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-веден$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-ражает".to_string(),
                замена: "ражает".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ражает$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-ством".to_string(),
                замена: "ством".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-ством$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тора".to_string(),
                замена: "тора".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тора$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-кусом".to_string(),
                замена: "кусом".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-кусом$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },Ячейка_замены {
                искомое_слово: "-тронной".to_string(),
                замена: "тронной".to_string(),
                re_образец:Regex::new(r"(?i)(?:\w)-тронной$").unwrap()
            },
                ]
        };
    }

    //
    let трёхбуквенные: Vec<Regex> = словарь_замен.трехбуквенные
        .par_iter().map(|ячейка| ячейка.re_образец.clone()).collect();
    let двубуквенные: Vec<Regex> = словарь_замен.двубуквенные
        .par_iter().map(|ячейка| ячейка.re_образец.clone()).collect();
    let многоуквенные: Vec<Regex> = словарь_замен.многобуквенные
        .par_iter().map(|ячейка| ячейка.re_образец.clone()).collect();
    let однобуквенные: Vec<Regex> = словарь_замен.однобуквенные
        .par_iter().map(|ячейка| ячейка.re_образец.clone()).collect();
    //проверка образцов
    проверка_ряда_regex(&трёхбуквенные,"удаление окончания из слова: трёхбуквенные");
    проверка_ряда_regex(&*двубуквенные,"удаление окончания из слова: двубуквенные");
    проверка_ряда_regex(&*многоуквенные,"удаление окончания из слова: многобуквенные");
    проверка_ряда_regex(&*однобуквенные,"удаление окончания из слова: однобуквенные");
    /*
    //проверка
    //прогон двубуквенного ряда
    match прогон_и_замена_в_слове_через_ряд_re_c_исключениями(
        &слово,
        &*re_многобуквенные_с_исключениями_образцы,
        &*re_многобуквенные_с_исключениями_замены,
    ) {
        Ok(итог) => return итог,
        //перебор в однобуквенном ряде
        Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
            &слово,
            &*re_многобуквенные,
        ) {
            Ok(итог) => return итог,
            //перебор в однобуквенном ряде
            Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
                &слово,
                &*re_трехбуквенные,
            ) {
                Ok(итог) => return итог,
                Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
                    &слово,
                    &*re_двубуквенные,
                ) {
                    Ok(итог) => return итог,
                    Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
                        &слово,
                        &*re_однобуквенные,
                    ) {
                        Ok(итог) => return итог,
                        Err(()) => return слово.to_string(),
                    },
                },
            },
        },
    }

     */
}
