use crate::utils::functions_add::system_pause;
//use crate::utils::regex::*;
use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use rayon::prelude::*;
use regex::Regex;
use std::sync::LazyLock;
//use std::fs::File;
//use std::thread;
//use std::time::Duration;
pub fn заменить_все_палки(
    строка: Text_Changer::Умная_Строка,
) -> Text_Changer::Умная_Строка {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\").unwrap());

    //  let mut  итог=строка.replace("\\", "/").to_string();
    let mut итог = строка.replace(r"\\", "/").to_string();
    итог = итог.replace(r"\", r"/");
    итог = RE.replace_all(&итог, "/").to_string();
    return Text_Changer::Умная_Строка::создать_значение(итог);
}
//use std::time::Duration;
pub fn заменить_все_палки_в_умной_строке(
    строка: Text_Changer::Умная_Строка,
) -> Text_Changer::Умная_Строка {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\").unwrap());

    //  let mut  итог=строка.replace("\\", "/").to_string();
    let mut итог: Text_Changer::Умная_Строка =
        Text_Changer::Умная_Строка::создать_значение(
            строка.replace(r"\\", "/"),
        );
    итог = итог.replace(r"\", r"/");
    итог = Text_Changer::Умная_Строка::создать_значение(
        RE.replace_all(&итог, "/"),
    );
    return Text_Changer::Умная_Строка::создать_значение(итог);
}

//получение пути до корня со скриптом в ОС
pub fn полный_путь_до_файла() -> std::io::Result<Text_Changer::Умная_Строка> {
    use std::env;
    let путь = env::current_dir().unwrap();
    //println!("The current directory is {}", path.display());
    let полный_путь: Text_Changer::Умная_Строка =
        Text_Changer::Умная_Строка::создать_значение(
            путь.into_os_string().into_string().unwrap(),
        );
    //println!("Итог пути: {}",&s);
    Ok(полный_путь)
}

pub fn строка_удалить_utf8_концы_строк(
    ряд_байтов: &Vec<u8>,
    указатель_строки: usize,
) -> String {
    use std::io::Read;
    let строка_utf8: String = match std::str::from_utf8(&ряд_байтов) {
        Ok(строка) => строка.to_string(),
        Err(_) => {
            let mut data = DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1251))
                .build(ряд_байтов.as_slice());

            let mut содержимое = String::new();
            //let ряд_в_байтах =
            match data.read_to_string(&mut содержимое) {
                Ok(число) => число,
                Err(почему) => {
                    eprintln!("Сбой при чтении данных из файла в ОЗУ!");
                    eprintln!("Строка № {}", указатель_строки);
                    eprintln!("Используемая кодировка: WINDOWS_1251.");
                    eprintln!("Попробуйте другой вид кодировки!");
                    println!("Ошибка при преобразовании данных в UTF-8 по причине: {почему}");
                    system_pause();
                    panic!("Ошибка при преобразовании данных в UTF-8 по причине: {почему}")
                }
            };
            содержимое
        }
    };
    // remove Window new строка: "\r\n"
    строка_utf8.trim_end_matches('\r').to_string()
    //строка_utf8
}

pub fn умная_строка_удалить_utf8_концы_строк(
    ряд_байтов: &Vec<u8>,
    указатель_строки: usize,
) -> Text_Changer::Умная_Строка {
    use std::io::Read;
    let строка_utf8: Text_Changer::Умная_Строка = match std::str::from_utf8(&ряд_байтов) {
        //
        Ok(строка) => Text_Changer::Умная_Строка::создать_значение(
            строка.to_string(),
        ),
        Err(_) => {
            let mut data = DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1251))
                .build(ряд_байтов.as_slice());
            //
            let mut содержимое: String = String::new();
            //let ряд_в_байтах =
            match data.read_to_string(&mut содержимое) {
                Ok(число) => число,
                Err(почему) => {
                    eprintln!("Сбой при чтении данных из файла в ОЗУ!");
                    eprintln!("Строка № {}", указатель_строки);
                    eprintln!("Используемая кодировка: WINDOWS_1251.");
                    eprintln!("Попробуйте другой вид кодировки!");
                    println!("Ошибка при преобразовании данных в UTF-8 по причине: {почему}");
                    system_pause();
                    panic!("Ошибка при преобразовании данных в UTF-8 по причине: {почему}")
                }
            };
            Text_Changer::Умная_Строка::создать_значение(содержимое)
        }
    };
    // remove Window new строка: "\r\n"
    Text_Changer::Умная_Строка::создать_значение(
        строка_utf8.trim_end_matches('\r').to_string(),
    )
    //строка_utf8
}

