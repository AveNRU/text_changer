//use std::default;
use crate::lib::{self, Полный_Словарь, Словарь, Сообщения_для_книги};
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
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use crate::utils::functions_txt::*;
use crate::utils;
use crate::utils::functions::*;
use crate::utils::stringzilla::{sz_найти, sz_упорядочить_ряд_строк};
use crate::xlsx::import_xlsx::поиск_уже_добавленных_слов;
use console::{Emoji, style};
use foldhash::{HashMap, HashSet, HashSetExt, fast::RandomState, quality::FixedState};
use indicatif::ProgressBar;
use std::time::Duration;
use stringzilla::sz;
use indicatif::*;


#[derive(Debug, Default, Clone)]
pub struct Исключения_для_кучи {
    pub указатель: usize,
    pub исключения: foldhash::HashSet<String>,
}
//изменение слов в книге
pub fn заменить_слова_в_книге(
    полный_словарь: &mut lib::Полный_Словарь, //вектор словарей
    mut книги: Vec<lib::Книги>,              //книги для изменения
    сообщения: &mut lib::Сообщения,
) -> Vec<lib::Книги> {
    //let куча_проверочная:Mutex<HashSet<String>>=Mutex::new(HashSet::default());
    //let счётчик_проверочный= AtomicU64::new(0);
    use crate::utils::stringzilla::{sz_найти};
    //шкала
    let mut временные_сообщения: Mutex<lib::Сообщения> = Mutex::new(сообщения.clone());
    let mut проверка_двойного_входа: Mutex<Vec<String>> = Mutex::new(Vec::new());
    //
    use crate::utils::regex::*;
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
    pb.set_style(ProgressStyle::default_bar()
        //.template("{spinner:.green} [{wide_bar:.cyan/blue}] {pos:>2}/{len:2} {msg}")
        .template("{msg}")
        .unwrap()
        .progress_chars("#>-"));
    
    static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
    let счётчик_составное_важное: Mutex<Vec<usize>> =
        Mutex::new(полный_словарь.счётчик_составное_важное.clone());
    let счётчик_составное: Mutex<Vec<usize>> = Mutex::new(полный_словарь.счётчик_составное.clone());
    let счётчик_простое: Mutex<Vec<usize>> = Mutex::new(полный_словарь.счётчик_простое.clone());
    let счётчик_вездесущее: Mutex<Vec<usize>> =
        Mutex::new(полный_словарь.счётчик_вездесущее.clone());
    
    //перебор
    let количество_книг = книги.len();
    книги.par_iter_mut().enumerate()
         .filter(|(главный_указатель, книга_взятая)|
             !изображение_расширение_с_точкой(&книга_взятая.путь)
         )
         .for_each(|(главный_указатель, книга_взятая)| {
    
        let расширение: String = книга_взятая.расширение.clone();
         //проверка допустимых расширений
        //если это doc, то ничего не делать
        if sz_найти(&расширение,"doc") {return}
        //остальные расширения
            //временная переменная для хранения всех строк для их сравнения в конце
            let mut вложения_изначальные: Vec<lib::Вложения> = книга_взятая.вложения.clone();
             //Вывод имени книги
             let текущий_шаг_всех_книг:Mutex<String> = Mutex::new(format!("[{}/{}]", главный_указатель + 1, количество_книг));
             println!(
                 "{}: {} {}",
                 style(текущий_шаг_всех_книг.into_inner().unwrap().clone()).strikethrough(),
                 style(&format!("{}.{}",
                     книга_взятая.название_книги,
                     книга_взятая.расширение,
                 )).cyan(),
                 LOOKING_GLASS
             );
             //счётчик файлов всех
             let счётчик_количества_вложенных_файлов:usize=книга_взятая.вложения.iter()
                 .filter(|вложение|
                 !изображение_расширение_с_точкой(&вложение.имя)&&
                     !мусорное_содержимое_архивов(&вложение.имя)
             ).count();
            //перебор всего содержимого книги
            //перебор каждого файла во вложении (в том числе zip)
            книга_взятая.вложения.par_iter_mut().enumerate()
                .filter(|(указатель, вложения)|
                                !изображение_расширение_с_точкой(&вложения.имя) &&
                    !мусорное_содержимое_архивов(&вложения.имя)
                )
                .for_each(|(указатель, вложения)| {
                    let текущий_шаг_всех_книг:Mutex<String> = Mutex::new(format!("[{}/{}]", главный_указатель + 1, количество_книг));
                    let шаг_вложенных_книг = format!("[{}/{}]", указатель + 1, счётчик_количества_вложенных_файлов);
                   
                    //вывод названия вложенного файла\
                // получение значений шагов всего для шкалы отсчёта
                    let к1= вложения.содержимое.len();
                let общее_количество =
                    полный_словарь.вездесущее.len()*к1+полный_словарь.простое.len()*к1
                +полный_словарь.составное.len()*к1+ полный_словарь.составное_важное.len()*к1;
                //получение указаталей на попуски
                //let куча_пропусков:HashSet<usize>=HashSet::default();

                let куча_пропусков: HashSet<usize> = utils::hash::проверка_содержимого_в_зависимости_от_расширения_книги(
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
                                                           style(текущий_шаг_всех_книг.into_inner().unwrap()).strikethrough(),
                                                           style(&книга_взятая.название_книги).green(),
                                                           style(&книга_взятая.расширение).green(),
                                                           style(шаг_вложенных_книг).strikethrough(),
                                                           style(&вложения.имя).yellow(),
                                                           LOOKING_GLASS
                    );
                    //let сообщение_текущее_вложение=format!("{}",&вложения.имя);
                //сначала меняются 1)составные (в 1 очередь), 2)вездесущие; 3)сложные слова 4)простые
                //составные важные
                //замена слов
                замена_слов_через_кучу(
                    &полный_словарь.составное_важное,
                    &mut вложения.содержимое,
                    &mut счётчик_составное_важное.lock().unwrap(),
                    //"[1/4] Составные важные слова",
                    &format!("{} | [1/4] Составные важные слова",сообщение_текущее_вложение),
                    &расширение,
                    &куча_пропусков,
                    &куча_словарь.составное_важное,
                    //    &mut pb,
                );
                //вездесущие
                //замена слов
                замена_слов_через_кучу(
                    &полный_словарь.вездесущее,
                    &mut вложения.содержимое,
                    &mut счётчик_вездесущее.lock().unwrap(),
                    //"[2/4] Вездесущие слова",
                    &format!("{} | [2/4] Вездесущие слова",сообщение_текущее_вложение),
                    &расширение,
                    &куча_пропусков,
                    &куча_словарь.вездесущее,
                    //  &mut pb,
                );
                //составные
                //замена слов
                замена_слов_через_кучу(
                    &полный_словарь.составное,
                    &mut вложения.содержимое,
                    &mut счётчик_составное.lock().unwrap(),
                    //"[3/4] Составные  слова",
                    &format!("{} | [3/4] Составные  слова",сообщение_текущее_вложение),
                    &расширение,
                    &куча_пропусков,
                    &куча_словарь.составное,
                    //  &mut pb,
                );
                    
                //println!("ВЛожение: {}",вложения.имя);
                //замена слов
                замена_слов_через_кучу(
                    &полный_словарь.простое,
                    &mut вложения.содержимое,
                    &mut счётчик_простое.lock().unwrap(),
                    //"[4/4] Простые слова",
                    &format!("{} | [4/4] Простые слова",сообщение_текущее_вложение),
                    &расширение,
                    &куча_пропусков,
                    &куча_словарь.простое,
                    //      &mut pb,
                );
                //   pb.finish_with_message("Готово!");
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
            
            
    });
    //output общего словаря
    полный_словарь.счётчик_составное_важное = счётчик_составное_важное.into_inner().unwrap();
    полный_словарь.счётчик_составное = счётчик_составное.into_inner().unwrap();
    полный_словарь.счётчик_вездесущее = счётчик_вездесущее.into_inner().unwrap();
    полный_словарь.счётчик_простое = счётчик_простое.into_inner().unwrap();
    //вывод словаря

    write::вывод_всех_словарей_в_xls(&полный_словарь).unwrap();
    println!(
        "Время занятое на замену слов: {:.2?}",
        точка_отсчёта_по_времени.elapsed()
    );
    println!();
    *сообщения = временные_сообщения.into_inner().unwrap();
    return книги;

    fn проверка_есть_ли_изменения(
        содержимое_изначальное: &Vec<lib::Вложения>,
        содержимое_изменённое: &Vec<lib::Вложения>,
        имя_книги: &String,
        условие: bool, //выводить на экран или нет
    )->Vec<String> {
        use rayon::prelude::*;
        //let шаг_внутренний = AtomicU64::new(0);
        //создаём ряд пустой
        let mut сообщения: Mutex<Vec<String>> = Mutex::new(vec!["".to_string();содержимое_изначальное.len()]);

        /*for указатель in 0..содержимое_изначальное.len() {
            if изображение_расширение_с_точкой(
                &содержимое_изначальное[указатель].имя,
            ) || мусорное_содержимое_архивов(
                &содержимое_изначальное[указатель].имя,
            ) {
                continue;
            }
        }*/
            содержимое_изначальное.par_iter().enumerate().filter(|(указатель,вложение)|
                !изображение_расширение_с_точкой(
                    &содержимое_изначальное[*указатель].имя,
                ) && !мусорное_содержимое_архивов(
                    &содержимое_изначальное[*указатель].имя,
                )
            ).for_each(|(указатель,вложение)|
                {
                   // шаг_внутренний.fetch_add(1, Ordering::Relaxed);
                 //   println!("{}", шаг_внутренний.load(Ordering::Relaxed));
                    if сравнение_двух_рядов_построчно(
                        &содержимое_изначальное[указатель].содержимое,
                        &содержимое_изменённое[указатель].содержимое,
                        &вложение.имя
                    ) 
                        
                    {
                        let сообщение=format!(
                            "Книга: {}|[{}/{}]| Файл: {}  замены не были произведены",
                            имя_книги,
                            указатель + 1,
                            содержимое_изначальное.len(),
                            содержимое_изначальное[указатель].имя
                        );
                        if условие {
                        
                        вывод_сообщения_на_экран_и_вложение_в_ряд_в_ячейку(
                            сообщение,
                            &mut сообщения.lock().unwrap(),
                            указатель,
                        )
                    } else {
                        сообщения.lock().unwrap()[указатель]=сообщение;
                    }
                    } else {
                        let сообщение=format!(
                            "Книга: {}|[{}/{}]| Файл: {}  были совершены замены",
                            имя_книги,
                            указатель + 1,
                            содержимое_изначальное.len(),
                            содержимое_изначальное[указатель].имя
                        );
                        if условие { 
                        вывод_сообщения_на_экран_и_вложение_в_ряд_в_ячейку(
                            сообщение,
                                &mut сообщения.lock().unwrap(),
                                указатель,
                            )
                        }else {
                            сообщения.lock().unwrap()[указатель]=сообщение;
                        }
                    }
                    
                }
            );
       
        
        let mut сообщения:Vec<String>=сообщения.into_inner().unwrap();
        сообщения.retain(|строка| !строка.is_empty());
        return сообщения
    }
   
}

//создание словаря regex
/*
pub fn добавить_все_слова_в_словарь(
    ряд_словарей: &Vec<Словарь>, //вектор словарей
) -> ПолныйСловарь {
    //итоговый словарь
    let mut полный_словарь: Mutex<ПолныйСловарь> = Mutex::new({
        lib::ПолныйСловарь {
            ..Default::default()
        }
    });
    //перебор словаря
    ряд_словарей
        .par_iter()
        .enumerate()
        .for_each(|(указатель, словарь)| {
            //вездесущие слова
            словарь
                .вездесушее
                .par_iter()
                .enumerate()
                .for_each(|(указатель_2, вездесущее_)| {
                    //for j in 0..словарь.вездесушее.len() {
                    //вложение в вектор искомых слов
                    полный_словарь
                        .lock()
                        .unwrap()
                        .re_вездесущее
                        .push(словарь.re_вездесушее[указатель_2].clone());
                    //вложение в вектор изначальных слов
                    полный_словарь
                        .lock()
                        .unwrap()
                        .вездесущее
                        .push(словарь.вездесушее[указатель_2].clone());
                    //вложение замен
                    полный_словарь
                        .lock()
                        .unwrap()
                        .замена_вездесущее
                        .push(словарь.замена_вездесушее[указатель_2].clone());
                });

            //составные слова
            словарь
                .составное
                .par_iter()
                .enumerate()
                .for_each(|(указатель_2, составное_)| {
                    //for j in 0..словарь.составное.len() {
                    //вложение в вектор искомых слов
                    полный_словарь
                        .lock()
                        .unwrap()
                        .re_составное
                        .push(словарь.re_составное[указатель_2].clone());
                    //вложение в вектор изначальных слов
                    полный_словарь
                        .lock()
                        .unwrap()
                        .составное
                        .push(словарь.составное[указатель_2].clone());
                    //вложение замен
                    полный_словарь
                        .lock()
                        .unwrap()
                        .замена_составное
                        .push(словарь.замена_составное[указатель_2].clone());
                });

            //составные слова (в 1 очередь)
            словарь.составное_важное.par_iter().enumerate().for_each(
                |(указатель_2, составное_важное_)| {
                    //for j in 0..словарь.составное_важное.len() {
                    //вложение в вектор искомых слов
                    полный_словарь
                        .lock()
                        .unwrap()
                        .re_составное_важное
                        .push(словарь.re_составное_важное[указатель_2].clone());
                    //вложение в вектор изначальных слов
                    полный_словарь
                        .lock()
                        .unwrap()
                        .составное_важное
                        .push(словарь.составное_важное[указатель_2].clone());
                    //вложение замен
                    полный_словарь
                        .lock()
                        .unwrap()
                        .замена_составное_важное
                        .push(словарь.замена_составное_важное[указатель_2].clone());
                },
            );
            //простые слова
            //перебор искомых слов под замену
            словарь
                .одиночное
                .par_iter()
                .enumerate()
                .for_each(|(указатель_2, простое_)| {
                    //for j in 0..словарь.одиночное.len() {
                    //вложение в вектор искомых слов
                    полный_словарь
                        .lock()
                        .unwrap()
                        .re_простое
                        .push(словарь.re_одиночное[указатель_2].clone());
                    //вложение в вектор изначальных слов
                    полный_словарь
                        .lock()
                        .unwrap()
                        .простое
                        .push(словарь.одиночное[указатель_2].clone());
                    //вложение замен
                    полный_словарь
                        .lock()
                        .unwrap()
                        .замена_простому
                        .push(словарь.замена_одичное[указатель_2].clone());
                });
        });
    let mut полный_словарь = полный_словарь.into_inner().unwrap();
    //установка значений замен по 0
    полный_словарь
        .счётчик_простое
        .resize(полный_словарь.простое.len(), 0);
    полный_словарь
        .счётчик_составное
        .resize(полный_словарь.составное.len(), 0);
    полный_словарь
        .счётчик_составное_важное
        .resize(полный_словарь.составное_важное.len(), 0);
    полный_словарь
        .счётчик_вездесущее
        .resize(полный_словарь.вездесущее.len(), 0);

    //поиск уже добавленных слов
    crate::xlsx::import_xlsx::поиск_уже_добавленных_слов_в_полном_словаре(
        &полный_словарь                    //номер страницы
    );
    return полный_словарь;
}

 */
//создание словаря regex
pub fn добавить_все_слова_в_словарь(
    mut ряд_словарей: Vec<Словарь>, //вектор словарей
) -> Полный_Словарь {
    use crate::xlsx::import_xlsx::поиск_уже_добавленных_слов_в_полном_словаре;
    //итоговый словарь
    let mut полный_словарь: Mutex<Полный_Словарь> = Mutex::new( Default::default() );
    //перебор словаря
    ряд_словарей.par_iter().enumerate().for_each(|(указатель,ячейка)|{
    //for i in 0..ряд_словарей.len() {
        полный_словарь.lock().unwrap().простое.extend(ячейка.простое.clone());
        полный_словарь.lock().unwrap().вездесущее.extend(ячейка.вездесущее.clone());
        полный_словарь.lock().unwrap().составное.extend(ячейка.составное.clone());
        полный_словарь.lock().unwrap().составное_важное.extend(ячейка.составное_важное.clone());
    });

    //поиск уже добавленных слов
    return поиск_уже_добавленных_слов_в_полном_словаре(
        полный_словарь                    //номер страницы
    );

}

pub fn создать_быстрый_словарь(
    слова_из_словаря: &Vec<String>,
    вид_слов: &str,
) -> HashMap<String, HashSet<usize>> {
    use crate::utils::stringzilla::sz_упорядочить_кучу;
    //let куча_пропусков:HashMap<String,Vec<usize>>=HashMap::with_hasher(foldhash::fast::RandomState::default());
    //let mut куча_простая=куча_пропусков.clone();
    let mut ряд_вывод: Mutex<Vec<String>> = Mutex::new(Vec::new());
    let словарь_куча: HashMap<String, HashSet<usize>> =
        выделить_кучу_из_ряда_для_словаря(&слова_из_словаря);
    let mut ряд_временный: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    //
    словарь_куча.par_iter().for_each(|(ключ, значения)| {
        ряд_временный.lock().unwrap().insert(ключ.to_string());
        let mut строка: Mutex<String> =
            Mutex::new(format!("ключ: |{ключ}| Значения ({}):", значения.len()));
        значения.par_iter().for_each(|значение| {
            строка
                .lock()
                .unwrap()
                .push_str(&format!("|{значение}-{значение}|"));
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
    let пути_вывода:lib::Пути_Вывода = Default::default();
    let mut пустой_ряд: Vec<String> = Vec::new();
    let путь_простой: String = format!("{}{}.txt", &пути_вывода.вывод_кучи_словаря, вид_слов,);
    let путь_ключи: String = format!("{}{}.txt", &пути_вывода.вывод_кучи_словаря_ключи, вид_слов,);
    let ряд_вывод = ряд_вывод.into_inner().unwrap();
    вывод_содержимого_в_txt(&ряд_вывод, &путь_простой, &mut пустой_ряд, false).unwrap();
    вывод_содержимого_в_txt(&ряд_временный, &путь_ключи, &mut пустой_ряд, false).unwrap();
    return словарь_куча;
}

pub fn выделить_кучу_из_ряда_для_словаря(
    ряд_слов: &Vec<String>,
) -> HashMap<String, HashSet<usize>> {
    let mut куча_пропусков: HashMap<String, HashSet<usize>> =
        HashMap::with_hasher(foldhash::fast::RandomState::default());
    //перебор ряда слов
    for i in 0..ряд_слов.len() {
        let слово: String = выделить_окончание_из_слова(&ряд_слов[i]);
        //создание пустой кучи
        let mut куча_usize = HashSet::with_hasher(foldhash::fast::RandomState::default());
        куча_usize.insert(i); // добавляем индекс в HashSet
        //проверка есть ли в куче
        if !куча_пропусков.contains_key(&слово) {
            куча_пропусков.insert(слово, куча_usize);
        }
        //если содержит куча ключ
        else {
            if let Some(значения) = куча_пропусков.get_mut(&слово) {
                // куча_пропусков.insert(слово, куча_usize)
                значения.insert(i);
            };
        }
    }
    return куча_пропусков;
}

pub fn выделить_окончание_из_слова(слово: &String) -> String {
    let mut куча_исключений_знак: HashSet<char> =
        HashSet::with_hasher(foldhash::fast::RandomState::default());
    куча_исключений_знак.insert('ы');
    куча_исключений_знак.insert('и');
    куча_исключений_знак.insert('а');
    куча_исключений_знак.insert('я');
    куча_исключений_знак.insert('у');
    куча_исключений_знак.insert('е');
    куча_исключений_знак.insert('ю');

    lazy_static! {
         static ref re_однобуквенные: Vec<Regex> = vec![
             Regex::new(r"(?i)о$").unwrap(),
             Regex::new(r"(?i)а$").unwrap(),
             Regex::new(r"(?i)я$").unwrap(),
             Regex::new(r"(?i)е$").unwrap(),
             Regex::new(r"(?i)ь$").unwrap(),
             Regex::new(r"(?i)ы$").unwrap(),
             Regex::new(r"(?i)и$").unwrap(),
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
         static ref re_двубуквенные_с_исключениями_замены: Vec<Regex> = vec![
                           Regex::new(r"(?i)ал$").unwrap(),//0
                                    Regex::new(r"(?i)ала$").unwrap(),//1
            Regex::new(r"(?i)ные$").unwrap(),//2
               Regex::new(r"(?i)ного$").unwrap(),//3
              Regex::new(r"(?i)ные$").unwrap(),//4
              Regex::new(r"(?i)ный$").unwrap(),//5
              Regex::new(r"(?i)ных$").unwrap(),//6
                         Regex::new(r"(?i)ких$").unwrap(),//7
             Regex::new(r"(?i)кой$").unwrap(),//8
             Regex::new(r"(?i)ость$").unwrap(),//9
                    Regex::new(r"(?i)ости$").unwrap(),//10
               Regex::new(r"(?i)остью$").unwrap(),//11
         ];
        static ref re_двубуквенные_с_исключениями_образцы: Vec<Regex> = vec![
                         //исключения
             Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})(ал)$").unwrap(),//0
             Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})(ала)$").unwrap(),//1
            Regex::new(r"(?i)нные$").unwrap(),//2
              Regex::new(r"(?i)нного$").unwrap(),//3
                    Regex::new(r"(?i)нные$").unwrap(),//4
              Regex::new(r"(?i)нный$").unwrap(),//5
                 Regex::new(r"(?i)нных$").unwrap(),//6
                         Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})ких$").unwrap(),//7
             Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})кой$").unwrap(),//8
                Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})ость$").unwrap(),//9
                           Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})ости$").unwrap(),//10
                           Regex::new(r"(?i)(?:[цкнгшщзхъфвпрлджчсмтьб]{1})остью$").unwrap(),//11
        ];

             static ref re_двубуквенные: Vec<Regex> = vec![
             //в первую очередь
           // Regex::new(r"(?i)ные$").unwrap(),
     Regex::new(r"(?i)ования$").unwrap(),
                 Regex::new(r"(?i)овать$").unwrap(),
    Regex::new(r"(?i)еям$").unwrap(),
     Regex::new(r"(?i)иев$").unwrap(),
             Regex::new(r"(?i)иал$").unwrap(),
               Regex::new(r"(?i)ием$").unwrap(),
              Regex::new(r"(?i)иум$").unwrap(),
               Regex::new(r"(?i)иумы$").unwrap(),
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
                 Regex::new(r"(?i)уете$").unwrap(),
             Regex::new(r"(?i)уем$").unwrap(),
             //

                Regex::new(r"(?i)иями$").unwrap(),

              Regex::new(r"(?i)ешь$").unwrap(),
                Regex::new(r"(?i)ишь$").unwrap(),
                Regex::new(r"(?i)ете$").unwrap(),
                Regex::new(r"(?i)ите$").unwrap(),
              Regex::new(r"(?i)ует$").unwrap(),
              Regex::new(r"(?i)ующие$").unwrap(),
              Regex::new(r"(?i)ующая$").unwrap(),
               Regex::new(r"(?i)ующий$").unwrap(),
             Regex::new(r"(?i)ующих$").unwrap(),
              Regex::new(r"(?i)уется$").unwrap(),
              Regex::new(r"(?i)уются$").unwrap(),
              Regex::new(r"(?i)\w+уют$").unwrap(),

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
              Regex::new(r"(?i)ичную$").unwrap(),
              Regex::new(r"(?i)ичных$").unwrap(),
               Regex::new(r"(?i)умя$").unwrap(),
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
               Regex::new(r"(?i)ими$").unwrap(),
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
    let mut куча_исключений_ал: HashSet<String> =
        HashSet::with_hasher(foldhash::fast::RandomState::default());
    куча_исключений_ал.insert("материал".to_string());
    куча_исключений_ал.insert("Материал".to_string());
    куча_исключений_ал.insert("Ритуал".to_string());
    куча_исключений_ал.insert("ритуал".to_string());
    куча_исключений_ал.insert("Идеал".to_string());
    куча_исключений_ал.insert("Идеал".to_string());
    let mut куча_исключений_ала: HashSet<String> =
        HashSet::with_hasher(foldhash::fast::RandomState::default());
    куча_исключений_ала.insert("ритуала".to_string());
    куча_исключений_ала.insert("Ритуала".to_string());
    куча_исключений_ала.insert("материала".to_string());
    куча_исключений_ала.insert("Материала".to_string());

    let mut исключения_двубуквенные: Vec<Исключения_для_кучи> = vec![
        Исключения_для_кучи {
            указатель: 0,
            исключения: куча_исключений_ал,
        },
        Исключения_для_кучи {
            указатель: 1,
            исключения: куча_исключений_ала,
        },
    ];
    /*
       if куча_исключений.contains(слово) {return слово.to_string()}
    */

    //
    проверка_ряда_regex(&re_двубуквенные);
    проверка_ряда_regex(&re_однобуквенные);

    //проверка
    //прогон двубуквенного ряда
    match прогон_и_замена_в_слове_через_ряд_re_c_исключениями(
        &слово,
        &re_двубуквенные_с_исключениями_образцы,
        &re_двубуквенные_с_исключениями_замены,
    ) {
        Ok(итог) => return итог,
        //перебор в однобуквенном ряде
        Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
            &слово,
            &re_двубуквенные,
        ) {
            Ok(итог) => return итог,
            Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
                &слово,
                &re_однобуквенные,
            ) {
                Ok(итог) => return итог,
                Err(()) => return слово.to_string(),
            },
        },
    }
}

