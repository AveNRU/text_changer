//use std::default;
use crate::lib::{self, ПолныйСловарь, Словарь};
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

use crate::utils;
use crate::utils::stringzilla::{sz_упорядочить_ряд_строк,sz_найти};
use console::{Emoji, style};
use foldhash::{HashMap, HashSet, HashSetExt, fast::RandomState, quality::FixedState};
use indicatif::ProgressBar;
use std::time::Duration;
use stringzilla::sz;
use crate::xlsx::import_xlsx::поиск_уже_добавленных_слов;

#[derive(Debug, Default, Clone)]
pub struct Исключения_для_кучи {
    pub указатель: usize,
    pub исключения: foldhash::HashSet<String>,
}
//изменение слов в книге
pub fn заменить_слова_в_книге(
    склад_словарей: &Vec<Словарь>,       //вектор словарей
    mut ряд_исходных_книг:Vec<lib::Книги>, //книги для изменения
    сообщения: &mut lib::Сообщения,
) -> Vec<lib::Книги> {
    use crate::utils::stringzilla::sz_найти;
    //шкала

    //
    use crate::utils::regex::{замена_слов_через_кучу,замена_слов_через_regex};
    let точка_отсчёта_по_времени: Instant = Instant::now();
    let mut стопка_книг_измененная: Vec<lib::Книги> = Vec::new();
    let пути_общие: lib::Пути_Общие = Default::default();
    //случаи замены слов
    //создание словаря regex
    let mut полный_словарь: ПолныйСловарь =
        добавить_все_слова_в_словарь(&склад_словарей);
    //быстрый словарь
    let словарь_куча:HashMap<String, HashSet<usize>>=создать_быстрый_словарь(&полный_словарь);
    println!("Размер кучи: {}",словарь_куча.len());
    //начало замены слов
    //перебор книг
    static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
    for i in 0..ряд_исходных_книг.len() {
        let расширение:String=ряд_исходных_книг[i].расширение.clone();
        let mut указатель_захода: usize = 1;
        if ряд_исходных_книг[i].расширение.contains("doc") {
        } else {
            //временный вектор для хранения слов книг
            let mut временный_ряд_книг: Vec<lib::Вложения> = ряд_исходных_книг[i].вложения.clone();
            //перебор всего содержимого


            //сначала меняются 1)составные (в 1 очередь), 2)вездесущие; 3)сложные слова 4)простые
            for j in 0..временный_ряд_книг.len() {
                //имя книги
                let текущий_шаг_всех_книг = format!("[{}/{}]", i + 1, ряд_исходных_книг.len());
                println!(
                    "{}: {} {}",
                    style(текущий_шаг_всех_книг).strikethrough(),
                    style(&ряд_исходных_книг[i].путь).cyan(),
                    LOOKING_GLASS
                );
                // получение значений шагов всего для шкалы отсчёта
                let количество_1 = полный_словарь.re_составное_важное.len()
                    * ряд_исходных_книг[i].вложения[j].содержимое.len();
                let количество_2 =
                    полный_словарь.re_вездесущее.len() * ряд_исходных_книг[i].вложения[j].содержимое.len();
                let количество_3 =
                    полный_словарь.re_составное.len() * ряд_исходных_книг[i].вложения[j].содержимое.len();
                let количество_4 =
                    полный_словарь.re_простое.len() * ряд_исходных_книг[i].вложения[j].содержимое.len();
                let общее_количество =
                    количество_1 + количество_2 + количество_3 + количество_4;
                //получение указаталей на попуски
                //let куча_пропусков:HashSet<usize>=HashSet::default();

                let куча_пропусков:HashSet<usize>=utils::hash::проверка_содержимого_в_зависимости_от_расширения_книги(
                    & ряд_исходных_книг[i].вложения[j].содержимое,
                    &ряд_исходных_книг[i].расширение);
             
                //создание пропщенных строк
                let mut пропущенные_строки: Vec<String> = Vec::new();
                for указатель in куча_пропусков.iter() {
                    пропущенные_строки.push(ряд_исходных_книг[i].вложения[j].содержимое[*указатель].clone());
                }
                пропущенные_строки =
                    crate::utils::stringzilla::sz_упорядочить_ряд_строк(
                        пропущенные_строки,
                    );
                crate::output::dir::создать_папку_книги(
                    &ряд_исходных_книг[i].название_книги,
                    &ряд_исходных_книг[i].расширение,
                );
                let mut путь_вывода_пропусков = format!(
                    "{}{}/пропуски.txt",
                    &пути_общие.вывод_пропуски, &ряд_исходных_книг[i].название_книги
                );
                if sz_найти(&ряд_исходных_книг[i].название_книги, "index")
                {
                    путь_вывода_пропусков = format!(
                        "{}{}/пропуски_{i}.txt",
                        &пути_общие.вывод_пропуски, &ряд_исходных_книг[i].название_книги
                    );
                }
                //вывод пропущенных строк
                вывод_содержимого_в_txt(
                    &пропущенные_строки,
                    &путь_вывода_пропусков,
                    &mut сообщения.общие,
                )
                .unwrap();

                // thread::sleep(Duration::from_micros(1));
                //составные важные
                //замена слов
                замена_слов_через_regex(
                    &полный_словарь.re_составное_важное,
                    &mut ряд_исходных_книг[i].вложения[j].содержимое,
                    &полный_словарь.замена_составное_важное_нижнее,
                    &mut полный_словарь.счётчик_составное_важное,
                    &полный_словарь.составное_важное,
                    "[1/4] Составные важные слова",
                    &расширение,
                    &mut указатель_захода,
                    &куча_пропусков,
                    //    &mut pb,
                );
                //вездесущие
                //замена слов
                замена_слов_через_regex(
                    &полный_словарь.re_вездесущее,
                    &mut ряд_исходных_книг[i].вложения[j].содержимое,
                    &полный_словарь.замена_вездесущее_нижнее,
                    &mut полный_словарь.счётчик_вездесущее,
                    &полный_словарь.вездесущее,
                    "[2/4] Вездесущие слова",
                    &расширение,
                    &mut указатель_захода,
                    &куча_пропусков,
                    //  &mut pb,
                );
                //составные
                //замена слов
                замена_слов_через_regex(
                    &полный_словарь.re_составное,
                    &mut ряд_исходных_книг[i].вложения[j].содержимое,
                    &полный_словарь.замена_составное_нижнее,
                    &mut полный_словарь.составное_счётчик_замен,
                    &полный_словарь.составное,
                    "[3/4] Составные  слова",
                    &расширение,
                    &mut указатель_захода,
                    &куча_пропусков,
                    //  &mut pb,
                );

                //замена слов
                замена_слов_через_regex(
                    &полный_словарь.re_простое,
                    &mut ряд_исходных_книг[i].вложения[j].содержимое,
                    &полный_словарь.замена_простому_нижнее,
                    &mut полный_словарь.простое_счётчик_замен,
                    &полный_словарь.простое,
                    "[4/4] Простые слова",
                    &расширение,
                    &mut указатель_захода,
                    &куча_пропусков,
                    //&словарь_куча,
                    //      &mut pb,
                );
                //   pb.finish_with_message("Готово!");
            }
            // println!("{}",временный_ряд_книг[0].содержимое[1]);
            // if временный_ряд_книг[0].содержимое==ряд_исходных_книг[i].вложения[0].содержимое {println!("совпадают книги")  }
            let архив: HashMap<String, Vec<u8>> =
                HashMap::with_hasher(foldhash::fast::RandomState::default());
            //вложение книги в стопку новую
           /* let временная_книга = lib::Книги {
                архив,
                вложения: выходной_ряд_книг.to_vec(),
                путь: ряд_исходных_книг[i].путь.clone(),
                название_книги: ряд_исходных_книг[i].название_книги.clone(),
                расширение: ряд_исходных_книг[i].расширение.clone(),
            };*/
            //вложение в общую стопку
            //стопка_книг_измененная.push(временная_книга);
        }
    }
    //output общего словаря

    write::вывод_всех_словарей_в_xls(&полный_словарь).unwrap();
    println!(
        "Время занятое на замену слов: {:.2?}",
        точка_отсчёта_по_времени.elapsed()
    );
    println!();
    return ряд_исходных_книг;
}