pub fn строка_utf8_без_удаления_концов_строк(
    ряд_байтов: &Vec<u8>,
) -> Vec<String> {
    use std::io::Read;
    // let mut ряд_строк: Vec<String> = Vec::new();
    let строка_utf8: String = match std::str::from_utf8(&ряд_байтов) {
        Ok(строка) => строка.to_string(),
        Err(_) => {
            let mut data = DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1251))
                .build(ряд_байтов.as_slice());

            let mut содержимое = String::new();
            //let ряд_в_байтах =
            match data.read_to_string(&mut содержимое) {
                Ok(число) => число,
                Err(почему) => {
                    eprintln!("Сбой при чтении данных из файла в ОЗУ!");
                    eprintln!("Строка № ",);
                    eprintln!("Используемая кодировка: WINDOWS_1251.");
                    eprintln!("Попробуйте другой вид кодировки!");
                    println!("Ошибка при преобразовании данных в UTF-8 по причине: {почему}");
                    system_pause();
                    panic!("Ошибка при преобразовании данных в UTF-8 по причине: {почему}")
                }
            };
            содержимое
        }
    };
    // remove Window new строка: "\r\n"
    vec![строка_utf8]
}

//получение строки в виде UTF-8
pub fn шкала_проход() {
    let total = 100;
    let width = 50; // Ширина прогресс-бара в символах

    for i in 0..=total {
        let percent = (i as f32 / total as f32) * 100.0;
        let filled = (width as f32 * percent / 100.0) as usize;
        let bar = "=".repeat(filled) + &" ".repeat(width - filled);

        print!("\r[{}] {:.1}%", bar, percent);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_micros(50));
    }
    println!();
}

pub fn вывод_кучи_с_ключом_сообщения_на_экран(
    строка: &str,
    куча: &rapidhash::fast::RapidHashMap<String, usize>,
) {
    println!("{}", строка);
    куча
        .iter()
        .for_each(|(сообщение, значение)| println!("{}: {}", сообщение, значение));
}

pub fn вывод_кучи_сообщения_на_экран(
    строка: &Text_Changer::Умная_Строка,
    куча: &rapidhash::fast::RapidHashSet<String>,
) {
    println!("{}", строка);
    куча.iter().for_each(|сообщение| println!("{}", сообщение));
}

pub fn вывод_сообщения_на_экран(
    строка: Text_Changer::Умная_Строка,
    ряд_сообщений: &Vec<Text_Changer::Умная_Строка>,
) {
    println!("{}", строка);
    ряд_сообщений
        .iter()
        .for_each(|сообщение| println!("{}", сообщение));
    //вложить_строку_в_ряд_с_проверкой(&mut ряд_сообщений, &строка)
}

pub fn вывод_сообщения_на_экран_и_вложение_в_ряд(
    строка: Text_Changer::Умная_Строка,
    mut ряд_сообщений: &mut Vec<Text_Changer::Умная_Строка>,
) {
    println!("{}", строка);
    вложить_строку_в_ряд_с_проверкой(&mut ряд_сообщений, &строка)
}

pub fn вывод_сообщения_на_экран_и_вложение_в_ряд_в_ячейку(
    строка: Text_Changer::Умная_Строка,
    ряд_сообщений: &mut Vec<Text_Changer::Умная_Строка>,
    указатель: usize,
) {
    println!("{}", строка);
    ряд_сообщений[указатель] = строка;
}

pub fn вложить_строку_в_ряд_с_проверкой(
    ряд: &mut Vec<Text_Changer::Умная_Строка>,
    строка: &Text_Changer::Умная_Строка,
) {
    let куча: rapidhash::fast::RapidHashSet<&str> =
        rapidhash::fast::RapidHashSet::from_par_iter(ряд.par_iter().map(|n| n.as_str()));

    if !куча.contains(строка.as_str()) {
        ряд.push(строка.clone());
    }
}
pub fn вложить_умные_строки_в_ряд_умных_строк_с_проверкой(
    ряд: &mut Vec<Text_Changer::Умная_Строка>,
    ряд_вкладываемый: &Vec<Text_Changer::Умная_Строка>,
) {
    /*for строка in ряд_вкладываемый.iter() {
        if !ряд.par_iter().any(|n| n.as_str() == строка.as_str()) {
            ряд.push(строка.clone());
        }
    }*/
    let куча: rapidhash::fast::RapidHashSet<String> = rapidhash::fast::RapidHashSet::from_par_iter(
        ряд.par_iter().map(|строка| строка.получить_значение()),
    );
    //
    for строка in ряд_вкладываемый.iter() {
        if !куча.contains(строка.as_str()) {
            ряд.push(строка.clone());
            //куча.insert(&строка.получить_значение());
        }
    }
}
pub fn вложить_умную_строку_в_ряд_умных_строк_с_проверкой(
    ряд: &mut Vec<Text_Changer::Умная_Строка>,
    строка: &Text_Changer::Умная_Строка,
) {
    /*if строка.не_пусто() {
        if !ряд.par_iter().any(|n| n.as_str() == строка.as_str()) {
            ряд.push(строка.clone());
        }
    }*/
    let куча: rapidhash::fast::RapidHashSet<&str> =
        rapidhash::fast::RapidHashSet::from_par_iter(ряд.par_iter().map(|n| n.as_str()));

    if !куча.contains(строка.as_str()) {
        ряд.push(строка.clone());
    }
}
pub fn не_изображение_или_мусор(
    стог_сена: &Text_Changer::Умная_Строка,
) -> bool {
    use crate::utils::regex::{
        изображение_расширение_с_точкой, мусорное_содержимое_архивов
    };
    return !изображение_расширение_с_точкой(&стог_сена)
        && !мусорное_содержимое_архивов(&стог_сена);
}
