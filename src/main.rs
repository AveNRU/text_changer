#![allow(non_ascii_idents)]
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use chrono::*;
//use unirust::*;
//use std::collections::HashMap;
use std::env;
use std::time::{
    //Duration,
    Instant,
};
use std::alloc;
use cap::Cap;
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
#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());
fn main() {

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    unsafe { env::set_var("RUST_BACKTRACE", "full") };
    unsafe { env::set_var("RUSTFLAGS", "-Awarnings") };
    unsafe { env::set_var("RUSTFLAGS", "-A dead_code") };
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

    //проверка файлов и папок
    check_1::проверка_содержимого_папок();
    //книги
    println!("Выделено памяти2(main)3: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    let исходная_книга: Vec<lib::Книги> = import::read::считать_книги(&mut сообщения);
    //словари
    println!("Выделено памяти(main)3: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    //словарь со словами в виде заглвных букв и маленьких
    let mut полный_словарь: lib::Полный_Словарь =
        xlsx::import_xlsx::загрузка_словарей();
    //замена слов в книге
    //сама замена слов
    println!("Выделено памяти(main)4: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    let выходные_книги: Vec<lib::Книги> = dictionary_0::заменить_слова_в_книге(
        &mut полный_словарь,
        исходная_книга,
        &mut сообщения,
    );
    println!("Выделено памяти(main)5: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    write::сохранить_книги(&выходные_книги, &mut сообщения).unwrap();
    //2write::сохранить_книги(&исходная_книга, &mut сообщения).unwrap();
    //время затраченное в итоге
    //вывод сообщений
    println!("Выделено памяти(main)6: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    write::вывод_всей_стопки_сообщений_в_txt(сообщения).unwrap();
    //output времени затраченного в итоге
    println!(
        "Время занятое всего выполнения (от начала до конца): {:.2?}",
        время_отсчёта.elapsed()
    );
    system_pause();
}
