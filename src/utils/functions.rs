use crate::utils::functions_add::system_pause;
use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::thread;
use std::time::Duration;
pub fn заменить_все_палки(строка: String) -> String {
    lazy_static! {
        static ref re: Regex = Regex::new(r"\\").unwrap();
    }
    //  let mut  итог=строка.replace("\\", "/").to_string();
    let mut итог = строка.replace(r"\\", "/").to_string();
    итог = итог.replace(r"\", r"/");
    итог = re.replace_all(&итог, "/").to_string();
    return итог;
}

//получение пути до корня со скриптом в ОС
pub fn полный_путь_до_файла() -> std::io::Result<String> {
    use std::env;
    let путь = env::current_dir().unwrap();
    //println!("The current directory is {}", path.display());
    let полный_путь = путь.into_os_string().into_string().unwrap();
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
            let ряд_в_байтах = match data.read_to_string(&mut содержимое) {
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
}

pub fn строка_utf8_без_удаления_концов_строк(
    ряд_байтов: &Vec<u8>,
) -> Vec<String> {
    use std::io::Read;
    let mut ряд_строк: Vec<String> = Vec::new();
    let строка_utf8: String = match std::str::from_utf8(&ряд_байтов) {
        Ok(строка) => строка.to_string(),
        Err(_) => {
            let mut data = DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1251))
                .build(ряд_байтов.as_slice());

            let mut содержимое = String::new();
            let ряд_в_байтах = match data.read_to_string(&mut содержимое) {
                Ok(число) => число,
                Err(почему) => {
                    eprintln!("Сбой при чтении данных из файла в ОЗУ!");
                    eprintln!("Строка № ", );
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

fn main2() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Число {i} вызвано из порожденного потока!");
            thread::sleep(Duration::from_micros(1));
        }
    });

    for i in 1..5 {
        println!("Число {i} вызвано из главного потока!");
        thread::sleep(Duration::from_micros(1));
    }
}

pub fn вывод_сообщения_на_экран_и_вложение_в_ряд(
    строка: String,
    mut ряд_сообщений: &mut Vec<String>,
) {
    println!("{}", строка);
    вложить_строку_в_ряд_с_проверкой(&mut ряд_сообщений, &строка)
}

pub fn вложить_строку_в_ряд_с_проверкой(
    ряд: &mut Vec<String>,
    строка: &String,
) {
    if !ряд.iter().any(|n| n.as_str() == строка.as_str()) {
        ряд.push(строка.clone());
    }
}