//создание словаря regex
pub fn добавить_все_слова_в_словарь(
    ряд_словарей: &Vec<Словарь>, //вектор словарей
) -> ПолныйСловарь {
    //итоговый словарь
    let mut полный_словарь: ПолныйСловарь = { Default::default() };
    //перебор словаря
    for i in 0..ряд_словарей.len() {
        //вездесущие слова
        for j in 0..ряд_словарей[i].вездесушее.len() {
            //вложение в вектор искомых слов
            полный_словарь
                .re_вездесущее
                .push(ряд_словарей[i].re_вездесушее[j].clone());
            //вложение в вектор изначальных слов
            полный_словарь
                .вездесущее
                .push(ряд_словарей[i].вездесушее[j].clone());
            //вложение замен
            полный_словарь
                .замена_вездесущее_нижнее
                .push(ряд_словарей[i].замена_вездесушее[j].clone());
        }

        //составные слова
        for j in 0..ряд_словарей[i].составное.len() {
            //вложение в вектор искомых слов
            полный_словарь
                .re_составное
                .push(ряд_словарей[i].re_составное[j].clone());
            //вложение в вектор изначальных слов
            полный_словарь
                .составное
                .push(ряд_словарей[i].составное[j].clone());
            //вложение замен
            полный_словарь
                .замена_составное_нижнее
                .push(ряд_словарей[i].замена_составное[j].clone());
        }

        //составные слова (в 1 очередь)
        for j in 0..ряд_словарей[i].составное_важное.len() {
            //вложение в вектор искомых слов
            полный_словарь
                .re_составное_важное
                .push(ряд_словарей[i].re_составное_важное[j].clone());
            //вложение в вектор изначальных слов
            полный_словарь
                .составное_важное
                .push(ряд_словарей[i].составное_важное[j].clone());
            //вложение замен
            полный_словарь
                .замена_составное_важное_нижнее
                .push(ряд_словарей[i].замена_составное_важное[j].clone());
        }
        //простые слова
        //перебор искомых слов под замену
        for j in 0..ряд_словарей[i].одиночное.len() {
            //вложение в вектор искомых слов
            полный_словарь
                .re_простое
                .push(ряд_словарей[i].re_одиночное[j].clone());
            //вложение в вектор изначальных слов
            полный_словарь
                .простое
                .push(ряд_словарей[i].одиночное[j].clone());
            //вложение замен
            полный_словарь
                .замена_простому_нижнее
                .push(ряд_словарей[i].замена_одичное[j].clone());
        }
    }
    //установка значений замен по 0
    полный_словарь
        .простое_счётчик_замен
        .resize(полный_словарь.простое.len(), 0);
    полный_словарь
        .составное_счётчик_замен
        .resize(полный_словарь.составное.len(), 0);
    полный_словарь
        .счётчик_составное_важное
        .resize(полный_словарь.составное_важное.len(), 0);
    полный_словарь
        .счётчик_вездесущее
        .resize(полный_словарь.вездесущее.len(), 0);


    //поиск уже добавленных слов
    crate::xlsx::import_xlsx::поиск_уже_добавленных_слов_в_полном_словаре(
        &полный_словарь                     //номер страницы
    );
    return полный_словарь;
}

