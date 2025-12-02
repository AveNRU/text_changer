//use std::default;
use crate::lib::{
    self, Полный_Словарь, Словарь, Сообщения_для_книги, Счётчики_Словаря
};
use lazy_static::lazy_static;
use std::thread;

use crate::output::write;
use crate::output::write::вывод_содержимого_строки_в_txt;
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
    use lib::{
        Словарь_Переносов, Счётчик_замен, Ячейка_замены
    };
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
    let создать_составной_ряд_замен: Словарь_Переносов = создать_словарь_замен();
    //let создать_второй_составной_ряд_замен=создать_составной_ряд_замен.1.clone();
    // Создаем атомарные счетчики для каждого шаблона
    // Создаем атомарные счетчики для каждого шаблона

    let словарь_переносов: [Словарь_Переносов; 3] = [
        создать_составной_ряд_замен.clone(),
        создать_второй_словарь_переносов(
            создать_составной_ряд_замен.clone(),
        ),
        создать_третий_словарь_переносов(создать_составной_ряд_замен),
    ];

    let mut счётчики_замен: [Arc<Счётчик_замен>; 3] = [
        создать_счётчик_словаря_замен(&словарь_переносов[0]),
        создать_счётчик_словаря_замен(&словарь_переносов[1]),
        создать_счётчик_словаря_замен(&словарь_переносов[2]),
    ];

    let лупа = format!("🔍");
    //создание счётчиков
    let mut счётчики_словаря: Arc<lib::Счётчики_Словаря> =
        создать_счётчики_словаря(&полный_словарь);
    // Обернем счетчики в Arc для безопасного разделения между потоками
    //перебор
    let количество_книг = книги.len();
    книги.par_iter_mut().enumerate()
         .filter(|(главный_указатель, книга_взятая)|
                 !sz_найти(&книга_взятая.расширение,"doc") //если это doc, то ничего не делать
                     ||книга_взятая.книга_ли==false
         )
         .for_each(|(главный_указатель, книга_взятая)| {
        if книга_взятая.книга_ли==false {return}

             //счётчик
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
                 лупа
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
                   // println!("Указатель: {указатель}, Вложение: {:?}",вложения.имя);
                    //счётчики замен переносов
                    let mut счётчики_замен_вложенные: [Arc<Счётчик_замен>;3] =
                        счётчики_замен.clone();
                  
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
                вывод_содержимого_строки_в_txt(
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
                                                           лупа
                    );
                    //убрать все переносы
                    let mut временный_указатель=главный_указатель+указатель;
                    //убираем переносы обычное тире - первое исполнение
                    for указатель_переносов in (0..словарь_переносов.len()).rev() {

                        //println!("заход: {указатель_переносов}: {указатель_переносов}",);
                        убрать_переносы(
                            &словарь_переносов[указатель_переносов],
                            //  &полный_словарь.неизменное,
                            &mut вложения.содержимое,
                            // &mut счётчик_неизменное_лок,
                            //"[0/4] Составные важные слова",
                            &format!("{} | Убрать переносы",сообщение_текущее_вложение),
                            &книга_взятая.расширение,
                            //&куча_пропусков,
                            //&куча_словарь.неизменное,
                            &mut временный_указатель,
                            &mut счётчики_замен_вложенные[указатель_переносов],
                            // указатель_словаря_переносов:
                           // указатель_переносов,
                            указатель_переносов
                        );
                       
                    }
                  
                    //сначала меняются 1)составные (в 1 очередь), 2)вездесущие; 3)сложные слова 4)простые
                    //неизменное
                         замена_слов_через_кучу(
                        &полный_словарь.неизменное,
                        &mut вложения.содержимое,
                        &счётчики_словаря.неизменное,
                       // &mut счётчик_неизменное_лок,
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
                    &счётчики_словаря.составное_важное,
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
                    &счётчики_словаря.вездесущее,
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
                    &счётчики_словаря.составное,
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
                    &счётчики_словаря.простое,
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
    /*
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

     */
    write::вывод_всех_словарей_в_xls(
        &полный_словарь,
        &куча_словарь,
        &счётчики_словаря,
    )
    .unwrap();
    for указатель_переносов in 0..словарь_переносов.len() {
        write::вывод_всех_счётчиков_замен_в_xls(
            &счётчики_замен[указатель_переносов],
            &словарь_переносов[указатель_переносов],
            указатель_переносов,
        )
        .unwrap();
    }
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
    mut счётчик_входа: &mut AtomicUsize,
) -> HashMap<String, HashSet<usize>> {
    use crate::lib::Куча_Слова_Замены;

    //let ряд_вывод2: Arc<Mutex<Vec<Куча_Слова_Замены>>> = Arc::new(Mutex::new(Vec::new()));
    use crate::utils::stringzilla::{
        sz_упорядочить_кучу, sz_упорядочить_кучу_словарь_замены
    };
    //let ряд_вывод: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let словарь_куча: HashMap<String, HashSet<usize>> =
        выделить_кучу_из_ряда_для_словаря(
            &слова_из_словаря,
            &mut счётчик_входа,
        );

    let ряд_временный: Vec<Куча_Слова_Замены> = словарь_куча
        .par_iter()
        .filter_map(|(ключ, значения)| {
            let строка = format!("|{ключ}| Значения ({}):", значения.len());

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
            // ряд_вывод.lock().unwrap().push(итог);
            /* ряд_вывод2.lock().unwrap().push(
            Куча_Слова_Замены {
                слово:ключ.to_string(),
                вложения:итог,
            });*/
            //Some(ключ.итог())
            Some(Куча_Слова_Замены {
                слово: ключ.to_string(),
                вложения: итог,
            })
        })
        .collect::<Vec<Куча_Слова_Замены>>();
    // let ряд_временный = sz_упорядочить_кучу(ряд_временный);
    let ряд_на_вывод: Vec<Куча_Слова_Замены> =
        sz_упорядочить_кучу_словарь_замены(ряд_временный);
    //
    let пути_общие: lib::Пути_Общие = Default::default();
    let пути_вывода: lib::Пути_Вывода = Default::default();
    let mut пустой_ряд: Vec<String> = Vec::new();
    let путь_простой: String = format!("{}{}.txt", &пути_вывода.вывод_кучи_словаря, вид_слов,);
    let путь_ключи: String = format!("{}{}.txt", &пути_вывода.вывод_кучи_словаря_ключи, вид_слов,);
    let ряд_на_вывод: Vec<String> = ряд_на_вывод
        .iter()
        .map(|строка| строка.вложения.to_string())
        .collect();
    вывод_содержимого_строки_в_txt(
        &ряд_на_вывод,
        &путь_простой,
        &mut пустой_ряд,
        false,
    )
    .unwrap();
    /* вывод_содержимого_строки_в_txt(
        &ряд_временный,
        &путь_ключи,
        &mut пустой_ряд,
        false,
    )
    .unwrap();*/
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
    mut счётчик_входа: &mut AtomicUsize,
) -> HashMap<String, HashSet<usize>> {
    let mut куча_пропусков: HashMap<String, HashSet<usize>> = HashMap::default();

    for (указатель, строка) in ряд_слов.iter().enumerate() {
        let слово = выделить_окончание_из_слова(строка, &mut счётчик_входа);

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

pub fn выделить_окончание_из_слова(
    слово: &String,
    mut счётчик_входа: &mut AtomicUsize,
) -> String {
    /*let куча_исключений_знак: HashSet<char> =
       HashSet::from_iter(['ы', 'и', 'а', 'я', 'у', 'е', 'ю'])
           .into_iter()
           .collect::<HashSet<char>>();
    */
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
                     static ref re_многобуквенные: [Regex;22] =[
                Regex::new(r"(?i)иумы$").unwrap(),
                   Regex::new(r"(?i)ования$").unwrap(),
                    Regex::new(r"(?i)овать$").unwrap(),
                Regex::new(r"(?i)ность$").unwrap(),
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
                 Regex::new(r"(?i)аные$").unwrap(),
               Regex::new(r"(?i)аться$").unwrap(),

                     Regex::new(r"(?i)ному$").unwrap(),
               Regex::new(r"(?i)овал$").unwrap(),
                 Regex::new(r"(?i)овала$").unwrap(),
                 Regex::new(r"(?i)овали$").unwrap(),
                 Regex::new(r"(?i)овало$").unwrap(),
                Regex::new(r"(?i)ными$").unwrap(),



               ];
            static ref re_трехбуквенные: [Regex;58] =[
                Regex::new(r"(?i)аны$").unwrap(),//5
               Regex::new(r"(?i)ано$").unwrap(),//5
               Regex::new(r"(?i)ная$").unwrap(),//5
                   Regex::new(r"(?i)ную$").unwrap(),//5
                 Regex::new(r"(?i)ных$").unwrap(),//5
                 Regex::new(r"(?i)ное$").unwrap(),//5
                Regex::new(r"(?i)ный$").unwrap(),//5
                Regex::new(r"(?i)ные$").unwrap(),//2 убрать
                 // Regex::new(r"(?i)нию$").unwrap(),//2 убрать
                 Regex::new(r"(?i)уют$").unwrap(),//\w
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
    //Regex::new(r"(?i)кой$").unwrap(),
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
                             Regex::new(r"(?i)ула$").unwrap(),//\w{2,}
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
                Regex::new(r"(?i)ной$").unwrap(),
                Regex::new(r"(?i)них$").unwrap(),
               Regex::new(r"(?i)ным$").unwrap(),
               Regex::new(r"(?i)ало$").unwrap(),
                Regex::new(r"(?i)ась$").unwrap(),
                Regex::new(r"(?i)ись$").unwrap(),
               Regex::new(r"(?i)ось$").unwrap(),
                   Regex::new(r"(?i)ном$").unwrap(),
           ];
                static ref re_двубуквенные: [Regex;56] =[
                //в первую очередь
                //двукбуквенные
                // гласные ([иаяуюыоеэё]+)
                //остальные
               Regex::new(r"(?i)на$").unwrap(),
                   Regex::new(r"(?i)го$").unwrap(),
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
                   Regex::new(r"(?i)ен$").unwrap(),
                   Regex::new(r"(?i)ут$").unwrap(),
                   Regex::new(r"(?i)ют$").unwrap(),
                   Regex::new(r"(?i)ят$").unwrap(),
               Regex::new(r"(?i)но$").unwrap(),

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
    if счётчик_входа.load(Ordering::Relaxed) == 0 {
        //println!("Заход проверки lazy static");
        //проверка на повторы
        проверка_ряда_regex(
            &*re_трехбуквенные,
            "Выделения окончаний из слова:трёхбуквенные",
        );
        проверка_ряда_regex(
            &*re_двубуквенные,
            "Выделения окончаний из слова:двубуквенные",
        );
        проверка_ряда_regex(
            &*re_многобуквенные,
            "Выделения окончаний из слова:многобуквенные",
        );
        проверка_ряда_regex(
            &*re_однобуквенные,
            "Выделения окончаний из слова:однобуквенные",
        );
        проверка_ряда_regex(
            &*re_многобуквенные_с_исключениями_образцы,
            "Выделения окончаний из слова: многобуквенные_с_исключениями_образцы",
        );
        проверка_ряда_regex(
            &*re_многобуквенные_с_исключениями_замены,
            "Выделения окончаний из слова: многобуквенные_с_исключениями_замены",
        );
        счётчик_входа.fetch_add(1, Ordering::Relaxed);
    }
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
pub fn проверка_ряда_regex(re_ряд: impl AsRef<[Regex]>, сообщение: &str) {
    let ряд = re_ряд.as_ref();
    let куча: HashSet<String> = (0..ряд.len())
        .into_par_iter()
        .flat_map(|i| {
            let mut куча_2: std::collections::HashSet<String, RandomState> = HashSet::default();

            // Проверка на отсутствие $
            // if !ряд[i].as_str().contains('$') {
            if !sz_найти(&ряд[i].to_string(), "$") {
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

fn получить_кучи_из_словарей(
    полный_словарь: &Полный_Словарь,
) -> lib::Куча_Словарь {
    let mut счётчик_входа: AtomicUsize = AtomicUsize::new(0);
    let простое: HashMap<String, HashSet<usize>> = создать_быстрый_словарь(
        &полный_словарь
            .простое
            .par_iter()
            .map(|ячейка| ячейка.искомое_слово.to_string())
            .collect(),
        "простые",
        &mut счётчик_входа,
    );
    let составное: HashMap<String, HashSet<usize>> = создать_быстрый_словарь(
        &полный_словарь
            .составное
            .par_iter()
            .map(|ячейка| ячейка.искомое_слово.to_string())
            .collect(),
        "составные",
        &mut счётчик_входа,
    );
    let составное_важное: HashMap<String, HashSet<usize>> =
        создать_быстрый_словарь(
            &полный_словарь
                .составное_важное
                .par_iter()
                .map(|ячейка| ячейка.искомое_слово.to_string())
                .collect(),
            "составные_важные",
            &mut счётчик_входа,
        );
    let вездесущее: HashMap<String, HashSet<usize>> = создать_быстрый_словарь(
        &полный_словарь
            .вездесущее
            .par_iter()
            .map(|ячейка| ячейка.искомое_слово.to_string())
            .collect(),
        "вездесущие",
        &mut счётчик_входа,
    );
    let неизменное: HashMap<String, HashSet<usize>> = создать_быстрый_словарь(
        &полный_словарь
            .неизменное
            .par_iter()
            .map(|ячейка| ячейка.искомое_слово.to_string())
            .collect(),
        "неизменные",
        &mut счётчик_входа,
    );

    return lib::Куча_Словарь {
        простое: простое,
        составное: составное,
        составное_важное: составное_важное,
        вездесущее: вездесущее,
        неизменное: неизменное,
    };
}
