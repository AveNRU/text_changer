#![allow(non_ascii_idents)]

use chrono::*;

//use unirust::*;
//use std::collections::HashMap;
use std::env;
use std::time::{
    //Duration,
    Instant,
};

pub mod check_1;
pub mod dictionary_0;
pub mod import;
pub mod lib;
pub mod output;
pub mod test_0;
pub mod utils;
pub mod xlsx;
//use time::*; //{self,OffsetDateTime};
use crate::output::write;
use crate::utils::functions_add::system_pause;

fn main() {
    // Текущие дата и время
    let текущая_время_дата: DateTime<Local> = Local::now();
    let исполнение = env!("CARGO_PKG_VERSION");
    let название = env!("CARGO_PKG_DESCRIPTION");
    let разработчики = env!("CARGO_PKG_DESCRIPTION");
    println!(
        "|{}| Исполнение # {} от {}",
        название,
        исполнение,
        текущая_время_дата.format("%d-%m-%Y время: %H:%M:%S")
    );
    let mut сообщения: lib::Сообщения = Default::default();
    //подсчёт начала запуска времени
    //начало нового
    let время_отсчёта: Instant = Instant::now();
    unsafe { env::set_var("RUST_BACKTRACE", "full") };
    unsafe { env::set_var("RUSTFLAGS", "-Awarnings") };
    unsafe { env::set_var("RUSTFLAGS", "-A dead_code") };
    //проверка файлов и папок
    check_1::проверка_содержимого_папок();
    //книги
    let исходная_книга: Vec<lib::Книги> = import::read::считать_книги(&mut сообщения);
    //словари
    let пути_до_словарей: Vec<String> = import::read::считать_словари();
    //словарь со словами в виде заглвных букв и маленьких
    let mut полный_словарь: lib::Полный_Словарь =
        xlsx::import_xlsx::загрузка_словарей(&пути_до_словарей);
    //замена слов в книге
    //сама замена слов
    let выходные_книги: Vec<lib::Книги> = dictionary_0::заменить_слова_в_книге(
        &mut полный_словарь,
        исходная_книга,
        &mut сообщения,
    );
    write::сохранить_книги(&выходные_книги, &mut сообщения).unwrap();
    //время затраченное в итоге
    //вывод сообщений
    output::write::вывод_всей_стопки_сообщений_в_txt(сообщения).unwrap();
    //output времени затраченного в итоге
    println!(
        "Время занятое всего выполнения (от начала до конца): {:.2?}",
        время_отсчёта.elapsed()
    );
    system_pause();
}

/*
rzhavchina::rzhavchina! {
    внешний ящик rzhavchina;

    использовать std::collections::Словарь как Слов;

    fn главный() {
    println!("Hello, world!");
    }
}
*/

/*
use quick_xml::events::{Event, BytesText};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::fs::File;
use std::io::BufWriter;

pub fn main () {
    replace_xml_text().unwrap();
}
fn replace_xml_text() -> Result<(), Box<dyn std::error::Error>> {
    // Читаем XML файл
    let mut reader = Reader::from_file("./books/document.xml").unwrap();
    //reader.config().trim_text(true);

    // Создаем writer для выходного файла
    let output_file = File::create("output.xml").unwrap();
    let mut writer = Writer::new(BufWriter::new(output_file));

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf).unwrap() {
            Event::Text(e) => {
                let text = String::from_utf8_lossy(e.as_ref());

                // Заменяем нужный текст
                let new_text = if text.contains("очень") {
                    text.replace("очень опасное", "waffen")
                } else if text.trim() == "братва" {
                    "waffen2".to_string()
                } else {
                    text.to_string()
                };

                // Записываем измененный текст
                writer.write_event(Event::Text(BytesText::from_escaped(&new_text))).unwrap();
            }
            Event::Eof => break,
            other => writer.write_event(other).unwrap(),
        }
        buf.clear();
    }
    println!("готово");




    //Ok(())
}
*/