pub fn создать_быстрый_словарь(
    полный_словарь: &ПолныйСловарь
)->HashMap<String, HashSet<usize>> {
    //let куча_пропусков:HashMap<String,Vec<usize>>=HashMap::with_hasher(foldhash::fast::RandomState::default());
    //let mut куча_простая=куча_пропусков.clone();
    let mut ряд_вывод: Vec<String> = Vec::new();
    let словарь_куча: HashMap<String, HashSet<usize>> =
        выделить_кучу_из_ряда_для_словаря(&полный_словарь.простое);
    let mut ряд_временный: Vec<String> = Vec::new();
    //
    for (ключ, значения) in словарь_куча.iter() {
        ряд_временный.push(ключ.to_string());
        let mut строка = String::new();
        строка = format!("ключ: |{ключ}| Значения ({}):", значения.len());
        for значение in значения.iter() {
            строка = format!("{строка}|{значение}-{}|", полный_словарь.простое[*значение]);
        }
        ряд_вывод.push(строка);
    }
    let ряд_временный=sz_упорядочить_ряд_строк(ряд_временный);
    //
    let пути_общие: lib::Пути_Общие = Default::default();
    let mut пустой_ряд: Vec<String> = Vec::new();
    вывод_содержимого_в_txt(
        &ряд_вывод,
        &пути_общие.вывод_кучи_словаря,
        &mut пустой_ряд,
    )
    .unwrap();
    вывод_содержимого_в_txt(
        &ряд_временный,
        &пути_общие.вывод_кучи_словаря_ключи,
        &mut пустой_ряд,
    )
        .unwrap();
        return словарь_куча
    
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
    match прогон_и_замена_в_слове_через_ряд_re_c_исключениями(&слово,&re_двубуквенные_с_исключениями_образцы,&re_двубуквенные_с_исключениями_замены) {
        Ok(итог) => return итог,
        //перебор в однобуквенном ряде
        Err(()) =>
            match прогон_и_замена_в_слове_через_ряд_re(&слово, &re_двубуквенные)
            {
                Ok(итог) => return итог,
                Err(()) => match прогон_и_замена_в_слове_через_ряд_re(
                    &слово,
                    &re_однобуквенные,
                ) {
                    Ok(итог) => return итог,
                    Err(()) => return слово.to_string(),
                },
            }
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
    re_ряд.par_iter().enumerate().find_map_any(
        |(указатель,re_образец)| {
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
    }).map(Ok)
        .unwrap_or(Err(()))
}
pub fn проверка_ряда_regex(ряд: &Vec<Regex>) {

    let куча: HashSet<String> = (0..ряд.len())
        .into_par_iter()
        .flat_map(|i| {
            ((i + 1)..ряд.len())
                .into_par_iter()
                .filter_map(move |j| {
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