pub fn прогон_и_замена_в_слове_через_ряд_re(
    слово: &String,
    re_ряд: &Vec<Regex>,
) -> Result<String, ()> {
    for re_образец in re_ряд.iter() {
        if re_образец.is_match(&слово) {
            //regex
            let замененная_строка: std::borrow::Cow<'_, str> = re_образец.replace(
                &слово, //строка, в которой происходит замена
                "",     //на что заменить
            );
            return Ok(замененная_строка.to_string());
        }
    }
    return Err(());
}

pub fn прогон_и_замена_в_слове_через_ряд_re_c_исключениями(
    слово: &String,
    re_ряд: &Vec<Regex>,
    исключения: &Vec<Regex>,
) -> Result<String, ()> {
    re_ряд
        .par_iter()
        .enumerate()
        .find_map_any(|(указатель, re_образец)| {
            //  for указатель in 0..re_ряд.len() {
            if re_образец.is_match(&слово) {
                //условие выполнения замены или нет
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
pub fn проверка_ряда_regex(ряд: &Vec<Regex>) {
    let куча: HashSet<String> = (0..ряд.len())
        .into_par_iter()
        .flat_map(|i| {
            ((i + 1)..ряд.len()).into_par_iter().filter_map(move |j| {
                if ряд[i].as_str() == ряд[j].as_str() {
                    Some(format!("есть совпадение Regex: {}", ряд[i]))
                } else {
                    None
                }
            })
        })
        .collect();

    /*let mut куча: HashSet<String> = HashSet::with_hasher(foldhash::fast::RandomState::default());
    'главный: for i in 0..ряд.len() {
        for j in i + 1..ряд.len() {
            if ряд[i].as_str() == ряд[j].as_str() {
                куча.insert(format!("есть совпадение Regex: {}", ряд[i]));
                continue 'главный;
            }
        }
    }*/
    for слово in куча.iter() {
        println!("{}", слово)
    }
}

fn получить_кучи_из_словарей(
    полный_словарь: &Полный_Словарь,
) -> lib::Куча_Словарь {
    let результаты = Mutex::new((None, None, None, None));
    rayon::scope(|s| {
        let результаты_ref = &результаты;

        s.spawn(|_| {
            let словарь =
                создать_быстрый_словарь(
                    &полный_словарь.простое.par_iter().map(
                        |ячейка|ячейка.искомое_слово.to_string()).collect()
                    , "простые");
            результаты_ref.lock().unwrap().0 = Some(словарь);
        });

        s.spawn(|_| {
            let словарь = создать_быстрый_словарь(
                &полный_словарь.составное.par_iter().map(
                    |ячейка|ячейка.искомое_слово.to_string()).collect()
                , "составные");

            результаты_ref.lock().unwrap().1 = Some(словарь);
        });

        s.spawn(|_| {
            let словарь = создать_быстрый_словарь(
                &полный_словарь.составное_важное.par_iter().map(
                    |ячейка|ячейка.искомое_слово.to_string()).collect()
                , "составные_важные");

            результаты_ref.lock().unwrap().2 = Some(словарь);
        });

        s.spawn(|_| {
            let словарь = создать_быстрый_словарь(
                &полный_словарь.вездесущее.par_iter().map(
                    |ячейка|ячейка.искомое_слово.to_string()).collect()
                , "вездесущие");

            результаты_ref.lock().unwrap().3 = Some(словарь);
        });
    });

    let результаты = результаты.into_inner().unwrap();
    return lib::Куча_Словарь {
        простое: результаты.0.unwrap(),
        составное: результаты.1.unwrap(),
        составное_важное: результаты.2.unwrap(),
        вездесущее: результаты.3.unwrap(),
    };
}
